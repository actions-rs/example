pub type DWORD = u32;

extern "C" {
    pub fn GetCurrentProcessId() -> DWORD;
}
