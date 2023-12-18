// #![windows_subsystem = "windows"]

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Lines".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            WorldInspectorPlugin::default()
                .run_if(in_state(lines::states::RunMode::Debug)),
            ShapePlugin,
            lines::states::StatesPlugin,
            lines::projection_2d_control::Projection2dControlPlugin,
            lines::draw::DrawPlugin,
            lines::ui::UIPlugin,
        ))
        .insert_resource(Msaa::Sample8)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        // .insert_resource(WinitSettings::desktop_app())
        .register_asset_reflect::<lines::chalk::ChalkMaterial>()
        .run();
}
