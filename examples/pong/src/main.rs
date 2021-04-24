extern crate jelly_engine;

use jelly_engine::{
    core::{Scene, Transaction, World},
    Engine,
};

fn main() {
    Engine::default().run(Game {
        execution_number: 10,
    });
}

pub struct Game {
    execution_number: u32,
}

impl Scene for Game {
    fn on_enter(&mut self, world: &mut World) {
        println!("Hello Game scene 👋");
    }

    fn on_exit(&mut self, world: &mut World) {
        println!("Bye bye Game scene 👋");
    }

    fn update(&mut self, world: &mut World) -> Transaction {
        match self.execution_number {
            0 => Transaction::Quit,
            _ => {
                self.execution_number -= 1;
                Transaction::None
            }
        }
    }
}
