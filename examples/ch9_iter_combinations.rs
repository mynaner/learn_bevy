//! combinations 组合
//! 遍历查询结果到组合

use bevy::{color::palettes::css::ORANGE_RED, math::FloatPow, prelude::*};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

const GRAVITY_CONSTANT: f32 = 0.001; // 重力常数
const NUM_BODIES: usize = 100; // 球体数量

#[derive(Component, Default)]
struct Mass(f32); // 质量

#[derive(Component, Default, Deref, DerefMut)]
struct Acceleration(Vec3); // 速度

#[derive(Component, Default, Deref, DerefMut)]
struct LastPos(Vec3);

#[derive(Component)]
struct Star;

#[derive(Bundle, Default)]
struct BodyBundle {
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    mass: Mass,
    last_pos: LastPos,
    acceleration: Acceleration,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 黑色背景
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, generate_bodies)
        // 物理计算
        .add_systems(FixedUpdate, (interact_bodies, integrate))
        .add_systems(Update, look_at_star)
        .run();
}

fn generate_bodies(
    mut commands: Commands,
    time: Res<Time<Fixed>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Sphere::new(1.0) 创建半径为1的球体参数
    // .mesh() 生成 MeshBuilder (网格构建器)
    // .ico(3) 用 ico 球算法细分3次
    // ico (0 20面 1 80面 2 320面 3 1280面 4 5120面)
    // 加入到资源中,用于后续渲染
    let mesh = meshes.add(Sphere::new(1.0).mesh().ico(3).unwrap());

    // 随机颜色范围
    let color_range = 0.5..1.0;

    let vel_range = -0.5..0.5;

    // 随机种子
    let mut rng = ChaCha8Rng::seed_from_u64(19878367467713);

    for _ in 0..NUM_BODIES {
        // 随机半径 在浮点数中 0.1..0.7 表示区间,不能直接转化为数组
        let readus: f32 = rng.random_range(0.1..0.7);
        // 体积 ∝ 半径³
        let mass_value = FloatPow::cubed(readus) * 10.;

        // 随机生成坐标
        let position = Vec3::new(
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
        )
        .normalize()
            * ops::cbrt(rng.random_range(0.2f32..1.0))
            * 15.0;

        // 创建 entity
        commands.spawn((
            BodyBundle {
                mesh: Mesh3d(mesh.clone()),
                material: MeshMaterial3d(materials.add(Color::srgb(
                    rng.random_range(color_range.clone()),
                    rng.random_range(color_range.clone()),
                    rng.random_range(color_range.clone()),
                ))),
                mass: Mass(mass_value),
                acceleration: Acceleration(Vec3::ZERO),
                last_pos: LastPos(
                    position
                        - Vec3::new(
                            rng.random_range(vel_range.clone()),
                            rng.random_range(vel_range.clone()),
                            rng.random_range(vel_range.clone()),
                        ) * time.timestep().as_secs_f32(),
                ),
            },
            Transform {
                translation: position,
                scale: Vec3::splat(readus),
                ..default()
            },
        ));
    }
    // 创建小恒星
    let star_radius = 1.1;

    commands
        .spawn((
            BodyBundle {
                mesh: Mesh3d(meshes.add(Sphere::new(1.0).mesh().ico(5).unwrap())),
                material: MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: ORANGE_RED.into(),
                    emissive: LinearRgba::from(ORANGE_RED) * 2.,
                    ..default()
                })),
                mass: Mass(500.0),
                ..default()
            },
            Transform::from_scale(Vec3::splat(star_radius)),
            Star,
        ))
        // 添加光源
        .with_child(PointLight {
            color: Color::WHITE,
            range: 100.0,
            radius: star_radius,
            ..default()
        });

    // 创建相机
    // 相机位置 Transform::from_xyz(0.0, 10.5, -30.0)
    // 相机朝向 .look_at(Vec3::ZERO, Vec3::Y) 看向ZERO 点位, 摄像头顶部朝向Y轴
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.5, -30.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn interact_bodies(mut query: Query<(&Mass, &GlobalTransform, &mut Acceleration)>) {
    let mut iter = query.iter_combinations_mut();
    // 两两组合
    while let Some(
        [
            (Mass(m1), transform1, mut acc1),
            (Mass(m2), transform2, mut acc2),
        ],
    ) = iter.fetch_next()
    {
        let delta = transform2.translation() - transform1.translation();
        let distance_sq: f32 = delta.length_squared();
        let f = GRAVITY_CONSTANT / distance_sq;
        let force_unit_mass = delta * f;
        acc1.0 += force_unit_mass * *m2;
        acc2.0 -= force_unit_mass * *m1;
    }
}

// 将影响结果进行渲染
fn integrate(time: Res<Time>, mut query: Query<(&mut Acceleration, &mut Transform, &mut LastPos)>) {
    let dt_sq = time.delta_secs() * time.delta_secs();
    for (mut acceleration, mut transform, mut last_pos) in &mut query {
        let new_pos = transform.translation * 2.0 - **last_pos + **acceleration * dt_sq;
        **acceleration = Vec3::ZERO;
        **last_pos = transform.translation;
        transform.translation = new_pos;
    }
}

// 相机会缓慢的 (lerp 插值运算) 朝向 小恒心
fn look_at_star(
    mut camera: Single<&mut Transform, (Without<Star>, With<Camera>)>,
    star: Single<&Transform, With<Star>>,
) {
    let new_rotation = camera
        .looking_at(star.translation, Vec3::Y)
        .rotation
        .lerp(camera.rotation, 2.0);
    camera.rotation = new_rotation;
}
