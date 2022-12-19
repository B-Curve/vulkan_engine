pub mod app_data;
pub mod shader;
pub mod pipeline;
pub mod instance;
pub mod render;
pub mod framebuffer;
pub mod device;
pub mod queue_family_indices;
pub mod swapchain_support;
pub mod swapchain;
pub mod command_pool;
pub mod command_buffer;
pub mod semaphore;
pub mod buffer;

#[derive(Debug, thiserror::Error)]
#[error("Missing {0}.")]
pub struct SuitabilityError(pub &'static str);

extern "system" fn debug_callback(
    severity: vulkanalia::vk::DebugUtilsMessageSeverityFlagsEXT,
    type_: vulkanalia::vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vulkanalia::vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut std::os::raw::c_void,
) -> vulkanalia::vk::Bool32 {
    let data = unsafe { *data };
    let message = unsafe { std::ffi::CStr::from_ptr(data.message) }.to_string_lossy();

    if severity >= vulkanalia::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR {
        log::error!("({:?}) {}", type_, message);
    } else if severity >= vulkanalia::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING {
        log::warn!("({:?}) {}", type_, message);
    } else if severity >= vulkanalia::vk::DebugUtilsMessageSeverityFlagsEXT::INFO {
        log::debug!("({:?}) {}", type_, message);
    } else {
        log::trace!("({:?}) {}", type_, message);
    }

    vulkanalia::vk::FALSE
}