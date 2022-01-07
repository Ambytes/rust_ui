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

use log::{debug, trace};
use vulkano::{
    device::physical::{PhysicalDevice, PhysicalDeviceType},
    device::DeviceExtensions,
    instance::Instance,
    sync::GpuFuture,
    Version,
};
use vulkano_win::VkSurfaceBuild;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};


pub fn setup() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging()?;

    let instance_extensions = vulkano_win::required_extensions();

    let instance = Instance::new(None, Version::V1_2, &instance_extensions, None).expect(
        "Failed to create Vulkan instance; maybe you are missing the required Vulkan runtime?",
    );

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
        })
        .unwrap();

    debug!(
        "Chose device: {}, (type {:?})",
        physical_device.properties().device_name,
        physical_device.properties().device_type
    );

    Ok(())
}

fn setup_logging() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
