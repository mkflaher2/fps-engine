use crate::objects::PhysicsObject;
pub struct State {
    pub objects: Vec<Box<dyn PhysicsObject>>,
}

impl State {
    pub fn new() -> State {
        State {
            objects: Vec::new()
        }
    }
}
