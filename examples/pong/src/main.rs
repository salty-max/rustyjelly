extern crate jelly_engine;

use jelly_engine::{
    core::{Component, Scene, Transaction, World},
    Engine,
};

fn main() {
    Engine::default().run(Game {
        execution_number: 10,
    });
}

#[derive(Debug)]
pub struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug)]
pub struct Test {
    data: u32,
}

impl Component for Position {}
impl Component for Test {}

pub struct Game {
    execution_number: u32,
}

impl Scene for Game {
    fn on_enter(&mut self, world: &mut World) {
        println!("Hello Game scene 👋");

        let e = world
            .build_entity()
            .with(Position { x: 24.0, y: 8.0 })
            .with(Test { data: 5 })
            .build();

        let e2 = world
            .build_entity()
            .with(Position { x: 8.0, y: 8.0 })
            .with(Test { data: 3 })
            .build();
    }

    fn on_exit(&mut self, world: &mut World) {
        println!("Bye bye Game scene 👋");
    }

    fn update(&mut self, world: &mut World) -> Transaction {
        match self.execution_number {
            0 => Transaction::Quit,
            _ => {
                self.execution_number -= 1;
                println!("World: {:#?}", world);
                Transaction::None
            }
        }
    }
}
