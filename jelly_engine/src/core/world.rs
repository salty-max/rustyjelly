use super::entity::{Entities, Entity};

#[derive(Debug, Default)]
pub struct World {
    entities: Entities,
}

impl World {
    pub fn build_entity(&mut self) -> Entity {
        self.entities.create_entity()
    }
}
