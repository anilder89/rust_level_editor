// render screen with macroquad
use macroquad::prelude::*;

// prep level editor screens
mod screen;
use screen::{Button, GameScreen, Key, LevelScreen, Mouse, ScreenPoint, SplitScreen};

fn window_conf() -> Conf {
    Conf {
        window_title: "Level Editor".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}
// start the level editor
#[macroquad::main(window_conf)]
async fn main() {
    /////////////////////////////////////////////////////
    // Init Screens and States                         //
    /////////////////////////////////////////////////////
    // TODO: Adjust for resize, here it will be ignored
    // beacause values are read outside of the loop

    // set screen width
    let width = screen_width();
    // set split position
    let split_position = screen_height() / 2.;
    // init game rendering screen
    let game_screen = GameScreen::new(width, split_position);

    // init level editor screen
    let level_screen = LevelScreen::new(width, split_position);
    // init split screen
    let mut split_screen = SplitScreen::new(game_screen, level_screen, split_position);

    loop {
        /////////////////////////////////////////////////////
        // Render Game(top screen) and Level Editor(bottom)//
        /////////////////////////////////////////////////////

        // sync the level editor and the game
        split_screen.sync_screen_states();

        // draw screens
        split_screen.top_screen.draw();
        split_screen.bottom_screen.draw(split_screen.split_position);

        ////////////////////////////////////////////////////
        // Keyboard Input and Player Movement Logic       //
        ////////////////////////////////////////////////////

        // generate an input queue each frame and pass to game lib
        let mut key_q: Vec<Key> = Vec::new();
        if is_key_down(KeyCode::Up) {
            key_q.push(Key::Up);
        }
        if is_key_down(KeyCode::Down) {
            key_q.push(Key::Down);
        }
        if is_key_down(KeyCode::Left) {
            key_q.push(Key::Left);
        }
        if is_key_down(KeyCode::Right) {
            key_q.push(Key::Right);
        }

        // push input and world to game
        split_screen.top_screen.input(&key_q);

        ////////////////////////////////////////////////////
        // Mouse Input and Level Edit Logic               //
        ////////////////////////////////////////////////////

        // generate an input queue each frame and pass to level editor
        let mut mouse_q: Vec<Mouse> = Vec::new();
        let (mouse_x, mouse_y) = mouse_position();

        if is_mouse_button_pressed(MouseButton::Left) {
            mouse_q.push(Mouse {
                button: Button::Left,
                position: ScreenPoint {
                    x: mouse_x,
                    y: mouse_y,
                },
            });
        }
        if is_mouse_button_pressed(MouseButton::Right) {
            mouse_q.push(Mouse {
                button: Button::Right,
                position: ScreenPoint {
                    x: mouse_x,
                    y: mouse_y,
                },
            });
        }

        // push input and world to level editor
        split_screen
            .bottom_screen
            .input(&mouse_q, split_screen.split_position);

        /////////////////////////////////////////////////////
        // End of the frame                                //
        ////////////////////////////////////////////////////
        next_frame().await;
    }
}
