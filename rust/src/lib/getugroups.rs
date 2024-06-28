use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn setgrent();
    fn endgrent();
    fn getgrent() -> *mut group;
}
pub type __gid_t = libc::c_uint;
pub type gid_t = __gid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
#[no_mangle]
pub unsafe extern "C" fn getugroups(
    mut maxcount: libc::c_int,
    mut grouplist: *mut gid_t,
    mut username: *const libc::c_char,
    mut gid: gid_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut count: libc::c_int = 0 as libc::c_int;
    if gid != -(1 as libc::c_int) as gid_t {
        if maxcount != 0 as libc::c_int {
            *grouplist.offset(count as isize) = gid;
        }
        count += 1;
        count;
    }
    setgrent();
    's_29: loop {
        let mut cp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut grp: *mut group = 0 as *mut group;
        *__errno_location() = 0 as libc::c_int;
        grp = getgrent();
        if grp.is_null() {
            current_block = 11298138898191919651;
            break;
        }
        cp = (*grp).gr_mem;
        while !(*cp).is_null() {
            let mut n: libc::c_int = 0;
            if strcmp(username, *cp) == 0 as libc::c_int {
                n = 0 as libc::c_int;
                while n < count {
                    if !grouplist.is_null()
                        && *grouplist.offset(n as isize) == (*grp).gr_gid
                    {
                        break;
                    }
                    n += 1;
                    n;
                }
                if n == count {
                    if maxcount != 0 as libc::c_int {
                        if count >= maxcount {
                            current_block = 6146560607471009292;
                            break 's_29;
                        }
                        *grouplist.offset(count as isize) = (*grp).gr_gid;
                    }
                    if count == 2147483647 as libc::c_int {
                        *__errno_location() = 75 as libc::c_int;
                        current_block = 6146560607471009292;
                        break 's_29;
                    } else {
                        count += 1;
                        count;
                    }
                }
            }
            cp = cp.offset(1);
            cp;
        }
    }
    match current_block {
        11298138898191919651 => {
            if *__errno_location() != 0 as libc::c_int {
                count = -(1 as libc::c_int);
            }
        }
        _ => {}
    }
    let mut saved_errno: libc::c_int = *__errno_location();
    endgrent();
    *__errno_location() = saved_errno;
    return count;
}
