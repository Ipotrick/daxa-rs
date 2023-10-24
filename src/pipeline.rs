use bitflags::bitflags;

use crate::{types, CompareOp, Format};

pub type RasterPipeline = daxa_sys::daxa_RasterPipeline;

#[repr(C)]
pub struct ShaderInfo<'a> {
    byte_code: *const u32,
    byte_code_size: usize,
    entry_point: types::StringView<'a>,
}

#[repr(C)]
pub struct DepthTestInfo {
    depth_attachment_format: types::Format,
    enable_depth_write: bool,
    depth_test_compare_op: CompareOp,
    min_depth_bounds: f32,
    max_depth_bounds: f32,
}

impl Default for DepthTestInfo {
    fn default() -> Self {
        Self {
            depth_attachment_format: Format::UNDEFINED,
            enable_depth_write: false,
            depth_test_compare_op: CompareOp::LessOrEqual,
            min_depth_bounds: 0.0,
            max_depth_bounds: 1.0,
        }
    }
}

#[repr(C)]
pub struct RenderAttachment {
    format: types::Format,
    blend: types::Option<BlendInfo>,
}

#[repr(u32)]
pub enum BlendFactor {
    Zero = daxa_sys::VkBlendFactor_VK_BLEND_FACTOR_ZERO,
    One = daxa_sys::VkBlendFactor_VK_BLEND_FACTOR_ONE,
    SrcColor = daxa_sys::VkBlendFactor_VK_BLEND_FACTOR_SRC_COLOR,
    OneMinusSrcColor = daxa_sys::VkBlendFactor_VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR,
    DstColor = daxa_sys::VkBlendFactor_VK_BLEND_FACTOR_DST_COLOR,
    OneMinusDstColor = daxa_sys::VkBlendFactor_VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR,
    SrcAlpha = daxa_sys::VkBlendFactor_VK_BLEND_FACTOR_SRC_ALPHA,
    OneMinusSrcAlpha = daxa_sys::VkBlendFactor_VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
    DstAlpha = daxa_sys::VkBlendFactor_VK_BLEND_FACTOR_DST_ALPHA,
    OneMinusDstAlpha = daxa_sys::VkBlendFactor_VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA,
    ConstantColor = daxa_sys::VkBlendFactor_VK_BLEND_FACTOR_CONSTANT_COLOR,
    OneMinusConstantColor = daxa_sys::VkBlendFactor_VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR,
    ConstantAlpha = daxa_sys::VkBlendFactor_VK_BLEND_FACTOR_CONSTANT_ALPHA,
    OneMinusConstantAlpha = daxa_sys::VkBlendFactor_VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA,
    SrcAlphaSaturate = daxa_sys::VkBlendFactor_VK_BLEND_FACTOR_SRC_ALPHA_SATURATE,
    Src1Color = daxa_sys::VkBlendFactor_VK_BLEND_FACTOR_SRC1_COLOR,
    OneMinusSrc1Color = daxa_sys::VkBlendFactor_VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR,
    Src1Alpha = daxa_sys::VkBlendFactor_VK_BLEND_FACTOR_SRC1_ALPHA,
    OneMinusSrc1Alpha = daxa_sys::VkBlendFactor_VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA,
}

