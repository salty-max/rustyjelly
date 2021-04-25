#[derive(Debug)]
pub struct Entity(u32);

#[derive(Debug, Default)]
pub struct Entities {
    current_id: u32,
}

impl Entities {
    pub fn create_entity(&mut self) -> Entity {
        let id = self.current_id;
        self.current_id += 1;

        Entity(id)
    }
}
