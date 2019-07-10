use std::sync::Arc;
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    command_buffer::{AutoCommandBufferBuilder, CommandBuffer},
    descriptor::descriptor_set::PersistentDescriptorSet,
    device::{Device, DeviceExtensions, Features},
    instance::{Instance, InstanceExtensions, PhysicalDevice, PhysicalDeviceType},
    pipeline::ComputePipeline,
    sync::GpuFuture,
};

fn main() {
    let instance =
        Instance::new(None, &InstanceExtensions::none(), None).expect("failed to create instance");
    let mut best_physical = None;
    for physical in PhysicalDevice::enumerate(&instance) {
        if best_physical.is_none() {
            best_physical = Some(physical);
        } else if physical.ty() == PhysicalDeviceType::DiscreteGpu {
            best_physical = Some(physical);
            break;
        }
    }
    let physical = best_physical.expect("no device available");
    let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");
    let (device, mut queues) = {
        Device::new(
            physical,
            &Features::none(),
            &DeviceExtensions::none(),
            [(queue_family, 0.5)].iter().cloned(),
        )
        .expect("failed to create device")
    };
    let queue = queues.next().unwrap();
    let data_iter = 0 .. 65536;
    let data_buffer = CpuAccessibleBuffer::from_iter(Arc::clone(&device), BufferUsage::all(),
                                                     data_iter).expect("failed to create buffer");
    let shader = cs::Shader::load(Arc::clone(&device)).expect("failed to create shader module");
    let compute_pipeline = Arc::new(ComputePipeline::new(Arc::clone(&device), &shader.main_entry_point(), &()).expect("failed to create compute pipeline"));
    let set = Arc::new(PersistentDescriptorSet::start(Arc::clone(&compute_pipeline), 0)
    .add_buffer(Arc::clone(&data_buffer)).unwrap().build().unwrap());
    let command_buffer = AutoCommandBufferBuilder::new(Arc::clone(&device), queue.family()).unwrap()
        .dispatch([1024, 1, 1], Arc::clone(&compute_pipeline), Arc::clone(&set), ()).unwrap().build().unwrap();
    let finished = command_buffer.execute(Arc::clone(&queue)).unwrap();
    finished.then_signal_fence_and_flush().unwrap().wait(None).unwrap();
    let content = data_buffer.read().unwrap();
    for (n, val) in content.iter().enumerate() {
        assert_eq!(*val, n as u32 * 12);
    }
}

mod cs {
    vulkano_shaders::shader!{
        ty: "compute",
        src: "
#version 450

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer Data {
    uint data[];
} buf;

void main() {
    uint idx = gl_GlobalInvocationID.x;
    buf.data[idx] *= 12;
}"
    }
}