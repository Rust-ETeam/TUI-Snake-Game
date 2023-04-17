use cursive::vec::Vec2;
use cursive::XY;
use rand::Rng;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ObjectKind {
    Head,
    Body,
    Bite,
    Wall,
    Empty,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    Left,
    Right,
    Up,
    Down,
}

pub struct Object {
    pub pos: Vec2,
    pub kind: ObjectKind,
}

pub struct GameModel {
    pub map_size: usize,
    pub game_map: Vec<Vec<ObjectKind>>,
    pub snake: VecDeque<Vec2>,
    pub bite: Vec2,
    pub forward: Action,
}

impl GameModel {
    pub fn new(map_size: usize) -> Self {
        let mut game_map: Vec<Vec<ObjectKind>> = vec![];

        let last = map_size - 1;
        for r in 0..map_size {
            let mut row = vec![];
            for c in 0..map_size {
                row.push(if c == 0 || c == last || r == 0 || r == last {
                    ObjectKind::Wall
                } else {
                    ObjectKind::Empty
                })
            }
            game_map.push(row);
        }

        let head: Vec2 = Vec2::new(rand_range(2, map_size - 2), rand_range(2, map_size - 2));
        let mut snake: VecDeque<Vec2> = VecDeque::new();
        snake.push_back(head);
        game_map[head.y][head.x] = ObjectKind::Head;

        let bite: Vec2 = Vec2::new(rand_range(1, map_size - 1), rand_range(1, map_size - 1));
        game_map[bite.y][bite.x] = ObjectKind::Bite;

        let forward: Action = Action::Right;

        GameModel {
            map_size,
            game_map,
            snake,
            bite,
            forward,
        }
    }

    pub fn set_forward(&mut self, forward: Action) {
        self.forward = forward;
    }

    pub fn do_action(&mut self) -> bool {
        let delta = match self.forward {
            Action::Left => XY::<isize>::new(-1, 0),
            Action::Right => XY::<isize>::new(1, 0),
            Action::Up => XY::<isize>::new(0, -1),
            Action::Down => XY::<isize>::new(0, 1),
        };

        self.move_snake(delta)
    }

    fn empty_rand_pos(&self) -> Vec2 {
        loop {
            let x = rand_range(1, self.map_size - 1);
            let y = rand_range(1, self.map_size - 1);
            if self.game_map[y][x] == ObjectKind::Empty {
                break Vec2::new(x, y);
            }
        }
    }

    fn move_snake(&mut self, delta: XY<isize>) -> bool {
        let head = Vec2::new(
            u_plus_i(self.snake[self.snake.len() - 1].x, delta.x),
            u_plus_i(self.snake[self.snake.len() - 1].y, delta.y),
        );

        self.change_game_map_kind(self.snake[self.snake.len() - 1], ObjectKind::Body);
        match self.game_map[head.y][head.x] {
            ObjectKind::Wall | ObjectKind::Head | ObjectKind::Body => return false,
            ObjectKind::Empty => {
                let last = self.snake.pop_front();
                self.change_game_map_kind(last.unwrap(), ObjectKind::Empty);
            }
            ObjectKind::Bite => {
                self.bite = self.empty_rand_pos();
                self.change_game_map_kind(self.bite, ObjectKind::Bite);
            }
        }
        self.snake.push_back(head);
        self.change_game_map_kind(self.snake[self.snake.len() - 1], ObjectKind::Head);
        return true;
    }

    fn change_game_map_kind(&mut self, pos: Vec2, kind: ObjectKind) {
        self.game_map[pos.y][pos.x] = kind;
    }
}

fn rand_range(b: usize, e: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(b..e)
}

fn u_plus_i(u: usize, i: isize) -> usize {
    if i < 0 {
        return u - (-i as usize);
    } else {
        return u + (i as usize);
    }
}
