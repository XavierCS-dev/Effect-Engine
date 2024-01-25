use crate::engine::entity::entity::Entity2DRaw;

pub trait EntityType {
    fn to_raw(&self) -> Entity2DRaw;
}
