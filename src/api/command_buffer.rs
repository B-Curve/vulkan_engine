use anyhow::Result;
use vulkanalia::{Device, vk};
use vulkanalia::vk::{DeviceV1_0, HasBuilder};

use crate::api::app_data::AppData;
use crate::render::vertex::VERTICES;

pub unsafe fn create_command_buffers(device: &Device, data: &mut AppData) -> Result<()> {
    let allocate_info = vk::CommandBufferAllocateInfo::builder()
        .command_pool(data.command_pool)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(data.framebuffers.len() as u32);

    data.command_buffers = device.allocate_command_buffers(&allocate_info)?;

    for (i, buffer) in data.command_buffers.iter().enumerate() {
        let inheritance = vk::CommandBufferInheritanceInfo::builder();

        let info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::empty())
            .inheritance_info(&inheritance);

        device.begin_command_buffer(*buffer, &info)?;

        let render_area = vk::Rect2D::builder()
            .offset(vk::Offset2D::default())
            .extent(data.swapchain_extent);

        let color_clear_value = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0],
            },
        };

        let clear_values = &[color_clear_value];
        let info = vk::RenderPassBeginInfo::builder()
            .render_pass(data.render_pass)
            .framebuffer(data.framebuffers[i])
            .render_area(render_area)
            .clear_values(clear_values);

        device.cmd_begin_render_pass(*buffer, &info, vk::SubpassContents::INLINE);

        device.cmd_bind_pipeline(*buffer, vk::PipelineBindPoint::GRAPHICS, data.pipeline);

        device.cmd_bind_vertex_buffers(*buffer, 0, &[data.vertex_buffer], &[0]);

        device.cmd_draw(*buffer, VERTICES.len() as u32, 1, 0, 0);

        device.cmd_end_render_pass(*buffer);

        device.end_command_buffer(*buffer)?;
    }

    Ok(())
}