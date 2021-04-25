use super::{component::Set, entity::Entities, Component, World};
use std::{
    any::Any,
    cell::{Ref, RefMut},
    fmt::Debug,
};

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
        S::Data::setup(world);
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
    fn setup(world: &mut World);
    fn fetch(world: &'a World) -> Self;
}

pub type ReadEntities<'a> = &'a Entities;
pub type ReadSet<'a, C> = Ref<'a, Set<C>>;
pub type WriteSet<'a, C> = RefMut<'a, Set<C>>;

impl<'a> Data<'a> for () {
    fn setup(world: &mut World) {}

    fn fetch(world: &'a World) -> Self {
        ()
    }
}

impl<'a> Data<'a> for ReadEntities<'a> {
    fn setup(world: &mut World) {}

    fn fetch(world: &'a World) -> Self {
        world.get_entities()
    }
}

impl<'a, C: Component> Data<'a> for ReadSet<'a, C> {
    fn setup(world: &mut World) {
        world.register_component::<C>();
    }

    fn fetch(world: &'a World) -> Self {
        world.get_components::<C>().unwrap()
    }
}

impl<'a, C: Component> Data<'a> for WriteSet<'a, C> {
    fn setup(world: &mut World) {
        world.register_component::<C>();
    }

    fn fetch(world: &'a World) -> Self {
        world.get_components_mut::<C>().unwrap()
    }
}

// impl<'a, C1, C2> Data<'a> for (C1, C2)
// where
//     C1: Data<'a>,
//     C2: Data<'a>,
// {
//     fn fetch(world: &'a World) -> Self {
//         (C1::fetch(world), C2::fetch(world))
//     }
// }

macro_rules! impl_data {
    ($($ty:ident), *) => {
        impl<'a, $($ty),*> Data<'a> for ($($ty, )*)
        where $($ty: Data<'a>, )* {
            fn setup(world: &mut World) {
                 $($ty::setup(world); )*
            }

            fn fetch(world: &'a World) -> Self {
                (
                    $($ty::fetch(world),)*
                )
            }
        }
    };
}

impl_data!(A);
impl_data!(A, B);
impl_data!(A, B, C);
impl_data!(A, B, C, D);
impl_data!(A, B, C, D, E);
impl_data!(A, B, C, D, E, F);
impl_data!(A, B, C, D, E, F, G);
impl_data!(A, B, C, D, E, F, G, H);
impl_data!(A, B, C, D, E, F, G, H, I);
impl_data!(A, B, C, D, E, F, G, H, I, J);
impl_data!(A, B, C, D, E, F, G, H, I, J, K);
impl_data!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
