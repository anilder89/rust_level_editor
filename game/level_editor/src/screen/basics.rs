#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub type Vector = Point;

// outcodes for cohen
const CLIPLEFT: i32 = 1;
const CLIPRIGHT: i32 = 2;
const CLIPLOWER: i32 = 4;
const CLIPUPPER: i32 = 8;

#[derive(Debug)]
pub struct Polygon {
    pub start: Vector,
    pub end: Vector,
}

impl Polygon {
    // cohen like
    pub fn clip_line(&mut self, (min_y, max_y): (f32, f32)) -> bool {
        let mut start_flag = 0;
        let mut end_flag = 0;

        if self.start.y < min_y {
            start_flag = CLIPLEFT;
        } else if self.start.y > max_y {
            start_flag = CLIPRIGHT;
        }
        if self.end.y < min_y {
            end_flag = CLIPLEFT;
        } else if self.end.y > max_y {
            end_flag = CLIPRIGHT;
        }

        // both points are inside the line
        if (start_flag | end_flag) == 0 {
            return true;
        }
        // both points are on the same side
        // if their & result is not 0
        else if (start_flag & end_flag) != 0 {
            return false;
        }
        // At least one of the points is not in the range
        if (start_flag & CLIPLEFT) != 0 {
            self.start.y = min_y;
        } else if (start_flag & CLIPRIGHT) != 0 {
            self.start.y = max_y;
        }

        if (end_flag & CLIPLEFT) != 0 {
            self.end.y = min_y;
        } else if (end_flag & CLIPRIGHT) != 0 {
            self.end.y = max_y;
        }

        true
    }

    // helper for cohen
    fn outcode(
        (p_x, p_y): (f32, f32),
        (min_x, max_x): (f32, f32),
        (min_y, max_y): (f32, f32),
    ) -> i32 {
        let mut code = 0;

        // start point
        if p_y < min_y {
            code = CLIPLOWER;
        } else if p_y > max_y {
            code = CLIPUPPER;
        }
        if p_x < min_x {
            code |= CLIPLEFT;
        } else if p_x > max_x {
            code |= CLIPRIGHT;
        }

        code
    }

    // Cohen Surtherland
    pub fn clip_cohen(&mut self, (min_x, max_x): (f32, f32), (min_y, max_y): (f32, f32)) -> bool {
        let mut start_flag =
            Self::outcode((self.start.x, self.start.y), (min_x, max_x), (min_y, max_y));
        let mut end_flag = Self::outcode((self.end.x, self.end.y), (min_x, max_x), (min_y, max_y));

        // this loop is not going to be done more than twice
        // one of the points of the line is not in the screen
        loop {
            // both points in the rect
            if (start_flag | end_flag) == 0 {
                return true;
            }
            // both points on upper/lower/left/right side
            else if (start_flag & end_flag) != 0 {
                return false;
            }
            // at least one point is inside rect

            // pick one side to clip
            let mut x = 0.;
            let mut y = 0.;

            let outcode = if start_flag > end_flag {
                start_flag
            } else {
                end_flag
            };

            let x0 = self.start.x;
            let x1 = self.end.x;
            let y0 = self.start.y;
            let y1 = self.end.y;

            if outcode & CLIPUPPER != 0 {
                x = x0 + (x1 - x0) * (max_y - y0) / (y1 - y0);
                y = max_y;
            } else if outcode & CLIPLOWER != 0 {
                x = x0 + (x1 - x0) * (min_y - y0) / (y1 - y0);
                y = min_y;
            } else if outcode & CLIPRIGHT != 0 {
                y = y0 + (y1 - y0) * (max_x - x0) / (x1 - x0);
                x = max_x;
            } else if outcode & CLIPLEFT != 0 {
                y = y0 + (y1 - y0) * (min_x - x0) / (x1 - x0);
                x = min_x;
            }

            // move the line points
            if outcode == start_flag {
                self.start.x = x;
                self.start.y = y;
                start_flag =
                    Self::outcode((self.start.x, self.start.y), (min_x, max_x), (min_y, max_y));
            } else {
                self.end.x = x;
                self.end.y = y;
                end_flag = Self::outcode((self.end.x, self.end.y), (min_x, max_x), (min_y, max_y));
            }
        }
    }
}

pub struct Player {
    pub position: Vector,
    pub angle: f32,
}

impl Player {
    pub fn hits_polygon(&self, desired: &Point, polygon: &Polygon) -> bool {
        let p0 = &polygon.start;
        let p1 = &polygon.end;
        // imagine player->desired as a line
        let i0 = &self.position;
        let i1 = desired;

        let d_x = p1.x - p0.x;
        let d_y = p1.y - p0.y;

        let p_x = i1.x - i0.x;
        let p_y = i1.y - i0.y;

        // for hitting following must be true
        // t*(p1-p0) + p0 = b*(i1-i0) + i0 with 0<=b<=1 (bc of param)
        // helper vars

        let m = i0.x - p0.x;
        let n = i0.y - p0.y;

        let b = (d_x * n - d_y * m) / (d_y * p_x - d_x * p_y);
        let t = (i0.x + b * (i1.x - i0.x) - p0.x) / d_x;

        (0. ..=1.).contains(&b) && (0. ..=1.).contains(&t)
    }
}

