use std::ops::{Add, Sub};

use libc::{iovec, process_vm_readv, process_vm_writev};

use crate::{casting::{AsCVoidPtr, CVoidPtr}, error::Errno};

pub fn get_value_at_location<T>(pid: u32, location: u32) -> Result<T, Errno> where T: Default + CVoidPtr {
    let mut value: T = T::default();
    
    let local = iovec {
        iov_base: value.cvoid_ptr(),
        iov_len: size_of::<T>(),
    };

    let remote = iovec {
        iov_base: location.as_cvoid_ptr(),
        iov_len: size_of::<T>(),
    };
    
    let readn;
    unsafe {
        readn = process_vm_readv(pid as i32, &local, 1, &remote, 1, 0);
    }
    
    if readn == -1 {
        return Err(Errno::get_last());
    }

    return Ok(value);
}

pub fn set_value_at_location<T>(pid: u32, location: u32, mut value: T) -> Result<isize, Errno> where T: CVoidPtr {
    let local = iovec {
        iov_base: value.cvoid_ptr(),
        iov_len: size_of::<T>(),
    };

    let remote = iovec {
        iov_base: location.as_cvoid_ptr(),
        iov_len: size_of::<T>(),
    };
    
    let writen;
    unsafe {
        writen = process_vm_writev(pid as i32, &local, 1, &remote, 1, 0);
    }
    
    if writen == -1 {
        return Err(Errno::get_last());
    }

    return Ok(writen);
}

pub fn add_value_at_location<T>(pid: u32, location: u32, value: T) -> Result<T, Errno> where T: Default + Copy + Add<T, Output=T> + CVoidPtr {
    let current: T = get_value_at_location(pid, location)?;
    let new_value = current + value;
    set_value_at_location(pid, location, new_value)?;
    Ok(new_value)
}

pub fn sub_value_at_location<T>(pid: u32, location: u32, value: T) -> Result<T, Errno> where T: Default + Copy + Sub<T, Output=T> + CVoidPtr {
    let current: T = get_value_at_location(pid, location)?;
    let new_value = current - value;
    set_value_at_location(pid, location, new_value)?;
    Ok(new_value)
}
