use std::ffi::c_void;

// Just becuase I hate this syntax and I want it out of my code
pub trait CVoidPtr {
    fn cvoid_ptr(&mut self) -> *mut c_void {
        self as *mut _ as *mut c_void
    }
}

impl CVoidPtr for f32 {}
impl CVoidPtr for i32 {}
impl CVoidPtr for i16 {}
impl CVoidPtr for i8  {}
impl CVoidPtr for u8  {}

pub trait AsCVoidPtr {
    fn as_cvoid_ptr(self) -> *mut c_void;
}

impl AsCVoidPtr for u32 {
    fn as_cvoid_ptr(self) -> *mut c_void {
        self as *mut c_void
    }
}