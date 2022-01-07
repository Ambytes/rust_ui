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
use vulkano::device::Device;
use vulkano::device::DeviceCreationError::FeatureRestrictionNotMet;
use vulkano::image::ImageUsage;
use vulkano::swapchain::{ColorSpace, Swapchain};
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

use crate::errors::NoSuitableDeviceError;

pub mod engine;
mod errors;

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
