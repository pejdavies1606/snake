//! Snake game
//!
//! Example of 2d graphics in Rust.
//!
//! Based on https://gist.github.com/AndrewJakubowicz/9972b5d46be474c186a2dc3a71326de4
//!
//! Author: @pejdavies1606

extern crate pancurses;
extern crate rand;

use itertools::{Itertools, Position};
use pancurses::*;
use std::collections::LinkedList;

struct Food {
    y: i32,
    x: i32,
    ch: char,
}

impl Food {
    fn render(&self, window: &Window) 
    {
        window.mvaddch(self.y, self.x, self.ch);
    }
    fn is_collide(&mut self, y: i32, x: i32) -> bool {
        self.y == y && self.x == x
    }
    fn update(&mut self, rows: i32, cols: i32, snake: &Snake) -> bool {
        let snake_head = snake.parts.front().unwrap();
        let eaten = self.is_collide(snake_head.0, snake_head.1);
        if eaten {
            use rand::Rng;
            use rand::thread_rng;
            let mut rng = thread_rng();
            loop {
                let new_y = rng.gen_range(0..rows);
                let new_x = rng.gen_range(0..cols);
                if !snake.is_collide(new_y, new_x) {
                    self.y = new_y;
                    self.x = new_x;
                    break;
                }
            }
        }
        eaten
    }
}

#[derive(Clone, PartialEq)]
enum Direction {
    Down,
    Right,
    Up,
    Left,
}

impl Direction {
    fn input(input: Input) -> Option<Direction> {
        match input {
            Input::Character('w') => Some(Direction::Up),
            Input::Character('a') => Some(Direction::Left),
            Input::Character('s') => Some(Direction::Down),
            Input::Character('d') => Some(Direction::Right),
            Input::Character('h') => Some(Direction::Left),
            Input::Character('j') => Some(Direction::Down),
            Input::Character('k') => Some(Direction::Up),
            Input::Character('l') => Some(Direction::Right),
            _ => None
        }
    }
}

#[derive(Clone)]
struct SnakePiece(i32, i32); // y, x

impl SnakePiece {
    fn get_visible_part(&self, pos: Position) -> char {
        match pos {
            Position::First     => '@',
            Position::Middle    => 'O',
            Position::Last      => 'o',
            Position::Only      => '@',
        }
    }
    fn is_collide_edge(&self, dir: &Direction, rows: i32, cols: i32) -> bool {
        match dir {
            Direction::Down     if self.0 >= rows - 1 => true,
            Direction::Right    if self.1 >= cols - 1 => true,
            Direction::Up       if self.0 <= 0 => true,
            Direction::Left     if self.1 <= 0 => true,
            _ => false,
        }
    }
    fn update(&mut self, dir: &Direction)
    {
        match dir {
            Direction::Down     => self.0 += 1,
            Direction::Right    => self.1 += 1,
            Direction::Up       => self.0 -= 1,
            Direction::Left     => self.1 -= 1,
        }
    }
}

struct Snake {
    parts: LinkedList<SnakePiece>,
    dir: Direction,
    just_eaten: bool,
    score: i32,
    speed: i32,
}

impl Snake {
    fn render(&self, window: &Window) {
        let visible_parts: Vec<(&SnakePiece, char)> = self.parts
            .iter()
            .with_position()
            .map(|(pos, p)| 
                (p, p.get_visible_part(pos))
            )
            .collect();
        visible_parts.iter().for_each(|p| {
            window.mvaddch(p.0.0, p.0.1, p.1);
        });
    }
    fn set_direction(&mut self, new_dir: Direction) {
        let last_dir = self.dir.clone();
        self.dir = match new_dir {
            Direction::Left     if last_dir != Direction::Right => Direction::Left,
            Direction::Down     if last_dir != Direction::Up    => Direction::Down,
            Direction::Up       if last_dir != Direction::Down  => Direction::Up,
            Direction::Right    if last_dir != Direction::Left  => Direction::Right,
            _ => last_dir.clone(),
        };
    }
    fn is_collide(&self, y: i32, x: i32) -> bool {
        self.parts.iter().any(|p| y == p.0 && x == p.1)
    }
    fn update(&mut self, rows: i32, cols: i32) -> bool {
        let mut new_head =
            (*self.parts.front().expect("Snake has no body")).clone();
        if new_head.is_collide_edge(&self.dir, rows, cols) { return false; }
        new_head.update(&self.dir);
        if self.is_collide(new_head.0, new_head.1) { return false; }
        self.parts.push_front(new_head);
        if self.just_eaten {
            self.score += 1;
            self.speed -= self.speed / 10;
            self.just_eaten = false;
        } else {
            self.parts.pop_back().unwrap();
        }
        true
    }
}

struct Game {
    rows: i32,
    cols: i32,
    snake: Snake,
    food: Food,
}

impl Game {
    fn render(&self, window: &Window) {
        window.bkgd(COLOR_PAIR(1));
        window.erase();
        window.mvaddstr(0, 0, "Snake: Help Kanka find food!");
        window.mvaddstr(1, 0, "Use wasd or hjkl to move.");
        window.mvaddstr(2, 0, "Press F1 to exit.");
        window.mvaddstr(3, 0, &format!("Score: {}", self.snake.score));
        self.food.render(window);
        self.snake.render(window);
    }
    fn input(&mut self, input: Input) {
        match Direction::input(input) {
            Some(dir) => self.snake.set_direction(dir),
            _ => (),
        }
    }
    fn update(&mut self) -> bool {
        if !self.snake.update(self.rows, self.cols) { return false; };
        self.snake.just_eaten = self.food.update(self.rows, self.cols, &self.snake);
        true
    }
}

fn main() {
    let window = initscr();

    start_color();
    use_default_colors();
    set_blink(true);

    cbreak();
    noecho();
    curs_set(0);

    init_pair(
        1,
        COLOR_WHITE,
        COLOR_BLACK);

    mousemask(ALL_MOUSE_EVENTS, None);

    window.keypad(true);
    window.clear();

    let mut game = Game{
        rows: window.get_max_y(),
        cols: window.get_max_x(),
        snake: Snake {
            parts: LinkedList::from_iter((vec![
                SnakePiece(window.get_max_y() / 2, window.get_max_x() / 2),
                SnakePiece(window.get_max_y() / 2, window.get_max_x() / 2 - 1)
            ]).into_iter()),
            dir: Direction::Right,
            just_eaten: false,
            score: 0,
            speed: 500, // ms timeout
        },
        food: Food {
            y: window.get_max_y() / 2 + 5,
            x: window.get_max_y() / 2 + 5,
            ch: '.',
        },
    };

    let mut quit = false;
    while !quit {
        // render
        game.render(&window);
        window.refresh();
        // input
        window.timeout(game.snake.speed);
        match window.getch() {
            Some(Input::KeyF1) => quit = true,
            Some(input) => game.input(input),
            _ => (),
        }
        // update
        if !game.update() { quit = true; }
    }

    curs_set(1);
    endwin();
}