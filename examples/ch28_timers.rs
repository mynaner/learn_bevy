/*
 * @Date: 2025-12-16 17:50:24
 * @LastEditors: myclooe 994386508@qq.com
 * @LastEditTime: 2025-12-16 18:25:14
 * @FilePath: /blibli_bevy2/examples/ch28_timers.rs
 */
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Countdown>()
        .add_systems(Startup, setup)
        .add_systems(Update, (countdown, print_when_completed))
        .run();
}

#[derive(Resource)]
struct Countdown {
    percent_trigger: Timer,
    main_timer: Timer,
}

impl Countdown {
    pub fn new() -> Self {
        Self {
            percent_trigger: Timer::from_seconds(4., TimerMode::Repeating),
            main_timer: Timer::from_seconds(20., TimerMode::Once),
        }
    }
}

impl Default for Countdown {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Component, DerefMut, Deref)]
struct PrintOnCompletionTimer(Timer);

fn setup(mut commands: Commands) {
    commands.spawn(PrintOnCompletionTimer(Timer::from_seconds(
        5.0,
        TimerMode::Once,
    )));
}

fn countdown(time: Res<Time>, mut countdown: ResMut<Countdown>) {
    countdown.main_timer.tick(time.delta());

    if countdown.percent_trigger.tick(time.delta()).just_finished() {
        if !countdown.main_timer.is_finished() {
            info!(
                "Timer is {:0.0}% complete!",
                countdown.main_timer.fraction() * 100.
            )
        } else {
            countdown.percent_trigger.pause();
            info!("Paused percent trigger timer");
        }
    }
}

fn print_when_completed(time: Res<Time>, mut query: Query<&mut PrintOnCompletionTimer>) {
    for mut timer in query {
        if timer.tick(time.delta()).just_finished() {
            info!("Entity timer just finished");
        }
    }
}
