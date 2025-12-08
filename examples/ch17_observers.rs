/*
 * @Date: 2025-11-18 23:01:39
 * @LastEditors: myclooe 994386508@qq.com
 * @LastEditTime: 2025-11-25 18:16:32
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

const CELL_SIZE: f32 = 64.0;

impl SpatialIndex {
    fn get_nearby(&self, pos: Vec2) -> Vec<Entity> {
        // 将世界分解为64*64像素的方格
        // 获取pos在这写方格的世界中的坐标
        let tite = (
            (pos.x / CELL_SIZE).floor() as i32,
            (pos.y / CELL_SIZE).floor() as i32,
        );

        let mut nearby = Vec::new();

        for x in -1..2 {
            for y in -1..2 {
                info!("map get:  {:?}", self.map.get(&(tite.0 + x, tite.1 + y)));
                // 查询3*3个的格子 mines 是一个集合Set
                if let Some(mines) = self.map.get(&(tite.0 + x, tite.1 + y)) {
                    nearby.extend(mines.iter());
                }
            }
        }
        info!("tite: {:?} ", tite);
        nearby
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<SpatialIndex>()
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_shapes, handle_click))
        // 观察ExplodeMine事件，如果被触发执行改系统
        .add_observer(
            |explode_mines: On<ExplodeMine>,
             mines: Query<&Mine>,
             index: Res<SpatialIndex>,
             mut commands: Commands| {
                // 查询 ExplodeMine事件触发的位置是否是一个Entity
                for entity in index.get_nearby(explode_mines.pos) {
                    // 在所有 mines 中使用entity查到 对应的mine;
                    let mine = mines.get(entity).unwrap();
                    // 计算两个圆是否相交，如果相交则去触发 Explode 事件
                    if mine.pos.distance(explode_mines.pos) < mine.size + explode_mines.radius {
                        commands.trigger(Explode { entity });
                    }
                }
            },
        )
        .add_observer(on_add_mine)
        .add_observer(on_remove_mine)
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

fn explode_mine(explode: On<Explode>, query: Query<&Mine>, mut commands: Commands) {
    let Ok(mut entity) = commands.get_entity(explode.entity) else {
        return;
    };

    info!("Bom! {} exploded.", explode.entity);

    entity.despawn();

    // 触发连环爆炸
    let mine = query.get(explode.entity).unwrap();
    commands.trigger(ExplodeMine {
        pos: mine.pos,
        radius: mine.size,
    });
}

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
    window: Query<&Window>,
    mut commands: Commands,
) {
    //
    let Ok(windows) = window.single() else {
        return;
    };

    // 解构 Single
    let (camera, camera_transform) = *camera;

    if let Some(pos) = windows
        // 光标位置(鼠标在窗口内返回像素坐标)
        .cursor_position()
         // 鼠标的像素坐标转换为世界坐标
        .and_then(|cursor|camera.viewport_to_world(camera_transform, cursor).ok())
        // 3d坐标移除z轴就转换为2d坐标了 
        .map(|ray|ray.origin.truncate())
        // 鼠标是否点击
        && mouse_button_input.just_pressed(MouseButton::Left)
    {
        // 触发事件
        commands.trigger(ExplodeMine { pos, radius: 1.0 });
    }
}

// 每当Mine 被加入的时候执行， 将对应的网格布局 HashMap<网格坐标，Entity> 加入到SpatialIndex资源中
fn on_add_mine(add: On<Add, Mine>, query: Query<&Mine>, mut index: ResMut<SpatialIndex>) {
    let mine = query.get(add.entity).unwrap();
    // pos 转换为网格坐标
    let tile = (
        (mine.pos.x / CELL_SIZE).floor() as i32,
        (mine.pos.y / CELL_SIZE).floor() as i32,
    );
    // 加入到资源中
    index.map.entry(tile).or_default().insert(add.entity);
}

fn on_remove_mine(remove: On<Remove, Mine>, query: Query<&Mine>, mut index: ResMut<SpatialIndex>) {
    let mine = query.get(remove.entity).unwrap();
    let tile = (
        (mine.pos.x / CELL_SIZE).floor() as i32,
        (mine.pos.y / CELL_SIZE).floor() as i32,
    );

    index.map.entry(tile).and_modify(|set| {
        set.remove(&remove.entity);
    });
}
