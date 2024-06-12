use wgpu::util::DeviceExt;

pub struct BufferAllocator {
    usage: wgpu::BufferUsages,
    mapped_at_creation: bool,
    size: u64,
    label: Option<&'static str>,
    data: Option<Vec<u8>>,
}

impl Default for BufferAllocator {
    /// Default usage: Vertex stage visibility and COPY_DST.
    /// Not mapped at creation
    /// Size of 5096 bytes
    /// No label
    fn default() -> Self {
        let usage = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST;
        let mapped_at_creation = false;
        let size = 5096;
        let label = None;
        let data = None;
        Self {
            usage,
            mapped_at_creation,
            size,
            label,
            data,
        }
    }
}

impl BufferAllocator {
    pub fn usage(mut self, usage: wgpu::BufferUsages) -> Self {
        self.usage = usage;
        self
    }

    pub fn mapped_at_creation(mut self, mapped_at_creation: bool) -> Self {
        self.mapped_at_creation = mapped_at_creation;
        self
    }

    /// Set the size of the buffer in bytes
    pub fn size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }

    pub fn label(mut self, label: &'static str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn data(mut self, data: Vec<u8>) -> Self {
        self.data = Some(data);
        self
    }

    /// Create the buffer
    pub fn allocate(mut self, device: &wgpu::Device) -> wgpu::Buffer {
        match self.data {
            Some(data) => device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: self.label,
                contents: &data,
                usage: self.usage,
            }),
            None => device.create_buffer(&wgpu::BufferDescriptor {
                label: self.label,
                size: self.size,
                usage: self.usage,
                mapped_at_creation: self.mapped_at_creation,
            }),
        }
    }
}
