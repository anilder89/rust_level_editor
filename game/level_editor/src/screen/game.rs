use super::{
    basics::{Player, Point, Polygon, Vector},
    GameScreen, Key,
};

const PLYSPD: f32 = 5.0;

pub type GamePlayer = Player;
pub type GamePoint = Point;

pub struct Color {
    pub r: i16,
    pub g: i16,
    pub b: i16,
}

pub struct Wall {
    pub start: Vector,
    pub end: Vector,
    pub color: Color,
}

pub struct GameState {
    pub walls: Vec<Wall>,
    pub player: Player,
}

impl GameState {
    pub fn player_can_move(&self, new_position: &Point) -> bool {
        let mut hits_some_wall = false;
        self.walls.iter().for_each(|wall| {
            hits_some_wall = hits_some_wall
                || self.player.hits_polygon(
                    new_position,
                    &Polygon {
                        start: Point {
                            x: wall.start.x,
                            y: wall.start.y,
                        },
                        end: Point {
                            x: wall.end.x,
                            y: wall.end.y,
                        },
                    },
                );
        });
        !hits_some_wall
    }
}

impl GameState {
    pub fn input(&mut self, input: &Key) {
        let norm_drc = Vector::new(self.player.angle);
        match input {
            Key::Up => {
                let new_position = Point {
                    x: self.player.position.x + norm_drc.x * PLYSPD,
                    y: self.player.position.y + norm_drc.y * PLYSPD,
                };
                if self.player_can_move(&new_position) {
                    self.player.position = new_position;
                }
            }
            Key::Down => {
                let new_position = Point {
                    x: self.player.position.x - norm_drc.x * PLYSPD,
                    y: self.player.position.y - norm_drc.y * PLYSPD,
                };
                if self.player_can_move(&new_position) {
                    self.player.position = new_position;
                }
            }
            Key::Left => {
                self.player.angle -= 0.05;
            }
            Key::Right => {
                self.player.angle += 0.05;
            }
        }
    }

    pub fn render(&self, screen: &GameScreen, near: f32) -> Vec<f32> {
        // prep draw buffer for the screen size, we just draw into x direction
        let draw_buffer = vec![0.; screen.frame.width as usize];
        // transform everything with the (player to the origin) vector
        let origin = Vector { x: 0., y: 0. };
        let player_position = Vector {
            x: self.player.position.x,
            y: self.player.position.y,
        };
        let (trans_x, trans_y) = player_position.trans(&origin);
        // rotate everything the same amount as the player to face x drc (having angle 0)
        let rot_a = -self.player.angle;

        self.walls.iter().for_each(|wall| {
            let mut wall_start = Vector {
                x: wall.start.x + trans_x,
                y: wall.start.y + trans_y,
            };
            wall_start.rotate(rot_a);

            let mut wall_end = Vector {
                x: wall.end.x + trans_x,
                y: wall.end.y + trans_y,
            };
            wall_end.rotate(rot_a);

            // calculate intersection points on the screen
            let _y_s_i = near * wall_start.y / wall_start.x;
            let _y_e_i = near * wall_end.y / wall_end.x;

            // calculate near-screen to pixel size
            // screen size 2*near for fov=1, between -near and near
            // let pixel_size = screen.frame.width / (2. * near);
        });
        draw_buffer
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rendering() {
        todo!("Fill me!");
    }
    #[test]
    fn player_movement() {
        let test_level = GameState {
            walls: vec![
                Wall {
                    start: Point { x: -125.0, y: 77.0 },
                    end: Point { x: -89.0, y: -14.0 },
                    color: Color { r: 0, g: 0, b: 0 },
                },
                Wall {
                    start: Point { x: -89.0, y: -14.0 },
                    end: Point { x: -33.0, y: -74.0 },
                    color: Color { r: 0, g: 0, b: 0 },
                },
                Wall {
                    start: Point { x: -33.0, y: -74.0 },
                    end: Point { x: 56.0, y: -128.0 },
                    color: Color { r: 0, g: 0, b: 0 },
                },
                Wall {
                    start: Point { x: 56.0, y: -128.0 },
                    end: Point { x: 125.0, y: -16.0 },
                    color: Color { r: 0, g: 0, b: 0 },
                },
                Wall {
                    start: Point { x: 125.0, y: -16.0 },
                    end: Point { x: 171.0, y: 58.0 },
                    color: Color { r: 0, g: 0, b: 0 },
                },
                Wall {
                    start: Point { x: 171.0, y: 58.0 },
                    end: Point { x: 75.0, y: 77.0 },
                    color: Color { r: 0, g: 0, b: 0 },
                },
                Wall {
                    start: Point { x: -22.0, y: 76.0 },
                    end: Point { x: -125.0, y: 77.0 },
                    color: Color { r: 0, g: 0, b: 0 },
                },
            ],
            player: Player {
                position: Point { x: 0.0, y: 0.0 },
                angle: 0.0,
            },
        };

        let test_target_positions = vec![
            Point { x: 10., y: 10. },
            Point { x: 100., y: 100. },
            Point { x: -150., y: 10. },
            Point { x: -10., y: -8. },
            Point { x: 2., y: -30. },
            Point { x: -5., y: 9. },
            Point { x: -100., y: -100. },
            Point { x: 3., y: 2. },
            Point { x: -10., y: -8. },
            Point { x: 0., y: 800. },
        ];

        let results_to_test = vec![
            true, false, false, true, true, true, false, true, true, true,
        ];

        test_target_positions
            .iter()
            .enumerate()
            .for_each(|(index, position)| {
                let calculated_result = test_level.player_can_move(position);
                assert_eq!(
                    calculated_result, results_to_test[index],
                    "Boolean error for {}, result being: {} at index {}",
                    calculated_result, results_to_test[index], index
                );
            });
    }
}
