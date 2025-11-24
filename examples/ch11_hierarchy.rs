//! hierarchy 层次结构

use std::f32::consts::PI;

use bevy::{
    color::palettes::css::{BLUE, LIME},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let textrue = asset_server.load("icon.png");
    // 创建一个根节点获取 Entity
    let parent = commands
        .spawn((
            Sprite::from_image(textrue.clone()),
            Transform::from_scale(Vec3::splat(0.75)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Sprite {
                    image: textrue.clone(),
                    color: BLUE.into(),
                    ..default()
                },
                // 基于父节点进行偏移和形变 ,缩放有继承关系
                Transform::from_xyz(250., 0., 0.).with_scale(Vec3::splat(0.75)),
            ));
        })
        .id();

    let child = commands
        .spawn((
            Sprite {
                image: textrue.clone(),
                color: LIME.into(),
                ..default()
            },
            Transform::from_xyz(0.0, 250.0, 0.0).with_scale(Vec3::splat(0.75)),
        ))
        .id();

    commands.entity(parent).add_child(child);
}

fn rotate(
    mut transform_query: Query<&mut Transform, With<Sprite>>,
    //  despawn_children 删除后 Children 不会存在,无法查询
    mut parent_query: Query<(Entity, Option<&Children>), With<Sprite>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (parent, children) in &mut parent_query {
        if let Ok(mut transform) = transform_query.get_mut(parent) {
            // 父节点旋转会带动整个进行旋转
            transform.rotate_z(-PI / 2.0 * time.delta_secs());
        }

        let Some(children) = children else {
            return;
        };
        for child in children {
            if let Ok(mut transform) = transform_query.get_mut(*child) {
                transform.rotate_z(PI * time.delta_secs());
            }
        }

        if time.elapsed_secs() >= 2.0 && children.len() == 2 {
            let child = children.first().unwrap();
            commands.entity(*child).despawn();
        }

        if time.elapsed_secs() >= 4.0 {
            commands.entity(parent).despawn_children();
        }
    }
}
