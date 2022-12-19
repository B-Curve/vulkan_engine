use anyhow::Result;
use vulkanalia::{Device, Instance, vk};
use vulkanalia::vk::{DeviceV1_0, HasBuilder};

use crate::api::app_data::AppData;
use crate::api::queue_family_indices::QueueFamilyIndices;

pub unsafe fn create_command_pool(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
) -> Result<()> {
    let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;

    let info = vk::CommandPoolCreateInfo::builder()
        .flags(vk::CommandPoolCreateFlags::empty())
        .queue_family_index(indices.graphics);

    data.command_pool = device.create_command_pool(&info, None)?;

    Ok(())
}