//! 如何知晓 [`Component`] 已被移除的情况，从而能够对此做出反应。
//! 当一个 [`组件`]从一个 [`实体`]中移除时，
//! 所有具有针对该 [`组件`] 的 [`移除`] 触发条件的 [`观察者`] 都会收到通知。这些观察者将在该 [`组件`] 被移除后立即被调用。

use bevy::prelude::*;

#[derive(Debug, Component)]
struct MyComponent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, remove_component)
        .add_observer(rect_on_removal)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        MyComponent,
        Sprite::from_image(asset_server.load("icon.png")),
    ));
}

fn remove_component(
    mut commands: Commands,
    query: Query<Entity, With<MyComponent>>,
    time: Res<Time>,
) {
    if time.elapsed_secs() > 2.0 {
        if let Some(entity) = query.iter().next() {
            commands.entity(entity).remove::<MyComponent>();
        }
    }
}

fn rect_on_removal(remove: On<Remove, MyComponent>, mut query: Query<&mut Sprite>) {
    if let Ok(mut sprite) = query.get_mut(remove.entity) {
        sprite.color = Color::srgb(0.5, 1., 1.);
    }
}
