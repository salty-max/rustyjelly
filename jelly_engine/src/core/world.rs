use super::{
    component::{Component, Components},
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

    pub fn delete_entity(&mut self, entity: &Entity) {
        self.components.delete_entity(entity);
    }

    pub fn insert_component<C: Component>(&mut self, entity: &Entity, component: C) {
        self.components.insert(entity, component);
    }
}
