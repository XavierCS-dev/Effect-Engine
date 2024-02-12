pub mod engine;
use anyhow::Result;
use engine::{
    engine as effect,
    entity::entity::Entity2D,
    layer::layer::LayerID,
    primitives::vector::Vector3,
    texture::{
        texture2d::{Texture2D, TextureID},
        texture_pool::TexturePool2D,
    },
};
use winit::{
    dpi::PhysicalSize,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct EffectSystem {
    pub engine: effect::Engine,
}

impl EffectSystem {
    pub fn new() -> (Self, EventLoop<()>) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        let window = WindowBuilder::new()
            .with_title("Effect Engine")
            .with_inner_size(PhysicalSize::new(800, 600))
            .with_resizable(false)
            .build(&event_loop)
            .unwrap();
        let engine = pollster::block_on(effect::Engine::new(window));
        (Self { engine }, event_loop)
    }

    pub fn sort(mut entities: Vec<&Entity2D>) -> Vec<Vec<&Entity2D>> {
        entities.sort_unstable_by_key(|v| v.layer_id().0);
        // nested vecs, so not only are lower layers drawn first,
        // but y sorting cn also be used, when implemented.
        let mut layers: Vec<Vec<&Entity2D>> = Vec::new();
        layers.push(Vec::new());
        layers.get_mut(0).unwrap().push(entities.first().unwrap());
        let mut last = entities.first().unwrap().layer_id().0;
        entities.remove(0);
        let mut index = 0;
        for entity in entities {
            if entity.layer_id().0 != last {
                layers.push(Vec::new());
                index += 1;
                last = entity.layer_id().0;
            }
            layers.get_mut(index).unwrap().push(entity);
        }
        layers
    }

    pub fn y_sort(layer: &mut Vec<&Entity2D>) {
        layer.sort_unstable_by(|a, b| b.position().y.partial_cmp(&a.position().y).unwrap());
    }

    /// Take an unordered Vec or Entity2Ds, then sort them into layers
    /// and sort the layers based on y position. (Higher y drawn first.)
    pub fn render_sorted(&mut self, entities: Vec<&Entity2D>, y_sorting: bool) {
        let mut sorted_ents = EffectSystem::sort(entities);
        if y_sorting {
            for layer in &mut sorted_ents {
                EffectSystem::y_sort(layer);
            }
        }
        self.engine.render(sorted_ents).unwrap();
    }

    /// Take a pre-sorted nested Vecs and render it as is.
    // The inner Vec is a singular layer.  May result in unexpexted behaviour if incorrectly sorted.
    pub unsafe fn render(
        &mut self,
        entities: Vec<Vec<&Entity2D>>,
    ) -> Result<(), wgpu::SurfaceError> {
        self.engine.render(entities)
    }

    pub fn init_entity(
        &mut self,
        position: Vector3,
        texture: &Texture2D,
        layer: LayerID,
    ) -> Entity2D {
        self.engine.init_entity(position, texture, layer)
    }

    pub fn init_texture(&self, id: TextureID, path: &str) -> Texture2D {
        Texture2D::new(id, path, self.engine.device(), self.engine.queue())
    }
}

pub fn init_engine() -> (EffectSystem, EventLoop<()>) {
    EffectSystem::new()
}
