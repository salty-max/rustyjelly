extern crate jelly_engine;

use jelly_engine::{
    core::{
        system::{ReadEntities, ReadSet, WriteSet},
        Component, Scene, System, Transaction, World,
    },
    Engine,
};

fn main() {
    Engine::default()
        .with_system(System1 {})
        .with_system(System2 {})
        .run(Game {
            execution_count: 10,
        });
}

#[derive(Debug)]
pub struct System1 {}

impl<'a> System<'a> for System1 {
    type Data = (WriteSet<'a, Test>, ReadSet<'a, Position>, ReadEntities<'a>);
    fn init(&mut self, world: &mut World) {
        println!("System 1 initialized");
    }

    fn run(&mut self, (mut tests, mut positions, entities): Self::Data) {
        for t in tests.values_mut() {
            t.data += 1;
        }
        println!("tests {:?}", tests);
        println!("entities {:?}", entities);

        for p in positions.values() {
            println!("position: {}:{}", p.x, p.y);
        }
    }

    fn dispose(&mut self, world: &mut World) {
        println!("System 1 disposed");
    }
}

#[derive(Debug)]
pub struct System2 {}

impl<'a> System<'a> for System2 {
    type Data = ();

    fn init(&mut self, world: &mut World) {
        println!("System 2 initialized");
    }

    fn run(&mut self, data: Self::Data) {
        println!("System 2 running...");
    }

    fn dispose(&mut self, world: &mut World) {
        println!("System 2 disposed");
    }
}

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Test {
    pub data: u32,
}

impl Component for Position {}
impl Component for Test {}

pub struct Game {
    execution_count: u32,
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
        match self.execution_count {
            0 => Transaction::Quit,
            _ => {
                self.execution_count -= 1;
                // println!("World: {:#?}", world);
                Transaction::None
            }
        }
    }
}
