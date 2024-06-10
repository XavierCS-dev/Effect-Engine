use effect_core::primitives::vertex::Vertex;

pub trait VertexLayout {
    const ATTRIBUTE_ARRAY: [wgpu::VertexAttribute; 2];
    fn layout() -> wgpu::VertexBufferLayout<'static>;
}

impl VertexLayout for Vertex {
    const ATTRIBUTE_ARRAY: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTE_ARRAY,
        }
    }
}
