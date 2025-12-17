/*
 * @Date: 2025-11-17 15:59:29
 * @LastEditors: myclooe 994386508@qq.com
 * @LastEditTime: 2025-12-15 10:07:57
 * @FilePath: /blibli_bevy2/examples/ch12_one_shot_systems.rs
 */
use bevy::{
    color::palettes::css::ORANGE,
    ecs::system::{RunSystemOnce, SystemId},
    prelude::*,
};

#[derive(Debug, Component)]
struct Callback(SystemId);

#[derive(Debug, Component)]
struct A;
#[derive(Debug, Component)]
struct B;

#[derive(Component, Debug)]
struct Triggered;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                setup_ui,
                setup_with_commands,
                // 在system中有立即执行 system_b 所以需呀setup_ui 加载完后执行
                setup_with_world.after(setup_ui),
            ),
        )
        .add_systems(Update, (trigger_system, evaluate_callback).chain())
        .run();
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands
        .spawn((
            Text::default(),
            TextLayout::new_with_justify(Justify::Center),
            Node {
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextSpan::new(
                "Press A or B to trigger a one-shot system \n",
            ));
            parent.spawn(TextSpan::new("Last Triggered:"));
            parent.spawn((TextSpan::new("-"), TextColor(ORANGE.into())));
        });
}

// 将 system_a 组册到 Command 得到id
// 并将id 包装成callback component (用户自定义)
// 将Callback component 与A component 绑定到一个Entity上
// spawn
fn setup_with_commands(mut commands: Commands) {
    let id = commands.register_system(system_a);
    // command.spwan 有缓冲区,取票排队,不会立即执行
    commands.spawn((Callback(id), A));
}

// 对ui中的text进行写入
fn system_a(entity_a: Single<Entity, With<Text>>, mut writer: TextUiWriter) {
    *writer.text(*entity_a, 3) = String::from("A");
    info!("A: one shot system registered with Commands was triggered");
}

fn setup_with_world(world: &mut World) {
    // 用 run system once 执行一次
    // commands 没有 run_system_once 因为,commands 是一个缓冲区;
    // 所以默认启动时执行, Last B 高亮
    world.run_system_once(system_b).unwrap();

    let system_id = world.register_system(system_b);
    // 会立即执行
    world.spawn((Callback(system_id), B));
}

fn system_b(entity_b: Single<Entity, With<Text>>, mut writer: TextUiWriter) {
    *writer.text(*entity_b, 3) = String::from("B");
}

// 触发系统,根据输入为相应的Entity 添加Triggered Component;
// 从而使 evaludate_callback 在update中不断查询 Triggered component 并执行对应的Callback
fn trigger_system(
    mut commands: Commands,
    query_a: Single<Entity, With<A>>,
    query_b: Single<Entity, With<B>>,

    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyA) {
        commands.entity(query_a.entity()).insert(Triggered);
    }
    if input.just_pressed(KeyCode::KeyB) {
        commands.entity(query_b.entity()).insert(Triggered);
    }
}

// 执行拥有Triggered component 的entity,和callback
// 然后移除Triggered component
fn evaluate_callback(query: Query<(Entity, &Callback), With<Triggered>>, mut commands: Commands) {
    for (entity, callback) in query {
        commands.run_system(callback.0);
        commands.entity(entity).remove::<Triggered>();
    }
}
