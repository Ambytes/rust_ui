/*
    rust_ui
    Copyright (C) 2022  Pascal Behmenburg, Jonas Lauschke

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/
use std::sync::Arc;

use log::{debug, trace};
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
use vulkano::device::{Device, DeviceExtensions, Queue};
use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::instance::Instance;
use vulkano::swapchain::{Surface, Swapchain};
use vulkano::Version;
use vulkano_win::VkSurfaceBuild;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

use crate::NoSuitableDeviceError;

pub struct Engine {
    instance: Arc<Instance>,
    _event_loop: EventLoop<()>,
    surface: Arc<Surface<Window>>,
    device: Arc<Device>,
    graphics_queue: Arc<Queue>,
    swapchain: Arc<Swapchain<Window>>,
    swapchain_images: Vec<Arc<SwapchainImage<Window>>>,
}

impl Engine {
    pub fn new() -> Result<Engine, Box<dyn std::error::Error>> {
        let instance_extensions = vulkano_win::required_extensions();

        let instance = Instance::new(None, Version::V1_2, &instance_extensions, None)?;

        let event_loop = EventLoop::new();
        let surface = WindowBuilder::new().build_vk_surface(&event_loop, instance.clone())?;

        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::none()
        };

        let (physical_device, queue_family) = PhysicalDevice::enumerate(&instance)
            .filter(|&p_dev| {
                for family in p_dev.queue_families() {
                    trace!("Found a queue family with {} queues and capabilities (C/G/sparse/explicit ransfer): {}/{}/{}/{}", family.queues_count(), family.supports_compute(), family.supports_graphics(), family.supports_sparse_binding(), family.explicitly_supports_transfers())
                }

                p_dev
                    .supported_extensions()
                    .is_superset_of(&device_extensions)
            })
            .filter_map(|p_dev| {
                p_dev
                    .queue_families()
                    .find(|&qf| qf.supports_graphics() && surface.is_supported(qf).unwrap_or(false))
                    .map(|qf| (p_dev, qf))
            })
            // Shamelessly stolen from: https://github.com/vulkano-rs/vulkano/blob/master/examples/src/bin/triangle.rs
            // All the physical devices that pass the filters above are suitable for the application.
            // However, not every device is equal, some are preferred over others. Now, we assign
            // each physical device a score, and pick the device with the
            // lowest ("best") score.
            //
            // In this example, we simply select the best-scoring device to use in the application.
            // In a real-life setting, you may want to use the best-scoring device only as a
            // "default" or "recommended" device, and let the user choose the device themselves.
            .min_by_key(|(p, _)| {
                // We assign a better score to device types that are likely to be faster/better.
                match p.properties().device_type {
                    PhysicalDeviceType::DiscreteGpu => 0,
                    PhysicalDeviceType::IntegratedGpu => 1,
                    PhysicalDeviceType::VirtualGpu => 2,
                    PhysicalDeviceType::Cpu => 3,
                    PhysicalDeviceType::Other => 4,
                }
            }).ok_or(NoSuitableDeviceError)?;

        debug!(
            "Chose device: {}, (type {:?})",
            physical_device.properties().device_name,
            physical_device.properties().device_type
        );

        trace!("Creating logical vulkan device...");

        let (device, mut queues) = Device::new(
            physical_device,
            &vulkano::device::Features::none(),
            &physical_device
                .required_extensions()
                .union(&device_extensions),
            [(queue_family, 0.5)].iter().cloned(),
        )?;

        let queue = queues.next().unwrap();

        let (mut swapchain, images) = {
            let capabilities = surface.capabilities(physical_device).unwrap();
            let composite_alpha = capabilities
                .supported_composite_alpha
                .iter()
                .next()
                .unwrap();
            let format = capabilities.supported_formats[0].0;
            let dimensions: [u32; 2] = surface.window().inner_size().into();

            Swapchain::start(device.clone(), surface.clone())
                .num_images(capabilities.min_image_count)
                .format(format)
                .dimensions(dimensions)
                .usage(ImageUsage::color_attachment())
                .sharing_mode(&queue)
                .composite_alpha(composite_alpha)
                .build()
        }?;

        Ok(Engine {
            instance,
            _event_loop: event_loop,
            surface,
            device,
            graphics_queue: queue,
            swapchain,
            swapchain_images: images,
        })
    }
}
