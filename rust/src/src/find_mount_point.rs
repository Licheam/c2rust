use ::libc;
extern "C" {
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn dir_name(file: *const libc::c_char) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn save_cwd(cwd: *mut saved_cwd) -> libc::c_int;
    fn restore_cwd(cwd: *const saved_cwd) -> libc::c_int;
    fn free_cwd(cwd: *mut saved_cwd);
    fn xgetcwd() -> *mut libc::c_char;
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
pub type size_t = libc::c_ulong;
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
pub type quoting_style = libc::c_uint;
pub const custom_quoting_style: quoting_style = 10;
pub const clocale_quoting_style: quoting_style = 9;
pub const locale_quoting_style: quoting_style = 8;
pub const escape_quoting_style: quoting_style = 7;
pub const c_maybe_quoting_style: quoting_style = 6;
pub const c_quoting_style: quoting_style = 5;
pub const shell_escape_always_quoting_style: quoting_style = 4;
pub const shell_escape_quoting_style: quoting_style = 3;
pub const shell_always_quoting_style: quoting_style = 2;
pub const shell_quoting_style: quoting_style = 1;
pub const literal_quoting_style: quoting_style = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saved_cwd {
    pub desc: libc::c_int,
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub _gl_dummy: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn find_mount_point(
    mut file: *const libc::c_char,
    mut file_stat: *const stat,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut cwd: saved_cwd = saved_cwd {
        desc: 0,
        name: 0 as *mut libc::c_char,
    };
    let mut last_stat: stat = stat {
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
    let mut mp: *mut libc::c_char = 0 as *mut libc::c_char;
    if save_cwd(&mut cwd) != 0 as libc::c_int {
        error(
            0 as libc::c_int,
            *__errno_location(),
            gettext(
                b"cannot get current directory\0" as *const u8 as *const libc::c_char,
            ),
        );
        return 0 as *mut libc::c_char;
    }
    if (*file_stat).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint
    {
        last_stat = *file_stat;
        if chdir(file) < 0 as libc::c_int {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(
                    b"cannot change to directory %s\0" as *const u8
                        as *const libc::c_char,
                ),
                quotearg_style(shell_escape_always_quoting_style, file),
            );
            return 0 as *mut libc::c_char;
        }
        current_block = 15904375183555213903;
    } else {
        let mut xdir: *mut libc::c_char = dir_name(file);
        let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
        dir = ({
            let mut __old: *const libc::c_char = xdir;
            let mut __len: size_t = (strlen(__old))
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            let mut fresh0 = ::std::vec::from_elem(0, __len as usize);
            let mut __new: *mut libc::c_char = fresh0.as_mut_ptr() as *mut libc::c_char;
            memcpy(__new as *mut libc::c_void, __old as *const libc::c_void, __len)
                as *mut libc::c_char
        });
        free(xdir as *mut libc::c_void);
        if chdir(dir) < 0 as libc::c_int {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(
                    b"cannot change to directory %s\0" as *const u8
                        as *const libc::c_char,
                ),
                quotearg_style(shell_escape_always_quoting_style, dir),
            );
            return 0 as *mut libc::c_char;
        }
        if stat(b".\0" as *const u8 as *const libc::c_char, &mut last_stat)
            < 0 as libc::c_int
        {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(
                    b"cannot stat current directory (now %s)\0" as *const u8
                        as *const libc::c_char,
                ),
                quotearg_style(shell_escape_always_quoting_style, dir),
            );
            current_block = 5094001751303942678;
        } else {
            current_block = 15904375183555213903;
        }
    }
    loop {
        match current_block {
            5094001751303942678 => {
                let mut save_errno: libc::c_int = *__errno_location();
                if restore_cwd(&mut cwd) != 0 as libc::c_int {
                    if ::core::mem::size_of::<C2RustUnnamed>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            gettext(
                                b"failed to return to initial working directory\0"
                                    as *const u8 as *const libc::c_char,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            gettext(
                                b"failed to return to initial working directory\0"
                                    as *const u8 as *const libc::c_char,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                free_cwd(&mut cwd);
                *__errno_location() = save_errno;
                break;
            }
            _ => {
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
                if stat(b"..\0" as *const u8 as *const libc::c_char, &mut st)
                    < 0 as libc::c_int
                {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        gettext(b"cannot stat %s\0" as *const u8 as *const libc::c_char),
                        quotearg_style(
                            shell_escape_always_quoting_style,
                            b"..\0" as *const u8 as *const libc::c_char,
                        ),
                    );
                    current_block = 5094001751303942678;
                } else if st.st_dev != last_stat.st_dev || st.st_ino == last_stat.st_ino
                {
                    mp = xgetcwd();
                    current_block = 5094001751303942678;
                } else if chdir(b"..\0" as *const u8 as *const libc::c_char)
                    < 0 as libc::c_int
                {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        gettext(
                            b"cannot change to directory %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_style(
                            shell_escape_always_quoting_style,
                            b"..\0" as *const u8 as *const libc::c_char,
                        ),
                    );
                    current_block = 5094001751303942678;
                } else {
                    last_stat = st;
                    current_block = 15904375183555213903;
                }
            }
        }
    }
    return mp;
}
