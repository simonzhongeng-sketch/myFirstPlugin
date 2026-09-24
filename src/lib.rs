//! Reference external add-on, built as a `cdylib` the host loads at runtime.
//!
//! It depends only on `ocs_plugin_api` (with the `host` feature) — never on the
//! `OpenCADStudio` binary — so it demonstrates the stable contract an
//! out-of-tree plugin targets: a `PluginManifest`, a `CadModule` ribbon tab, a
//! `BuiltinPlugin` entry point, and the `export_plugin!` C-ABI export.

use ocs_plugin_api::host::{acadrust, BuiltinPlugin, HostApi};
use ocs_plugin_api::manifest::{ApiVersion, PluginManifest};
use ocs_plugin_api::ribbon::{CadModule, IconKind, ModuleEvent, RibbonGroup, RibbonItem, ToolDef};

static MANIFEST: PluginManifest = PluginManifest {
    id: "myFirstPlugin",
    name: "myFirstPlugin",
    version: env!("CARGO_PKG_VERSION"),
    description: "First custom OpenCAD plugin",
    api_version: ApiVersion::CURRENT,
    ribbon_order: 50,
    xdata_apps: &[],
    command_prefixes: &["MF_"],
};

/// Ribbon tab for the plugin.
struct MyFirstModule;

impl CadModule for MyFirstModule {
    fn id(&self) -> &'static str {
        "myFirstPlugin"
    }
    fn title(&self) -> &'static str {
        "myFirstPlugin"
    }
    fn ribbon_groups(&self) -> &[RibbonGroup] {
        static GROUPS: std::sync::OnceLock<Vec<RibbonGroup>> = std::sync::OnceLock::new();
        GROUPS.get_or_init(|| {
            vec![RibbonGroup {
                title: "Demo",
                tools: vec![RibbonItem::LargeTool(ToolDef {
                    id: "MF_RECT",
                    label: "Rect 100x200",
                    icon: IconKind::Glyph("▭"),
                    event: ModuleEvent::Command("MF_RECT".to_string()),
                })],
            }]
        })
    }
}

/// The plugin entry point handed to the host.
struct MyFirstPlugin;

impl BuiltinPlugin for MyFirstPlugin {
    fn manifest(&self) -> &'static PluginManifest {
        &MANIFEST
    }
    fn ribbon(&self) -> Box<dyn CadModule> {
        Box::new(MyFirstModule)
    }
    fn dispatch(&self, host: &mut dyn HostApi, cmd: &str) -> bool {
        match cmd {
            "MF_RECT" => {
                let mut rect = acadrust::entities::LwPolyline::new();
                rect.is_closed = true;
                rect.vertices = vec![
                    acadrust::entities::LwVertex {
                        location: acadrust::types::Vector2::new(0.0, 0.0),
                        bulge: 0.0,
                        start_width: 0.0,
                        end_width: 0.0,
                        vertex_id: 0,
                    },
                    acadrust::entities::LwVertex {
                        location: acadrust::types::Vector2::new(100.0, 0.0),
                        bulge: 0.0,
                        start_width: 0.0,
                        end_width: 0.0,
                        vertex_id: 0,
                    },
                    acadrust::entities::LwVertex {
                        location: acadrust::types::Vector2::new(100.0, 200.0),
                        bulge: 0.0,
                        start_width: 0.0,
                        end_width: 0.0,
                        vertex_id: 0,
                    },
                    acadrust::entities::LwVertex {
                        location: acadrust::types::Vector2::new(0.0, 200.0),
                        bulge: 0.0,
                        start_width: 0.0,
                        end_width: 0.0,
                        vertex_id: 0,
                    },
                ];

                rect.common.layer = "0".to_string();
                let handle = host.add_entity(acadrust::EntityType::LwPolyline(rect));
                host.bump_geometry();
                host.push_info(&format!("myFirstPlugin: added 100x200 rectangle with handle {handle}"));
                true
            }
            _ => false,
        }
    }
}

// Emit the C-ABI symbols the host loader looks for.
ocs_plugin_api::export_plugin!(MyFirstPlugin);
