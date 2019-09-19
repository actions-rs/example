#[allow(non_camel_case_types)]
pub type pid_t = i32;

#[cfg(unix)]
extern "C" {
    pub fn getpid() -> pid_t;
}
