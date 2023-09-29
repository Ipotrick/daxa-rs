use types::*;
use daxa_sys;

pub struct Instance
{
    instance: daxa_sys::daxa_Instance,
}

enum InstanceFlags
{
    EnableDebugUtil,
}

struct InstanceInfo
{
    flags: InstanceFlags,
}

impl Instance
{
    pub fn new(info: &InstanceInfo) -> Self
    {

    }
}