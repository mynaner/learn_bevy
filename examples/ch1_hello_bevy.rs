/*
 * @Date: 2025-09-23 14:46:26
 * @LastEditors: myclooe 994386508@qq.com
 * @LastEditTime: 2025-10-20 09:07:05
 * @FilePath: /blibli_bevy2/examples/ch1_hello_bevy.rs
 */
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(GeetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_plugins(DefaultPlugins)
        // chain 按前后顺序依次执行
        .add_systems(Startup, (setup_add_person, update_name).chain())
        .add_systems(Update, print_names)
        .run();
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

// 自定义时间
#[derive(Resource)]
struct GeetTimer(Timer);

fn setup_add_person(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo hume".to_string())));
    commands.spawn((Person, Name("ZaynaNieves".to_string())));
}

fn print_names(query: Query<&Name, With<Person>>, time: Res<Time>, mut timer: ResMut<GeetTimer>) {
    // 检测是否满足自定义时间,如果满足就执行里面内容

    // time.delta() 返回 Duration 上一帧到当前帧到时间差
    // timer.0.tick 通过一个 Duration 推进timer
    // just_finished 表示计时器是否完成
    // timer 会更新自己的进度与状态,需要可变
    if timer.0.tick(time.delta()).just_finished() {
        for name in query {
            println!("{}", name.0)
        }
    }
}

fn update_name(query: Query<&mut Name, With<Person>>) {
    for mut name in query {
        if name.0 == "Renzo hume" {
            name.0 = "hahahaha".to_string();
        }
    }
}
