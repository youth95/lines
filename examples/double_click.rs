use bevy::prelude::*;
use lines::double_click::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DoubleClickPlugin))
        .add_systems(Update, read_double_click)
        .run();
}

fn read_double_click(mut double_click_events: EventReader<DoubleClickEvent>) {
    for event in double_click_events.read() {
        info!("{:?}", event);
    }
}
