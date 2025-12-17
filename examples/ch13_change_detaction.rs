/*
 * @Date: 2025-11-17 21:56:18
 * @LastEditors: myclooe 994386508@qq.com
 * @LastEditTime: 2025-12-15 10:07:54
 * @FilePath: /blibli_bevy2/examples/ch13_change_detaction.rs
 */
//! 被动检测component 与 resource 的变更
//!
//!
use bevy::prelude::*;
use rand::Rng;

#[derive(Debug, Component, PartialEq)]
struct MyComponent(f32);

#[derive(Debug, Resource, PartialEq)]
struct MyResource(f32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                change_component,
                change_component1,
                change_resource,
                change_detaction,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    // add
    commands.spawn(MyComponent(0.0));
    commands.insert_resource(MyResource(0.0));
}

// 更新 component
fn change_component(mut query: Query<(Entity, &mut MyComponent)>, time: Res<Time>) {
    for (entity, mut myc) in &mut query {
        if rand::rng().random_bool(0.01) {
            let new_component = MyComponent(time.elapsed_secs().round());
            info!("New value {new_component:?} entity:{entity:?}");
            myc.set_if_neq(new_component);
        }
    }
}

fn change_component1(time: Res<Time>, mut query: Query<(Entity, &mut MyComponent)>) {
    for (entity, mut myc) in &mut query {
        if rand::rng().random_bool(0.01) {
            let new_component = MyComponent(time.elapsed_secs().round());
            info!("New value {new_component:?} entity:{entity:?}");
            myc.set_if_neq(new_component);
        }
    }
}

fn change_resource(time: Res<Time>, mut my_resource: ResMut<MyResource>) {
    if rand::rng().random_bool(0.3) {
        let new_resource = MyResource(time.elapsed_secs().round());
        info!("new value:{new_resource:?}");
        my_resource.set_if_neq(new_resource);
    }
}

fn change_detaction(
    change_component: Query<Ref<MyComponent>, Changed<MyComponent>>,
    my_resource: Res<MyResource>,
) {
    for component in &change_component {
        warn!(
            "Change detected\n\t-> value:{:?}->added:{}\n\t->change:{}\n\t->change_by:{}",
            component,
            component.is_added(),
            component.is_changed(),
            component.changed_by()
        )
    }

    if my_resource.is_changed() {
        warn!(
            "Change detected\n\t-> value:{:?}->added:{}\n\t->change:{}\n\t->change_by:{}",
            my_resource,
            my_resource.is_added(),
            my_resource.is_changed(),
            my_resource.changed_by()
        )
    }
}