#[repr(u32)]
pub enum BlendOp {
    Add = daxa_sys::VkBlendOp_VK_BLEND_OP_ADD,
    Subtract = daxa_sys::VkBlendOp_VK_BLEND_OP_SUBTRACT,
    ReverseSubtract = daxa_sys::VkBlendOp_VK_BLEND_OP_REVERSE_SUBTRACT,
    Min = daxa_sys::VkBlendOp_VK_BLEND_OP_MIN,
    Max = daxa_sys::VkBlendOp_VK_BLEND_OP_MAX,
    ZeroExt = daxa_sys::VkBlendOp_VK_BLEND_OP_ZERO_EXT,
    SrcExt = daxa_sys::VkBlendOp_VK_BLEND_OP_SRC_EXT,
    DstExt = daxa_sys::VkBlendOp_VK_BLEND_OP_DST_EXT,
    SrcOverExt = daxa_sys::VkBlendOp_VK_BLEND_OP_SRC_OVER_EXT,
    DstOverExt = daxa_sys::VkBlendOp_VK_BLEND_OP_DST_OVER_EXT,
    SrcInExt = daxa_sys::VkBlendOp_VK_BLEND_OP_SRC_IN_EXT,
    DstInExt = daxa_sys::VkBlendOp_VK_BLEND_OP_DST_IN_EXT,
    SrcOutExt = daxa_sys::VkBlendOp_VK_BLEND_OP_SRC_OUT_EXT,
    DstOutExt = daxa_sys::VkBlendOp_VK_BLEND_OP_DST_OUT_EXT,
    SrcAtopExt = daxa_sys::VkBlendOp_VK_BLEND_OP_SRC_ATOP_EXT,
    DstAtopExt = daxa_sys::VkBlendOp_VK_BLEND_OP_DST_ATOP_EXT,
    XorExt = daxa_sys::VkBlendOp_VK_BLEND_OP_XOR_EXT,
    MultiplyExt = daxa_sys::VkBlendOp_VK_BLEND_OP_MULTIPLY_EXT,
    ScreenExt = daxa_sys::VkBlendOp_VK_BLEND_OP_SCREEN_EXT,
    OverlayExt = daxa_sys::VkBlendOp_VK_BLEND_OP_OVERLAY_EXT,
    DarkenExt = daxa_sys::VkBlendOp_VK_BLEND_OP_DARKEN_EXT,
    LightenExt = daxa_sys::VkBlendOp_VK_BLEND_OP_LIGHTEN_EXT,
    ColordodgeExt = daxa_sys::VkBlendOp_VK_BLEND_OP_COLORDODGE_EXT,
    ColorburnExt = daxa_sys::VkBlendOp_VK_BLEND_OP_COLORBURN_EXT,
    HardlightExt = daxa_sys::VkBlendOp_VK_BLEND_OP_HARDLIGHT_EXT,
    SoftlightExt = daxa_sys::VkBlendOp_VK_BLEND_OP_SOFTLIGHT_EXT,
    DifferenceExt = daxa_sys::VkBlendOp_VK_BLEND_OP_DIFFERENCE_EXT,
    ExclusionExt = daxa_sys::VkBlendOp_VK_BLEND_OP_EXCLUSION_EXT,
    InvertExt = daxa_sys::VkBlendOp_VK_BLEND_OP_INVERT_EXT,
    InvertRgbExt = daxa_sys::VkBlendOp_VK_BLEND_OP_INVERT_RGB_EXT,
    LineardodgeExt = daxa_sys::VkBlendOp_VK_BLEND_OP_LINEARDODGE_EXT,
    LinearburnExt = daxa_sys::VkBlendOp_VK_BLEND_OP_LINEARBURN_EXT,
    VividlightExt = daxa_sys::VkBlendOp_VK_BLEND_OP_VIVIDLIGHT_EXT,
    LinearlightExt = daxa_sys::VkBlendOp_VK_BLEND_OP_LINEARLIGHT_EXT,
    PinlightExt = daxa_sys::VkBlendOp_VK_BLEND_OP_PINLIGHT_EXT,
    HardmixExt = daxa_sys::VkBlendOp_VK_BLEND_OP_HARDMIX_EXT,
    HslHueExt = daxa_sys::VkBlendOp_VK_BLEND_OP_HSL_HUE_EXT,
    HslSaturationExt = daxa_sys::VkBlendOp_VK_BLEND_OP_HSL_SATURATION_EXT,
    HslColorExt = daxa_sys::VkBlendOp_VK_BLEND_OP_HSL_COLOR_EXT,
    HslLuminosityExt = daxa_sys::VkBlendOp_VK_BLEND_OP_HSL_LUMINOSITY_EXT,
    PlusExt = daxa_sys::VkBlendOp_VK_BLEND_OP_PLUS_EXT,
    PlusClampedExt = daxa_sys::VkBlendOp_VK_BLEND_OP_PLUS_CLAMPED_EXT,
    PlusClampedAlphaExt = daxa_sys::VkBlendOp_VK_BLEND_OP_PLUS_CLAMPED_ALPHA_EXT,
    PlusDarkerExt = daxa_sys::VkBlendOp_VK_BLEND_OP_PLUS_DARKER_EXT,
    MinusExt = daxa_sys::VkBlendOp_VK_BLEND_OP_MINUS_EXT,
    MinusClampedExt = daxa_sys::VkBlendOp_VK_BLEND_OP_MINUS_CLAMPED_EXT,
    ContrastExt = daxa_sys::VkBlendOp_VK_BLEND_OP_CONTRAST_EXT,
    InvertOvgExt = daxa_sys::VkBlendOp_VK_BLEND_OP_INVERT_OVG_EXT,
    RedExt = daxa_sys::VkBlendOp_VK_BLEND_OP_RED_EXT,
    GreenExt = daxa_sys::VkBlendOp_VK_BLEND_OP_GREEN_EXT,
    BlueExt = daxa_sys::VkBlendOp_VK_BLEND_OP_BLUE_EXT,
}

