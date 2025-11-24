//! 有时候,会希望在一个system 中既能发送又能接受同一种类型事件
//! 这会导致 MessageWriter 和 MessageReader 的引用存在重叠
//! 解决这个问题
//! 1 ,使用 ParamSet 来依次回去 MessageWriter 和 MessageReader
//! 2, 使用 Local EventCursor 而不是 MessageReader 并使用ResMut 来访问 Events
use bevy::{diagnostic::FrameCount, ecs::message::MessageCursor, prelude::*};

#[derive(Debug, Message, Clone)]
struct DebugMessage {
    resend_from_param_set: bool,
    resend_from_local_message_reader: bool,
    times_sent: u8,
}

#[derive(Debug, Message)]
struct A;

#[derive(Debug, Message)]
struct B;

fn main() {
    let mut app = App::new();
    // 极简化的一个插件,只包含运行的核心系统
    app.add_plugins(MinimalPlugins)
        .add_message::<DebugMessage>()
        .add_message::<A>()
        .add_message::<B>()
        .add_systems(Update, read_and_write_different_event_types)
        .add_systems(
            Update,
            (
                send_message,
                debug_message,
                send_and_receive_param_set,
                send_and_receive_manual_event_reader,
            )
                .chain(),
        );

    app.update();
    app.update();
}

// 因为类型不同,所有可以运行 ,
// 注意: 借用规则是在系统运行时检查,而不是编译时检查
fn read_and_write_different_event_types(mut a: MessageWriter<A>, mut b: MessageReader<B>) {
    for _ in b.read() {}
    a.write(A);
}

fn send_message(mut debug_message: MessageWriter<DebugMessage>, frame_count: Res<FrameCount>) {
    // 打印程序渲染的帧数
    println!("Sending message for frame {}", frame_count.0);

    debug_message.write(DebugMessage {
        resend_from_param_set: false,
        resend_from_local_message_reader: false,
        times_sent: 1,
    });

    debug_message.write(DebugMessage {
        resend_from_param_set: true,
        resend_from_local_message_reader: false,
        times_sent: 1,
    });

    debug_message.write(DebugMessage {
        resend_from_param_set: false,
        resend_from_local_message_reader: true,
        times_sent: 1,
    });

    debug_message.write(DebugMessage {
        resend_from_param_set: true,
        resend_from_local_message_reader: true,
        times_sent: 1,
    });
}

// 接收 广播的消息,并打印
fn debug_message(mut message: MessageReader<DebugMessage>) {
    for msg in message.read() {
        println!("{msg:?}");
    }
}

// 使用 param_set 实现同时发送和接收在同一系统
// 使用 param_set 将同一类型的 MessageWriter 和 MessageReader 进行分离
fn send_and_receive_param_set(
    mut param_set: ParamSet<(MessageReader<DebugMessage>, MessageWriter<DebugMessage>)>,
    frame_count: Res<FrameCount>,
) {
    println!(
        "send and receive events for frame {} with a 'ParamSet'",
        frame_count.0
    );
    // 事件重发集合,因为我们不能在迭代读取器时访问写入器
    let mut event_to_send = Vec::new();

    // 迭代一个事件读取器,将接收到的消息过滤放入数组
    for event in param_set.p0().read() {
        if event.resend_from_param_set {
            event_to_send.push(event.clone());
        }
    }
    // 迭代接收到的消息,然后发送出去
    for mut evnet in event_to_send {
        // 更改消息
        evnet.times_sent += 1;
        // 这里发送的数据将会在下一次update 的时候被接收
        param_set.p1().write(evnet);
    }
}

// 使用 Local EventCursor 完成同时接收和发送事件的系统
fn send_and_receive_manual_event_reader(
    // EventCursor 可以理解为是MessageReader 的读取 游标指针
    mut local_event_reader: Local<MessageCursor<DebugMessage>>,
    // 一个可以被修改的Evnets资源
    // 这是一个资源,不是一个迭代器,主要作用完成send
    mut events: ResMut<Messages<DebugMessage>>,
    frame_count: Res<FrameCount>,
) {
    println!(
        "sending and receving events for frame {} with a 'Local<EventCursor>'",
        frame_count.0
    );

    let mut event_to_resend = Vec::new();

    for event in local_event_reader.read(&events) {
        if event.resend_from_local_message_reader {
            event_to_resend.push(event.clone());
        }
    }

    for mut event in event_to_resend {
        event.times_sent += 1;
        events.write(event);
    }
}
