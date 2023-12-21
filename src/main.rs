// #![windows_subsystem = "windows"]

use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        settings::{RenderCreation, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
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
                }),
            WireframePlugin,
            WorldInspectorPlugin::default()
                .run_if(in_state(lines::states::RunMode::Debug)),
            ShapePlugin,
            lines::states::StatesPlugin,
            lines::projection_2d_control::Projection2dControlPlugin,
            lines::draw::DrawPlugin,
            lines::ui::UIPlugin,
        ))
        .insert_resource(WireframeConfig {
            // The global wireframe config enables drawing of wireframes on every mesh,
            // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
            // regardless of the global configuration.
            global: true,
            // Controls the default color of all wireframes. Used as the default color for global wireframes.
            // Can be changed per mesh using the `WireframeColor` component.
            default_color: Color::GREEN,
        })
        .insert_resource(Msaa::Sample8)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        // .insert_resource(WinitSettings::desktop_app())
        .register_asset_reflect::<lines::chalk::ChalkMaterial>()
        .run();
}
