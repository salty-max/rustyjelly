use std::{
    any::TypeId,
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    fmt::Debug,
};

use downcast_rs::Downcast;

use super::{
    component::{Component, Components, Set},
    entity::{Entities, Entity, EntityBuilder},
};

pub trait Resource: Downcast + Debug + 'static {}
downcast_rs::impl_downcast!(Resource);

#[derive(Debug, Default)]
pub struct World {
    entities: Entities,
    components: Components,
    resources: HashMap<TypeId, RefCell<Box<dyn Resource>>>,
}

impl World {
    pub fn build_entity(&mut self) -> EntityBuilder {
        EntityBuilder::new(self.entities.create_entity(), self)
    }

    pub fn remove_entity(&self, entity: &Entity) {
        self.components.remove_entity(entity);
    }

    pub fn get_entities(&self) -> &Entities {
        &self.entities
    }

    pub fn get_components<C: Component>(&self) -> Option<Ref<Set<C>>> {
        self.components.get::<C>()
    }

    pub fn get_components_mut<C: Component>(&self) -> Option<RefMut<Set<C>>> {
        self.components.get_mut::<C>()
    }

    pub fn insert_component<C: Component>(&mut self, entity: &Entity, component: C) {
        self.components.insert(entity, component);
    }

    pub fn register_component<C: Component>(&mut self) {
        self.components.register::<C>();
    }

    pub fn get_resource<R: Resource>(&self) -> Option<Ref<R>> {
        let type_id = TypeId::of::<R>();

        match self.resources.get(&type_id) {
            Some(resource) => Some(Ref::map(resource.borrow(), |b| {
                b.downcast_ref::<R>().expect("Downcast resource error")
            })),
            None => None,
        }
    }

    pub fn get_resource_mut<R: Resource>(&self) -> Option<RefMut<R>> {
        let type_id = TypeId::of::<R>();

        match self.resources.get(&type_id) {
            Some(resource) => Some(RefMut::map(resource.borrow_mut(), |b| {
                b.downcast_mut::<R>().expect("Downcast resource error")
            })),
            None => None,
        }
    }

    pub fn insert_resource<R: Resource>(&mut self, resource: R) {
        let type_id = TypeId::of::<R>();
        self.resources
            .insert(type_id, RefCell::new(Box::new(resource)));
    }
}
