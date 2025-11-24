//! 展示了如何发送、修改和接收消息。
//! 它还演示了如何控制系统顺序，
//! 以便按照特定顺序处理消息。
//! 它通过模拟游戏中可能存在的持续伤害效果来实现这一点。

use bevy::prelude::*;

// 试图造成伤害时发送此消息
#[derive(Message, Debug)]
struct DealDamage {
    amount: i32,
}

// 当实体受到伤害时发送此消息
#[derive(Debug, Message, Default)]
struct DamageReceived;

// 护甲完全格挡伤害时发送此消息
#[derive(Debug, Message, Default)]
struct ArmorBlockedDamage;

#[derive(Resource, Deref, DerefMut)]
struct DamageTimer(Timer);

impl Default for DamageTimer {
    fn default() -> Self {
        DamageTimer(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_message::<DealDamage>()
        .add_message::<DamageReceived>()
        .add_message::<ArmorBlockedDamage>()
        .insert_resource(DamageTimer::default())
        .add_systems(
            Update,
            (
                deal_damage_over_time,
                apply_armor_to_damage,
                apply_damage_received_sound,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                play_damage_received_sound,
                play_damage_received_particle_effect,
            ),
        )
        .run();
}

// 每隔1秒发送受到10点伤害的消息
fn deal_damage_over_time(
    time: Res<Time>,
    mut state: ResMut<DamageTimer>,
    mut deal_damage_writer: MessageWriter<DealDamage>,
) {
    if state.0.tick(time.delta()).just_finished() {
        deal_damage_writer.write(DealDamage { amount: 10 });
    }
}

// 接收收到伤寒, 并格挡
// 在发送受到伤害信息
fn apply_armor_to_damage(
    mut dmg_message: MessageMutator<DealDamage>,
    mut armor_message: MessageWriter<ArmorBlockedDamage>,
) {
    for message in dmg_message.read() {
        message.amount -= 1;
        // 完全格挡完后发送此消息
        if message.amount <= 0 {
            armor_message.write(ArmorBlockedDamage);
        }
    }
}

// 当实体受到伤害时发送此消息
fn apply_damage_received_sound(
    mut deal_damage_reader: MessageReader<DealDamage>,
    mut damage_received_writer: MessageWriter<DamageReceived>,
) {
    for message in deal_damage_reader.read() {
        if message.amount > 0 {
            damage_received_writer.write(DamageReceived);
        }
    }
}

// 读取受到伤害的消息, 发出声音
fn play_damage_received_sound(mut damage_received_reader: MessageReader<DamageReceived>) {
    for _ in damage_received_reader.read() {
        info!("Playing a sound.")
    }
}
// 读取受到伤害的消息, 生成粒子效果
fn play_damage_received_particle_effect(mut damage_received_reader: MessageReader<DamageReceived>) {
    for _ in damage_received_reader.read() {
        info!("Playing particle effect.")
    }
}
