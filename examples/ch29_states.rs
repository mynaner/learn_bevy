//! 模拟游戏状态机，包括菜单和游戏状态

use bevy::prelude::*;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
enum AppState {
    #[default]
    Menu,
    InGame,
}
#[derive(Resource)]
struct MenuData {
    button_entity: Entity,
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::Menu), setup_menu)
        .add_systems(Update, menu.run_if(in_state(AppState::Menu)))
        .add_systems(OnExit(AppState::Menu), cleanup_menu)
        .add_systems(OnEnter(AppState::InGame), setup_game)
        .add_systems(Update, (movement, change_color))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVER_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);

fn setup_menu(mut commands: Commands) {
    let button_entity = commands
        .spawn(
            // node 主要影响其子实体布局,所以如果在当前实体添加一个Text不会剧中
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        )
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.),
                        height: Val::Px(65.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_child((
                    Text("Play".to_string()),
                    TextFont {
                        font_size: 33.,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        })
        .id();

    commands.insert_resource(MenuData { button_entity });
}

fn menu(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<Button>, Changed<Interaction>),
    >,
) {
    for (interaction, mut bg) in &mut interaction_query {
        match interaction {
            Interaction::Hovered => {
                *bg = HOVER_BUTTON.into();
            }
            Interaction::Pressed => {
                *bg = PRESSED_BUTTON.into();
                next_state.set(AppState::InGame);
            }
            Interaction::None => {
                *bg = NORMAL_BUTTON.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn();
}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("icon.png");

    commands.spawn(Sprite::from_image(texture_handle));
}

fn movement(
    query: Query<&mut Transform, With<Sprite>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    const SPEED: f32 = 100.0;

    for mut transform in query {
        let mut direction = Vec3::ZERO;

        if input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.;
        }
        if input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.;
        }
        if input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.;
        }
        if input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.;
        }

        if direction != Vec3::ZERO {
            transform.translation += direction.normalize() * SPEED * time.delta_secs();
        }
    }
}

fn change_color(time: Res<Time>, query: Query<&mut Sprite>) {
    for mut sprite in query {
        let new_color = LinearRgba {
            blue: ops::sin(time.elapsed_secs() * 0.5) + 2.0,
            ..default()
        };
        sprite.color = new_color.into();
    }
}
