// 手动执行调度

// Bevy 的 “Stepping 模式” 下，默认所有调度阶段都是禁用的，所以你必须显式启用你想要运行的阶段。

// stepping.add_schedule(Update).enable(); 启用 update 的调度
// stepping.continue_frame(); 执行剩余的update
// stepping.step_frame(); 执行下一针的第一个update
// stepping.always_run(Update, update_system_two); 每次update 都会执行  update_system_two;
// stepping.never_run(Update, update_system_two); 屏蔽update_system_two ,不执行
// stepping.clear_breakpoint 清除断点
use bevy::{ecs::schedule::Stepping, log::LogPlugin, prelude::*};

fn main() {
    let mut app = App::new();
    app.add_plugins(LogPlugin::default())
        .add_systems(
            Update,
            (
                update_system_one,
                update_system_two.after(update_system_one),
                update_system_three.after(update_system_two),
                update_system_four,
            ),
        )
        .add_systems(PreUpdate, pre_update_system);

    println!(r#"################ 1, schedule 没有受到影响 所有system 都会被执行"#);
    app.update();
    println!("################ 2, Stepping 还没有设置,schedule 没有受到影响 所有system 都会被执行");
    app.insert_resource(Stepping::new());
    app.update();
    println!("################ 3, 内建schedule受到影响,只有Preupdate 执行");
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    // 组册 update 调度
    stepping.add_schedule(Update).enable();
    app.update();

    println!(
        "################ 4, 内建schedule受到影响, preupdate 被执行, update 的多个system 被执行"
    );
    // let mut stepping = app.world_mut().resource_mut::<Stepping>();
    // stepping.step_frame();
    // app.update();
    println!("################ 5, 没有执行 stepping.step_frame,只有Preupdate 执行 ");
    app.update();

    println!(
        "################ 6, 执行剩余的system, 在4的时候执行了一个,所以只有剩余update中的3个执行,每次app.update() 执行的时候都会先执行per update"
    );
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.continue_frame();
    app.update();

    println!("################ 7,执行四次, 每次都会执行pre update的system 和update中的一个system");
    for _ in 0..4 {
        let mut stepping = app.world_mut().resource_mut::<Stepping>();

        stepping.step_frame();
        app.update();
    }
    println!(
        "################ 8, always_run 会使得 update_system_two 每次都被执行,跳出set_frame 每次执行一个点限制"
    );
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.always_run(Update, update_system_two);
    for _ in 0..3 {
        let mut stepping = app.world_mut().resource_mut::<Stepping>();
        stepping.step_frame();
        app.update();
    }
    println!("################ 9 never_run 屏蔽指定 update_system_two");

    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.never_run(Update, update_system_two);
    stepping.continue_frame();
    app.update();

    println!("################ 10 set_breakpoint 会在 update_system_two 执行时停止(断点一次)");
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.set_breakpoint(Update, update_system_two);
    stepping.continue_frame();
    app.update();

    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.step_frame();
    app.update();

    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.continue_frame();
    app.update();

    println!("################ 11 clear_breakpoint 清除断点");
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.clear_breakpoint(Update, update_system_two);
    stepping.continue_frame();
    app.update();
    println!("################ 12 关闭调试的 stepping,恢复内建的Schedule");
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.disable();
    app.update();
}

fn pre_update_system() {
    println!("pre update")
}

fn update_system_one() {
    println!("one update")
}
fn update_system_two() {
    println!("two update")
}
fn update_system_three() {
    println!("three update")
}
fn update_system_four() {
    println!("four update")
}
