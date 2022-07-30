use super::{basics::*, GameScreen, Key};

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
    fn player_can_move(&self, new_position: &Point) -> bool {
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
    #[test]
    fn rendering() {
        todo!("Fill me!");
    }
    #[test]
    fn player_movement() {
        todo!("Fill me!");
    }
}
