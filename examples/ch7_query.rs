//! 模拟一个玩家遭遇站的场景
//! 4个敌人,4种类型
//! boos 红色怪,绿色怪,隐形怪
//! 3种攻击方式
//! (B)omb 爆炸 Boss 以外都扣血
//! (L)ash 鞭打 除了Player 和隐形怪都扣血
//! (G)lare 所有敌人强制 BodyColor::White (隐身怪不在隐身)

//! system pipe
//! Query 用法 QueryData 与QueryFilter
//! Query 的延伸用法 Single 和 Populated 和 Option<Single>

use bevy::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
enum QueryError {
    #[error("none")]
    None,
}
#[derive(Debug, Resource)]
struct NotifyPlayerTimer(Timer);

#[derive(Resource, Debug)]
struct NotifyEnemiesTimer(Timer);

// 玩家
#[derive(Component)]
struct Player;

// boss
#[derive(Component, Debug)]
struct Boss;

// 敌人
#[derive(Component, Debug)]
struct Enemy;

// 生命值
#[derive(Component, Deref, DerefMut, Debug)]
struct Health(i64);

// 护甲
#[derive(Component, Debug)]
struct Armor;

// 颜色(是否隐身)
#[derive(Component, Debug)]
enum BodyColor {
    Red,
    Green,
    White,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(NotifyPlayerTimer(Timer::from_seconds(
            5.0,
            TimerMode::Repeating,
        )))
        .insert_resource(NotifyEnemiesTimer(Timer::from_seconds(
            5.0,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            choice
                .pipe(bomb_all)
                .pipe(lash_enemy)
                .pipe(glare_all)
                .map(in_choice_map),
        )
        .add_systems(Update, refresh_all)
        .add_systems(Update, (notify_player, notify_enemies).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Player, Health(100), Armor));
    commands.spawn((Enemy, Health(30), BodyColor::Green));
    commands.spawn((Enemy, Health(1)));
    commands.spawn((Boss, BodyColor::Red, Armor, Health(100)));
}

// 等待用户输入
fn choice(input: Res<ButtonInput<KeyCode>>) -> Result<KeyCode, QueryError> {
    let Some(code) = input.get_just_pressed().next() else {
        return Err(QueryError::None);
    };
    Ok(*code)
}
// 丢炸弹, 场景中的所有生物都要扣血
// 玩家和怪物受到伤害不同,需要分别处理(两个query)
fn bomb_all(
    In(key): In<Result<KeyCode, QueryError>>,
    mut player_query: Query<&mut Health, (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<&mut Health, (With<Enemy>, Without<Player>)>,
) -> Result<KeyCode, QueryError> {
    let Ok(code) = key else {
        return Err(QueryError::None);
    };

    if code == KeyCode::KeyB {
        let bomb_damage = 60;

        for mut health in player_query.iter_mut() {
            **health -= bomb_damage / 2;
        }

        for mut health in enemy_query.iter_mut() {
            **health -= bomb_damage;
        }
    }
    key
}

// # lash 除了玩家和隐身怪都扣血
//  (Without<Player>, With<BodyColor>) 这个可以过滤玩家和隐身怪
fn lash_enemy(
    In(key): In<Result<KeyCode, QueryError>>,
    mut query: Query<&mut Health, (Without<Player>, With<BodyColor>)>,
) -> Result<KeyCode, QueryError> {
    let Ok(code) = key else {
        return Err(QueryError::None);
    };

    if code == KeyCode::KeyL {
        let lash_damage = 10;

        for mut health in query.iter_mut() {
            **health -= lash_damage;
        }
    }

    key
}

//
fn glare_all(
    In(key): In<Result<KeyCode, QueryError>>,
    mut query: Query<(Option<&mut BodyColor>, Entity), Without<Player>>,
    mut commands: Commands,
) -> Result<KeyCode, QueryError> {
    let Ok(code) = key else {
        return Err(QueryError::None);
    };
    if code == KeyCode::KeyG {
        for (bg, entity) in &mut query {
            match bg {
                Some(mut bc) => {
                    *bc = BodyColor::White;
                }
                None => {
                    commands.entity(entity).insert(BodyColor::White);
                }
            }
        }
    }
    key
}

// 打印用户输入
// map 只能连接函数,不能是system
fn in_choice_map(key: Result<KeyCode, QueryError>) {
    if let Ok(code) = key {
        println!("map show: ({:?}\n\n)", code)
    }
}

// 移出 health <=0的对象
fn refresh_all(mut commands: Commands, query: Query<(Entity, &Health), With<Health>>) {
    for (entity, health) in &query {
        if **health <= 0 {
            commands.entity(entity).despawn();
        }
    }
}

// 当玩家不存在后,Single无法执行,并且会panic
fn notify_player(
    time: Res<Time>,
    mut timer: ResMut<NotifyPlayerTimer>,
    player: Option<Single<EntityRef, With<Player>>>, // 获取entity 的组件
) {
    timer.0.tick(time.delta());
    // 不足自定义的时间,就什么都不做
    if !timer.0.just_finished() {
        return;
    }
    let Some(player) = player else {
        println!("no player alive.");
        return;
    };

    println!("---------------------------");
    println!("(B)omb -60 (L)ash -10 (G)lare");

    let Some(health) = player.get::<Health>() else {
        return;
    };

    let Some(armor) = player.get::<Armor>() else {
        return;
    };

    println!(
        "player ({}): health:{:?} armor:{:?}",
        player.id(),
        health,
        armor
    );
}
fn notify_enemies(
    time: Res<Time>,
    mut timer: ResMut<NotifyEnemiesTimer>,
    enemies: Populated<
        (
            Entity,
            &Health,
            Option<&Armor>,
            Option<&BodyColor>,
            Option<&Boss>,
            Option<&Enemy>,
        ),
        Without<Player>,
    >,
) {
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }

    for (entity, health, armor, bg, boss, enemy) in enemies {
        print!("entity:{}", entity);
        print!(" ({})", **health);
        print!(" / ({:?})", armor);
        if let Some(bg) = bg {
            print!(" / {bg:?}")
        }

        if boss.is_some() {
            print!(" / boss")
        }
        if enemy.is_some() {
            print!(" / enemy")
        }
        println!()
    }
    println!()
}
