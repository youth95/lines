use bevy::{
    input::common_conditions::{
        input_just_released, input_pressed, input_toggle_active,
    },
    prelude::*,
};

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AppState {
    #[default]
    Hovering,
    Drawing,
    MovingCamera,
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum RunMode {
    #[default]
    Normal,
    Debug,
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        let run_mode_condition = || input_toggle_active(true, KeyCode::Escape);
        let to_hovering = to_state(AppState::Hovering)
            .run_if(run_mode_condition())
            .run_if(input_just_released(MouseButton::Left))
            .run_if(
                in_state(AppState::Drawing)
                    .or_else(in_state(AppState::MovingCamera)),
            );

        let to_moving_camera = to_state(AppState::MovingCamera)
            .run_if(input_pressed(MouseButton::Left))
            .run_if(input_pressed(KeyCode::Space));

        let to_drawing = to_state(AppState::Drawing)
            .run_if(input_pressed(MouseButton::Left))
            .run_if(not(input_pressed(KeyCode::Space)));

        let to_next_state_in_hovering = (to_drawing, to_moving_camera).run_if(
            run_mode_condition().and_then(in_state(AppState::Hovering)),
        );

        let to_normal_mode = to_state(RunMode::Normal)
            .run_if(input_toggle_active(true, KeyCode::Escape));

        let to_debug_mode = to_state(RunMode::Debug)
            .run_if(input_toggle_active(false, KeyCode::Escape));

        let in_normal_mode = (to_next_state_in_hovering, to_hovering)
            .run_if(in_state(RunMode::Normal));

        app.add_state::<AppState>()
            .add_state::<RunMode>()
            .add_systems(
                Update,
                (in_normal_mode, to_normal_mode, to_debug_mode),
            );
    }
}

fn to_state<S: States>(state: S) -> impl FnMut(ResMut<NextState<S>>) -> () {
    move |mut next_state: ResMut<NextState<S>>| {
        next_state.set(state.clone());
    }
}
