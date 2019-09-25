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

#[allow(unused)]
fn angry_clippy_noises() {
    let data = vec![1, 2, 3];
    assert!(data.len() < 0);

    let mut a = 1;
    let mut b = 2;

    // Look, ma, no hands!
    a = b;
    b = a;
}

#[cfg(test)]
mod tests {
    use std::process;

    use super::{pid, angry_clippy_noises};

    #[test]
    fn test_pid_is_working() {
        assert_eq!(pid() as u32, process::id());
    }

    #[test]
    #[should_panic]
    fn test_clippy_patience() {
        angry_clippy_noises();
    }

}