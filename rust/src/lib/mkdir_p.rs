use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn dirchownmod(
        _: libc::c_int,
        _: *const libc::c_char,
        _: mode_t,
        _: uid_t,
        _: gid_t,
        _: mode_t,
        _: mode_t,
    ) -> libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn mkancesdirs(
        _: *mut libc::c_char,
        _: *mut savewd,
        _: Option::<
            unsafe extern "C" fn(
                *const libc::c_char,
                *const libc::c_char,
                *mut libc::c_void,
            ) -> libc::c_int,
        >,
        _: *mut libc::c_void,
    ) -> ptrdiff_t;
    fn savewd_chdir(
        wd: *mut savewd,
        dir: *const libc::c_char,
        options: libc::c_int,
        open_result: *mut libc::c_int,
    ) -> libc::c_int;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct savewd {
    pub state: C2RustUnnamed_0,
    pub val: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub fd: libc::c_int,
    pub errnum: libc::c_int,
    pub child: pid_t,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const FINAL_STATE: C2RustUnnamed_0 = 5;
pub const ERROR_STATE: C2RustUnnamed_0 = 4;
pub const FORKING_STATE: C2RustUnnamed_0 = 3;
pub const FD_POST_CHDIR_STATE: C2RustUnnamed_0 = 2;
pub const FD_STATE: C2RustUnnamed_0 = 1;
pub const INITIAL_STATE: C2RustUnnamed_0 = 0;
pub type ptrdiff_t = libc::c_long;
pub const SAVEWD_CHDIR_SKIP_READABLE: C2RustUnnamed_1 = 2;
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
pub const SAVEWD_CHDIR_NOFOLLOW: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn make_dir_parents(
    mut dir: *mut libc::c_char,
    mut wd: *mut savewd,
    mut make_ancestor: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    mut options: *mut libc::c_void,
    mut mode: mode_t,
    mut announce: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> (),
    >,
    mut mode_bits: mode_t,
    mut owner: uid_t,
    mut group: gid_t,
    mut preserve_existing: bool,
) -> bool {
    let mut mkdir_errno: libc::c_int = if *dir.offset(0 as libc::c_int as isize)
        as libc::c_int == '/' as i32
    {
        0 as libc::c_int
    } else {
        savewd_errno(wd)
    };
    if mkdir_errno == 0 as libc::c_int {
        let mut prefix_len: ptrdiff_t = 0 as libc::c_int as ptrdiff_t;
        let mut savewd_chdir_options: libc::c_int = if 1 as libc::c_int != 0 {
            SAVEWD_CHDIR_SKIP_READABLE as libc::c_int
        } else {
            0 as libc::c_int
        };
        if make_ancestor.is_some() {
            prefix_len = mkancesdirs(dir, wd, make_ancestor, options);
            if prefix_len < 0 as libc::c_int as libc::c_long {
                if prefix_len < -(1 as libc::c_int) as libc::c_long {
                    return 1 as libc::c_int != 0;
                }
                mkdir_errno = *__errno_location();
            }
        }
        if 0 as libc::c_int as libc::c_long <= prefix_len {
            let mut keep_owner: bool = owner == -(1 as libc::c_int) as uid_t
                && group == -(1 as libc::c_int) as gid_t;
            let mut keep_special_mode_bits: bool = mode_bits
                & (0o4000 as libc::c_int | 0o2000 as libc::c_int) as libc::c_uint
                | mode & 0o1000 as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint;
            let mut mkdir_mode: mode_t = mode;
            if !keep_owner {
                mkdir_mode
                    &= !((0o400 as libc::c_int | 0o200 as libc::c_int
                        | 0o100 as libc::c_int) >> 3 as libc::c_int
                        | (0o400 as libc::c_int | 0o200 as libc::c_int
                            | 0o100 as libc::c_int) >> 3 as libc::c_int
                            >> 3 as libc::c_int) as libc::c_uint;
            } else if !keep_special_mode_bits {
                mkdir_mode
                    &= !(0o200 as libc::c_int >> 3 as libc::c_int
                        | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                        as libc::c_uint;
            }
            if mkdir(dir.offset(prefix_len as isize), mkdir_mode) == 0 as libc::c_int {
                let mut umask_must_be_ok: bool = mode & mode_bits
                    & (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
                        | (0o400 as libc::c_int | 0o200 as libc::c_int
                            | 0o100 as libc::c_int) >> 3 as libc::c_int
                        | (0o400 as libc::c_int | 0o200 as libc::c_int
                            | 0o100 as libc::c_int) >> 3 as libc::c_int
                            >> 3 as libc::c_int) as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint;
                announce.expect("non-null function pointer")(dir, options);
                preserve_existing = keep_owner as libc::c_int
                    & keep_special_mode_bits as libc::c_int
                    & umask_must_be_ok as libc::c_int != 0;
                savewd_chdir_options |= SAVEWD_CHDIR_NOFOLLOW as libc::c_int;
            } else {
                mkdir_errno = *__errno_location();
                mkdir_mode = -(1 as libc::c_int) as mode_t;
            }
            if preserve_existing {
                if mkdir_errno == 0 as libc::c_int {
                    return 1 as libc::c_int != 0;
                }
                if mkdir_errno != 2 as libc::c_int && make_ancestor.is_some() {
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
                    if stat(dir.offset(prefix_len as isize), &mut st) == 0 as libc::c_int
                    {
                        if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                            == 0o40000 as libc::c_int as libc::c_uint
                        {
                            return 1 as libc::c_int != 0;
                        }
                    } else if mkdir_errno == 17 as libc::c_int
                        && *__errno_location() != 2 as libc::c_int
                        && *__errno_location() != 20 as libc::c_int
                    {
                        error(
                            0 as libc::c_int,
                            *__errno_location(),
                            gettext(
                                b"cannot stat %s\0" as *const u8 as *const libc::c_char,
                            ),
                            quote(dir),
                        );
                        return 0 as libc::c_int != 0;
                    }
                }
            } else {
                let mut open_result: [libc::c_int; 2] = [0; 2];
                let mut chdir_result: libc::c_int = savewd_chdir(
                    wd,
                    dir.offset(prefix_len as isize),
                    savewd_chdir_options,
                    open_result.as_mut_ptr(),
                );
                if chdir_result < -(1 as libc::c_int) {
                    return 1 as libc::c_int != 0
                } else {
                    let mut chdir_ok: bool = chdir_result == 0 as libc::c_int;
                    let mut subdir: *const libc::c_char = if chdir_ok as libc::c_int != 0
                    {
                        b".\0" as *const u8 as *const libc::c_char
                    } else {
                        dir.offset(prefix_len as isize) as *const libc::c_char
                    };
                    if dirchownmod(
                        open_result[0 as libc::c_int as usize],
                        subdir,
                        mkdir_mode,
                        owner,
                        group,
                        mode,
                        mode_bits,
                    ) == 0 as libc::c_int
                    {
                        return 1 as libc::c_int != 0;
                    }
                    if mkdir_errno == 0 as libc::c_int
                        || mkdir_errno != 2 as libc::c_int && make_ancestor.is_some()
                            && *__errno_location() != 20 as libc::c_int
                    {
                        error(
                            0 as libc::c_int,
                            *__errno_location(),
                            gettext(
                                if keep_owner as libc::c_int != 0 {
                                    b"cannot change permissions of %s\0" as *const u8
                                        as *const libc::c_char
                                } else {
                                    b"cannot change owner and permissions of %s\0" as *const u8
                                        as *const libc::c_char
                                },
                            ),
                            quote(dir),
                        );
                        return 0 as libc::c_int != 0;
                    }
                }
            }
        }
    }
    error(
        0 as libc::c_int,
        mkdir_errno,
        gettext(b"cannot create directory %s\0" as *const u8 as *const libc::c_char),
        quote(dir),
    );
    return 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn savewd_errno(mut wd: *const savewd) -> libc::c_int {
    return if (*wd).state as libc::c_uint == ERROR_STATE as libc::c_int as libc::c_uint {
        (*wd).val.errnum
    } else {
        0 as libc::c_int
    };
}