bitflags! {
    #[derive(Default)]
    pub struct ColorComponentFlags: u32 {
        const R = daxa_sys::VkColorComponentFlagBits_VK_COLOR_COMPONENT_R_BIT;
        const G = daxa_sys::VkColorComponentFlagBits_VK_COLOR_COMPONENT_G_BIT;
        const B = daxa_sys::VkColorComponentFlagBits_VK_COLOR_COMPONENT_B_BIT;
        const A = daxa_sys::VkColorComponentFlagBits_VK_COLOR_COMPONENT_A_BIT;
    }
}

#[repr(C)]
pub struct BlendInfo {
    src_color_blend_factor: BlendFactor,
    dst_color_blend_factor: BlendFactor,
    color_blend_op: BlendOp,
    src_alpha_blend_factor: BlendFactor,
    dst_alpha_blend_factor: BlendFactor,
    alpha_blend_op: BlendOp,
    color_write_mask: ColorComponentFlags,
}

impl Default for BlendInfo {
    fn default() -> Self {
        Self {
            src_color_blend_factor: BlendFactor::One,
            dst_color_blend_factor: BlendFactor::Zero,
            color_blend_op: BlendOp::Add,
            src_alpha_blend_factor: BlendFactor::One,
            dst_alpha_blend_factor: BlendFactor::Zero,
            alpha_blend_op: BlendOp::Add,
            color_write_mask: ColorComponentFlags::R
                | ColorComponentFlags::G
                | ColorComponentFlags::B
                | ColorComponentFlags::A,
        }
    }
}

#[repr(u32)]
pub enum TesselationDomainOrigin {
    UpperLeft = daxa_sys::VkTessellationDomainOrigin_VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT,
    LowerLeft = daxa_sys::VkTessellationDomainOrigin_VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT,
    UpperLeftKhr =
        daxa_sys::VkTessellationDomainOrigin_VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT_KHR,
    LowerLeftKhr =
        daxa_sys::VkTessellationDomainOrigin_VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT_KHR,
}

#[repr(C)]
pub struct TesselationInfo {
    control_points: u32,
    origin: TesselationDomainOrigin,
}

#[repr(u32)]
pub enum PrimitiveTopology {
    PointList = daxa_sys::VkPrimitiveTopology_VK_PRIMITIVE_TOPOLOGY_POINT_LIST,
    LineList = daxa_sys::VkPrimitiveTopology_VK_PRIMITIVE_TOPOLOGY_LINE_LIST,
    LineStrip = daxa_sys::VkPrimitiveTopology_VK_PRIMITIVE_TOPOLOGY_LINE_STRIP,
    TriangleList = daxa_sys::VkPrimitiveTopology_VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
    TriangleStrip = daxa_sys::VkPrimitiveTopology_VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP,
    TriangleFan = daxa_sys::VkPrimitiveTopology_VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN,
    LineListWithAdjacency =
        daxa_sys::VkPrimitiveTopology_VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY,
    LineStripWithAdjacency =
        daxa_sys::VkPrimitiveTopology_VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY,
    TriangleListWithAdjacency =
        daxa_sys::VkPrimitiveTopology_VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY,
    TriangleStripWithAdjacency =
        daxa_sys::VkPrimitiveTopology_VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY,
    PatchList = daxa_sys::VkPrimitiveTopology_VK_PRIMITIVE_TOPOLOGY_PATCH_LIST,
}

