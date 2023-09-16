extern crate pancurses;

use pancurses::*;
use std::collections::LinkedList;
use std::iter::FromIterator;

#[derive(Clone, PartialEq)]
enum Direction {
    Down,
    Right,
    Up,
    Left,
}

#[derive(Clone)]
struct SnakePiece(i32, i32);

struct Snake {
    parts: LinkedList<SnakePiece>,
    ch: char,
    dir: Direction,
}

impl Snake {
    fn render(&mut self, window: &Window) {
        self.parts.iter().for_each(|p| {
            window.mvaddch(p.0, p.1, self.ch);
        });
    }
    fn set_direction(&mut self, new_dir: Direction) {
        let last_dir = self.dir.clone();
        self.dir = match new_dir {
            Direction::Left     if last_dir != Direction::Right => Direction::Left,
            Direction::Down     if last_dir != Direction::Up    => Direction::Down,
            Direction::Up       if last_dir != Direction::Down  => Direction::Up,
            Direction::Right    if last_dir != Direction::Left  => Direction::Right,
            _ => last_dir,
        }
    }
    fn update(&mut self) {
        let mut new_head =
            (*self.parts.front().expect("Snake has no body")).clone();
        match self.dir {
            Direction::Down     => new_head.0 += 1,
            Direction::Right    => new_head.1 += 1,
            Direction::Up       => new_head.0 -= 1,
            Direction::Left     => new_head.1 -= 1,
        }
        self.parts.push_front(new_head);
        self.parts.pop_back().unwrap();
    }
}

struct Game {
    snake: Snake,
}

fn get_direction(input: Input) -> Option<Direction> {
    match input {
        Input::Character('h') => Some(Direction::Left),
        Input::Character('j') => Some(Direction::Down),
        Input::Character('k') => Some(Direction::Up),
        Input::Character('l') => Some(Direction::Right),
        _ => None
    }
}

impl Game {
    fn render(&mut self, window: &Window) {
        window.bkgd(COLOR_PAIR(1));
        window.erase();
        window.mvaddstr(0, 0, "Snake: Help Kanka find food!");
        window.mvaddstr(1, 0, "Press F1 to exit.");
        self.snake.render(window);
    }
    fn input(&mut self, input: Input) {
        match get_direction(input) {
            Some(dir) => self.snake.set_direction(dir),
            _ => (),
        }
    }
    fn update(&mut self) {
        self.snake.update();
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
    window.timeout(500);
    window.clear();

    let mut game = Game{
        snake: Snake{
            parts: LinkedList::from_iter((vec![
                SnakePiece(window.get_max_y() / 2, window.get_max_x() / 2),
                SnakePiece(window.get_max_y() / 2, window.get_max_x() / 2 - 1)
            ]).into_iter()),
            ch: 'S',
            dir: Direction::Right,
        }
    };

    let mut quit = false;
    while !quit {
        // render
        game.render(&window);
        window.refresh();
        // input
        match window.getch() {
            Some(Input::KeyF1) => quit = true,
            Some(input) => game.input(input),
            _ => (),
        }
        // update
        game.update();
    }

    curs_set(1);
    endwin();
}