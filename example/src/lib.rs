#[cfg(unix)]
pub type Pid = example_sys::unix::pid_t;

#[cfg(target_os = "windows")]
pub type Pid = example_sys::windows::DWORD;

#[cfg(unix)]
pub fn pid() -> Pid {
    unsafe {
        example_sys::unix::getpid()
    }
}

#[cfg(target_os = "windows")]
pub fn pid() -> Pid {
    unsafe {
        example_sys::windows::GetCurrentProcessId()
    }
}
