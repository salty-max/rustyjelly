use super::World;

pub enum Transaction {
    None,
    Quit,
}

pub trait Scene {
    fn on_enter(&mut self, world: &mut World) {}
    fn on_exit(&mut self, world: &mut World) {}
    fn update(&mut self, world: &mut World) -> Transaction {
        Transaction::None
    }
}
