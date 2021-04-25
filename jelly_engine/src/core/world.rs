use std::cell::{Ref, RefMut};

use super::{
    component::{Component, Components, Set},
    entity::{Entities, Entity, EntityBuilder},
};

#[derive(Debug, Default)]
pub struct World {
    entities: Entities,
    components: Components,
}

impl World {
    pub fn build_entity(&mut self) -> EntityBuilder {
        EntityBuilder::new(self.entities.create_entity(), self)
    }

    pub fn remove_entity(&self, entity: &Entity) {
        self.components.remove_entity(entity);
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
}
