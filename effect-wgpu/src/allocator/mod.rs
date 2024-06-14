use anyhow::Result;
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
    fn allocate_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        match self.data.as_ref() {
            Some(data) => device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: self.label,
                contents: data,
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

    pub fn allocate(self, device: &wgpu::Device) -> Buffer {
        let buffer = self.allocate_buffer(device);
        let size = match self.data {
            Some(data) => data.len(),
            None => 0,
        };
        let capacity = self.size as usize;
        let usage = self.usage;
        Buffer::new(buffer, size, capacity, usage)
    }
}

pub struct Buffer {
    buffer: wgpu::Buffer,
    size: usize,
    capacity: usize,
    usage: wgpu::BufferUsages,
}

impl Buffer {
    pub fn new(
        buffer: wgpu::Buffer,
        size: usize,
        capacity: usize,
        usage: wgpu::BufferUsages,
    ) -> Self {
        Self {
            buffer,
            size,
            capacity,
            usage,
        }
    }

    /// Overwrite the data in the buffer, data must be a slice of bytes
    pub fn write(&mut self, data: &[u8], device: &wgpu::Device, queue: &wgpu::Queue) {
        let size = std::mem::size_of_val(data);
        if size > self.capacity {
            let capacity = size * 2;
            let buffer = BufferAllocator::default()
                .size(capacity as u64)
                .usage(self.usage)
                .allocate_buffer(device);
            queue.write_buffer(&buffer, 0, data);
            self.capacity = capacity;
            self.size = size;
        } else {
            queue.write_buffer(&self.buffer, 0, data);
            self.size = size;
        }
    }

    pub fn buffer(&self) -> wgpu::BufferSlice {
        self.buffer.slice(..self.size as u64)
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn usage(&self) -> wgpu::BufferUsages {
        self.usage
    }

    unsafe fn write_at(
        &mut self,
        data: &[u8],
        offset: wgpu::BufferAddress,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        unimplemented!()
    }
}
