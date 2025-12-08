// 添加自定义调度任务（生命周期）
use bevy::{
    app::MainScheduleOrder,
    ecs::schedule::{ExecutorKind, ScheduleLabel},
    prelude::*,
};

#[derive(ScheduleLabel, Hash, Debug, Eq, PartialEq, Clone)]
struct SingleThreadSchedule;
#[derive(Debug, ScheduleLabel, Hash, PartialEq, Eq, Clone)]
struct CustomSchedule;

fn main() {
    let mut app = App::new();
    // 将自定义schedule 实例化，并将执行修改为单线程（独占）
    let mut custom_update_schedule = Schedule::new(SingleThreadSchedule);
    custom_update_schedule.set_executor_kind(ExecutorKind::SingleThreaded);

    // 先将 schedule 添加至资源，并不会执行
    // schedule 实例的作用在后续操作中就不大了，后续操作中可以直接使用 SingleThreadSchedule 作为标识
    app.add_schedule(custom_update_schedule);

    // App 默认的 Runner 会调用默认 main_schedule_label 字段 称其为 main schedule， 运行时main schedule会响应
    // Update Startup .. 这些都是内置的schedule
    //
    // 可以通过MainScheduleOrder的标识，读取这个main schedule，可以对其进行配置，将自定义的schedule插入
    // 注意： 对于MainScheduleOrder的配置，只能在main主函数中进行，不能将这个过程`系统化`
    // 因为system的执行本身依赖于 MainScheduleOrder
    let mut main_schedule_order = app.world_mut().resource_mut::<MainScheduleOrder>();
    main_schedule_order.insert_after(Update, SingleThreadSchedule);

    // 再添加一个自定义多线程的schedule(默认是多线程)
    app.add_schedule(Schedule::new(CustomSchedule));
    let mut main_schedule_order = app.world_mut().resource_mut::<MainScheduleOrder>();

    main_schedule_order.insert_startup_after(PreStartup, CustomSchedule);

    app.add_systems(SingleThreadSchedule, || {
        println!("SingleThreadSchedule");
    })
    .add_systems(CustomSchedule, || println!("CustomSchedule"))
    .add_systems(PreStartup, || println!("PreStartup"))
    .add_systems(Startup, || println!("Startup"))
    .add_systems(First, || println!("First"))
    .add_systems(Update, || println!("Update"))
    .add_systems(Last, |mut w: MessageWriter<AppExit>| {
        println!("Last");
        w.write(AppExit::Success);
    });

    app.run();
}
