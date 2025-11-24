//! 使用 parallelIterator 进行并行迭代器查询
//! 并行迭代器的优势是在于大量的物理性多线程运算

use bevy::{ecs::batching::BatchingStrategy, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

// 速度
#[derive(Component, Deref)]
struct Velocity(Vec2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, spawn_system)
        .add_systems(Update, (move_system, bounce_system))
        .run();
}

fn spawn_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let texture = asset_server.load("icon.png");
    let mut rng = ChaCha8Rng::seed_from_u64(19878367467713);
    for _ in 0..1280 {
        // 获取一个随机向量
        let v = 20.0 * Vec2::new(rng.random::<f32>() - 0.5, rng.random::<f32>() - 0.5);
        commands.spawn((
            Sprite::from_image(texture.clone()),
            Transform::from_scale(Vec3::splat(0.1)),
            Velocity(v),
        ));
    }
}
// 移动
fn move_system(mut query: Query<(&mut Transform, &Velocity), With<Sprite>>) {
    query.par_iter_mut().for_each(|(mut transform, velocity)| {
        transform.translation += velocity.extend(0.0);
    });
}

// 反弹
// 当 entity 超出边界时,反弹回来
// bounce
fn bounce_system(
    window: Single<&Window>,
    mut sprites: Query<(&Transform, &mut Velocity), With<Sprite>>,
) {
    let width = window.width();
    let height = window.height();

    let left = width / -2.;
    let right = width / 2.;
    let top = height / 2.;
    let button = height / -2.;

    sprites
        .par_iter_mut()
        .batching_strategy(BatchingStrategy::fixed(32)) // 更改任务的颗粒度 N个任务合并执行
        .for_each(|(transform, mut velocity)| {
            if !(transform.translation.x > left
                && transform.translation.x < right
                && transform.translation.y < top
                && transform.translation.y > button)
            {
                velocity.0 = -velocity.0;
            }
        });
}
