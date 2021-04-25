use super::World;
use std::{any::Any, fmt::Debug};

pub trait System: Any + Debug {
    fn init(&mut self, world: &mut World) {}
    fn run(&mut self, world: &mut World) {}
    fn dispose(&mut self, world: &mut World) {}
}
