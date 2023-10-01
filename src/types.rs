use bitflags::bitflags;
use std::os;

#[repr(i32)]
pub enum Result {
    Unknown = daxa_sys::daxa_Result_DAXA_RESULT_UNKNOWN,
    Success = daxa_sys::daxa_Result_DAXA_RESULT_SUCCESS,
    MissingExtension = daxa_sys::daxa_Result_DAXA_RESULT_MISSING_EXTENSION,
}

#[repr(i32)]
pub enum ImageLayout {
    Undefined = daxa_sys::daxa_ImageLayout_DAXA_IMAGE_LAYOUT_UNDEFINED,
    General = daxa_sys::daxa_ImageLayout_DAXA_IMAGE_LAYOUT_GENERAL,
    TransferSrcOptimal = daxa_sys::daxa_ImageLayout_DAXA_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
    TransferDstOptimal = daxa_sys::daxa_ImageLayout_DAXA_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
    ReadOnlyOptimal = daxa_sys::daxa_ImageLayout_DAXA_IMAGE_LAYOUT_READ_ONLY_OPTIMAL,
    AttachmentOptimal = daxa_sys::daxa_ImageLayout_DAXA_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL,
    PresentSrc = daxa_sys::daxa_ImageLayout_DAXA_IMAGE_LAYOUT_PRESENT_SRC,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec2<T> {
    x: T,
    y: T,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec2<T> {
    x: T,
    y: T,
    z: T,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec2<T> {
    x: T,
    y: T,
    z: T,
    w: T,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ImageMipArraySlice {
    base_mip_level: u32,
    level_count: u32,
    base_array_layer: u32,
    layer_count: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ImageArraySlice {
    mip_level: u32,
    base_array_layer: u32,
    layer_count: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ImageSlice {
    mip_level: u32,
    array_layer: u32,
}

bitflags! {
    #[derive(Default)]
    pub struct MemoryFlags: u32 {
        const DEDICATED_MEMORY = daxa_sys::DAXA_MEMORY_FLAG_DEDICATED_MEMORY;
        const CAN_ALIAS = daxa_sys::DAXA_MEMORY_FLAG_CAN_ALIAS;
        const SEQUENTIAL_WRITE = daxa_sys::DAXA_MEMORY_FLAG_HOST_ACCESS_SEQUENTIAL_WRITE;
        const HOST_ACCESS_RANDOM = daxa_sys::DAXA_MEMORY_FLAG_HOST_ACCESS_RANDOM;
        const MIN_MEMORY = daxa_sys::DAXA_MEMORY_FLAG_STRATEGY_MIN_MEMORY;
        const MIN_TIME = daxa_sys::DAXA_MEMORY_FLAG_STRATEGY_MIN_TIME;
    }
}

pub type VkDeviceSize = u64;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MemoryBlockInfo {
    requirements: daxa_sys::VkMemoryRequirements,
    flags: MemoryFlags,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ManualAllocInfo {
    memory_block: MemoryBlockInfo,
    offset: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union AllocInfo {
    auto_alloc_info: MemoryFlags,
    manual_alloc_info: ManualAllocInfo,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AllocateInfo {
    index: u64,
    info: AllocInfo,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AllocateInfo<'a> {
    query_count: u32,
    name: StringView<'a>,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Option<T> {
    data: T,
    has_value: bool,
}

impl<T: Clone + Copy> Into<std::option::Option<&T>> for *const self::Option<T> {
    fn into(self) -> std::option::Option<&T> {
        unsafe {
            let this = self.as_ref().unwrap();
            if this.has_value {
                drop(this);
                Some(unsafe { self.cast::<T>().as_ref().unwrap() })
            } else {
                None
            }
        }
    }
}

impl<T: Clone + Copy> Into<std::option::Option<&mut T>> for *mut self::Option<T> {
    fn into(self) -> std::option::Option<&T> {
        unsafe {
            let this = self.as_ref().unwrap();
            if this.has_value {
                drop(this);
                Some(unsafe { self.cast::<T>().as_mut().unwrap() })
            } else {
                None
            }
        }
    }
}


pub struct StringView<'a> {
    ptr: *const os::raw::c_char,
    len: usize,
    //this does not affect size or alignment, only lifetime
    marker: PhantomData<&'a str>,
}

impl<'a> StringView<'a> {
    pub unsafe fn from_ptr(ptr: *const os::raw::c_char) -> StringView<'a> {
        let mut len = 0;
        while *ptr.add(len) != 0 {
            len += 1;
        }
        Self {
            ptr,
            len,
            marker: PhantomData,
        }
    }
    pub unsafe fn from_mut_ptr(ptr: *mut os::raw::c_char) -> StringView<'a> {
        let mut len = 0;
        while *ptr.add(len) != 0 {
            len += 1;
        }
        Self {
            ptr,
            len,
            marker: PhantomData,
        }
    }
}

impl<'a, const N: usize> From<&'a [u8; N]> for StringView<'a> {
    fn from(data: &'a [u8; N]) -> Self {
        let ptr = data as *const u8 as *const os::raw::c_char;
        Self {
            ptr,
            len: N as _,
            marker: PhantomData,
        }
    }
}

bitflags! {
    #[derive(Default)]
    pub struct Format: i32 {
        pub const UNDEFINED                                                 = daxa_sys::VkFormat_VK_FORMAT_UNDEFINED                                                  ;
        pub const R4G4_UNORM_PACK8                                          = daxa_sys::VkFormat_VK_FORMAT_R4G4_UNORM_PACK8                                           ;
        pub const R4G4B4A4_UNORM_PACK16                                     = daxa_sys::VkFormat_VK_FORMAT_R4G4B4A4_UNORM_PACK16                                      ;
        pub const B4G4R4A4_UNORM_PACK16                                     = daxa_sys::VkFormat_VK_FORMAT_B4G4R4A4_UNORM_PACK16                                      ;
        pub const R5G6B5_UNORM_PACK16                                       = daxa_sys::VkFormat_VK_FORMAT_R5G6B5_UNORM_PACK16                                        ;
        pub const B5G6R5_UNORM_PACK16                                       = daxa_sys::VkFormat_VK_FORMAT_B5G6R5_UNORM_PACK16                                        ;
        pub const R5G5B5A1_UNORM_PACK16                                     = daxa_sys::VkFormat_VK_FORMAT_R5G5B5A1_UNORM_PACK16                                      ;
        pub const B5G5R5A1_UNORM_PACK16                                     = daxa_sys::VkFormat_VK_FORMAT_B5G5R5A1_UNORM_PACK16                                      ;
        pub const A1R5G5B5_UNORM_PACK16                                     = daxa_sys::VkFormat_VK_FORMAT_A1R5G5B5_UNORM_PACK16                                      ;
        pub const R8_UNORM                                                  = daxa_sys::VkFormat_VK_FORMAT_R8_UNORM                                                   ;
        pub const R8_SNORM                                                  = daxa_sys::VkFormat_VK_FORMAT_R8_SNORM                                                   ;
        pub const R8_USCALED                                                = daxa_sys::VkFormat_VK_FORMAT_R8_USCALED                                                 ;
        pub const R8_SSCALED                                                = daxa_sys::VkFormat_VK_FORMAT_R8_SSCALED                                                 ;
        pub const R8_UINT                                                   = daxa_sys::VkFormat_VK_FORMAT_R8_UINT                                                    ;
        pub const R8_SINT                                                   = daxa_sys::VkFormat_VK_FORMAT_R8_SINT                                                    ;
        pub const R8_SRGB                                                   = daxa_sys::VkFormat_VK_FORMAT_R8_SRGB                                                    ;
        pub const R8G8_UNORM                                                = daxa_sys::VkFormat_VK_FORMAT_R8G8_UNORM                                                 ;
        pub const R8G8_SNORM                                                = daxa_sys::VkFormat_VK_FORMAT_R8G8_SNORM                                                 ;
        pub const R8G8_USCALED                                              = daxa_sys::VkFormat_VK_FORMAT_R8G8_USCALED                                               ;
        pub const R8G8_SSCALED                                              = daxa_sys::VkFormat_VK_FORMAT_R8G8_SSCALED                                               ;
        pub const R8G8_UINT                                                 = daxa_sys::VkFormat_VK_FORMAT_R8G8_UINT                                                  ;
        pub const R8G8_SINT                                                 = daxa_sys::VkFormat_VK_FORMAT_R8G8_SINT                                                  ;
        pub const R8G8_SRGB                                                 = daxa_sys::VkFormat_VK_FORMAT_R8G8_SRGB                                                  ;
        pub const R8G8B8_UNORM                                              = daxa_sys::VkFormat_VK_FORMAT_R8G8B8_UNORM                                               ;
        pub const R8G8B8_SNORM                                              = daxa_sys::VkFormat_VK_FORMAT_R8G8B8_SNORM                                               ;
        pub const R8G8B8_USCALED                                            = daxa_sys::VkFormat_VK_FORMAT_R8G8B8_USCALED                                             ;
        pub const R8G8B8_SSCALED                                            = daxa_sys::VkFormat_VK_FORMAT_R8G8B8_SSCALED                                             ;
        pub const R8G8B8_UINT                                               = daxa_sys::VkFormat_VK_FORMAT_R8G8B8_UINT                                                ;
        pub const R8G8B8_SINT                                               = daxa_sys::VkFormat_VK_FORMAT_R8G8B8_SINT                                                ;
        pub const R8G8B8_SRGB                                               = daxa_sys::VkFormat_VK_FORMAT_R8G8B8_SRGB                                                ;
        pub const B8G8R8_UNORM                                              = daxa_sys::VkFormat_VK_FORMAT_B8G8R8_UNORM                                               ;
        pub const B8G8R8_SNORM                                              = daxa_sys::VkFormat_VK_FORMAT_B8G8R8_SNORM                                               ;
        pub const B8G8R8_USCALED                                            = daxa_sys::VkFormat_VK_FORMAT_B8G8R8_USCALED                                             ;
        pub const B8G8R8_SSCALED                                            = daxa_sys::VkFormat_VK_FORMAT_B8G8R8_SSCALED                                             ;
        pub const B8G8R8_UINT                                               = daxa_sys::VkFormat_VK_FORMAT_B8G8R8_UINT                                                ;
        pub const B8G8R8_SINT                                               = daxa_sys::VkFormat_VK_FORMAT_B8G8R8_SINT                                                ;
        pub const B8G8R8_SRGB                                               = daxa_sys::VkFormat_VK_FORMAT_B8G8R8_SRGB                                                ;
        pub const R8G8B8A8_UNORM                                            = daxa_sys::VkFormat_VK_FORMAT_R8G8B8A8_UNORM                                             ;
        pub const R8G8B8A8_SNORM                                            = daxa_sys::VkFormat_VK_FORMAT_R8G8B8A8_SNORM                                             ;
        pub const R8G8B8A8_USCALED                                          = daxa_sys::VkFormat_VK_FORMAT_R8G8B8A8_USCALED                                           ;
        pub const R8G8B8A8_SSCALED                                          = daxa_sys::VkFormat_VK_FORMAT_R8G8B8A8_SSCALED                                           ;
        pub const R8G8B8A8_UINT                                             = daxa_sys::VkFormat_VK_FORMAT_R8G8B8A8_UINT                                              ;
        pub const R8G8B8A8_SINT                                             = daxa_sys::VkFormat_VK_FORMAT_R8G8B8A8_SINT                                              ;
        pub const R8G8B8A8_SRGB                                             = daxa_sys::VkFormat_VK_FORMAT_R8G8B8A8_SRGB                                              ;
        pub const B8G8R8A8_UNORM                                            = daxa_sys::VkFormat_VK_FORMAT_B8G8R8A8_UNORM                                             ;
        pub const B8G8R8A8_SNORM                                            = daxa_sys::VkFormat_VK_FORMAT_B8G8R8A8_SNORM                                             ;
        pub const B8G8R8A8_USCALED                                          = daxa_sys::VkFormat_VK_FORMAT_B8G8R8A8_USCALED                                           ;
        pub const B8G8R8A8_SSCALED                                          = daxa_sys::VkFormat_VK_FORMAT_B8G8R8A8_SSCALED                                           ;
        pub const B8G8R8A8_UINT                                             = daxa_sys::VkFormat_VK_FORMAT_B8G8R8A8_UINT                                              ;
        pub const B8G8R8A8_SINT                                             = daxa_sys::VkFormat_VK_FORMAT_B8G8R8A8_SINT                                              ;
        pub const B8G8R8A8_SRGB                                             = daxa_sys::VkFormat_VK_FORMAT_B8G8R8A8_SRGB                                              ;
        pub const A8B8G8R8_UNORM_PACK32                                     = daxa_sys::VkFormat_VK_FORMAT_A8B8G8R8_UNORM_PACK32                                      ;
        pub const A8B8G8R8_SNORM_PACK32                                     = daxa_sys::VkFormat_VK_FORMAT_A8B8G8R8_SNORM_PACK32                                      ;
        pub const A8B8G8R8_USCALED_PACK32                                   = daxa_sys::VkFormat_VK_FORMAT_A8B8G8R8_USCALED_PACK32                                    ;
        pub const A8B8G8R8_SSCALED_PACK32                                   = daxa_sys::VkFormat_VK_FORMAT_A8B8G8R8_SSCALED_PACK32                                    ;
        pub const A8B8G8R8_UINT_PACK32                                      = daxa_sys::VkFormat_VK_FORMAT_A8B8G8R8_UINT_PACK32                                       ;
        pub const A8B8G8R8_SINT_PACK32                                      = daxa_sys::VkFormat_VK_FORMAT_A8B8G8R8_SINT_PACK32                                       ;
        pub const A8B8G8R8_SRGB_PACK32                                      = daxa_sys::VkFormat_VK_FORMAT_A8B8G8R8_SRGB_PACK32                                       ;
        pub const A2R10G10B10_UNORM_PACK32                                  = daxa_sys::VkFormat_VK_FORMAT_A2R10G10B10_UNORM_PACK32                                   ;
        pub const A2R10G10B10_SNORM_PACK32                                  = daxa_sys::VkFormat_VK_FORMAT_A2R10G10B10_SNORM_PACK32                                   ;
        pub const A2R10G10B10_USCALED_PACK32                                = daxa_sys::VkFormat_VK_FORMAT_A2R10G10B10_USCALED_PACK32                                 ;
        pub const A2R10G10B10_SSCALED_PACK32                                = daxa_sys::VkFormat_VK_FORMAT_A2R10G10B10_SSCALED_PACK32                                 ;
        pub const A2R10G10B10_UINT_PACK32                                   = daxa_sys::VkFormat_VK_FORMAT_A2R10G10B10_UINT_PACK32                                    ;
        pub const A2R10G10B10_SINT_PACK32                                   = daxa_sys::VkFormat_VK_FORMAT_A2R10G10B10_SINT_PACK32                                    ;
        pub const A2B10G10R10_UNORM_PACK32                                  = daxa_sys::VkFormat_VK_FORMAT_A2B10G10R10_UNORM_PACK32                                   ;
        pub const A2B10G10R10_SNORM_PACK32                                  = daxa_sys::VkFormat_VK_FORMAT_A2B10G10R10_SNORM_PACK32                                   ;
        pub const A2B10G10R10_USCALED_PACK32                                = daxa_sys::VkFormat_VK_FORMAT_A2B10G10R10_USCALED_PACK32                                 ;
        pub const A2B10G10R10_SSCALED_PACK32                                = daxa_sys::VkFormat_VK_FORMAT_A2B10G10R10_SSCALED_PACK32                                 ;
        pub const A2B10G10R10_UINT_PACK32                                   = daxa_sys::VkFormat_VK_FORMAT_A2B10G10R10_UINT_PACK32                                    ;
        pub const A2B10G10R10_SINT_PACK32                                   = daxa_sys::VkFormat_VK_FORMAT_A2B10G10R10_SINT_PACK32                                    ;
        pub const R16_UNORM                                                 = daxa_sys::VkFormat_VK_FORMAT_R16_UNORM                                                  ;
        pub const R16_SNORM                                                 = daxa_sys::VkFormat_VK_FORMAT_R16_SNORM                                                  ;
        pub const R16_USCALED                                               = daxa_sys::VkFormat_VK_FORMAT_R16_USCALED                                                ;
        pub const R16_SSCALED                                               = daxa_sys::VkFormat_VK_FORMAT_R16_SSCALED                                                ;
        pub const R16_UINT                                                  = daxa_sys::VkFormat_VK_FORMAT_R16_UINT                                                   ;
        pub const R16_SINT                                                  = daxa_sys::VkFormat_VK_FORMAT_R16_SINT                                                   ;
        pub const R16_SFLOAT                                                = daxa_sys::VkFormat_VK_FORMAT_R16_SFLOAT                                                 ;
        pub const R16G16_UNORM                                              = daxa_sys::VkFormat_VK_FORMAT_R16G16_UNORM                                               ;
        pub const R16G16_SNORM                                              = daxa_sys::VkFormat_VK_FORMAT_R16G16_SNORM                                               ;
        pub const R16G16_USCALED                                            = daxa_sys::VkFormat_VK_FORMAT_R16G16_USCALED                                             ;
        pub const R16G16_SSCALED                                            = daxa_sys::VkFormat_VK_FORMAT_R16G16_SSCALED                                             ;
        pub const R16G16_UINT                                               = daxa_sys::VkFormat_VK_FORMAT_R16G16_UINT                                                ;
        pub const R16G16_SINT                                               = daxa_sys::VkFormat_VK_FORMAT_R16G16_SINT                                                ;
        pub const R16G16_SFLOAT                                             = daxa_sys::VkFormat_VK_FORMAT_R16G16_SFLOAT                                              ;
        pub const R16G16B16_UNORM                                           = daxa_sys::VkFormat_VK_FORMAT_R16G16B16_UNORM                                            ;
        pub const R16G16B16_SNORM                                           = daxa_sys::VkFormat_VK_FORMAT_R16G16B16_SNORM                                            ;
        pub const R16G16B16_USCALED                                         = daxa_sys::VkFormat_VK_FORMAT_R16G16B16_USCALED                                          ;
        pub const R16G16B16_SSCALED                                         = daxa_sys::VkFormat_VK_FORMAT_R16G16B16_SSCALED                                          ;
        pub const R16G16B16_UINT                                            = daxa_sys::VkFormat_VK_FORMAT_R16G16B16_UINT                                             ;
        pub const R16G16B16_SINT                                            = daxa_sys::VkFormat_VK_FORMAT_R16G16B16_SINT                                             ;
        pub const R16G16B16_SFLOAT                                          = daxa_sys::VkFormat_VK_FORMAT_R16G16B16_SFLOAT                                           ;
        pub const R16G16B16A16_UNORM                                        = daxa_sys::VkFormat_VK_FORMAT_R16G16B16A16_UNORM                                         ;
        pub const R16G16B16A16_SNORM                                        = daxa_sys::VkFormat_VK_FORMAT_R16G16B16A16_SNORM                                         ;
        pub const R16G16B16A16_USCALED                                      = daxa_sys::VkFormat_VK_FORMAT_R16G16B16A16_USCALED                                       ;
        pub const R16G16B16A16_SSCALED                                      = daxa_sys::VkFormat_VK_FORMAT_R16G16B16A16_SSCALED                                       ;
        pub const R16G16B16A16_UINT                                         = daxa_sys::VkFormat_VK_FORMAT_R16G16B16A16_UINT                                          ;
        pub const R16G16B16A16_SINT                                         = daxa_sys::VkFormat_VK_FORMAT_R16G16B16A16_SINT                                          ;
        pub const R16G16B16A16_SFLOAT                                       = daxa_sys::VkFormat_VK_FORMAT_R16G16B16A16_SFLOAT                                        ;
        pub const R32_UINT                                                  = daxa_sys::VkFormat_VK_FORMAT_R32_UINT                                                   ;
        pub const R32_SINT                                                  = daxa_sys::VkFormat_VK_FORMAT_R32_SINT                                                   ;
        pub const R32_SFLOAT                                                = daxa_sys::VkFormat_VK_FORMAT_R32_SFLOAT                                                 ;
        pub const R32G32_UINT                                               = daxa_sys::VkFormat_VK_FORMAT_R32G32_UINT                                                ;
        pub const R32G32_SINT                                               = daxa_sys::VkFormat_VK_FORMAT_R32G32_SINT                                                ;
        pub const R32G32_SFLOAT                                             = daxa_sys::VkFormat_VK_FORMAT_R32G32_SFLOAT                                              ;
        pub const R32G32B32_UINT                                            = daxa_sys::VkFormat_VK_FORMAT_R32G32B32_UINT                                             ;
        pub const R32G32B32_SINT                                            = daxa_sys::VkFormat_VK_FORMAT_R32G32B32_SINT                                             ;
        pub const R32G32B32_SFLOAT                                          = daxa_sys::VkFormat_VK_FORMAT_R32G32B32_SFLOAT                                           ;
        pub const R32G32B32A32_UINT                                         = daxa_sys::VkFormat_VK_FORMAT_R32G32B32A32_UINT                                          ;
        pub const R32G32B32A32_SINT                                         = daxa_sys::VkFormat_VK_FORMAT_R32G32B32A32_SINT                                          ;
        pub const R32G32B32A32_SFLOAT                                       = daxa_sys::VkFormat_VK_FORMAT_R32G32B32A32_SFLOAT                                        ;
        pub const R64_UINT                                                  = daxa_sys::VkFormat_VK_FORMAT_R64_UINT                                                   ;
        pub const R64_SINT                                                  = daxa_sys::VkFormat_VK_FORMAT_R64_SINT                                                   ;
        pub const R64_SFLOAT                                                = daxa_sys::VkFormat_VK_FORMAT_R64_SFLOAT                                                 ;
        pub const R64G64_UINT                                               = daxa_sys::VkFormat_VK_FORMAT_R64G64_UINT                                                ;
        pub const R64G64_SINT                                               = daxa_sys::VkFormat_VK_FORMAT_R64G64_SINT                                                ;
        pub const R64G64_SFLOAT                                             = daxa_sys::VkFormat_VK_FORMAT_R64G64_SFLOAT                                              ;
        pub const R64G64B64_UINT                                            = daxa_sys::VkFormat_VK_FORMAT_R64G64B64_UINT                                             ;
        pub const R64G64B64_SINT                                            = daxa_sys::VkFormat_VK_FORMAT_R64G64B64_SINT                                             ;
        pub const R64G64B64_SFLOAT                                          = daxa_sys::VkFormat_VK_FORMAT_R64G64B64_SFLOAT                                           ;
        pub const R64G64B64A64_UINT                                         = daxa_sys::VkFormat_VK_FORMAT_R64G64B64A64_UINT                                          ;
        pub const R64G64B64A64_SINT                                         = daxa_sys::VkFormat_VK_FORMAT_R64G64B64A64_SINT                                          ;
        pub const R64G64B64A64_SFLOAT                                       = daxa_sys::VkFormat_VK_FORMAT_R64G64B64A64_SFLOAT                                        ;
        pub const B10G11R11_UFLOAT_PACK32                                   = daxa_sys::VkFormat_VK_FORMAT_B10G11R11_UFLOAT_PACK32                                    ;
        pub const E5B9G9R9_UFLOAT_PACK32                                    = daxa_sys::VkFormat_VK_FORMAT_E5B9G9R9_UFLOAT_PACK32                                     ;
        pub const D16_UNORM                                                 = daxa_sys::VkFormat_VK_FORMAT_D16_UNORM                                                  ;
        pub const X8_D24_UNORM_PACK32                                       = daxa_sys::VkFormat_VK_FORMAT_X8_D24_UNORM_PACK32                                        ;
        pub const D32_SFLOAT                                                = daxa_sys::VkFormat_VK_FORMAT_D32_SFLOAT                                                 ;
        pub const S8_UINT                                                   = daxa_sys::VkFormat_VK_FORMAT_S8_UINT                                                    ;
        pub const D16_UNORM_S8_UINT                                         = daxa_sys::VkFormat_VK_FORMAT_D16_UNORM_S8_UINT                                          ;
        pub const D24_UNORM_S8_UINT                                         = daxa_sys::VkFormat_VK_FORMAT_D24_UNORM_S8_UINT                                          ;
        pub const D32_SFLOAT_S8_UINT                                        = daxa_sys::VkFormat_VK_FORMAT_D32_SFLOAT_S8_UINT                                         ;
        pub const BC1_RGB_UNORM_BLOCK                                       = daxa_sys::VkFormat_VK_FORMAT_BC1_RGB_UNORM_BLOCK                                        ;
        pub const BC1_RGB_SRGB_BLOCK                                        = daxa_sys::VkFormat_VK_FORMAT_BC1_RGB_SRGB_BLOCK                                         ;
        pub const BC1_RGBA_UNORM_BLOCK                                      = daxa_sys::VkFormat_VK_FORMAT_BC1_RGBA_UNORM_BLOCK                                       ;
        pub const BC1_RGBA_SRGB_BLOCK                                       = daxa_sys::VkFormat_VK_FORMAT_BC1_RGBA_SRGB_BLOCK                                        ;
        pub const BC2_UNORM_BLOCK                                           = daxa_sys::VkFormat_VK_FORMAT_BC2_UNORM_BLOCK                                            ;
        pub const BC2_SRGB_BLOCK                                            = daxa_sys::VkFormat_VK_FORMAT_BC2_SRGB_BLOCK                                             ;
        pub const BC3_UNORM_BLOCK                                           = daxa_sys::VkFormat_VK_FORMAT_BC3_UNORM_BLOCK                                            ;
        pub const BC3_SRGB_BLOCK                                            = daxa_sys::VkFormat_VK_FORMAT_BC3_SRGB_BLOCK                                             ;
        pub const BC4_UNORM_BLOCK                                           = daxa_sys::VkFormat_VK_FORMAT_BC4_UNORM_BLOCK                                            ;
        pub const BC4_SNORM_BLOCK                                           = daxa_sys::VkFormat_VK_FORMAT_BC4_SNORM_BLOCK                                            ;
        pub const BC5_UNORM_BLOCK                                           = daxa_sys::VkFormat_VK_FORMAT_BC5_UNORM_BLOCK                                            ;
        pub const BC5_SNORM_BLOCK                                           = daxa_sys::VkFormat_VK_FORMAT_BC5_SNORM_BLOCK                                            ;
        pub const BC6H_UFLOAT_BLOCK                                         = daxa_sys::VkFormat_VK_FORMAT_BC6H_UFLOAT_BLOCK                                          ;
        pub const BC6H_SFLOAT_BLOCK                                         = daxa_sys::VkFormat_VK_FORMAT_BC6H_SFLOAT_BLOCK                                          ;
        pub const BC7_UNORM_BLOCK                                           = daxa_sys::VkFormat_VK_FORMAT_BC7_UNORM_BLOCK                                            ;
        pub const BC7_SRGB_BLOCK                                            = daxa_sys::VkFormat_VK_FORMAT_BC7_SRGB_BLOCK                                             ;
        pub const ETC2_R8G8B8_UNORM_BLOCK                                   = daxa_sys::VkFormat_VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK                                    ;
        pub const ETC2_R8G8B8_SRGB_BLOCK                                    = daxa_sys::VkFormat_VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK                                     ;
        pub const ETC2_R8G8B8A1_UNORM_BLOCK                                 = daxa_sys::VkFormat_VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK                                  ;
        pub const ETC2_R8G8B8A1_SRGB_BLOCK                                  = daxa_sys::VkFormat_VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK                                   ;
        pub const ETC2_R8G8B8A8_UNORM_BLOCK                                 = daxa_sys::VkFormat_VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK                                  ;
        pub const ETC2_R8G8B8A8_SRGB_BLOCK                                  = daxa_sys::VkFormat_VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK                                   ;
        pub const EAC_R11_UNORM_BLOCK                                       = daxa_sys::VkFormat_VK_FORMAT_EAC_R11_UNORM_BLOCK                                        ;
        pub const EAC_R11_SNORM_BLOCK                                       = daxa_sys::VkFormat_VK_FORMAT_EAC_R11_SNORM_BLOCK                                        ;
        pub const EAC_R11G11_UNORM_BLOCK                                    = daxa_sys::VkFormat_VK_FORMAT_EAC_R11G11_UNORM_BLOCK                                     ;
        pub const EAC_R11G11_SNORM_BLOCK                                    = daxa_sys::VkFormat_VK_FORMAT_EAC_R11G11_SNORM_BLOCK                                     ;
        pub const ASTC_4x4_UNORM_BLOCK                                      = daxa_sys::VkFormat_VK_FORMAT_ASTC_4x4_UNORM_BLOCK                                       ;
        pub const ASTC_4x4_SRGB_BLOCK                                       = daxa_sys::VkFormat_VK_FORMAT_ASTC_4x4_SRGB_BLOCK                                        ;
        pub const ASTC_5x4_UNORM_BLOCK                                      = daxa_sys::VkFormat_VK_FORMAT_ASTC_5x4_UNORM_BLOCK                                       ;
        pub const ASTC_5x4_SRGB_BLOCK                                       = daxa_sys::VkFormat_VK_FORMAT_ASTC_5x4_SRGB_BLOCK                                        ;
        pub const ASTC_5x5_UNORM_BLOCK                                      = daxa_sys::VkFormat_VK_FORMAT_ASTC_5x5_UNORM_BLOCK                                       ;
        pub const ASTC_5x5_SRGB_BLOCK                                       = daxa_sys::VkFormat_VK_FORMAT_ASTC_5x5_SRGB_BLOCK                                        ;
        pub const ASTC_6x5_UNORM_BLOCK                                      = daxa_sys::VkFormat_VK_FORMAT_ASTC_6x5_UNORM_BLOCK                                       ;
        pub const ASTC_6x5_SRGB_BLOCK                                       = daxa_sys::VkFormat_VK_FORMAT_ASTC_6x5_SRGB_BLOCK                                        ;
        pub const ASTC_6x6_UNORM_BLOCK                                      = daxa_sys::VkFormat_VK_FORMAT_ASTC_6x6_UNORM_BLOCK                                       ;
        pub const ASTC_6x6_SRGB_BLOCK                                       = daxa_sys::VkFormat_VK_FORMAT_ASTC_6x6_SRGB_BLOCK                                        ;
        pub const ASTC_8x5_UNORM_BLOCK                                      = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x5_UNORM_BLOCK                                       ;
        pub const ASTC_8x5_SRGB_BLOCK                                       = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x5_SRGB_BLOCK                                        ;
        pub const ASTC_8x6_UNORM_BLOCK                                      = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x6_UNORM_BLOCK                                       ;
        pub const ASTC_8x6_SRGB_BLOCK                                       = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x6_SRGB_BLOCK                                        ;
        pub const ASTC_8x8_UNORM_BLOCK                                      = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x8_UNORM_BLOCK                                       ;
        pub const ASTC_8x8_SRGB_BLOCK                                       = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x8_SRGB_BLOCK                                        ;
        pub const ASTC_10x5_UNORM_BLOCK                                     = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x5_UNORM_BLOCK                                      ;
        pub const ASTC_10x5_SRGB_BLOCK                                      = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x5_SRGB_BLOCK                                       ;
        pub const ASTC_10x6_UNORM_BLOCK                                     = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x6_UNORM_BLOCK                                      ;
        pub const ASTC_10x6_SRGB_BLOCK                                      = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x6_SRGB_BLOCK                                       ;
        pub const ASTC_10x8_UNORM_BLOCK                                     = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x8_UNORM_BLOCK                                      ;
        pub const ASTC_10x8_SRGB_BLOCK                                      = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x8_SRGB_BLOCK                                       ;
        pub const ASTC_10x10_UNORM_BLOCK                                    = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x10_UNORM_BLOCK                                     ;
        pub const ASTC_10x10_SRGB_BLOCK                                     = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x10_SRGB_BLOCK                                      ;
        pub const ASTC_12x10_UNORM_BLOCK                                    = daxa_sys::VkFormat_VK_FORMAT_ASTC_12x10_UNORM_BLOCK                                     ;
        pub const ASTC_12x10_SRGB_BLOCK                                     = daxa_sys::VkFormat_VK_FORMAT_ASTC_12x10_SRGB_BLOCK                                      ;
        pub const ASTC_12x12_UNORM_BLOCK                                    = daxa_sys::VkFormat_VK_FORMAT_ASTC_12x12_UNORM_BLOCK                                     ;
        pub const ASTC_12x12_SRGB_BLOCK                                     = daxa_sys::VkFormat_VK_FORMAT_ASTC_12x12_SRGB_BLOCK                                      ;
        pub const G8B8G8R8_422_UNORM                                         = daxa_sys::VkFormat_VK_FORMAT_G8B8G8R8_422_UNORM                                        ;
        pub const B8G8R8G8_422_UNORM                                         = daxa_sys::VkFormat_VK_FORMAT_B8G8R8G8_422_UNORM                                        ;
        pub const G8_B8_R8_3PLANE_420_UNORM                                  = daxa_sys::VkFormat_VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM                                 ;
        pub const G8_B8R8_2PLANE_420_UNORM                                   = daxa_sys::VkFormat_VK_FORMAT_G8_B8R8_2PLANE_420_UNORM                                  ;
        pub const G8_B8_R8_3PLANE_422_UNORM                                  = daxa_sys::VkFormat_VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM                                 ;
        pub const G8_B8R8_2PLANE_422_UNORM                                   = daxa_sys::VkFormat_VK_FORMAT_G8_B8R8_2PLANE_422_UNORM                                  ;
        pub const G8_B8_R8_3PLANE_444_UNORM                                  = daxa_sys::VkFormat_VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM                                 ;
        pub const R10X6_UNORM_PACK16                                         = daxa_sys::VkFormat_VK_FORMAT_R10X6_UNORM_PACK16                                        ;
        pub const R10X6G10X6_UNORM_2PACK16                                   = daxa_sys::VkFormat_VK_FORMAT_R10X6G10X6_UNORM_2PACK16                                  ;
        pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16                         = daxa_sys::VkFormat_VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16                        ;
        pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16                     = daxa_sys::VkFormat_VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16                    ;
        pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16                     = daxa_sys::VkFormat_VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16                    ;
        pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16                 = daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16                ;
        pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16                  = daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16                 ;
        pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16                 = daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16                ;
        pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16                  = daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16                 ;
        pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16                 = daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16                ;
        pub const R12X4_UNORM_PACK16                                         = daxa_sys::VkFormat_VK_FORMAT_R12X4_UNORM_PACK16                                        ;
        pub const R12X4G12X4_UNORM_2PACK16                                   = daxa_sys::VkFormat_VK_FORMAT_R12X4G12X4_UNORM_2PACK16                                  ;
        pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16                         = daxa_sys::VkFormat_VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16                        ;
        pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16                     = daxa_sys::VkFormat_VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16                    ;
        pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16                     = daxa_sys::VkFormat_VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16                    ;
        pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16                 = daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16                ;
        pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16                  = daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16                 ;
        pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16                 = daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16                ;
        pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16                  = daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16                 ;
        pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16                 = daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16                ;
        pub const G16B16G16R16_422_UNORM                                     = daxa_sys::VkFormat_VK_FORMAT_G16B16G16R16_422_UNORM                                    ;
        pub const B16G16R16G16_422_UNORM                                     = daxa_sys::VkFormat_VK_FORMAT_B16G16R16G16_422_UNORM                                    ;
        pub const G16_B16_R16_3PLANE_420_UNORM                               = daxa_sys::VkFormat_VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM                              ;
        pub const G16_B16R16_2PLANE_420_UNORM                                = daxa_sys::VkFormat_VK_FORMAT_G16_B16R16_2PLANE_420_UNORM                               ;
        pub const G16_B16_R16_3PLANE_422_UNORM                               = daxa_sys::VkFormat_VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM                              ;
        pub const G16_B16R16_2PLANE_422_UNORM                                = daxa_sys::VkFormat_VK_FORMAT_G16_B16R16_2PLANE_422_UNORM                               ;
        pub const G16_B16_R16_3PLANE_444_UNORM                               = daxa_sys::VkFormat_VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM                              ;
        pub const G8_B8R8_2PLANE_444_UNORM                                   = daxa_sys::VkFormat_VK_FORMAT_G8_B8R8_2PLANE_444_UNORM                                  ;
        pub const G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16                  = daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16                 ;
        pub const G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16                  = daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16                 ;
        pub const G16_B16R16_2PLANE_444_UNORM                                = daxa_sys::VkFormat_VK_FORMAT_G16_B16R16_2PLANE_444_UNORM                               ;
        pub const A4R4G4B4_UNORM_PACK16                                      = daxa_sys::VkFormat_VK_FORMAT_A4R4G4B4_UNORM_PACK16                                     ;
        pub const A4B4G4R4_UNORM_PACK16                                      = daxa_sys::VkFormat_VK_FORMAT_A4B4G4R4_UNORM_PACK16                                     ;
        pub const ASTC_4x4_SFLOAT_BLOCK                                      = daxa_sys::VkFormat_VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK                                     ;
        pub const ASTC_5x4_SFLOAT_BLOCK                                      = daxa_sys::VkFormat_VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK                                     ;
        pub const ASTC_5x5_SFLOAT_BLOCK                                      = daxa_sys::VkFormat_VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK                                     ;
        pub const ASTC_6x5_SFLOAT_BLOCK                                      = daxa_sys::VkFormat_VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK                                     ;
        pub const ASTC_6x6_SFLOAT_BLOCK                                      = daxa_sys::VkFormat_VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK                                     ;
        pub const ASTC_8x5_SFLOAT_BLOCK                                      = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK                                     ;
        pub const ASTC_8x6_SFLOAT_BLOCK                                      = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK                                     ;
        pub const ASTC_8x8_SFLOAT_BLOCK                                      = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK                                     ;
        pub const ASTC_10x5_SFLOAT_BLOCK                                     = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK                                    ;
        pub const ASTC_10x6_SFLOAT_BLOCK                                     = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK                                    ;
        pub const ASTC_10x8_SFLOAT_BLOCK                                     = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK                                    ;
        pub const ASTC_10x10_SFLOAT_BLOCK                                    = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK                                   ;
        pub const ASTC_12x10_SFLOAT_BLOCK                                    = daxa_sys::VkFormat_VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK                                   ;
        pub const ASTC_12x12_SFLOAT_BLOCK                                    = daxa_sys::VkFormat_VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK                                   ;
        pub const PVRTC1_2BPP_UNORM_BLOCK_IMG                                = daxa_sys::VkFormat_VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG                               ;
        pub const PVRTC1_4BPP_UNORM_BLOCK_IMG                                = daxa_sys::VkFormat_VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG                               ;
        pub const PVRTC2_2BPP_UNORM_BLOCK_IMG                                = daxa_sys::VkFormat_VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG                               ;
        pub const PVRTC2_4BPP_UNORM_BLOCK_IMG                                = daxa_sys::VkFormat_VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG                               ;
        pub const PVRTC1_2BPP_SRGB_BLOCK_IMG                                 = daxa_sys::VkFormat_VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG                                ;
        pub const PVRTC1_4BPP_SRGB_BLOCK_IMG                                 = daxa_sys::VkFormat_VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG                                ;
        pub const PVRTC2_2BPP_SRGB_BLOCK_IMG                                 = daxa_sys::VkFormat_VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG                                ;
        pub const PVRTC2_4BPP_SRGB_BLOCK_IMG                                 = daxa_sys::VkFormat_VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG                                ;
        pub const R16G16_S10_5_NV                                            = daxa_sys::VkFormat_VK_FORMAT_R16G16_S10_5_NV                                           ;
        pub const A1B5G5R5_UNORM_PACK16_KHR                                  = daxa_sys::VkFormat_VK_FORMAT_A1B5G5R5_UNORM_PACK16_KHR                                 ;
        pub const A8_UNORM_KHR                                               = daxa_sys::VkFormat_VK_FORMAT_A8_UNORM_KHR                                              ;
        pub const ASTC_4x4_SFLOAT_BLOCK_EXT                                  = daxa_sys::VkFormat_VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK_EXT                                 ;
        pub const ASTC_5x4_SFLOAT_BLOCK_EXT                                  = daxa_sys::VkFormat_VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK_EXT                                 ;
        pub const ASTC_5x5_SFLOAT_BLOCK_EXT                                  = daxa_sys::VkFormat_VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK_EXT                                 ;
        pub const ASTC_6x5_SFLOAT_BLOCK_EXT                                  = daxa_sys::VkFormat_VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK_EXT                                 ;
        pub const ASTC_6x6_SFLOAT_BLOCK_EXT                                  = daxa_sys::VkFormat_VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK_EXT                                 ;
        pub const ASTC_8x5_SFLOAT_BLOCK_EXT                                  = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK_EXT                                 ;
        pub const ASTC_8x6_SFLOAT_BLOCK_EXT                                  = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK_EXT                                 ;
        pub const ASTC_8x8_SFLOAT_BLOCK_EXT                                  = daxa_sys::VkFormat_VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK_EXT                                 ;
        pub const ASTC_10x5_SFLOAT_BLOCK_EXT                                 = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK_EXT                                ;
        pub const ASTC_10x6_SFLOAT_BLOCK_EXT                                 = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK_EXT                                ;
        pub const ASTC_10x8_SFLOAT_BLOCK_EXT                                 = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK_EXT                                ;
        pub const ASTC_10x10_SFLOAT_BLOCK_EXT                                = daxa_sys::VkFormat_VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK_EXT                               ;
        pub const ASTC_12x10_SFLOAT_BLOCK_EXT                                = daxa_sys::VkFormat_VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK_EXT                               ;
        pub const ASTC_12x12_SFLOAT_BLOCK_EXT                                = daxa_sys::VkFormat_VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK_EXT                               ;
        pub const G8B8G8R8_422_UNORM_KHR                                     = daxa_sys::VkFormat_VK_FORMAT_G8B8G8R8_422_UNORM_KHR                                    ;
        pub const B8G8R8G8_422_UNORM_KHR                                     = daxa_sys::VkFormat_VK_FORMAT_B8G8R8G8_422_UNORM_KHR                                    ;
        pub const G8_B8_R8_3PLANE_420_UNORM_KHR                              = daxa_sys::VkFormat_VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM_KHR                             ;
        pub const G8_B8R8_2PLANE_420_UNORM_KHR                               = daxa_sys::VkFormat_VK_FORMAT_G8_B8R8_2PLANE_420_UNORM_KHR                              ;
        pub const G8_B8_R8_3PLANE_422_UNORM_KHR                              = daxa_sys::VkFormat_VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM_KHR                             ;
        pub const G8_B8R8_2PLANE_422_UNORM_KHR                               = daxa_sys::VkFormat_VK_FORMAT_G8_B8R8_2PLANE_422_UNORM_KHR                              ;
        pub const G8_B8_R8_3PLANE_444_UNORM_KHR                              = daxa_sys::VkFormat_VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM_KHR                             ;
        pub const R10X6_UNORM_PACK16_KHR                                     = daxa_sys::VkFormat_VK_FORMAT_R10X6_UNORM_PACK16_KHR                                    ;
        pub const R10X6G10X6_UNORM_2PACK16_KHR                               = daxa_sys::VkFormat_VK_FORMAT_R10X6G10X6_UNORM_2PACK16_KHR                              ;
        pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR                     = daxa_sys::VkFormat_VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR                    ;
        pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR                 = daxa_sys::VkFormat_VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR                ;
        pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR                 = daxa_sys::VkFormat_VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR                ;
        pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR             = daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR            ;
        pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR              = daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR             ;
        pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR             = daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR            ;
        pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR              = daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR             ;
        pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR             = daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR            ;
        pub const R12X4_UNORM_PACK16_KHR                                     = daxa_sys::VkFormat_VK_FORMAT_R12X4_UNORM_PACK16_KHR                                    ;
        pub const R12X4G12X4_UNORM_2PACK16_KHR                               = daxa_sys::VkFormat_VK_FORMAT_R12X4G12X4_UNORM_2PACK16_KHR                              ;
        pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR                     = daxa_sys::VkFormat_VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR                    ;
        pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR                 = daxa_sys::VkFormat_VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR                ;
        pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR                 = daxa_sys::VkFormat_VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR                ;
        pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR             = daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR            ;
        pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR              = daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR             ;
        pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR             = daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR            ;
        pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR              = daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR             ;
        pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR             = daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR            ;
        pub const G16B16G16R16_422_UNORM_KHR                                 = daxa_sys::VkFormat_VK_FORMAT_G16B16G16R16_422_UNORM_KHR                                ;
        pub const B16G16R16G16_422_UNORM_KHR                                 = daxa_sys::VkFormat_VK_FORMAT_B16G16R16G16_422_UNORM_KHR                                ;
        pub const G16_B16_R16_3PLANE_420_UNORM_KHR                           = daxa_sys::VkFormat_VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM_KHR                          ;
        pub const G16_B16R16_2PLANE_420_UNORM_KHR                            = daxa_sys::VkFormat_VK_FORMAT_G16_B16R16_2PLANE_420_UNORM_KHR                           ;
        pub const G16_B16_R16_3PLANE_422_UNORM_KHR                           = daxa_sys::VkFormat_VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM_KHR                          ;
        pub const G16_B16R16_2PLANE_422_UNORM_KHR                            = daxa_sys::VkFormat_VK_FORMAT_G16_B16R16_2PLANE_422_UNORM_KHR                           ;
        pub const G16_B16_R16_3PLANE_444_UNORM_KHR                           = daxa_sys::VkFormat_VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM_KHR                          ;
        pub const G8_B8R8_2PLANE_444_UNORM_EXT                               = daxa_sys::VkFormat_VK_FORMAT_G8_B8R8_2PLANE_444_UNORM_EXT                              ;
        pub const G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16_EXT              = daxa_sys::VkFormat_VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16_EXT             ;
        pub const G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16_EXT              = daxa_sys::VkFormat_VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16_EXT             ;
        pub const G16_B16R16_2PLANE_444_UNORM_EXT                            = daxa_sys::VkFormat_VK_FORMAT_G16_B16R16_2PLANE_444_UNORM_EXT                           ;
        pub const A4R4G4B4_UNORM_PACK16_EXT                                  = daxa_sys::VkFormat_VK_FORMAT_A4R4G4B4_UNORM_PACK16_EXT                                 ;
        pub const A4B4G4R4_UNORM_PACK16_EXT                                  = daxa_sys::VkFormat_VK_FORMAT_A4B4G4R4_UNORM_PACK16_EXT                                 ;
    }
}