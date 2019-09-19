//! Just for example purposes this tiny crate creates its own bindings to get PID routines
//! both for Unixes and Windows, so we can test code coverage for multiple platforms later.

#[cfg(unix)]
pub mod unix;

#[cfg(target_os = "windows")]
pub mod windows;
