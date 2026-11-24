use libc::{iovec, process_vm_readv, process_vm_writev};

use crate::casting::{AsCVoidPtr, CVoidPtr};

pub fn get_value_at_location<T>(pid: u32, location: u32) -> Option<T> where T: Default + CVoidPtr {
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
        println!("Error reading value at location {:x}", location);
        return None;
    }

    return Some(value);
}

pub fn set_value_at_location<T>(pid: u32, location: u32, mut value: T) -> bool where T: CVoidPtr {
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
        println!("Error reading value at location {:x}", location);
        return false;
    }

    return true;
}
