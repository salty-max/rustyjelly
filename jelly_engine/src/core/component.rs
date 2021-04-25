use downcast_rs::Downcast;
use std::{any::Any, collections::HashMap};
use std::{any::TypeId, fmt::Debug};

use super::entity::Entity;

pub trait Component: Any + Debug {}

#[derive(Debug, Default)]
pub struct Components {
    storages: HashMap<TypeId, Box<dyn AnySet>>,
}

impl Components {
    pub fn insert<C: Component>(&mut self, entity: &Entity, component: C) {
        let type_id = TypeId::of::<C>();

        match self.storages.get_mut(&type_id) {
            Some(storage) => {
                storage
                    .downcast_mut::<Set<C>>()
                    .expect("Downcast set error")
                    .insert(*entity, component);
            }
            None => {
                let mut storage = Set::<C>::default();
                storage.insert(*entity, component);

                self.storages.insert(type_id, Box::new(storage));
            }
        }
    }

    pub fn get<C: Component>(&self) -> Option<&Set<C>> {
        let type_id = TypeId::of::<C>();

        match self.storages.get(&type_id) {
            Some(storage) => storage.downcast_ref::<Set<C>>(),
            None => None,
        }
    }

    pub fn get_mut<C: Component>(&mut self) -> Option<&mut Set<C>> {
        let type_id = TypeId::of::<C>();

        match self.storages.get_mut(&type_id) {
            Some(storage) => storage.downcast_mut::<Set<C>>(),
            None => None,
        }
    }

    pub fn delete_entity(&mut self, entity: &Entity) {
        for s in self.storages.iter_mut() {
            s.1.remove(&entity);
        }
    }
}

pub type Set<C> = HashMap<Entity, C>;
impl<C: Component> AnySet for Set<C> {
    fn remove(&mut self, entity: &Entity) {
        self.remove(entity);
    }
}

pub trait AnySet: Downcast + Debug {
    fn remove(&mut self, _entity: &Entity) {}
}
downcast_rs::impl_downcast!(AnySet);
