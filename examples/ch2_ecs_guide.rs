use std::{
    fmt::{self},
    time::Duration,
};

use bevy::{app::ScheduleRunnerPlugin, prelude::*};

// 一个完整 esc(entriy system component) 游戏

fn main() {
    App::new().add_plugins(GamePlugins).run();
}

// 定义玩家
#[derive(Component, Debug)]
struct Player(String);

#[derive(Component, Debug)]
struct Score(usize);

#[derive(Component, Debug)]
enum PlayerStreak {
    Hot(usize),
    None,
    Cold(usize),
}

impl fmt::Display for PlayerStreak {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cold(e) => write!(f, "{e} 回合 连败"),
            Self::Hot(n) => write!(f, "{n} 回合 连胜"),
            Self::None => write!(f, "0 回合"),
        }
    }
}

// 定义状态
#[derive(Resource, Default)]
struct GameState {
    current_round: usize,           // 当前回合数
    total_player: usize,            // 玩家总数
    winning_palyer: Option<String>, // 获胜者
}

// 定义规则
#[derive(Resource)]
struct GameRule {
    winning_score: usize, // 获胜分数
    max_round: usize,     // 最大回合数
    max_player: usize,    // 最大玩家数
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
enum MySetRound {
    BeforRound,
    Round,
    AflterRound,
}

struct GamePlugins;

// first        每帧开始的时候执行          重置状态
// preupdate    主要逻辑前                 处理输入,事件收集
// update       游戏主要逻辑               移动 AI,物理,状态更新
// postupdate   主要逻辑后                 同步状态渲染准备
// last         最后执行                   清除,统计,调试输出
impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>()
            .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs(5)))
            .add_systems(Startup, startup_system)
            .add_systems(Update, print_message_system)
            // last 没帧的最后执行
            .add_systems(Last, print_at_end_round)
            // 自定义一个执行顺序
            .configure_sets(
                Update,
                (
                    MySetRound::AflterRound,
                    MySetRound::Round,
                    MySetRound::BeforRound,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    (
                        new_round_system,
                        new_player_system.run_if(is_add_player),
                        exclusive_player_system,
                    )
                        .in_set(MySetRound::AflterRound),
                    score_system.in_set(MySetRound::Round),
                    (
                        score_check_system,
                        game_over_system.after(score_check_system),
                    )
                        .in_set(MySetRound::BeforRound),
                ),
            );
    }
}

fn startup_system(mut command: Commands, mut game_state: ResMut<GameState>) {
    // 定义规则
    let game_rule = GameRule {
        winning_score: 4,
        max_round: 10,
        max_player: 4,
    };
    command.insert_resource(game_rule);
    // 定义两个玩家
    let players = ("张三", "李四");
    command.spawn_batch([
        (Player(players.0.to_string()), Score(0), PlayerStreak::None),
        (Player(players.1.to_string()), Score(0), PlayerStreak::None),
    ]);
    game_state.total_player = 2;
}

fn new_player_system(
    mut command: Commands,
    game_rule: Res<GameRule>,
    mut game_state: ResMut<GameState>,
) {
    if game_rule.max_player <= game_state.total_player {
        println!("游戏玩家已达最大值{}", game_rule.max_player);
        return;
    }

    let add_new_player = rand::random::<bool>();
    println!("是否添加游戏玩家 {}", add_new_player);
    if add_new_player && game_rule.max_player > game_state.total_player {
        command.spawn((
            Player(format!("玩家{}", game_state.total_player + 1)),
            Score(0),
            PlayerStreak::None,
        ));
        game_state.total_player += 1;
        println!(
            "游戏玩家添加成功 当前游戏玩家数量{}",
            game_state.total_player
        );
    }
}

// 玩家是否达到最大值
fn is_add_player(game_state: Res<GameState>, game_rule: Res<GameRule>) -> bool {
    game_state.total_player < game_rule.max_player
}

fn print_message_system() {
    println!("回合开始的准备....")
}

fn print_at_end_round(mut counter: Local<u32>) {
    *counter += 1;
    println!("系统在每帧最后的 Last 阶段被执行时，第 {} 次运行", *counter);
    println!()
}

fn new_round_system(game_rule: Res<GameRule>, mut game_state: ResMut<GameState>) {
    game_state.current_round += 1;

    println!(
        "回合开始 {}/{}",
        game_state.current_round, game_rule.max_round
    );
}

fn score_system(query: Query<(&mut Score, &mut PlayerStreak, &Player)>) {
    for (mut score, mut streak, player) in query {
        let score_a_point = rand::random::<bool>();
        if score_a_point {
            score.0 += 1;
            *streak = match *streak {
                PlayerStreak::Hot(e) => PlayerStreak::Hot(e + 1),
                PlayerStreak::None | PlayerStreak::Cold(_) => PlayerStreak::Hot(1),
            };
            println!(
                "玩家 {} 获得胜利积分+1 当前积分 {} ({})",
                player.0, score.0, *streak
            )
        } else {
            *streak = match *streak {
                PlayerStreak::Cold(n) => PlayerStreak::Cold(n + 1),
                PlayerStreak::None | PlayerStreak::Hot(_) => PlayerStreak::Cold(1),
            };
            println!(
                "玩家 {} 失败! 未获得积分 当前积分 {} ({})",
                player.0, score.0, *streak
            )
        }
    }
}

fn score_check_system(
    query: Query<(&Score, &Player), With<Player>>,
    game_rule: Res<GameRule>,
    mut game_state: ResMut<GameState>,
) {
    for (score, player) in query {
        if game_rule.winning_score == score.0 {
            game_state.winning_palyer = Some(player.0.clone());
        }
    }
}

fn game_over_system(
    game_rule: Res<GameRule>,
    game_state: Res<GameState>,
    mut app_exit_writer: MessageWriter<AppExit>,
) {
    if let Some(n) = &game_state.winning_palyer {
        print!("玩家 {} 赢得了该场游戏", n);
        app_exit_writer.write(AppExit::Success);
    } else if game_rule.max_round == game_state.current_round {
        print!("没有玩家胜利,游戏结束");
        app_exit_writer.write(AppExit::Success);
    }
}

// word.span
fn exclusive_player_system(world: &mut World) {
    let total_player = world.resource::<GameState>().total_player;

    let should_add_player = {
        let max_player = world.resource::<GameRule>().max_player;
        let add_new_playter = rand::random::<bool>();

        add_new_playter && max_player > total_player
    };

    if should_add_player {
        println!("玩家 {} 加入游戏", total_player + 1);

        world.spawn((
            Player(format!("玩家{}", total_player + 1)),
            Score(0),
            PlayerStreak::None,
        ));

        let mut game_state = world.resource_mut::<GameState>();
        game_state.total_player += 1;
    }
}
