use super::{component::Set, Component, World};
use std::{any::Any, cell::Ref, fmt::Debug};

pub trait AnySystem {
    fn init(&mut self, world: &mut World);
    fn run_now(&mut self, world: &World);
    fn dispose(&mut self, world: &mut World);
}

impl<S> AnySystem for S
where
    S: for<'a> System<'a>,
{
    fn init(&mut self, world: &mut World) {
        self.init(world);
    }
    fn run_now(&mut self, world: &World) {
        let data = S::Data::fetch(world);
        self.run(data);
    }
    fn dispose(&mut self, world: &mut World) {
        self.dispose(world);
    }
}

pub trait System<'a>: Any + Debug {
    type Data: Data<'a>;
    fn init(&mut self, world: &mut World);
    fn run(&mut self, data: Self::Data);
    fn dispose(&mut self, world: &mut World);
}

pub trait Data<'a> {
    fn fetch(world: &'a World) -> Self;
}

pub type ReadSet<'a, C> = Ref<'a, Set<C>>;

impl<'a, C: Component> Data<'a> for ReadSet<'a, C> {
    fn fetch(world: &'a World) -> Self {
        world.get_components::<C>().unwrap()
    }
}
