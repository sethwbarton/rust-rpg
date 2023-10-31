pub mod lib {
    use console_engine::{pixel, Color, KeyCode};

    // Size of my laptop monitor in iTerm at full screen
    pub const SCREEN_HEIGHT: u32 = 55;
    pub const SCREEN_WIDTH: u32 = 214;

    pub fn run() {
        let mut engine =
            console_engine::ConsoleEngine::init(SCREEN_WIDTH, SCREEN_HEIGHT, 60).unwrap();

        let mut player_x = 3;
        let mut player_y = 3;

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

            if engine.is_key_pressed(KeyCode::Char('w')) {
                player_y -= 1;
            }

            if engine.is_key_pressed(KeyCode::Char('s')) {
                player_y += 1;
            }

            if engine.is_key_pressed(KeyCode::Char('a')) {
                player_x -= 1;
            }
            if engine.is_key_pressed(KeyCode::Char('d')) {
                player_x += 1;
            }

            if engine.is_key_pressed(KeyCode::Char('q')) {
                // if the user presses 'q' :
                break; // exits app
            }

            engine.set_pxl(player_x, player_y, pixel::pxl_fg('O', Color::Green));

            engine.draw(); // draw the screen
        }
    }
}