#[repr(u32)]
pub enum PolygonMode {
    Fill = daxa_sys::VkPolygonMode_VK_POLYGON_MODE_FILL,
    Line = daxa_sys::VkPolygonMode_VK_POLYGON_MODE_LINE,
    Point = daxa_sys::VkPolygonMode_VK_POLYGON_MODE_POINT,
    FillRectangleNV = daxa_sys::VkPolygonMode_VK_POLYGON_MODE_FILL_RECTANGLE_NV,
}

bitflags! {
    #[derive(Default)]
    pub struct CullModeFlags: u32 {
        const NONE = daxa_sys::VkCullModeFlagBits_VK_CULL_MODE_NONE;
        const FRONT_BIT = daxa_sys::VkCullModeFlagBits_VK_CULL_MODE_FRONT_BIT;
        const BACK_BIT = daxa_sys::VkCullModeFlagBits_VK_CULL_MODE_BACK_BIT;
        const FRONT_AND_BACK = daxa_sys::VkCullModeFlagBits_VK_CULL_MODE_FRONT_AND_BACK;
    }
}

#[repr(u32)]
pub enum FrontFace {
    CounterClockwise = daxa_sys::VkFrontFace_VK_FRONT_FACE_COUNTER_CLOCKWISE,
    Clockwise = daxa_sys::VkFrontFace_VK_FRONT_FACE_CLOCKWISE,
}

#[repr(u32)]
pub enum ConsevativeRasterizationModeEXT {
    DisabledExt = daxa_sys::VkConservativeRasterizationModeEXT_VK_CONSERVATIVE_RASTERIZATION_MODE_DISABLED_EXT,
    OverestimateExt = daxa_sys::VkConservativeRasterizationModeEXT_VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT,
    UnderestimateExt = daxa_sys::VkConservativeRasterizationModeEXT_VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT,
}

#[repr(C)]
pub struct ConsevativeRasterInfo {
    mode: ConsevativeRasterizationModeEXT,
    size: f32,
}

#[repr(C)]
pub struct RasterizerInfo {
    primitive_topology: PrimitiveTopology,
    primitive_restart_enable: bool,
    polygon_mode: PolygonMode,
    face_culling: CullModeFlags,
    front_face_winding: FrontFace,
    depth_clamp_enable: bool,
    rasterizer_discard_enable: bool,
    depth_bias_enable: bool,
    depth_bias_constant_factor: f32,
    depth_bias_clamp: f32,
    depth_bias_slope_factor: f32,
    line_width: f32,
    conservative_raster_info: types::Option<ConsevativeRasterInfo>,
}

impl Default for RasterizerInfo {
    fn default() -> Self {
        Self {
            primitive_topology: PrimitiveTopology::TriangleList,
            primitive_restart_enable: false,
            polygon_mode: PolygonMode::Fill,
            face_culling: CullModeFlags::None,
            front_face_winding: FrontFace::Clockwise,
            depth_clamp_enable: false,
            rasterizer_discard_enable: false,
            depth_bias_enable: false,
            depth_bias_constant_factor: 0.0,
            depth_bias_clamp: 0.0,
            depth_bias_slope_factor: 0.0,
            line_width: 1.0,
            conservative_raster_info: Default::default(),
        }
    }
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
    tesselation: types::Option<TesselationInfo>,
    raster: RasterizerInfo,
    push_constant_size: u32,
    name: types::StringView<'a>,
}
