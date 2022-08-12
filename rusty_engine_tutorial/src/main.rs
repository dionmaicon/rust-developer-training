// in src/main.rs
use rusty_engine::prelude::*;

// Define a struct to hold custom data for your game (it can be a lot more complicated than this one!)
struct GameState {
    high_score: i32,
    current_score: i32,
    enemy_labels: Vec<String>,
    spawn_times: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            current_score: 0,
            high_score: 0,
            enemy_labels: Vec::new(),
            spawn_times: Timer::from_seconds(1.0, false),
        }
    }
}
fn main() {
    // Create a game
    let mut game = Game::new();

    // Set up your game. `Game` exposes all of the methods and fields of `Engine`
    let sprite = game.add_sprite("player", SpritePreset::RacingCarBlue);
    sprite.scale = 2.0;

    game.audio_manager.play_music(MusicPreset::Classy8Bit, 1.0);

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut EngineState, game_state: &mut GameState) -> bool {
    return true;
}
