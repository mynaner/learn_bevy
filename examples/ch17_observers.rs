/*
 * @Date: 2025-11-18 23:01:39
 * @LastEditors: myclooe 994386508@qq.com
 * @LastEditTime: 2025-11-21 16:14:17
 * @FilePath: /blibli_bevy2/examples/ch17_observers.rs
 */
use bevy::{
    platform::collections::{HashMap, HashSet},
    prelude::*,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

// 地雷的组件与属性
#[derive(Component)]
struct Mine {
    pos: Vec2,
    size: f32,
}

impl Mine {
    fn random(rand: &mut ChaCha8Rng) -> Self {
        Mine {
            pos: Vec2::new(
                (rand.random::<f32>() - 0.5) * 1200.,
                (rand.random::<f32>() - 0.5) * 600.,
            ),
            size: 4.0 + rand.random::<f32>() * 16.0,
        }
    }
}

#[derive(Debug, Event)]
struct ExplodeMine {
    pos: Vec2,
    radius: f32,
}
#[derive(EntityEvent)]
struct Explode {
    entity: Entity,
}

#[derive(Debug, Resource, Default)]
struct SpatialIndex {
    map: HashMap<(i32, i32), HashSet<Entity>>,
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<SpatialIndex>()
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_shapes, handle_click))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Text::new(
            "Click on a \"Mine\" to trigger it .\n\
            when it explodes it will trigger  all overlapping mines.",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
    // 随机种子
    let mut rng = ChaCha8Rng::seed_from_u64(93124213123123);
    // 1,通过链式调用组册观察者函数
    commands.spawn(Mine::random(&mut rng)).observe(explode_mine);

    let mut observer = Observer::new(explode_mine);
    // 2 手动创建观察者实体
    for _ in 0..1000 {
        // 加入 component 得到 Entity 加入 observer
        let entity = commands.spawn(Mine::random(&mut rng)).id();
        //
        observer.watch_entity(entity);
    }

    commands.spawn(observer);
}

fn explode_mine(explode: On<Explode>, query: Query<&Mine>, mut commands: Commands) {}

// 渲染,查出所有 Mine 使用 Gizoms 进行 circle_2d 渲染
fn draw_shapes(mut gizmos: Gizmos, mines: Query<&Mine>) {
    // 共计 1001 个 Mine
    for mine in mines {
        gizmos.circle_2d(
            mine.pos,
            mine.size,
            Color::hsl((mine.size - 4.) / 16. * 360., 1.0, 0.8),
        );
    }
}

// 在整个视窗 添加一个click 事件 ,将点击事件
fn handle_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mut commands: Commands,
) {
}
