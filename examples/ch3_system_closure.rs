/*
 * @Date: 2025-10-21 16:45:34
 * @LastEditors: myclooe 994386508@qq.com
 * @LastEditTime: 2025-11-12 21:32:14
 * @FilePath: /blibli_bevy2/examples/ch3_system_closure.rs
 */
use bevy::{log::LogPlugin, prelude::*};

fn main() {
    let simple_closure = || {
        info!("可以直接传入一个闭包的变量");
    };

    let complex_closure = |mut val: String| {
        move || {
            info!("可以穿入一个接收参数并返回一个闭包的闭包变量");
            val = format!("{val} - update");
        }
    };
    let outside_variable = "bar".to_string();

    App::new()
        .add_plugins(LogPlugin::default())
        .add_systems(Update, simple_closure)
        .add_systems(Update, complex_closure("传入的参数".into()))
        .add_systems(Update, || info!("直接使用闭包"))
        .add_systems(Update, move || {
            info!("转移外部变量的所有权: {outside_variable}")
        })
        .run();
}
