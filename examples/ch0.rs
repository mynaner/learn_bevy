/*
 * @Date: 2025-09-23 14:46:26
 * @LastEditors: myclooe 994386508@qq.com
 * @LastEditTime: 2025-10-16 17:18:21
 * @FilePath: /blibli_bevy2/examples/ch0.rs
 */
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup_system)
        .run();
}

fn startup_system(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: auto(),
            aspect_ratio: Some(16.0 / 9.0),
            ..default()
        },
        ImageNode {
            image: assets_server.load("image.jpeg"),

            ..Default::default()
        },
    ));
}
