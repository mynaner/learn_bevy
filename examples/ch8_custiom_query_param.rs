//! [`QueryData`]该宏允许定义自定义的查询和筛选类型。
//! 在大多数简单场景中，常规的元组查询表现得非常出色。然而，使用以命名结构体形式声明的自定义查询则能带来以下优势：
//！ - 它们有助于避免解构操作或使用“q.0, q.1, ...”这样的访问模式。
//！ - 通过使用结构体来添加、删除组件或改变元素顺序，能极大地减轻维护负担，因为您无需更新解构元组的语句、无需关注元素的顺序等。相反，您只需添加或删除某个元素被使用的位置即可。
//！ - 命名结构体支持组合模式，这使得查询类型更易于复用。
//！ - 您可以绕过查询元组中 15 个组件的限制。//!

use bevy::{
    ecs::query::{QueryData, QueryFilter},
    prelude::*,
};
use std::fmt::Debug;

#[derive(Debug, Component)]
struct ComponentA;
#[derive(Debug, Component)]
struct ComponentB;
#[derive(Debug, Component)]
struct ComponentC;
#[derive(Debug, Component)]
struct ComponentD;
#[derive(Debug, Component)]
struct ComponentZ;

// QueryData 的目的是集成查询结果到一个自定义的结构体
// 所以如果Query 中的Entity 不包含(非Option集成)某个Component 时就不会有结果,该System 就不会被执行
#[derive(QueryData)]
#[query_data(derive(Debug))]
struct ReadOnlyCustomQuery<T: Component + Debug, P: Component + Debug> {
    entity: Entity,
    a: &'static ComponentA,
    b: Option<&'static ComponentB>,
    nested: NestedQuery,
    optional_nested: Option<NestedQuery>,
    optional_tuple: Option<(&'static ComponentB, &'static ComponentZ)>,
    z: Option<&'static ComponentZ>, // 因为z 没有加入
    generic: GenericQuery<T, P>,
    enpty: EmptyQuery,
}

#[derive(QueryData)]
#[query_data(derive(Debug))]
struct NestedQuery {
    c: &'static ComponentC,
    d: Option<&'static ComponentD>,
}

#[derive(QueryData)]
#[query_data(derive(Debug))]
struct GenericQuery<T: Component, P: Component> {
    generic: (&'static T, &'static P),
}

#[derive(QueryData)]
#[query_data(derive(Debug))]
struct EmptyQuery {
    empt: (),
}

// 所有的结构属性都是and 联合,所以一旦有一个条件不满足,将没有结果
#[derive(QueryFilter)]
struct CustomQueryFilter<T: Component, P: Component> {
    _c: With<ComponentC>,
    _d: With<ComponentD>,
    // 如果不将 Added 加入到Or 条件,那么query就会得到一次结果
    _or: Or<(Added<ComponentC>, Changed<ComponentD>, Without<ComponentZ>)>,
    _generic_tuple: (With<T>, With<P>),
    // 满足不了CompoenntZ 条件
    // _generic_tuple2:(With<T>,With<P>,With<ComponentZ>)
}

#[derive(QueryData)]
#[query_data(mutable, derive(Debug))]
struct CustomQuery<T: Component + Debug, P: Component + Debug> {
    entity: Entity,
    a: &'static mut ComponentA,
    b: Option<&'static mut ComponentB>,
    nested: NestedQuery,
    optional_nested: Option<NestedQuery>,
    optional_tuple: Option<(NestedQuery, &'static mut ComponentZ)>,
    generic: GenericQuery<T, P>,
    empty: EmptyQuery,
}

type NestedTupleQuery<'w> = (&'w ComponentC, &'w ComponentD);
type GenericTupleQuery<'w, T, P> = (&'w T, &'w P);

fn main() {
    App::new()
        .add_systems(Startup, spawn)
        .add_systems(
            Update,
            (
                print_components_read_only,
                print_components_iter_mut,
                print_components_iter,
                print_components_tuple,
            )
                .chain(),
        )
        .run();
}

fn spawn(mut commands: Commands) {
    // z 不被加入
    commands.spawn((ComponentA, ComponentB, ComponentC, ComponentD));
}

// 只读的查询
fn print_components_read_only(
    query: Query<
        ReadOnlyCustomQuery<ComponentC, ComponentD>,
        CustomQueryFilter<ComponentC, ComponentD>,
    >,
) {
    println!("print components (read_only)");
    for e in &query {
        println!("Entity: {:?}", e.entity);
        println!("A: {:?}", e.a);
        println!("B: {:?}", e.b);
        println!("Nested: {:?}", e.nested);
        println!("Optional nested: {:?}", e.optional_nested);
        println!("Optional tuple: {:?}", e.optional_tuple);
        println!("Generic: {:?}", e.generic);
        println!("enpty: {:?}", e.enpty);
    }
    println!()
}

// 当使用可写的版本时,需要两个地方加mut
fn print_components_iter_mut(
    mut query: Query<
        CustomQuery<ComponentC, ComponentD>,
        CustomQueryFilter<ComponentC, ComponentD>,
    >,
) {
    println!("print components (iter mut)");
    for e in &mut query {
        let e: CustomQueryItem<'_, '_, _, _> = e;
        println!("Entity: {:?}", e.entity);
        println!("A: {:?}", e.a);
        println!("B: {:?}", e.b);
        println!("Nested: {:?}", e.nested);
        println!("Optional nested: {:?}", e.optional_nested);
        println!("Optional tuple: {:?}", e.optional_tuple);
        println!("Generic: {:?}", e.generic);
    }
}
//  这是只读版本
fn print_components_iter(
    query: Query<CustomQuery<ComponentC, ComponentD>, CustomQueryFilter<ComponentC, ComponentD>>,
) {
    println!("print components (iter mut)");
    for e in &query {
        let e: CustomQueryReadOnlyItem<'_, '_, _, _> = e;
        println!("Entity: {:?}", e.entity);
        println!("A: {:?}", e.a);
        println!("B: {:?}", e.b);
        println!("Nested: {:?}", e.nested);
        println!("Optional nested: {:?}", e.optional_nested);
        println!("Optional tuple: {:?}", e.optional_tuple);
        println!("Generic: {:?}", e.generic);
    }
}
// 这是一个常规传统的方式
// 复杂度集中到了参数重,不易阅读
fn print_components_tuple(
    query: Query<
        (
            Entity,
            &ComponentA,
            &ComponentB,
            NestedTupleQuery,
            GenericTupleQuery<ComponentC, ComponentD>,
        ),
        (
            With<ComponentC>,
            With<ComponentD>,
            Or<(Added<ComponentC>, Changed<ComponentD>, Without<ComponentZ>)>,
        ),
    >,
) {
    println!("Print components (tuple):");
    for (entity, a, b, nested, (generic_c, generic_d)) in &query {
        println!("Entity: {:?}", entity);
        println!("A: {:?}", a);
        println!("B: {:?}", b);
        println!("Nested: {:?}", nested);
        println!("generic_c {:?}", generic_c);
        println!("generic_d {:?}", generic_d);
    }
}
