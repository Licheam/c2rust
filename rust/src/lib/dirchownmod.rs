use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn lchmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn fchmod(__fd: libc::c_int, __mode: __mode_t) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn chown(
        __file: *const libc::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> libc::c_int;
    fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
    fn lchown(
        __file: *const libc::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> libc::c_int;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[no_mangle]
pub unsafe extern "C" fn dirchownmod(
    mut fd: libc::c_int,
    mut dir: *const libc::c_char,
    mut mkdir_mode: mode_t,
    mut owner: uid_t,
    mut group: gid_t,
    mut mode: mode_t,
    mut mode_bits: mode_t,
) -> libc::c_int {
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut result: libc::c_int = if fd < 0 as libc::c_int {
        stat(dir, &mut st)
    } else {
        fstat(fd, &mut st)
    };
    if result == 0 as libc::c_int {
        let mut dir_mode: mode_t = st.st_mode;
        if !(dir_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint)
        {
            *__errno_location() = 20 as libc::c_int;
            result = -(1 as libc::c_int);
        } else {
            let mut indeterminate: mode_t = 0 as libc::c_int as mode_t;
            if owner != -(1 as libc::c_int) as uid_t && owner != st.st_uid
                || group != -(1 as libc::c_int) as gid_t && group != st.st_gid
            {
                result = if 0 as libc::c_int <= fd {
                    fchown(fd, owner, group)
                } else if mkdir_mode != -(1 as libc::c_int) as mode_t {
                    lchown(dir, owner, group)
                } else {
                    chown(dir, owner, group)
                };
                if result == 0 as libc::c_int
                    && dir_mode
                        & (0o100 as libc::c_int
                            | 0o100 as libc::c_int >> 3 as libc::c_int
                            | 0o100 as libc::c_int >> 3 as libc::c_int
                                >> 3 as libc::c_int) as libc::c_uint != 0
                {
                    indeterminate = dir_mode
                        & (0o4000 as libc::c_int | 0o2000 as libc::c_int)
                            as libc::c_uint;
                }
            }
            if result == 0 as libc::c_int
                && (dir_mode ^ mode | indeterminate) & mode_bits != 0
            {
                let mut chmod_mode: mode_t = mode
                    | dir_mode
                        & (0o4000 as libc::c_int | 0o2000 as libc::c_int
                            | 0o1000 as libc::c_int
                            | (0o400 as libc::c_int | 0o200 as libc::c_int
                                | 0o100 as libc::c_int)
                            | (0o400 as libc::c_int | 0o200 as libc::c_int
                                | 0o100 as libc::c_int) >> 3 as libc::c_int
                            | (0o400 as libc::c_int | 0o200 as libc::c_int
                                | 0o100 as libc::c_int) >> 3 as libc::c_int
                                >> 3 as libc::c_int) as libc::c_uint & !mode_bits;
                result = if 1 as libc::c_int != 0 && 0 as libc::c_int <= fd {
                    fchmod(fd, chmod_mode)
                } else if mkdir_mode != -(1 as libc::c_int) as mode_t {
                    lchmod(dir, chmod_mode)
                } else {
                    chmod(dir, chmod_mode)
                };
            }
        }
    }
    if 0 as libc::c_int <= fd {
        if result == 0 as libc::c_int {
            result = close(fd);
        } else {
            let mut e: libc::c_int = *__errno_location();
            close(fd);
            *__errno_location() = e;
        }
    }
    return result;
}
