//! observer propagation
//！事件穿透
use std::time::Duration;

use bevy::{log::LogPlugin, prelude::*, time::common_conditions::on_timer};
use rand::{Rng, rng, seq::IteratorRandom};

#[derive(Component, Deref, DerefMut)]
struct HitPoints(u16);

#[derive(Clone, Component, EntityEvent)]
#[entity_event(propagate, auto_propagate)]
struct Attack {
    entity: Entity,
    damage: u16,
}

#[derive(Component, Deref)]
struct Armor(u16);

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, LogPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            attack_armor.run_if(on_timer(Duration::from_millis(200))),
        )
        // 全局监视器用于显示收到攻击的部位
        // add_observer 全局监视器（最优先触发）
        // observe (局部) 监视器
        .add_observer(attack_hits)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn((Name::new("哥布林"), HitPoints(50)))
        .observe(take_damage)
        .with_children(|parent| {
            parent
                .spawn((Name::new("头部"), Armor(5)))
                .observe(block_attack);
            parent
                .spawn((Name::new("身体"), Armor(15)))
                .observe(block_attack);
            parent
                .spawn((Name::new("腿部"), Armor(10)))
                .observe(block_attack);
        });
}

// 子实体对伤害进行格挡，对剩余伤害进行结算
fn take_damage(
    attack: On<Attack>,
    mut hp: Query<(&mut HitPoints, &Name)>,
    mut commands: Commands,
    mut app_exit: MessageWriter<AppExit>,
) {
    let (mut hp, name) = hp.get_mut(attack.entity).unwrap();

    **hp = hp.saturating_sub(attack.damage);

    if **hp > 0 {
        info!("{} 还有 {:.1}点声明", name, hp.0);
    } else {
        warn!("💀 {} 已经死亡", name);
        commands.entity(attack.entity).despawn();
        app_exit.write(AppExit::Success);
    }
    info!("====")
}

// 更新装甲吸收后的伤害值，并控制是否继续传播
fn block_attack(mut attack: On<Attack>, query: Query<(&Armor, &Name)>) {
    let (armor, name) = query.get(attack.entity).unwrap();

    let damage = attack.damage.saturating_sub(**armor);

    if damage > 0 {
        info!("🩸 {} 收到 {} 点伤害", name, damage);
        attack.damage = damage;
    } else {
        info!("🛡️ {}点伤害被{}全部格挡", attack.damage, name);
        // 终止事件传播
        attack.propagate(false);
        info!("传播前停止")
    }
}

/// 模拟攻击 触发 Attack 事件
fn attack_armor(entitles: Query<Entity, With<Armor>>, mut commands: Commands) {
    let mut rng = rng();
    if let Some(entity) = entitles.iter().choose(&mut rng) {
        let damage = rng.random_range(0..20);
        commands.trigger(Attack { damage, entity });
        info!("⚔️  造成  {} 点伤害", damage)
    }
}

fn attack_hits(attack: On<Attack>, name: Query<&Name>) {
    if let Ok(name) = name.get(attack.entity) {
        info!("{} 被击中", name)
    } else {
        info!("没有被击中")
    }
}
