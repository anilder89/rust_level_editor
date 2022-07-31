use super::{
    basics::{Player, Point, Polygon, Vector},
    GameScreen, Key,
};

const PLYSPD: f32 = 5.0;

pub type GamePlayer = Player;
pub type GamePoint = Point;
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
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
        for wall in self.walls.iter() {
            if self.player.hits_polygon(
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
            ) {
                return false;
            }
        }

        true
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

    pub fn render(&self, screen: &GameScreen, near: f32, far: f32) -> Vec<(Color, f32)> {
        // prep draw buffer for the screen size, we just draw into x direction
        let mut draw_buffer = vec![
            (
                Color {
                    r: 0.,
                    b: 0.,
                    g: 0.
                },
                -1.
            );
            screen.frame.width as usize
        ];

        // transform everything with the (player to the origin) vector
        let (trans_x, trans_y) = self.player.position.trans(&Vector { x: 0., y: 0. });

        self.walls.iter().for_each(|wall| {
            let mut wall_start = Vector {
                x: wall.start.x - trans_x,
                y: wall.start.y - trans_y,
            };
            wall_start.rotate(-self.player.angle);

            let mut wall_end = Vector {
                x: wall.end.x - trans_x,
                y: wall.end.y - trans_y,
            };
            wall_end.rotate(-self.player.angle);

            let mut wall_poly = Polygon {
                start: Vector {
                    x: wall_start.x,
                    y: wall_start.y,
                },
                end: Vector {
                    x: wall_end.x,
                    y: wall_end.y,
                },
            };

            // clip everything behind and too far
            if wall_poly.clip_cohen((near, far), (-far / 2., far / 2.)) {
                let (real_wall_start, real_wall_end) = if wall_poly.end.y > wall_poly.start.y {
                    (wall_poly.start, wall_poly.end)
                } else {
                    (wall_poly.end, wall_poly.start)
                };

                // calculate intersection points on the screen
                let y_s_i = near * real_wall_start.y / real_wall_start.x;
                let y_e_i = near * real_wall_end.y / real_wall_end.x;

                // for abs interpolation
                let d_x = real_wall_end.x - real_wall_start.x;
                let d_y = real_wall_end.y - real_wall_start.y;

                // clip the projection to screen size
                let mut clip_screen = Polygon {
                    start: Point { x: near, y: y_s_i },
                    end: Point { x: near, y: y_e_i },
                };
                if clip_screen.clip_line((-near, near)) {
                    // calculate near-screen to pixel size
                    // screen size 2*near for fov=1, between -near and near
                    let pixel_size = screen.frame.width / (2. * near);
                    let pixel_start = clip_screen.start.y * pixel_size;
                    let pixel_end = clip_screen.end.y * pixel_size;

                    let (mut pixel, limit) = if pixel_start < pixel_end {
                        (pixel_start, pixel_end)
                    } else {
                        (pixel_end, pixel_start)
                    };
                    while pixel < limit {
                        let k = (pixel / pixel_size) / near;
                        let p_abs =
                            (real_wall_start.y - real_wall_start.x * d_y / d_x) / (k - d_y / d_x);
                        let pixel_index = (pixel + screen.frame.width / 2.) as usize;

                        if draw_buffer[pixel_index].1 + 1. < f32::EPSILON
                            || draw_buffer[pixel_index].1 > p_abs
                        {
                            draw_buffer[pixel_index].1 = p_abs;
                            draw_buffer[pixel_index].0 = wall.color;
                        }
                        pixel += 1.;
                    }
                }
            }
        });
        draw_buffer
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rendering() {
        // load level
        // TODO: load level from file
        let _test_level = GameState {
            walls: vec![
                Wall {
                    start: Point { x: -125.0, y: 77.0 },
                    end: Point { x: -89.0, y: -14.0 },
                    color: Color {
                        r: 0.,
                        g: 0.,
                        b: 0.,
                    },
                },
                Wall {
                    start: Point { x: -89.0, y: -14.0 },
                    end: Point { x: -33.0, y: -74.0 },
                    color: Color {
                        r: 0.,
                        g: 0.,
                        b: 0.,
                    },
                },
                Wall {
                    start: Point { x: -33.0, y: -74.0 },
                    end: Point { x: 56.0, y: -128.0 },
                    color: Color {
                        r: 0.,
                        g: 0.,
                        b: 0.,
                    },
                },
                Wall {
                    start: Point { x: 56.0, y: -128.0 },
                    end: Point { x: 125.0, y: -16.0 },
                    color: Color {
                        r: 0.,
                        g: 0.,
                        b: 0.,
                    },
                },
                Wall {
                    start: Point { x: 125.0, y: -16.0 },
                    end: Point { x: 171.0, y: 58.0 },
                    color: Color {
                        r: 0.,
                        g: 0.,
                        b: 0.,
                    },
                },
                Wall {
                    start: Point { x: 171.0, y: 58.0 },
                    end: Point { x: 75.0, y: 77.0 },
                    color: Color {
                        r: 0.,
                        g: 0.,
                        b: 0.,
                    },
                },
                Wall {
                    start: Point { x: -22.0, y: 76.0 },
                    end: Point { x: -125.0, y: 77.0 },
                    color: Color {
                        r: 0.,
                        g: 0.,
                        b: 0.,
                    },
                },
            ],
            player: Player {
                position: Point { x: 0.0, y: 0.0 },
                angle: 0.0,
            },
        };

        // check total number of pixels, should be screen.width
        // check pixel generated into player direction, should be all "FULL"
        // check pixel generated into y direction, some should be "EMPTY"
        todo!("See comments!");
    }
    #[test]
    fn player_movement() {
        // load level
        // TODO: load level from file
        let test_level = GameState {
            walls: vec![
                Wall {
                    start: Point { x: -125.0, y: 77.0 },
                    end: Point { x: -89.0, y: -14.0 },
                    color: Color {
                        r: 0.,
                        g: 0.,
                        b: 0.,
                    },
                },
                Wall {
                    start: Point { x: -89.0, y: -14.0 },
                    end: Point { x: -33.0, y: -74.0 },
                    color: Color {
                        r: 0.,
                        g: 0.,
                        b: 0.,
                    },
                },
                Wall {
                    start: Point { x: -33.0, y: -74.0 },
                    end: Point { x: 56.0, y: -128.0 },
                    color: Color {
                        r: 0.,
                        g: 0.,
                        b: 0.,
                    },
                },
                Wall {
                    start: Point { x: 56.0, y: -128.0 },
                    end: Point { x: 125.0, y: -16.0 },
                    color: Color {
                        r: 0.,
                        g: 0.,
                        b: 0.,
                    },
                },
                Wall {
                    start: Point { x: 125.0, y: -16.0 },
                    end: Point { x: 171.0, y: 58.0 },
                    color: Color {
                        r: 0.,
                        g: 0.,
                        b: 0.,
                    },
                },
                Wall {
                    start: Point { x: 171.0, y: 58.0 },
                    end: Point { x: 75.0, y: 77.0 },
                    color: Color {
                        r: 0.,
                        g: 0.,
                        b: 0.,
                    },
                },
                Wall {
                    start: Point { x: -22.0, y: 76.0 },
                    end: Point { x: -125.0, y: 77.0 },
                    color: Color {
                        r: 0.,
                        g: 0.,
                        b: 0.,
                    },
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
