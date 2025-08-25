use crate::draw::draw_block;
use piston_window::{Context, G2d, types::Color};
use std::collections::LinkedList;

const SNAKE_COLOR: Color = [0.0, 0.8, 0.0, 1.0];

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Block {
    fn from(tuple: (i32, i32)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn forward(&self, mut block: Block) -> Block {
        match *self {
            Direction::Up => block.y -= 1,
            Direction::Down => block.y += 1,
            Direction::Left => block.x -= 1,
            Direction::Right => block.x += 1,
        };

        block
    }

    pub fn opposite(&self) -> Self {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn with_detail(x: i32, y: i32, direction: Direction, length: u16) -> Self {
        assert!(length > 0, "length must be positive");
        let mut body: LinkedList<Block> = LinkedList::new();
        let mut last_block = Block { x, y };

        for _ in 0..length {
            body.push_front(last_block.clone());
            last_block = direction.forward(last_block);
        }

        Self {
            direction,
            body,
            tail: None,
        }
    }

    pub fn new(x: i32, y: i32) -> Self {
        Self::with_detail(x, y, Direction::Right, 3)
    }

    pub fn draw(&self, ctx: &Context, graphics_buf: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, ctx, graphics_buf);
        }
    }

    pub fn head(&self) -> &Block {
        self.body.front().expect("empty body")
    }

    pub fn pop(&mut self) -> Option<Block> {
        self.body.pop_back()
    }

    pub fn move_forward(&mut self, direction: Option<Direction>) {
        if let Some(direction) = direction {
            self.direction = direction;
        }

        let next_block = self.direction.forward(self.head().clone());
        self.body.push_front(next_block);

        self.tail = self.pop();
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, direction: Option<Direction>) -> Block {
        direction
            .unwrap_or(self.direction)
            .forward(self.head().clone())
    }

    pub fn restore_tail(&mut self) {
        let block = self.tail.clone().expect("empty body");
        self.body.push_back(block);
    }

    pub fn tail_overlaps(&self, x: i32, y: i32) -> bool {
        //        let mut idx = 1;

        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            //            if idx == self.body.len() - 1 {
            //                break;
            //            }
            //            idx += 1;
        }

        false
    }
}
