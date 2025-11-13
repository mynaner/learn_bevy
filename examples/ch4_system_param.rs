use bevy::{ecs::system::SystemParam, prelude::*};
fn main() {
    App::new()
        // .add_plugins(DefaultPlugins)
        .insert_resource(PlayerCount::default())
        .add_systems(Startup, spawn)
        .add_systems(Update, count_players)
        .run();
}
#[derive(Component)]
struct Player;

#[derive(Resource, Default)]
struct PlayerCount(usize);

// 自定义参数签名
//
#[derive(SystemParam)]
struct PlayerCounter<'w, 's> {
    players: Query<'w, 's, &'static Player>,
    count: ResMut<'w, PlayerCount>,
}

impl<'w, 's> PlayerCounter<'w, 's> {
    fn count(&mut self) {
        self.count.0 = self.players.iter().len();
    }
}

fn spawn(mut commands: Commands) {
    commands.spawn(Player);
    commands.spawn(Player);
    commands.spawn(Player);
}

fn count_players(mut counter: PlayerCounter) {
    counter.count();
    println!("{} players in the game", counter.count.0);
}
