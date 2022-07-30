use super::basics::*;
// for the level editor it is fine to use
// the macroquad lib as much as possibble
use macroquad::prelude::{Color, GREEN};

pub const NR: f32 = 5.;
pub const NS: i32 = -1;

pub type LevelPlayer = Player;
pub type LevelPoint = Point;

pub struct Node {
    pub position: LevelPoint,
    pub radius: f32,
    pub selected: bool,
}
impl Node {
    // check if p is within node area
    pub fn point_on(&self, p: &LevelPoint) -> bool {
        let dst = ((self.position.x - p.x) * (self.position.x - p.x)
            + (self.position.y - p.y) * (self.position.y - p.y))
            .sqrt();
        if dst as f32 <= self.radius {
            return true;
        }
        false
    }
    // check if a given node is in a given node list
    pub fn index_node(&self, list: &[Node]) -> i32 {
        let mut index = NS;
        for element in list.iter() {
            index += 1;
            if self.point_on(&element.position) {
                return index;
            }
        }
        NS
    }
}
pub struct Line {
    pub start: Vector,
    pub end: Vector,
    pub color: Color,
}

pub struct LevelState {
    pub nodes: Vec<Node>,
    pub lines: Vec<Line>,
    pub player: LevelPlayer,
    pub selected: i32,
}

impl LevelState {
    // input is a left click on the level screen
    pub fn left_on(&mut self, position: LevelPoint) {
        // is it on an empty space?
        // create a phantom node
        let p_node = Node {
            position,
            radius: NR,
            selected: false,
        };
        let is_selecting = p_node.index_node(&self.nodes);
        // attemp to select
        if is_selecting != NS {
            // check if it was selected
            if self.selected == is_selecting {
                // remove selection
                self.nodes[is_selecting as usize].selected = false;
                self.selected = NS;
            } else {
                // look if it is the first selection
                if self.selected == NS {
                    self.nodes[is_selecting as usize].selected = true;
                    self.selected = is_selecting;
                } else {
                    // time to add a line
                    let start_position = Point {
                        x: self.nodes[self.selected as usize].position.x,
                        y: self.nodes[self.selected as usize].position.y,
                    };
                    let end_position = Point {
                        x: self.nodes[is_selecting as usize].position.x,
                        y: self.nodes[is_selecting as usize].position.y,
                    };

                    self.lines.push(Line {
                        start: start_position,
                        end: end_position,
                        color: GREEN,
                    });

                    // reset cycle
                    self.nodes[self.selected as usize].selected = false;
                    self.selected = NS;
                }
            }
        } else {
            // click on empty space, just add another node
            self.nodes.push(Node {
                position: p_node.position,
                radius: NR,
                selected: false,
            });
        }
    }
    // input is a right click on the level screen
    pub fn right_on(&mut self, position: LevelPoint) {
        // is it on an empty space?
        // create phantom node
        let p_node = Node {
            position,
            radius: NR,
            selected: false,
        };
        let is_selecting = p_node.index_node(&self.nodes);
        // attemp to delete
        if is_selecting != NS {
            // remove the node from the list
            let removed_node = self.nodes.remove(is_selecting as usize);
            // was it a selected one?
            if removed_node.selected {
                self.selected = NS;
            }
            // remove all lines corresponding to the node
            self.lines.retain(|line| {
                let start_position = (line.start.x - removed_node.position.x).abs() < f32::EPSILON
                    && line.start.y == removed_node.position.y;
                let end_position = (line.end.x - removed_node.position.x).abs() < f32::EPSILON
                    && (line.end.y - removed_node.position.y).abs() < f32::EPSILON;

                // if no node at any end of the line, keep it
                !(start_position || end_position)
            });
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn all_lines_on_nodes() {
        todo!("Fill me!");
    }
}
