/*
 * @Date: 2025-11-12 21:45:52
 * @LastEditors: myclooe 994386508@qq.com
 * @LastEditTime: 2025-11-12 22:25:10
 * @FilePath: /blibli_bevy2/examples/ch5_system_piping.rs
 */

// Deref 自动解引用
// str::parse 将str 类型转换为需要的类型

use std::num::ParseIntError;

use bevy::{
    log::{Level, LogPlugin, info},
    prelude::*,
};

fn main() {
    App::new()
        .insert_resource(Message("42".to_string()))
        .insert_resource(OptionalWarning(Err("Got to rusty?".to_string())))
        .add_plugins(LogPlugin {
            level: Level::TRACE,
            filter: "".to_string(),
            ..default()
        })
        .add_systems(
            Update,
            // 将 parse_message_system 的返回值以管道的方式传递给 handler_system
            (
                parse_message_system.pipe(handler_system),
                data_pip_system.map(|out| info!("{out}")),
                parse_message_system.map(|out| debug!("{out:?}")),
                warning_pipe_system.map(|out| {
                    if let Err(err) = out {
                        error!("{err}");
                    }
                }),
                parse_error_message_system.map(|out| {
                    if let Err(err) = out {
                        error!("{err}");
                    }
                }),
                parse_message_system.map(drop),
            ),
        )
        .run();
}

#[derive(Resource, Deref)]
struct Message(String);

#[derive(Resource, Deref)]
struct OptionalWarning(Result<(), String>);

fn parse_message_system(message: Res<Message>) -> Result<usize, ParseIntError> {
    message.parse::<usize>()
}

fn parse_error_message_system(message: Res<Message>) -> Result<(), ParseIntError> {
    message.parse::<usize>()?;
    Ok(())
}

fn handler_system(In(result): In<Result<usize, ParseIntError>>) {
    match result {
        Ok(value) => println!("解析后的消息:{value}"),
        Err(err) => println!("遇到 error: {err:?}"),
    }
}

fn data_pip_system(message: Res<Message>) -> String {
    message.0.clone()
}

fn warning_pipe_system(message: Res<OptionalWarning>) -> Result<(), String> {
    message.0.clone()
}
