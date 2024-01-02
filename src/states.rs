use bevy::{
    input::common_conditions::{
        input_just_released, input_pressed, input_toggle_active,
    },
    prelude::*,
};

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CursorState {
    #[default]
    Hovering,
    Draging,
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum RunMode {
    #[default]
    Normal,
    Debug,
}

#[derive(States, Component, Debug, Hash, Default, PartialEq, Eq, Clone)]
pub enum ToolButton {
    #[default]
    Pen,
    Eraser,
    Cursor,
    MoveCamera,
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        let run_mode_condition = || input_toggle_active(true, KeyCode::Escape);
        let to_hovering = to_state(CursorState::Hovering)
            .run_if(run_mode_condition())
            .run_if(input_just_released(MouseButton::Left))
            .run_if(in_state(CursorState::Draging));

        let to_drawing = to_state(CursorState::Draging)
            .run_if(input_pressed(MouseButton::Left));

        let to_next_state_in_hovering = to_drawing.run_if(
            run_mode_condition().and_then(in_state(CursorState::Hovering)),
        );

        let to_normal_mode = to_state(RunMode::Normal)
            .run_if(input_toggle_active(true, KeyCode::Escape));

        let to_debug_mode = to_state(RunMode::Debug)
            .run_if(input_toggle_active(false, KeyCode::Escape));

        let in_normal_mode = (to_next_state_in_hovering, to_hovering)
            .run_if(in_state(RunMode::Normal));

        let (push, pop) = state_stack(ToolButton::MoveCamera);
        let push = push
            .run_if(input_pressed(KeyCode::Space))
            .run_if(not(in_state(ToolButton::MoveCamera)));
        let pop = pop
            .run_if(input_just_released(KeyCode::Space))
            .run_if(in_state(ToolButton::MoveCamera));
        app.add_state::<CursorState>()
            .add_state::<RunMode>()
            .add_state::<ToolButton>()
            .init_resource::<StateStack<ToolButton>>()
            .add_systems(
                Update,
                (in_normal_mode, to_normal_mode, to_debug_mode, push, pop),
            );
    }
}

fn to_state<S: States>(state: S) -> impl FnMut(ResMut<NextState<S>>) -> () {
    move |mut next_state| {
        next_state.set(state.clone());
    }
}

#[derive(Resource, Default)]
struct StateStack<S: States>(Vec<S>);

fn state_stack<S: States>(
    state: S,
) -> (
    impl FnMut(ResMut<NextState<S>>, Res<State<S>>, ResMut<StateStack<S>>) -> (),
    impl FnMut(ResMut<NextState<S>>, ResMut<StateStack<S>>) -> (),
) {
    (
        move |mut next_state, current_state, mut stack| {
            stack.0.push(current_state.clone());
            next_state.set(state.clone());
        },
        move |mut next_state, mut stack| {
            if let Some(state) = stack.0.pop() {
                next_state.set(state);
            }
        },
    )
}
