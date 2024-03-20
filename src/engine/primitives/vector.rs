use num::Float;

#[derive(Debug, Clone, Copy)]
pub struct Vector3<T>
where
    T: Float,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T>
where
    T: Float,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}
