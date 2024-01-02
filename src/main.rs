#![windows_subsystem = "windows"]

use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        settings::{RenderCreation, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
    winit::WinitSettings,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;

fn main() {
    let default_plugins = DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Lines".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        })
        .set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                // WARN this is a native only feature. It will not work with webgl or webgpu
                features: WgpuFeatures::POLYGON_MODE_LINE,
                ..default()
            }),
        });
    let world_inspector_plugin = WorldInspectorPlugin::default()
        .run_if(in_state(lines::states::RunMode::Debug));
    App::new()
        .add_plugins((
            default_plugins,
            WireframePlugin,
            world_inspector_plugin,
            ShapePlugin,
            lines::states::StatesPlugin,
            lines::projection_2d_control::Projection2dControlPlugin,
            lines::draw::DrawPlugin,
            lines::ui::UIPlugin,
        ))
        .insert_resource(WireframeConfig {
            global: true,
            default_color: Color::GREEN,
        })
        .insert_resource(Msaa::Sample8)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WinitSettings::desktop_app())
        .register_asset_reflect::<lines::chalk::ChalkMaterial>()
        .run();
}
