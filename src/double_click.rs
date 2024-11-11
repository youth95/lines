use std::time::Duration;

use bevy::{
    app::RunFixedUpdateLoop,
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    time::run_fixed_update_schedule,
};

#[derive(Event, Debug)]
pub struct DoubleClickEvent {
    pub window: Entity,
}

pub struct DoubleClickPlugin;

impl Plugin for DoubleClickPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DoubleClickEvent>()
            .init_resource::<AfterTime>()
            .add_systems(
                RunFixedUpdateLoop,
                system.after(run_fixed_update_schedule),
            );
    }
}

const DOUBLE_DELY: Duration = Duration::from_millis(200);

pub fn on_double_click(
    mut double_click_events: EventReader<DoubleClickEvent>,
) -> bool {
    if double_click_events.is_empty() {
        false
    } else {
        double_click_events.clear();
        true
    }
}

#[derive(Resource, Default)]
struct AfterTime(Option<Time<Real>>);

fn system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut double_click_event_writer: EventWriter<DoubleClickEvent>,
    time: Res<Time<Real>>,
    mut after_time_res: ResMut<AfterTime>,
) {
    for event in mouse_button_input_events.read() {
        if event.state == ButtonState::Pressed
            && event.button == MouseButton::Left
        {
            if let AfterTime(Some(after_time)) = *after_time_res {
                if (time.elapsed() - after_time.elapsed()) < DOUBLE_DELY {
                    double_click_event_writer.send(DoubleClickEvent {
                        window: event.window,
                    });
                }
            }
            *after_time_res = AfterTime(Some(time.clone()));
        }
    }
}
