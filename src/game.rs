use piston_window::{*, types::Color};
use rand::{rng, Rng};
use crate::snake::{Block, Direction, Snake};
use crate::draw::{draw_block, draw_rectangle};

const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const GAMEOVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

const STARTING_FOOD: Block = Block { x: 6, y: 4 };
const STARTING_POSITION: (i32, i32) = (2, 2);

pub struct Game {
    snake: Snake,

    food: Option<Block>,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            snake: Snake::new(STARTING_POSITION.0, STARTING_POSITION.1),
            food: Some(STARTING_FOOD.clone()),
            width,
            height,
            game_over: false,
            waiting_time: 0.0,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let direction = match key {
            Key::Up => Direction::Up,
            Key::Down => Direction::Down,
            Key::Left => Direction::Left,
            Key::Right => Direction::Right,
            _ => return,
        };

        if direction == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(Some(direction));
    }

    pub fn draw(&self, ctx: &Context, graphics_buf: &mut G2d) {
        self.snake.draw(ctx, graphics_buf);

        if let Some(food_block) = &self.food {
            draw_block(FOOD_COLOR, food_block.x, food_block.y, ctx, graphics_buf);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, ctx, graphics_buf);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, ctx, graphics_buf);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, ctx, graphics_buf);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, ctx, graphics_buf);

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, ctx, graphics_buf);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }

            return;
        }

        if self.food.is_none() {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let snake_head = self.snake.head();

        if let Some(food_block) = &self.food {
            if food_block == snake_head {
                self.food = None;
                self.snake.restore_tail();
            }
        }
    }

    fn is_snake_alive(&self, direction: Option<Direction>) -> bool {
        let next = self.snake.next_head(direction);

        if self.snake.tail_overlaps(next.x, next.y) {
            return false;
        }

        next.x > 0 && next.y > 0 && next.x < self.width - 1 && next.y < self.height - 1
    }

    fn add_food(&mut self) {
        let mut rng = rng();

        let mut food = Block {
            x: rng.random_range(1..self.width),
            y: rng.random_range(1..self.height)
        };

        while self.snake.tail_overlaps(food.x, food.y) {
            food.x = rng.random_range(1..self.width);
            food.y = rng.random_range(1..self.height);
        }

        self.food = Some(food);
    }

    fn update_snake(&mut self, direction: Option<Direction>) {
        if self.is_snake_alive(direction) {
            self.snake.move_forward(direction);
            self.check_eating();
        } else {
            self.game_over = true;
        }

        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(STARTING_POSITION.0, STARTING_POSITION.1);
        self.food = Some(STARTING_FOOD.clone());
        self.waiting_time = 0.0;
        self.game_over = false;
        self.waiting_time = 0.0;
    }
}

