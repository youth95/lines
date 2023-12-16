// #![windows_subsystem = "windows"]

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
            ShapePlugin,
            lines::states::StatesPlugin,
            lines::projection_2d_control::Projection2dControlPlugin,
            lines::draw::DrawPlugin,
            lines::ui::UIPlugin,
        ))
        .insert_resource(Msaa::Sample8)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .register_asset_reflect::<lines::chalk::ChalkMaterial>()
        .run();
}
