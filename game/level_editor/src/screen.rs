// need vector for player direction drawing
pub mod basics;
use basics::{Point, Polygon, Vector};

// load the game implementation
pub mod game;
use game::GamePlayer;
use game::GamePoint;
use game::GameState;

// load the level editor implementation
pub mod editor;
use editor::LevelPlayer;
use editor::LevelPoint;
use editor::LevelState;

// define some const for the worlds
const PLYX: f32 = 0.;
const PLYY: f32 = 0.;
const PLYA: f32 = 0.;
const LT: f32 = 5.;

// add basic definitions for main
pub enum Key {
    Up,
    Down,
    Left,
    Right,
}
pub enum Button {
    Left,
    Right,
}
pub struct Mouse {
    pub button: Button,
    pub position: ScreenPoint,
}

pub struct Screen {
    pub width: f32,
    pub height: f32,
}
pub struct ScreenPoint {
    pub x: f32,
    pub y: f32,
}

// screen definitions for main
pub struct GameScreen {
    pub frame: Screen,
    pub game_state: GameState,
}
impl GameScreen {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            frame: Screen { width, height },
            game_state: GameState {
                walls: Vec::new(),
                player: GamePlayer {
                    position: GamePoint { x: PLYX, y: PLYY },
                    angle: PLYA,
                },
            },
        }
    }

    pub fn draw(&self) {
        let draw_buffer = self.game_state.render(self, 10., 800.);

        // the size is "fix"
        draw_buffer
            .iter()
            .enumerate()
            .for_each(|(draw_index, draw_buffer)| {
                let buffer_color = draw_buffer.0;
                // draw left to right
                let scale = 8. * 1600. / draw_buffer.1;
                let cap_scale = if scale > self.frame.height / 2. {
                    self.frame.height / 2.
                } else {
                    scale
                };
                macroquad::prelude::draw_rectangle(
                    draw_index as f32,
                    self.frame.height / 2.,
                    1.,
                    -cap_scale,
                    macroquad::prelude::Color {
                        r: buffer_color.r,
                        g: buffer_color.g,
                        b: buffer_color.b,
                        a: 1.0,
                    },
                );
                macroquad::prelude::draw_rectangle(
                    draw_index as f32,
                    self.frame.height / 2.,
                    1.,
                    cap_scale,
                    macroquad::prelude::Color {
                        r: buffer_color.r,
                        g: buffer_color.g,
                        b: buffer_color.b,
                        a: 1.0,
                    },
                );
            });
    }

    pub fn input(&mut self, key_q: &[Key]) {
        key_q.iter().for_each(|input| {
            self.game_state.input(input);
        });
    }
}

