use crate::engine::adts::entity::Entity2DRaw;

pub trait EntityType {
    fn to_raw(&self) -> Entity2DRaw;
}
