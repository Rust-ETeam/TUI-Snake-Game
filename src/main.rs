use cursive::event::Key;
use cursive::view::Nameable;
use cursive::views::{Dialog, TextView};
use cursive::{Cursive, CursiveExt};

pub mod game;
use crate::game::gameview::GameView;

fn main() {
    let mut siv = Cursive::default();

    siv.add_global_callback(Key::Esc, |s| s.quit());
    siv.set_fps(30);

    show_intro(&mut siv);
    siv.run();
}

fn show_intro(s: &mut Cursive) {
    let message = "\
        Snake Game in Rust by Team-E\n\
        \n\
        Arrow keys or WASD to move\n\
        ESC to quit\n\
    ";

    let dialog = Dialog::new()
        .title("Snake")
        .content(TextView::new(message))
        .button("Play", |s| {
            s.pop_layer();
            show_game(s)
        })
        .button("Quit", |s| s.quit());

    s.add_layer(dialog);
}

fn show_game(s: &mut Cursive) {
    s.add_layer(
        Dialog::around(
            GameView::new(|s| {
                let mut dialog = Dialog::text("Game Over!");
                s.call_on_name("gameview", |_view: &mut GameView| {
                    dialog.add_button("Restart", |s| {
                        s.pop_layer();
                        s.call_on_name("gameview", |view: &mut GameView| {
                            view.new_game();
                        });
                    });
                    dialog.add_button("Quit", |s| s.quit());
                });
                s.add_layer(dialog);
            })
            .with_name("gameview"),
        )
        .title("Snake"),
    );
}
