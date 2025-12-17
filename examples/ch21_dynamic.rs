//! 动态创建组建
//! 使用这些组件实体，以及查询具有这些组件的实体
//!
//! 没写完，似乎用处不大，如果以后实在需要，再学习

use std::{alloc::Layout, io::Write, ptr::NonNull};

use bevy::{
    ecs::component::{
        ComponentCloneBehavior, ComponentDescriptor, ComponentId, ComponentInfo, StorageType,
    },
    platform::collections::HashMap,
    prelude::*,
    ptr::{Aligned, OwningPtr},
};

const PROMPT: &str = "
命令:
  - comp, c 创建一个新的组件
  - spawn, s 生成实体
  - query, q 查询实体
输入无参数的指令即可查看使用方法
";

const COMPONENT_PROMPT: &str = "
comp, c  创建新组件
  - 输入一个以逗号分隔的类型名称列表（可选地，后面跟着以 u64 为单位的尺寸值）。
  - 示例：CompA 3， CompB， CompC 2
";

const ENTITY_PROMPT: &str = "
spawn, s 生成实体
  - 输入一个由逗号分隔的组件列表（可选地后跟值）。
  - 例如：CompA 0 1 0, CompB, CompC 1";

const QUERY_PROMPT: &str = "
query, q  查询实体
  - 输入查询内容以获取并更新实体
  - 具有读取或写入权限的组件将显示其值
  - 具有写入权限的组件其字段值将增加一
  - 访问权限：“A”表示读取，“&A”表示读取并修改，“&mut A”表示可读可写
  - 运算符：“||”表示或，“，”表示和，“？”表示可选
  -  例如：&A || &B，&mut C、D，？E";
fn main() {
    let mut world = World::new();

    // 获得一个安行读取的迭代器
    let mut lines = std::io::stdin().lines();
    let mut component_name = HashMap::<String, ComponentId>::new();
    let mut component_info = HashMap::<ComponentId, ComponentInfo>::new();
    println!("{PROMPT}");
    loop {
        print!("\n>");
        let _ = std::io::stdout().flush();
        // 读取第一个字符
        let Some(Ok(line)) = lines.next() else {
            return;
        };
        // 不能为空
        if line.is_empty() {
            return;
        }

        // 根据单字符进入下一个级菜单
        // split_one 依次传入 char
        // 以空白字符（is_whitespace可以判断 空格,TAB等多种类型）进行一次分割为两个&str,(如果没有空白字符就进入else)，
        let Some((first, rest)) = line.trim().split_once(char::is_whitespace) else {
            match &line.chars().next() {
                Some('c') => println!("{COMPONENT_PROMPT}"),
                Some('s') => println!("{ENTITY_PROMPT}"),
                Some('q') => println!("{QUERY_PROMPT}"),
                _ => println!("{PROMPT}"),
            }
            continue;
        };

        match &first[0..1] {
            // 创建component
            "c" => {
                rest.split(",").for_each(|component| {
                    let mut component = component.split_whitespace();
                    let Some(name) = component.next() else {
                        return;
                    };
                    // 初始化一个用于创建component.Layout 的size大小
                    let size = match component.next().map(str::parse) {
                        Some(Ok(size)) => size,
                        // 因为这里是0 ,所以推断的值是i32
                        _ => 0,
                    };
                    // 注册一个Component
                    let id = world.register_component_with_descriptor(unsafe {
                        ComponentDescriptor::new_with_layout(
                            name.to_string(),
                            StorageType::Table,
                            Layout::array::<u64>(size).unwrap(),
                            None,
                            true,
                            ComponentCloneBehavior::default(),
                        )
                    });
                    let Some(info) = world.components().get_info(id) else {
                        return;
                    };

                    component_name.insert(name.to_string(), id);
                    component_info.insert(id, info.clone());
                    println!(
                        "component {} create with id:{:?}/{:?} info:{:?}",
                        name,
                        id.index(),
                        id,
                        info
                    );
                });
            }
            "s" => {
                let mut to_insert_ids = Vec::new();
                let mut to_insert_data = Vec::new();
                rest.split(",").for_each(|component| {
                    let mut component = component.split_whitespace();

                    // 获得component 的名字
                    let Some(name) = component.next() else {
                        return;
                    };

                    // 通过name获取component id
                    let Some(&id) = component_name.get(name) else {
                        println!("Component {name} does not exist");
                        return;
                    };
                    // 获取component layout 大小，保证Entity创建的时候大小不一致
                    let info = world.components().get_info(id).unwrap();
                    let len = info.layout().size() / size_of::<u64>();

                    // 解析命令行创建entity输入的info其他内容
                    // 保证每个 whitespace 分割的str都可以被parse 为一个u64
                    // 获得一个vec<64>,确保内容以Entity创建内容为主，如果少于Component 预设大小以0填充
                    let mut values = component
                        .take(len)
                        .filter_map(|val| val.parse::<u64>().ok())
                        .collect::<Vec<u64>>();

                    values.resize(len, 0);

                    to_insert_ids.push(id);
                    to_insert_data.push(values);
                });

                // 开始创建entity
                let mut entity = world.spawn_empty();

                // 将component 的vec<64> 类型转换为指针迭代器 Vec<OwningPtr>
                let to_insert_per = to_owning_ptr(&mut to_insert_data);

                unsafe {
                    entity.insert_by_ids(&to_insert_ids, to_insert_per.into_iter());
                }
                println!("Entity spawned with id:{}", entity.id());
            }
            "q" => {}
            _ => {}
        }
    }
}

// 对于“components”中的每个项，构建“拥有指针”对象
// 通过将“components”的生存期与生成的指针共享，我们确保在使用数据之前不会将其删除
fn to_owning_ptr(component: &mut [Vec<u64>]) -> Vec<OwningPtr<'_, Aligned>> {
    component
        .iter_mut()
        .map(|data| {
            let ptr = data.as_mut_ptr();
            // 安全性：
            // - 指针必定不会为 null
            // - 所指向的内存在 `components` 被销毁之前不会被释放
            unsafe {
                let non_null = NonNull::new_unchecked(ptr.cast());
                OwningPtr::new(non_null)
            }
        })
        .collect()
}
