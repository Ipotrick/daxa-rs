use crate::types;

pub type RasterPipeline = daxa_sys::RasterPipeline;

#[repr(C)]
pub struct ShaderInfo<'a> {
    byte_code: *const u32,
    byte_code_size: usize,
    entry_point: types::StringView<'a>,
}

#[repr(i32)]
pub enum CompareOp {
    Never = daxa_sys::VkCompareOp_VK_COMPARE_OP_NEVER,
    Less = daxa_sys::VkCompareOp_VK_COMPARE_OP_LESS,
    Equal = daxa_sys::VkCompareOp_VK_COMPARE_OP_EQUAL,
    LessOrEqual = daxa_sys::VkCompareOp_VK_COMPARE_OP_LESS_OR_EQUAL,
    Greater = daxa_sys::VkCompareOp_VK_COMPARE_OP_GREATER,
    NotEqual = daxa_sys::VkCompareOp_VK_COMPARE_OP_NOT_EQUAL,
    GreaterOrEqual = daxa_sys::VkCompareOp_VK_COMPARE_OP_GREATER_OR_EQUAL,
    Always = daxa_sys::VkCompareOp_VK_COMPARE_OP_ALWAYS,
}

#[repr(C)]
pub struct DepthTestInfo {
    depth_attachment_format: types::Format,
    enable_depth_write: bool,
    depth_test_compare_op: CompareOp,
    min_depth_bounds: f32,
    max_depth_bounds: f32,
}

#[repr(C)]
pub struct RasterPipelineInfo<'a> {
    mesh_shader_info: types::Option<ShaderInfo<'a>>, 
    vertex_shader_info: types::Option<ShaderInfo<'a>>, 
    tesselation_control_shader_info: types::Option<ShaderInfo<'a>>, 
    tesselation_evaluation_shader_info: types::Option<ShaderInfo<'a>>, 
    fragment_shader_info: types::Option<ShaderInfo<'a>>, 
    task_shader_info: types::Option<ShaderInfo<'a>>,
    color_attachments: [RenderAttachment; 8],
    depth_test: types::Option<DepthTestInfo>,
    tesselation: types::Option<TesselationInfo>, // TODO: 
    raster: RasterizerInfo, // TODO: 
    push_constant_size: u32,
    name: types::StringView<'a>
}

/*
   DONE: shader info
   TODO: render attachment
   TODO: depthtestinfo
   TODO: tesselationinfo
   TODO: rasterizerinfo
 */
