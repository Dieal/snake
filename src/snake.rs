use std::collections::LinkedList;

use log::info;

use crate::Border;
use crate::Direction;
use crate::Position;

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct SnakeNode {
    position: Position,
}

#[allow(dead_code)]
impl SnakeNode {
    pub fn new(position: Position) -> Self {
        SnakeNode { position }
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn get_position_mut(&mut self) -> &mut Position {
        &mut self.position
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }
}

#[derive(Debug)]
pub struct Snake {
    direction: Direction,
    list: LinkedList<SnakeNode>,
    boundaries: Option<Border>,
}

#[allow(dead_code)]
impl Snake {
    pub fn new(direction: Direction, mut head: SnakeNode, boundaries: Option<Border>) -> Self {
        if let Some(boundaries) = boundaries {
            head.get_position_mut().set_boundaries(boundaries);
        }

        Snake {
            direction,
            list: LinkedList::from([head]),
            boundaries,
        }
    }

    pub fn set_boundaries(&mut self, boundaries: Border) {
        self.boundaries = Some(boundaries);
        for node in self.list.iter_mut() {
            node.get_position_mut().set_boundaries(boundaries);
        }
    }

    // Checks if the head is blocked by another node and can't go in that direction
    pub fn can_go_in_direction(&self, direction: Direction) -> bool {
        let mut iterator = self.list.iter();
        if let (Some(head), Some(node)) = (iterator.next(), iterator.next()) {
            let head_position = head.get_position();
            let node_position = node.get_position();
            let same_line: bool = head_position.line == node_position.line;
            let same_column: bool = head_position.column == node_position.column;

            // Returns false if it can't go in that direction
            match direction {
                Direction::Up => return !(same_column && head_position.line - 1 == node_position.line),
                Direction::Down => return !(same_column && head_position.line + 1 == node_position.line),
                Direction::Left => return !(same_line && head_position.column - 1 == node_position.column),
                Direction::Right => return !(same_line && head_position.column + 1 == node_position.column),
            }
        }
        true
    }

    pub fn get_positions(&self) -> Vec<Position> {
        let mut positions: Vec<Position> = Vec::new();
        for node in self.list.iter() {
            positions.push(*node.get_position());
        }
        positions
    }

    pub fn update_positions(&mut self) {
        info!("==== START UPDATING POSITION ====");
        let direction = self.direction;
        let mut iterator = self.list.iter_mut();
        if let Some(head) = iterator.next() {
            let head_position = head.get_position_mut();
            let mut previous_position: Position = *head_position;
            match direction {
                Direction::Up => head_position.decrement_line(1),
                Direction::Down => head_position.increment_line(1),
                Direction::Right => head_position.increment_col(1),
                Direction::Left => head_position.decrement_col(1),
            }
            info!("Direction {:?}", direction);
            info!("[HEAD] Before: {:?}, After: {:?}", previous_position, head_position);
            for node in iterator {
                let temp_position = *node.get_position();
                node.set_position(previous_position);
                info!("[NODE] Before: {:?}, After: {:?}", previous_position, temp_position);
                previous_position = temp_position;
            }
        }
        info!("==== END UPDATING POSITION ====");
    }

    pub fn get_head(&mut self) -> Option<&mut SnakeNode> {
        self.list.front_mut()
    }

    pub fn get_tail(&mut self) -> Option<&mut SnakeNode> {
        self.list.back_mut()
    }

    pub fn get_list(&self) -> &LinkedList<SnakeNode> {
        &self.list
    }
    
    pub fn is_eating_tail(&self) -> bool {
        let mut iter = self.list.iter();
        let head = iter.next();
        if let Some(head) = head {
            let position = head.get_position();
            for node in iter {
                if node.get_position().eq(position) {
                    return true;
                }
            }
        }
        false
    }

    pub fn add_tails(&mut self, count: u32) {
        for _ in 0..count {
            self.add_tail();
        }
    }

    pub fn add_tail(&mut self) {
        let current_tail = self.get_tail();
        if let Some(current_tail) = current_tail {
            let prev_position: Position = *current_tail.get_position();
            let mut position: Position = prev_position;
            match self.direction {
                Direction::Up => position.increment_line(1),
                Direction::Down => position.decrement_line(1),
                Direction::Right => position.decrement_col(1),
                Direction::Left => position.increment_col(1),
            }
            let new_tail = SnakeNode::new(position);
            info!("[Added tail] Direction: {:?}, Before: {:?}, After: {:?}", self.direction, prev_position, position);
            self.list.push_back(new_tail);
        }
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn change_direction(&mut self, direction: Direction) {
        // self.prev_direction = self.direction;
        self.direction = direction;
    }
}

impl Default for Snake {
    fn default() -> Self {
        Self::new(Direction::Up, SnakeNode::default(), None)
    }
}
