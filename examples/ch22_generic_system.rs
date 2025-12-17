//! 泛型类型使我们能够在许多相关系统中复用逻辑，让我们能够根据传入的类型（或多个类型）来专门化函数的行为。
//! 这通常在处理相关组件或资源时很有用，在这种情况下，我们希望为查询目的拥有独特的类型，但又希望它们都能以相同的方式工作。
//! 当与用户定义的特征结合使用时，这尤其强大，可以为这些相关类型添加更多功能。请记住，对于您想要操作的每个类型，都要在调度表中插入该系统的专用副本

use bevy::prelude::*;

#[derive(Debug, States, Default, Hash, PartialEq, Eq, Clone)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
}

#[derive(Component, Deref, DerefMut)]
struct PrinterTick(Timer);

#[derive(Component)]
struct TextTpPrint(String);

#[derive(Component)]
struct MenuClose;

#[derive(Component)]
struct LevelUnload;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(Startup, setup_system)
        .add_systems(
            Update,
            (
                // 通过定义的时间打印 不同 component 中的内容
                print_text_system,
                // xx前提下执行system
                transition_to_in_game_system.run_if(in_state(AppState::MainMenu)),
            ),
        )
        // 使用范型 ，用相同逻辑的 system 处理，但是确是不同的component
        .add_systems(OnExit(AppState::MainMenu), cleanup_system::<MenuClose>)
        .add_systems(OnExit(AppState::InGame), cleanup_system::<LevelUnload>)
        .run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn((
        PrinterTick(Timer::from_seconds(1.0, TimerMode::Repeating)),
        TextTpPrint("I will print until you press space ".to_string()),
        MenuClose,
    ));

    commands.spawn((
        PrinterTick(Timer::from_seconds(1.0, TimerMode::Repeating)),
        TextTpPrint("I will always print".to_string()),
        LevelUnload,
    ));
}

fn print_text_system(mut query: Query<(&TextTpPrint, &mut PrinterTick)>, time: Res<Time>) {
    for (text, mut tick) in &mut query {
        if tick.tick(time.delta()).just_finished() {
            println!("{}", text.0);
        }
    }
}

fn transition_to_in_game_system(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        println!("Space");
        next_state.set(AppState::InGame);
    }
}

fn cleanup_system<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for e in query {
        commands.entity(e).despawn();
    }
}
