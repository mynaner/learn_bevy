/*
 * @Date: 2025-12-12 10:37:35
 * @LastEditors: myclooe 994386508@qq.com
 * @LastEditTime: 2025-12-15 11:12:21
 * @FilePath: /blibli_bevy2/examples/ch23_run_conditions.rs
 */
use bevy::prelude::*;

#[derive(Resource, Default)]
struct InputCounter(usize);

#[derive(Resource)]
struct Unused;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<InputCounter>()
        .add_systems(
            Update,
            (
                increment_input_counter
                    // 检测到 InputCounter 存在就运行
                    .run_if(resource_exists::<InputCounter>)
                    // or 满足两个中的一个就运行(这里不会短路求值，所以Unused存在has_user_input也会执行出结果 )
                    .run_if(resource_exists::<Unused>.or(has_user_input)),
                // 显式的定义一个闭包
                print_input_counter.run_if(
                    resource_exists::<InputCounter>.and(|counter: Res<InputCounter>| {
                        counter.is_changed() && !counter.is_added()
                    }),
                ),
                print_time_message
                    // 除了可以直接返回 true/false ,还可以返回一个闭包
                    .run_if(time_passed(2.0))
                    // not 是common_conditions 模块中的一个函数，用于取反
                    .run_if(not(time_passed(2.5))),
            ),
        )
        .run();
}

fn increment_input_counter(mut counter: ResMut<InputCounter>) {
    counter.0 += 1;
}

/// 这个函数式一个自定义的运行条件
/// 可以像普通system 一样，获得正常的系统参数，但只能是只读的（除了本地参数可以是可变的Local<T>）
fn has_user_input(
    // 键盘输入
    keyboard_input: Res<ButtonInput<KeyCode>>,
    // 鼠标输入
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    // 触摸输入
    touch_input: Res<Touches>,
) -> bool {
    keyboard_input.just_pressed(KeyCode::Space)
        || keyboard_input.just_pressed(KeyCode::Enter)
        || mouse_button_input.just_pressed(MouseButton::Left)
        || mouse_button_input.just_pressed(MouseButton::Right)
        || touch_input.any_just_pressed()
}

fn print_input_counter(counter: Res<InputCounter>) {
    println!("inputCounter :{}", counter.0)
}

fn print_time_message() {
    println!("自程序启动以来已经过去了超过 2 秒钟，而此时距离启动还不到 2.5 秒钟。")
}

// 返回一个符合run_if 条件的闭包
// 这样会使运行条件更加灵活
fn time_passed(t: f32) -> impl FnMut(Local<f32>, Res<Time>) -> bool {
    move |mut timer: Local<f32>, time: Res<Time>| {
        *timer += time.delta_secs();
        *timer >= t
    }
}
