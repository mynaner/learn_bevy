use std::time::Duration;

use bevy::{
    color::palettes::css::GOLD, input::common_conditions::input_just_pressed, prelude::*,
    time::common_conditions::on_real_timer,
};

#[derive(Component)]
struct RealTime;

#[derive(Component)]
struct VirtualTime;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_real_time_sprites,
                move_virtual_time_sprites,
                toggle_pause.run_if(input_just_pressed::<KeyCode>(KeyCode::Space)),
                change_time_speed::<1>.run_if(input_just_pressed::<KeyCode>(KeyCode::ArrowUp)),
                change_time_speed::<0>.run_if(input_just_pressed::<KeyCode>(KeyCode::ArrowDown)),
                (update_real_time_info_text, update_virtual_time_info_text)
                    // 为了方便阅读,更新频率设低点
                    .run_if(on_real_timer(Duration::from_millis(250))),
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut timer: ResMut<Time<Virtual>>) {
    // 创建一个2d相机
    commands.spawn(Camera2d);

    // 设置2倍速
    timer.set_relative_speed(2.0);

    // 图片资源句柄
    let texture_handle = asset_server.load("./icon.png");

    let virtual_color = GOLD.into();

    // 初始化一个二维矢量，
    // 然后扩展二维矢量至三维
    let sprite_scale = Vec2::splat(0.5).extend(1.);

    // 创建一个原速度的实体，的外观和形变
    commands.spawn((
        Sprite::from_image(texture_handle.clone()),
        Transform::from_scale(sprite_scale),
        RealTime,
    ));

    // 创建一个倍速实体的外观和形变，位置
    commands.spawn((
        Sprite {
            image: texture_handle,
            color: virtual_color, // 加颜色区别原速
            ..default()
        },
        Transform {
            scale: sprite_scale,
            translation: Vec3 {
                x: 0.,
                y: 160.,
                z: 0.,
            }, // 位置不要重合
            ..default()
        },
        VirtualTime,
    ));

    let font_size = 33.0;

    commands
        .spawn(Node {
            display: Display::Flex,
            justify_content: JustifyContent::SpaceBetween,
            width: Val::Percent(100.),
            position_type: PositionType::Absolute,
            top: Val::Px(0.),
            padding: UiRect::all(Val::Px(20.)),
            ..default()
        })
        .with_children(|builder| {
            // 左对齐变速信息
            builder.spawn((
                Text::default(),
                TextFont {
                    font_size: font_size,
                    ..default()
                },
                RealTime,
            ));

            // 控制提示
            builder.spawn((
                Text::new("CONTROLS:\n un/pause:Space \n Speed+ :Up \n Speed-: down"),
                TextFont {
                    font_size: font_size,
                    ..default()
                },
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
                TextLayout::new_with_justify(Justify::Center),
            ));

            // 右对齐变速信息
            builder.spawn((
                Text::default(),
                TextFont {
                    font_size: font_size,
                    ..default()
                },
                TextColor(virtual_color),
                TextLayout::new_with_justify(Justify::Right),
                VirtualTime,
            ));
        });
}

// 虚拟世界 移动 Sprite
fn move_virtual_time_sprites(
    sprite: Query<&mut Transform, (With<Sprite>, With<VirtualTime>)>,
    time: Res<Time>,
) {
    for mut transform in sprite {
        transform.translation.x = get_sprite_translation_x(time.elapsed_secs());
    }
}

fn get_sprite_translation_x(elapsed: f32) -> f32 {
    ops::sin(elapsed) * 500.0
}

fn move_real_time_sprites(
    sprite_query: Query<&mut Transform, (With<RealTime>, With<Sprite>)>,
    time: Res<Time<Real>>,
) {
    for mut transform in sprite_query {
        transform.translation.x = get_sprite_translation_x(time.elapsed_secs());
    }
}

fn toggle_pause(mut time: ResMut<Time<Virtual>>) {
    if time.is_paused() {
        time.unpause();
    } else {
        time.pause();
    }
}

fn change_time_speed<const DELTA: i8>(mut time: ResMut<Time<Virtual>>) {
    let val = if DELTA == 0 { -1_f32 } else { 1_f32 };

    let time_speed = (time.relative_speed() + val)
        .round() // 四舍五入
        .clamp(0.25, 5.0); // 限制范围，防止负数和超高速

    time.set_relative_speed(time_speed);
}

fn update_real_time_info_text(time: Res<Time<Real>>, query: Query<&mut Text, With<RealTime>>) {
    for mut text in query {
        **text = format!(
            "REAL TIME  \n Elapsed(运行时长):{:.1} \n Delta(上一帧->当前帧间隔):{:.5}",
            time.elapsed_secs(),
            time.delta_secs()
        );
    }
}

fn update_virtual_time_info_text(
    time: Res<Time<Virtual>>,
    query: Query<&mut Text, With<VirtualTime>>,
) {
    for mut text in query {
        **text = format!(
            "VIRTUAL TIME: \n Elapsed(运行时长):{:.1} \n Delta(上一帧->当前帧间隔):{:.5}",
            time.elapsed_secs(),
            time.delta_secs()
        );
    }
}
