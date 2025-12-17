/*
 * @Date: 2025-12-15 10:23:40
 * @LastEditors: myclooe 994386508@qq.com
 * @LastEditTime: 2025-12-15 11:30:02
 * @FilePath: /blibli_bevy2/examples/ch26_fixed_timestep.rs
 */
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, frame_update)
        .add_systems(FixedUpdate, fixed_update)
        .insert_resource(Time::<Fixed>::from_seconds(0.5))
        .run();
}

// 使用 system 关联的 Local 局部存储计算出来的 delta 和 delta_secs() 相差无几
// Local<T> 是一种 system 独有的局部变量，与每个system 关联,T where Default
// 会在第一次使用钱被初始化
fn frame_update(mut last_time: Local<f32>, time: Res<Time>) {
    info!(
        "自上次帧更新以来的时间:{}/{:?}",
        time.elapsed_secs() - *last_time,
        time.delta_secs()
    );
    *last_time = time.elapsed_secs();
}

fn fixed_update(mut last_time: Local<f32>, time: Res<Time>, fixed_time: Res<Time<Fixed>>) {
    // 默认情况下，“Time”类型为“Time<Fixed>”类型
    info!(
        "自上次 fixed_update 更新以来的时长:{}/{:?}\n",
        time.elapsed_secs() - *last_time,
        time.delta_secs()
    );

    info!("固定时间间隔:{}\n", time.delta_secs());

    // 如果我们想要查看超时情况，就需要专门访问 `Time<Fixed>` 这个对象。
    // 这里输出溢出的delta
    info!(
        "时间累积到下一个fixed_update 的时间,{}\n",
        fixed_time.overstep().as_secs_f32()
    );
    *last_time = time.elapsed_secs();
}
