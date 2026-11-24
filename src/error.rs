use thiserror::Error;

#[derive(Debug, Error)]
pub enum Errno {
    #[error(
        "EFAULT: The memory described by (local/remote)_iov is outside the caller's accessible address space."
    )]
    Fault,
    #[error(
        "EINVAL: The sum of the iov_len values of either local_iov or remote_iov overflows a ssize_t value."
    )]
    Inval,
    #[error(
        "EPERM: The caller does not have permission to access the address space of the process pid."
    )]
    Perm,
    #[error("ESRCH: No process with ID pid exists.")]
    Srch,
    #[error("Unknown error: {0}")]
    Unknown(i32),
}

impl From<i32> for Errno {
    fn from(value: i32) -> Self {
        match value {
            libc::EFAULT => Self::Fault,
            libc::EINVAL => Self::Inval,
            libc::EPERM => Self::Perm,
            libc::ESRCH => Self::Srch,
            v => Self::Unknown(v),
        }
    }
}

impl Errno {
    pub fn get_last() -> Self {
        unsafe { Errno::from(*libc::__errno_location()) }
    }
}
