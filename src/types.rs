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
pub struct AllocateInfo {
    query_count: u32,
    name: *const os::raw::c_char,
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
