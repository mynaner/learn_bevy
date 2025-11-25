<!--
 * @Date: 2025-11-14 09:29:11
 * @LastEditors: myclooe 994386508@qq.com
 * @LastEditTime: 2025-11-25 09:12:49
 * @FilePath: /blibli_bevy2/README.md
-->
# 学习bevy 笔记





## example示例 

[1.hello bevy 第一个bevy 程序](examples/ch1_hello_bevy.rs) 

[2.ecs_guide 一个完整 esc 游戏](examples/ch2_ecs_guide.rs)

[3.system可以使用闭包和匿名函数](examples/ch3_system_closure.rs)

[4.【SystemParam】自定义system参数签名](examples/ch4_system_param.rs)

[5, 按顺序运行system函数,并将返回值传递给下一个system](examples/ch5_system_piping.rs)

[6, Stepping 模式 手动执行调度 (debug模式)](examples/ch6_system_stepping.rs)

[7, 模拟一个玩家遭遇战的场景 Query的用法【Single】【Populated】【Options<Single>】](examples/ch7_query.rs) 

[8，【QueryData】 允许自定义查询 【QueryFilter】自定义筛选类型](examples/ch8_custiom_query_param.rs)

[9, 【combinations】遍历查询结果的组合 `query.iter_combinations` ](examples/ch9_iter_combinations.rs)

[10,【parallelIterator】 进行并行迭代器查询(大量的物理性多线程运算)](examples/ch10_parallel_query.rs)

[11.hierarchy 层次结构](examples/ch11_hierarchy.rs)

[12,`一次性系统`的注册与触发](examples/ch12_one_shot_systems.rs)

[13.被动检测component 与 resource 的变更](examples/ch13_change_detaction.rs)

[14.【Message】发送修改和接收消息](examples/ch14_message.rs)

[15.在一个system中既能发送又能接收同一消息类型](examples/ch15_send_and_receive_messages.rs)

[16,监听component，如果被移除会触发对应函数](examples/ch16_removal_decetion.rs)

[17,如何观察事件包括组件生命周期事件以及自定义事件](examples/ch17_observers.rs)