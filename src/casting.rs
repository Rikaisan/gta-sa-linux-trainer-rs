use std::ffi::c_void;

pub trait CVoidPtr {
    fn cvoid_ptr(&mut self) -> *mut c_void {
        std::ptr::from_mut(self).cast::<c_void>()
    }
}

impl CVoidPtr for f32 {}
impl CVoidPtr for i32 {}
impl CVoidPtr for i16 {}
impl CVoidPtr for i8 {}
impl CVoidPtr for u8 {}

pub trait ToCVoidPtr {
    fn to_cvoid_ptr(self) -> *mut c_void;
}

impl ToCVoidPtr for u32 {
    fn to_cvoid_ptr(self) -> *mut c_void {
        self as *mut c_void
    }
}
