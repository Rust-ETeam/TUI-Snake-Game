use cursive::event::{Callback, Event, EventResult, Key};
use cursive::vec::Vec2;
use cursive::{Cursive, Printer};

use crate::game::gamemodel::{Action, GameModel, ObjectKind};

const MAP_SIZE: usize = 16;

pub struct GameView {
    model: GameModel,
    callback: Callback,
    tick: usize,
}

impl GameView {
    pub fn new<F>(cb: F) -> Self
    where
        F: 'static + Fn(&mut Cursive),
    {
        GameView {
            model: GameModel::new(MAP_SIZE),
            callback: Callback::from_fn(cb),
            tick: 0,
        }
    }

    pub fn new_game(&mut self) {
        self.model = GameModel::new(MAP_SIZE);
    }
}

impl cursive::view::View for GameView {
    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        Vec2::new(2 * self.model.map_size - 1, self.model.map_size)
    }

    fn draw(&self, printer: &Printer) {
        for (y, row) in self.model.game_map.iter().enumerate() {
            for (x, object) in row.iter().enumerate() {
                printer.print(
                    Vec2::new(2 * x, y),
                    match object {
                        ObjectKind::Empty => " ",
                        ObjectKind::Wall => "▩",
                        ObjectKind::Head => "◆",
                        ObjectKind::Body => "◇",
                        ObjectKind::Bite => "★",
                    },
                );
                if x + 1 < self.model.map_size {
                    printer.print(Vec2::new(2 * x + 1, y), " ");
                }
            }
        }
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        let mut game_over = false;
        self.tick += 1;
        match event {
            Event::Key(Key::Up) | Event::Char('w') => self.model.set_forward(Action::Up),
            Event::Key(Key::Down) | Event::Char('s') => self.model.set_forward(Action::Down),
            Event::Key(Key::Left) | Event::Char('a') => self.model.set_forward(Action::Left),
            Event::Key(Key::Right) | Event::Char('d') => self.model.set_forward(Action::Right),
            Event::Char('r') => self.model = GameModel::new(MAP_SIZE),
            Event::Refresh => {}
            _ => self.tick -= 1,
        }

        let base = 5;
        let pow = (self.model.snake.len() + base - 1) as f64;
        let difficulty = 10.0 / pow.log(base as f64);
        if self.tick >= difficulty as usize {
            game_over = !self.model.do_action();
            self.tick = 0;
        }

        EventResult::Consumed(if game_over {
            Some(self.callback.clone())
        } else {
            None
        })
    }
}
