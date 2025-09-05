use std::collections::LinkedList;

use log::info;

use crate::{Direction, Position};

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
    // prev_direction: Direction,
    list: LinkedList<SnakeNode>,
}

#[allow(dead_code)]
impl Snake {
    pub fn new(direction: Direction, head: SnakeNode) -> Self {
        Snake {
            direction,
            list: LinkedList::from([head]),
            // prev_direction: Direction::Up,
        }
    }

    pub fn update_positions(&mut self) {
        info!("====== START UPDATING POSITIONS =======");
        info!("[NODES NUMBER], {}", self.list.len());
        let mut iterator = self.list.iter_mut();
        let head = iterator.next().unwrap();
        let position = head.get_position_mut();
        let mut prev_pos = *position;
        match self.direction {
            Direction::Up => position.decrement_line(1), // Goes UP
            Direction::Down => position.increment_line(1),
            Direction::Left => position.decrement_col(1),
            Direction::Right => position.increment_col(1),
        }
        info!("[DIRECTION], {:?}", self.direction);
        info!("[HEAD], before: {:?}, after: {:?}", prev_pos, position);

        for node in iterator {
            let current_position = *node.get_position();
            node.set_position(prev_pos);
            info!(
                "[NODE], before: {:?}, after: {:?}",
                current_position, prev_pos
            );
            prev_pos = current_position;
        }
        info!("====== END UPDATING POSITIONS =======");
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

    pub fn add_tail(&mut self) {
        let current_tail = self.list.back_mut().unwrap();
        let mut tail_position = *current_tail.get_position();
        match self.direction {
            Direction::Up => tail_position.increment_line(1),
            Direction::Down => tail_position.decrement_line(1),
            Direction::Right => tail_position.decrement_col(1),
            Direction::Left => tail_position.increment_col(1),
        }
        self.list.push_back(SnakeNode::new(tail_position));
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
        Self::new(Direction::Up, SnakeNode::default())
    }
}
