use super::{entity::Entity2D, layer::LayerID};

// Used instead of a tuple to get contigious memory, increase performance
pub struct EntityGroup2D {
    layer: LayerID,
    entities: Vec<Entity2D>,
}

impl EntityGroup2D {
    pub fn new(layer: LayerID, entities: Vec<Entity2D>) -> EntityGroup2D {
        Self { layer, entities }
    }

    pub fn layer(&self) -> LayerID {
        self.layer
    }

    pub fn entities(&self) -> &Vec<Entity2D> {
        &self.entities
    }
}