impl Vector {
    pub fn new(angle: f32) -> Self {
        Self {
            x: angle.cos(),
            y: angle.sin(),
        }
    }
    pub fn rotate(&mut self, angle: f32) {
        let x = self.x * angle.cos() - self.y * angle.sin();
        let y = self.x * angle.sin() + self.y * angle.cos();
        self.x = x;
        self.y = y;
    }
    pub fn trans(&self, origin_vector: &Vector) -> (f32, f32) {
        let vector_trans = Vector {
            x: self.x - origin_vector.x,
            y: self.y - origin_vector.y,
        };
        (vector_trans.x, vector_trans.y)
    }
}

fn compare_epsilon(first: f32, second: f32, epsilon: f32) -> bool {
    (first - second).abs() < epsilon
}

const EPSILON: f32 = 10e-4; // precision for the tests

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        compare_epsilon(self.x, other.x, EPSILON) && compare_epsilon(self.y, other.y, EPSILON)
    }
}

impl PartialEq for Polygon {
    fn eq(&self, other: &Self) -> bool {
        compare_epsilon(self.start.x, other.start.x, EPSILON)
            && compare_epsilon(self.start.y, other.start.y, EPSILON)
            && compare_epsilon(self.end.x, other.end.x, EPSILON)
            && compare_epsilon(self.end.y, other.end.y, EPSILON)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_rotations() {
        let vectors_to_test = vec![
            Vector { x: 1., y: 0. },
            Vector { x: 0., y: 1. },
            Vector { x: 1., y: 1. },
        ];

        let rotation_angle = std::f32::consts::PI / 2.;

        // results of the vectors above after a pi angle rotation counter clockwise
        let results_to_test = vec![
            Vector { x: 0., y: 1. },
            Vector { x: -1., y: 0. },
            Vector { x: -1., y: 1. },
        ];

        vectors_to_test
            .iter()
            .enumerate()
            .for_each(move |(index, test_vector)| {
                let mut calculated_result = Vector {
                    x: test_vector.x,
                    y: test_vector.y,
                };

                calculated_result.rotate(rotation_angle);
                assert_eq!(
                    calculated_result, results_to_test[index],
                    "Rotation error for {:#?}, result being: {:#?} at index {index}",
                    calculated_result, results_to_test[index]
                );
            });
    }
    #[test]
    fn line_clipping() {
        let near = 20.;

        let lines_to_cut = vec![
            Polygon {
                start: Vector { x: near, y: 200. },
                end: Vector { x: near, y: near },
            },
            Polygon {
                start: Vector { x: near, y: 150. },
                end: Vector { x: near, y: 40. },
            },
            Polygon {
                start: Vector { x: near, y: 125. },
                end: Vector { x: near, y: -33. },
            },
            Polygon {
                start: Vector { x: near, y: 200. },
                end: Vector { x: near, y: -5. },
            },
            Polygon {
                start: Vector { x: near, y: 70. },
                end: Vector { x: near, y: 0. },
            },
            Polygon {
                start: Vector { x: near, y: 0. },
                end: Vector { x: near, y: 0. },
            },
            Polygon {
                start: Vector { x: near, y: -90. },
                end: Vector { x: near, y: 10. },
            },
            Polygon {
                start: Vector { x: near, y: -178. },
                end: Vector { x: near, y: -210. },
            },
            Polygon {
                start: Vector { x: near, y: -10. },
                end: Vector { x: near, y: -10. },
            },
            Polygon {
                start: Vector { x: near, y: -5. },
                end: Vector { x: near, y: 7. },
            },
        ];

        // results of the vectors above after a pi angle rotation counter clockwise
        let p_results_to_test = vec![
            Polygon {
                start: Vector { x: near, y: near },
                end: Vector { x: near, y: near },
            },
            Polygon {
                start: Vector { x: near, y: 150. },
                end: Vector { x: near, y: 40. },
            },
            Polygon {
                start: Vector { x: near, y: near },
                end: Vector { x: near, y: -near },
            },
            Polygon {
                start: Vector { x: near, y: near },
                end: Vector { x: near, y: -5. },
            },
            Polygon {
                start: Vector { x: near, y: near },
                end: Vector { x: near, y: 0. },
            },
            Polygon {
                start: Vector { x: near, y: 0. },
                end: Vector { x: near, y: 0. },
            },
            Polygon {
                start: Vector { x: near, y: -near },
                end: Vector { x: near, y: 10. },
            },
            Polygon {
                start: Vector { x: near, y: -178. },
                end: Vector { x: near, y: -210. },
            },
            Polygon {
                start: Vector { x: near, y: -10. },
                end: Vector { x: near, y: -10. },
            },
            Polygon {
                start: Vector { x: near, y: -5. },
                end: Vector { x: near, y: 7. },
            },
        ];

        let results_to_test = vec![true, false, true, true, true, true, true, false, true, true];

        lines_to_cut.iter().enumerate().for_each(|(index, line)| {
            let mut calculated_p_result = Polygon {
                start: Point {
                    x: line.start.x,
                    y: line.start.y,
                },
                end: Point {
                    x: line.end.x,
                    y: line.end.y,
                },
            };

            let calculated_result = calculated_p_result.clip_line((-near, near));
            assert_eq!(
                calculated_result, results_to_test[index],
                "Boolean error for {}, result being: {} at index {}",
                calculated_result, results_to_test[index], index
            );
            assert_eq!(
                calculated_p_result, p_results_to_test[index],
                "Boolean error for {:?}, result being: {:?} at index {}",
                calculated_p_result, p_results_to_test[index], index
            );
        });
    }
}
