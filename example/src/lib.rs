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

#[cfg(test)]
mod tests {
    use std::process;

    use super::pid;

    #[test]
    fn test_pid_is_working() {
        assert_eq!(pid() as u32, process::id());
    }

}