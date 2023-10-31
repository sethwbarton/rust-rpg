pub mod game;

pub mod lib {
    use crate::game::drawable::Drawable;
    use crate::game::game_state::GameState;
    use crate::game::input::handle_input;
    use crate::game::player::Player;
    use console_engine::{pixel, Color, KeyCode};

    // Size of my laptop monitor in iTerm at full screen
    pub const SCREEN_HEIGHT: u32 = 55;
    pub const SCREEN_WIDTH: u32 = 214;

    pub fn run() {
        let mut engine =
            console_engine::ConsoleEngine::init(SCREEN_WIDTH, SCREEN_HEIGHT, 60).unwrap();

        let mut game_state = GameState {
            player: Player { x: 3, y: 3 },
        };

        loop {
            engine.wait_frame();
            engine.clear_screen();

            engine.rect(
                0,
                0,
                (SCREEN_WIDTH - 1) as i32,
                (SCREEN_HEIGHT - 1) as i32,
                pixel::pxl_fg('█', Color::Cyan),
            );

            handle_input(&mut game_state, &engine);

            game_state.player.draw(&mut engine);

            engine.draw(); // draw the screen

            if engine.is_key_pressed(KeyCode::Char('q')) {
                // if the user presses 'q' :
                break;
            }
        }
    }
}
