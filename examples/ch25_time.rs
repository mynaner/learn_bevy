use bevy::prelude::*;
use std::{
    io::{self, BufRead},
    time::Duration,
};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        // virtual 设置了主循环（单帧）时间的步长，会影响到 Time<Fixed>
        .insert_resource(Time::<Virtual>::from_max_delta(Duration::from_secs(5)))
        // 受主循环（单帧）时间的步长影响
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs(1)))
        .add_systems(PreUpdate, print_real_time)
        .add_systems(FixedUpdate, print_fixed_time)
        .add_systems(Update, print_time)
        // 手动设置一个call function 只会调用一次
        // 为了持续交互runner中实现了一个循环
        .set_runner(runner)
        .run();
}

// Time<Real> 真实世界时间，直接来自操作系统的时钟。
// 需要显示声明获取
fn print_real_time(time: Res<Time<Real>>) {
    println!(
        "PreUpdate: 这是实时时钟,当前耗时{:?},累计时间{:?}",
        time.delta(),
        time.elapsed()
    )
}

// Time<Fixed> 固定时间（模拟时间）
// 所有Fixed 前缀的Schedule 的Time 都是 Time<Fixed>
// 固定步长的模拟时间，主要用于稳定、可复现的逻辑。
fn print_fixed_time(time: Res<Time<Fixed>>) {
    println!(
        "FixedUpdate: 固定步长的模拟时间，主要用于稳定、可复现的逻辑。当前耗时{:?},累计时间{:?}",
        time.delta(),
        time.elapsed()
    )
}
// Time<Virtual> 虚拟时间（游戏时间）
// 除了Fixed 前缀的Schedule的time都是time<Virtual>
// 游戏世界的时间，是基于 Time<Real> 派生出来的。
fn print_time(time: Res<Time<Virtual>>) {
    println!(
        "Update: 游戏世界的时间，是基于 Time<Real> 派生出来的。 当前耗时{:?},累计时间{:?}",
        time.delta(),
        time.elapsed()
    )
}
fn banner() {
    println!("这个示例旨在直观地演示 Bevy 中 Time 的工作方式。");
    println!();
    println!("时间将在应用中的三个不同调度阶段被打印：");
    println!("- PreUpdate：打印真实时间（Time<Real>）");
    println!("- FixedUpdate：打印固定时间步长的时间，可能会执行 0 次或多次（Time<Fixed>）");
    println!("- Update：打印虚拟的游戏时间（Time<Virtual>）");
    println!();
    println!("最大 delta 时间被设置为 5 秒，Fixed 的时间步长被设置为 1 秒。");
    println!();
}

fn help() {
    println!("应用会从标准输入逐行读取命令。");
    println!();
    println!("命令说明：");
    println!("  空行：在 Bevy App 上执行一次 app.update()");
    println!("  q：退出应用");
    println!("  f：设置为快速模式，2 倍速");
    println!("  n：设置为正常速度，1 倍速");
    println!("  s：设置为慢速模式，0.5 倍速");
    println!("  p：暂停");
    println!("  u：取消暂停");
}
fn runner(mut app: App) -> AppExit {
    banner();
    help();

    let stdin = io::stdin();
    // 阻塞等待用户输入
    for line in stdin.lock().lines() {
        if let Err(err) = line {
            println!("read err:{err:#}");
            break;
        }
        // 只有用户输入才会响应

        match line.unwrap().as_str() {
            "" => {
                // 非控制字符，执行所有schedule(PreUpdate,Update,FixedUpdate，。。。)
                app.update();
            }
            // 将Time<Virtual> 的相对速度设置为2X
            "f" => {
                println!("FAST: 相对速度设置为2X");
                app.world_mut()
                    .resource_mut::<Time<Virtual>>()
                    .set_relative_speed(2.);
            }
            // 将Time<Virtual> 的相对数独设置为1x
            "n" => {
                println!("NORMAL：相对速度设置为1x");
                app.world_mut()
                    .resource_mut::<Time<Virtual>>()
                    .set_relative_speed(1.0);
            }
            "s" => {
                println!("SLOW：相对速度置为0.5x");
                app.world_mut()
                    .resource_mut::<Time<Virtual>>()
                    .set_relative_speed(0.5);
            }
            "p" => {
                println!("PAUSE: 暂停 Time<Virtual>");
                app.world_mut().resource_mut::<Time<Virtual>>().pause();
            }
            "u" => {
                println!("UNPAUSE: 恢复 Time<Virtual>");
                app.world_mut().resource_mut::<Time<Virtual>>().unpause();
            }
            "q" => {
                println!("QUITTING");
                break;
            }
            _ => {
                help();
            }
        }
    }
    AppExit::Success
}
