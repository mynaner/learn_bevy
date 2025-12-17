//! 默认情况下，Bevy 系统会并行运行。除非明确指定顺序，否则它们之间的相对运行顺序是不确定的。
//!
//! 在很多情况下，这一点并不重要，甚至还是很有益处的！
//！ 假设有两个系统，一个系统写入资源 A，另一个系统写入资源 B。
//！ 通过允许它们的执行顺序是任意的，我们可以根据可用的数据贪婪地进行评估。
//！ 因为它们的数据访问是**兼容的**，所以无论它们的运行顺序如何，都不会产生**可观察到的**差异。//!
//! 但倘若我们有两个系统分别对同一数据进行修改，或者一个系统读取数据而另一个系统对其进行修改，
//！ 那么实际观察到的结果将会因评估的不确定性顺序而有所不同。
//！ 这些可观察到的冲突被称为“系统执行顺序的模糊性”。//!
//! 本示例展示了如何检测并解决（或消除）这些模糊性问题。

use bevy::{
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    prelude::*,
};

fn main() {
    App::new()
        .edit_schedule(Update, |schedule| {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Warn,
                ..default()
            });
        })
        .init_resource::<A>()
        .init_resource::<B>()
        .add_systems(
            Update,
            (
                //  因为没有指定先后顺序，可以会造成数据访问冲突
                read_a,
                writes_a,
                // 数据访问冲突可以通过明确先后顺序解决
                add_one_to_b,
                doubles_b.after(add_one_to_b),
                reads_b.after(doubles_b),
                // ambiguous_with 消除了其歧义
                // 屏蔽那些由于这些系统存在模糊性（存在冲突的访问但顺序不确定）而导致的警告和错误，这些模糊性会与集合中的其他系统产生冲突。
                read_a_and_b.ambiguous_with(add_one_to_b),
            ),
        )
        .add_plugins(DefaultPlugins)
        .run();
}

#[derive(Resource, Debug, Default)]
struct A(usize);

#[derive(Resource, Debug, Default)]
struct B(usize);

fn read_a(_a: Res<A>) {}

fn writes_a(mut a: ResMut<A>) {
    a.0 += 1
}

fn add_one_to_b(mut b: ResMut<B>) {
    b.0 = b.0.saturating_add(1);
}
fn doubles_b(mut b: ResMut<B>) {
    b.0 = b.0.saturating_mul(2);
}

fn reads_b(b: Res<B>) {
    assert!((b.0 % 2 == 0) || (b.0 == usize::MAX))
}

fn read_a_and_b(a: Res<A>, b: Res<B>) {
    if b.0 < 10 {
        info!("{},{}", a.0, b.0)
    }
}
