use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

use crate::{
    common::{hide_window_cursor, show_window_cursor},
    states::{RunMode, ToolButton},
    cursor::Cursor,
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiMaterialPlugin::<IconsUiMaterial>::default())
            .add_systems(Startup, setup_ui)
            .add_systems(
                Update,
                show_window_cursor.run_if(
                    is_hover_tool_button_bar
                        .or_else(resource_equals(Cursor::Default))
                        .or_else(in_state(RunMode::Debug)),
                ),
            )
            .add_systems(
                Update,
                hide_window_cursor
                    .run_if(in_state(RunMode::Normal))
                    .run_if(not(is_hover_tool_button_bar))
                    .run_if(not(resource_equals(Cursor::Default))),
            )
            .add_systems(
                Update,
                (update_tool_button_background, focused_tool_by_key_code)
                    .run_if(in_state(RunMode::Normal)),
            );
    }
}

const TOOL_BUTTON_BACKGROUND: Color = Color::rgb(0.16, 0.16, 0.18);

const TOOL_BUTTON_HOVER: Color = Color::rgb(0.2, 0.2, 0.24);
const TOOL_BUTTON_FOCUS: Color = Color::rgb(0.26, 0.25, 0.41);

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
struct IconsUiMaterial {
    #[uniform(0)]
    pos: Vec2,
    #[uniform(1)]
    size: f32,

    #[texture(2)]
    #[sampler(3)]
    icons_texture: Handle<Image>,
}

impl UiMaterial for IconsUiMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/icons.wgsl".into()
    }
}

#[derive(Component)]
struct ToolButtonBar;

#[derive(Component)]
struct ToolButtonKeyCode(KeyCode);

fn setup_ui(
    mut commands: Commands,
    mut ui_materials: ResMut<Assets<IconsUiMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                ..default()
            },
            z_index: ZIndex::Global(1000),
            ..default()
        },))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(550.),
                            height: Val::Px(44.),
                            margin: UiRect::top(Val::Px(16.)),
                            padding: UiRect::horizontal(Val::Px(12.)),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: TOOL_BUTTON_BACKGROUND.into(),
                        ..default()
                    },
                    bevy::ui::Interaction::None,
                    ToolButtonBar,
                ))
                .with_children(|parent| {
                    let icons_texture = asset_server.load("images/ui.png");
                    let mut icon = |pos: Vec2| MaterialNodeBundle {
                        style: Style {
                            width: Val::Px(16.),
                            height: Val::Px(16.),
                            ..default()
                        },
                        material: ui_materials.add(IconsUiMaterial {
                            pos,
                            size: 16.0,
                            icons_texture: icons_texture.clone(),
                        }),
                        ..default()
                    };
                    struct ToolButtonConfig {
                        /** 工具类型 */
                        tool: ToolButton,
                        /** 图标纹理位置 */
                        pos: Vec2,
                        /** 按键绑定 */
                        key_code: KeyCode,
                        /** 鼠标指针 */
                        cursor: Cursor,
                    }
                    let mut tool_btn = |config: ToolButtonConfig| {
                        let ToolButtonConfig {
                            tool,
                            pos,
                            key_code,
                            cursor,
                        } = config;
                        parent
                            .spawn((
                                NodeBundle {
                                    style: Style {
                                        padding: UiRect::all(Val::Px(10.0)),
                                        ..default()
                                    },
                                    ..default()
                                },
                                bevy::ui::Interaction::None,
                                tool,
                                cursor,
                                ToolButtonKeyCode(key_code),
                            ))
                            .with_children(|parent| {
                                parent.spawn(icon(pos));
                            });
                    };
                    tool_btn(ToolButtonConfig {
                        tool: ToolButton::Cursor,
                        pos: Vec2::new(2., 0.),
                        key_code: KeyCode::Key1,
                        cursor: Cursor::Default,
                    });
                    tool_btn(ToolButtonConfig {
                        tool: ToolButton::Pen,
                        pos: Vec2::new(0., 0.),
                        key_code: KeyCode::Key2,
                        cursor: Cursor::default(),
                    });
                    tool_btn(ToolButtonConfig {
                        tool: ToolButton::Eraser,
                        pos: Vec2::new(1., 0.),
                        key_code: KeyCode::Key3,
                        cursor: Cursor::default(),
                    });
                });
        });
}

fn is_hover_tool_button_bar(
    interaction_query: Query<&Interaction, With<ToolButtonBar>>,
) -> bool {
    if let Ok(interaction) = interaction_query.get_single() {
        match interaction {
            Interaction::Pressed => true,
            Interaction::Hovered => true,
            _ => false,
        }
    } else {
        false
    }
}

fn update_tool_button_background(
    mut interaction_query: Query<(
        &Interaction,
        &mut BackgroundColor,
        &ToolButton,
    )>,
    mut next_focused_tool: ResMut<NextState<ToolButton>>,
    focused_tool: Res<State<ToolButton>>,
) {
    for (interaction, mut background_color, tool) in
        interaction_query.iter_mut()
    {
        *background_color = match interaction {
            Interaction::Pressed => {
                next_focused_tool.set(tool.clone());
                TOOL_BUTTON_FOCUS.into()
            }
            Interaction::Hovered => TOOL_BUTTON_HOVER.into(),
            Interaction::None => Color::NONE.into(),
        };

        if tool == focused_tool.get() {
            *background_color = TOOL_BUTTON_FOCUS.into();
        } else {
            *background_color = Color::NONE.into();
        }
    }
}

fn focused_tool_by_key_code(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    tool_button_query: Query<(&ToolButtonKeyCode, &ToolButton, &Cursor)>,
    mut focused_tool: ResMut<NextState<ToolButton>>,
    mut cursor_resource: ResMut<Cursor>,
) {
    for ev in keyboard_input_events.read() {
        if ev.state == ButtonState::Pressed {
            for (key_code, tool, cursor) in tool_button_query.iter() {
                if let (Some(key1), ToolButtonKeyCode(key2)) =
                    (ev.key_code, key_code)
                {
                    if key1 == *key2 {
                        focused_tool.set(tool.clone());
                        *cursor_resource = cursor.clone();
                    }
                }
            }
        }
    }
}
