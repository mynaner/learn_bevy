//! 通常情况event 性能更好，更加灵活
//! 但 hooks 在维护索引与组织结构时，更有优势
//! 在本例中 trigger_hooks 只负责对 MyComponent 的添加与删除,
//! 而 hook 负责 MyComponentIndex 索引的维护与 Entity 的移除
//!
//! 通过监听component并触发hooks
use bevy::{
    ecs::{
        component::{Mutable, StorageType},
        lifecycle::{ComponentHook, HookContext},
    },
    platform::collections::HashMap,
    prelude::*,
};

#[derive(Debug)]
struct MyComponent(KeyCode);

impl Component for MyComponent {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Mutable;
    fn on_add() -> Option<ComponentHook> {
        None
    }
}
#[derive(Resource, Default, Debug, Deref, DerefMut)]
struct MyComponentIndex(HashMap<KeyCode, Entity>);

#[derive(Message)]
struct MyMessage;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 组册hooks closure
        .add_systems(Startup, setup)
        // 模拟 component 的动态添加与删除
        .add_systems(Update, trigger_hooks)
        // 在on_add 触发 hook 需要更新的索引
        .init_resource::<MyComponentIndex>()
        // 模拟在hooks 发送事件
        .add_message::<MyMessage>()
        .run();
}

fn setup(world: &mut World) {
    world
        .register_component_hooks::<MyComponent>()
        // 当发现新的component 添加到某个实体中时触发
        // 从实体中获得 component 在从component中获得keycode
        // 将keycode和entity 添加到资源索引中（hashMap）
        .on_add(
            |mut world,
             HookContext {
                 entity,
                 component_id,
                 caller,
                 ..
             }| {
                let value = world.get::<MyComponent>(entity).unwrap().0;
                println!("(on_add) component:{component_id:?} added to:{entity:?} with value:{value:?},caller:{caller:?}");

                world.resource_mut::<MyComponentIndex>().insert(value, entity);
                world.write_message(MyMessage);
            },

        )
        // 当发现一个新的 component 添加到实体时触发
        // 不管该实体是否已拥有 component 都会在 on_add 后执行
        .on_insert(|world,_|{
            println!("on_insert Current Index:{:?}",world.resource::<MyComponentIndex>())
        })
        // 当替换component时触发，
        // 同样也会在 on_remove 前触发
        // 获得 keycode 后，按其从资源索引中移除
        .on_replace(|mut world,context:HookContext|{

            let  value = world.get::<MyComponent>(context.entity).unwrap().0;
         world.resource_mut::<MyComponentIndex>().remove(&value);
            println!("(on_replace)：{:?}",world.resource::<MyComponentIndex>());
      })
        //   从world 移除entity，因为component hook 触发了，所以entity被移除，
      .on_remove(|mut world,context|{
        let value = world.get::<MyComponent>(context.entity).unwrap().0;
        println!("(on_remove) {:?} removed from {:?} with value {value:?}{}",context.caller,context.entity,context.caller.map(|location|format!("due to {location}")).unwrap_or_default());
        // 移除 entity
        world.commands().entity(context.entity).despawn();
      });
}

fn trigger_hooks(
    keycode: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    index: Res<MyComponentIndex>,
) {
    // 遍历循环,查找非按压状态的时候触发 (按键抬起时)
    for (key, entity) in index.iter() {
        if !keycode.pressed(*key) {
            println!(">>> up)");
            commands.entity(*entity).remove::<MyComponent>();
        }
    }

    for key in keycode.get_just_pressed() {
        println!(">>> down");
        commands.spawn(MyComponent(*key));
    }
}
