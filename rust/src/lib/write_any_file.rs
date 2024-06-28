use ::libc;
extern "C" {
    fn geteuid() -> __uid_t;
}
pub type __uid_t = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn can_write_any_file() -> bool {
    static mut initialized: bool = false;
    static mut can_write: bool = false;
    if !initialized {
        let mut can: bool = 0 as libc::c_int != 0;
        can = geteuid() == 0 as libc::c_int as libc::c_uint;
        can_write = can;
        initialized = 1 as libc::c_int != 0;
    }
    return can_write;
}