pub struct LevelScreen {
    pub frame: Screen,
    pub level_state: LevelState,
}
impl LevelScreen {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            frame: Screen { width, height },
            level_state: LevelState {
                nodes: Vec::new(),
                lines: Vec::new(),
                player: LevelPlayer {
                    position: LevelPoint { x: PLYX, y: PLYY },
                    angle: PLYA,
                },
                selected: editor::NS,
            },
        }
    }

    // draw logic for the level editor
    pub fn draw(&self, y_correction: f32) {
        // draw everything in the center of the map
        // and rotate everything
        let map_center = Vector {
            x: self.frame.width / 2.0,
            y: self.frame.height / 2.0,
        };
        let player_position = Vector {
            x: self.level_state.player.position.x,
            y: self.level_state.player.position.y,
        };
        let (trans_x, trans_y) = player_position.trans(&map_center);
        // draw player
        macroquad::prelude::draw_circle(
            map_center.x,
            map_center.y + y_correction,
            editor::NR,
            macroquad::prelude::WHITE,
        );

        let norm_drc = Vector::new(self.level_state.player.angle);

        macroquad::prelude::draw_line(
            map_center.x,
            map_center.y + y_correction,
            map_center.x + 20. * norm_drc.x,
            map_center.y + y_correction + 20. * norm_drc.y,
            LT,
            macroquad::prelude::YELLOW,
        );
        // draw nodes
        self.level_state.nodes.iter().for_each(|node| {
            let node_on_map = Vector {
                x: node.position.x - trans_x,
                y: node.position.y - trans_y,
            };

            let correct_y = node_on_map.y + y_correction;
            let mut color = macroquad::prelude::RED;
            if node.selected {
                color = macroquad::prelude::BLUE;
            }
            // clip to the screen size
            if node_on_map.x > 0.
                && node_on_map.x < self.frame.width
                && correct_y > y_correction
                && correct_y < 2. * y_correction
            {
                macroquad::prelude::draw_circle(node_on_map.x, correct_y, node.radius, color);
            }
        });
        // draw lines
        self.level_state.lines.iter().for_each(|line| {
            let line_on_map_start = Vector {
                x: line.start.x - trans_x,
                y: line.start.y - trans_y,
            };
            let line_on_map_end = Vector {
                x: line.end.x - trans_x,
                y: line.end.y - trans_y,
            };

            let correct_start_y = line_on_map_start.y + y_correction;
            let correct_end_y = line_on_map_end.y + y_correction;
            let mut line_to_clip = Polygon {
                start: Point {
                    x: line_on_map_start.x,
                    y: correct_start_y,
                },
                end: Point {
                    x: line_on_map_end.x,
                    y: correct_end_y,
                },
            };
            // clip to the screen size
            if line_to_clip.clip_cohen((0., self.frame.width), (y_correction, 2. * y_correction)) {
                macroquad::prelude::draw_line(
                    line_to_clip.start.x,
                    line_to_clip.start.y,
                    line_to_clip.end.x,
                    line_to_clip.end.y,
                    LT,
                    line.color,
                );
            }
        });
    }

    pub fn input(&mut self, mouse_q: &[Mouse], y_correction: f32) {
        for input in mouse_q.iter() {
            // only accept input on the LevelScreen
            if input.position.y > y_correction {
                // do correction because clicks are "relative" to the player
                // draw everything in the center of the map
                let map_center = basics::Vector {
                    x: self.frame.width / 2.0,
                    y: self.frame.height / 2.0,
                };
                let player_position = basics::Vector {
                    x: self.level_state.player.position.x,
                    y: self.level_state.player.position.y,
                };
                let (trans_x, trans_y) = player_position.trans(&map_center);
                // deploy to the editor for the rest of the logic
                let correct_y = input.position.y + trans_y - y_correction;
                match input.button {
                    Button::Left => {
                        self.level_state.left_on(LevelPoint {
                            x: input.position.x + trans_x,
                            y: correct_y,
                        });
                    }
                    Button::Right => {
                        self.level_state.right_on(LevelPoint {
                            x: input.position.x + trans_x,
                            y: correct_y,
                        });
                    }
                }
            }
        }
    }
}

pub struct SplitScreen {
    pub top_screen: GameScreen,
    pub bottom_screen: LevelScreen,
    pub split_position: f32,
}
impl SplitScreen {
    pub fn new(top_screen: GameScreen, bottom_screen: LevelScreen, split_position: f32) -> Self {
        Self {
            top_screen,
            bottom_screen,
            split_position,
        }
    }

    pub fn sync_screen_states(&mut self) {
        self.bottom_screen.level_state.player.angle = self.top_screen.game_state.player.angle;
        self.bottom_screen.level_state.player.position.x =
            self.top_screen.game_state.player.position.x;
        self.bottom_screen.level_state.player.position.y =
            self.top_screen.game_state.player.position.y;

        // TODO: This is kind of brute force sync needs to be improved
        let mut new_walls: Vec<game::Wall> = Vec::new();
        self.bottom_screen
            .level_state
            .lines
            .iter()
            .for_each(|wall| {
                new_walls.push(game::Wall {
                    start: game::GamePoint {
                        x: wall.start.x,
                        y: wall.start.y,
                    },
                    end: game::GamePoint {
                        x: wall.end.x,
                        y: wall.end.y,
                    },
                    color: game::Color {
                        r: wall.color.r,
                        g: wall.color.g,
                        b: wall.color.b,
                    },
                });
            });
        self.top_screen.game_state.walls = new_walls;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn coord_screen() {
        todo!("Fill me!");
    }
    #[test]
    fn screen_coord() {
        todo!("Fill me!");
    }
    #[test]
    fn screen_sync() {
        todo!("Fill me!");
    }
}
