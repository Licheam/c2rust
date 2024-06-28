use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    pub type cycle_check_state;
    pub type hash_table;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn fstatat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn faccessat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __type: libc::c_int,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn unlinkat(
        __fd: libc::c_int,
        __name: *const libc::c_char,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn quotearg_n_style(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn openat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __oflag: libc::c_int,
        _: ...
    ) -> libc::c_int;
    fn fdopendir(__fd: libc::c_int) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    static mut program_name: *const libc::c_char;
    fn abort() -> !;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn file_type(_: *const stat) -> *const libc::c_char;
    fn file_name_concat(
        dir: *const libc::c_char,
        base: *const libc::c_char,
        base_in_result: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn can_write_any_file() -> bool;
    fn rpl_fts_set(_: *mut FTS, _: *mut FTSENT, _: libc::c_int) -> libc::c_int;
    fn rpl_fts_read(_: *mut FTS) -> *mut FTSENT;
    fn xfts_open(
        _: *const *mut libc::c_char,
        options: libc::c_int,
        _: Option::<
            unsafe extern "C" fn(*mut *const FTSENT, *mut *const FTSENT) -> libc::c_int,
        >,
    ) -> *mut FTS;
    fn rpl_fts_close(_: *mut FTS) -> libc::c_int;
    fn yesno() -> bool;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ptrdiff_t = libc::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type C2RustUnnamed = libc::c_uint;
pub const DT_WHT: C2RustUnnamed = 14;
pub const DT_SOCK: C2RustUnnamed = 12;
pub const DT_LNK: C2RustUnnamed = 10;
pub const DT_REG: C2RustUnnamed = 8;
pub const DT_BLK: C2RustUnnamed = 6;
pub const DT_DIR: C2RustUnnamed = 4;
pub const DT_CHR: C2RustUnnamed = 2;
pub const DT_FIFO: C2RustUnnamed = 1;
pub const DT_UNKNOWN: C2RustUnnamed = 0;
pub type DIR = __dirstream;
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
pub struct dev_ino {
    pub st_ino: ino_t,
    pub st_dev: dev_t,
}
pub type rm_interactive = libc::c_uint;
pub const RMI_NEVER: rm_interactive = 5;
pub const RMI_SOMETIMES: rm_interactive = 4;
pub const RMI_ALWAYS: rm_interactive = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rm_options {
    pub ignore_missing_files: bool,
    pub interactive: rm_interactive,
    pub one_file_system: bool,
    pub recursive: bool,
    pub remove_empty_directories: bool,
    pub root_dev_ino: *mut dev_ino,
    pub preserve_all_root: bool,
    pub stdin_tty: bool,
    pub verbose: bool,
    pub require_restore_cwd: bool,
}
pub type RM_status = libc::c_uint;
pub const RM_NONEMPTY_DIR: RM_status = 5;
pub const RM_ERROR: RM_status = 4;
pub const RM_USER_DECLINED: RM_status = 3;
pub const RM_OK: RM_status = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FTS {
    pub fts_cur: *mut _ftsent,
    pub fts_child: *mut _ftsent,
    pub fts_array: *mut *mut _ftsent,
    pub fts_dev: dev_t,
    pub fts_path: *mut libc::c_char,
    pub fts_rfd: libc::c_int,
    pub fts_cwd_fd: libc::c_int,
    pub fts_pathlen: size_t,
    pub fts_nitems: size_t,
    pub fts_compar: Option::<
        unsafe extern "C" fn(*mut *const _ftsent, *mut *const _ftsent) -> libc::c_int,
    >,
    pub fts_options: libc::c_int,
    pub fts_leaf_optimization_works_ht: *mut hash_table,
    pub fts_cycle: C2RustUnnamed_0,
    pub fts_fd_ring: I_ring,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct I_ring {
    pub ir_data: [libc::c_int; 4],
    pub ir_default_val: libc::c_int,
    pub ir_front: libc::c_uint,
    pub ir_back: libc::c_uint,
    pub ir_empty: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ht: *mut hash_table,
    pub state: *mut cycle_check_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ftsent {
    pub fts_cycle: *mut _ftsent,
    pub fts_parent: *mut _ftsent,
    pub fts_link: *mut _ftsent,
    pub fts_dirp: *mut DIR,
    pub fts_number: libc::c_long,
    pub fts_pointer: *mut libc::c_void,
    pub fts_accpath: *mut libc::c_char,
    pub fts_path: *mut libc::c_char,
    pub fts_errno: libc::c_int,
    pub fts_symfd: libc::c_int,
    pub fts_pathlen: size_t,
    pub fts_fts: *mut FTS,
    pub fts_level: ptrdiff_t,
    pub fts_namelen: size_t,
    pub fts_info: libc::c_ushort,
    pub fts_flags: libc::c_ushort,
    pub fts_instr: libc::c_ushort,
    pub fts_statp: [stat; 1],
    pub fts_name: [libc::c_char; 0],
}
pub type FTSENT = _ftsent;
pub type Ternary = libc::c_uint;
pub const T_YES: Ternary = 4;
pub const T_NO: Ternary = 3;
pub const T_UNKNOWN: Ternary = 2;
pub type Prompt_action = libc::c_uint;
pub const PA_REMOVE_DIR: Prompt_action = 3;
pub const PA_DESCEND_INTO_DIR: Prompt_action = 2;
#[inline]
unsafe extern "C" fn lstatat(
    mut fd: libc::c_int,
    mut name: *const libc::c_char,
    mut st: *mut stat,
) -> libc::c_int {
    return fstatat(fd, name, st, 0x100 as libc::c_int);
}
#[inline]
unsafe extern "C" fn dot_or_dotdot(mut file_name: *const libc::c_char) -> bool {
    if *file_name.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
        let mut sep: libc::c_char = *file_name
            .offset(
                ((*file_name.offset(1 as libc::c_int as isize) as libc::c_int
                    == '.' as i32) as libc::c_int + 1 as libc::c_int) as isize,
            );
        return sep == 0 || sep as libc::c_int == '/' as i32;
    } else {
        return 0 as libc::c_int != 0
    };
}
#[inline]
unsafe extern "C" fn readdir_ignoring_dot_and_dotdot(
    mut dirp: *mut DIR,
) -> *const dirent {
    loop {
        let mut dp: *const dirent = readdir(dirp);
        if dp.is_null() || !dot_or_dotdot(((*dp).d_name).as_ptr()) {
            return dp;
        }
    };
}
#[inline]
unsafe extern "C" fn is_empty_dir(
    mut fd_cwd: libc::c_int,
    mut dir: *const libc::c_char,
) -> bool {
    let mut dirp: *mut DIR = 0 as *mut DIR;
    let mut dp: *const dirent = 0 as *const dirent;
    let mut saved_errno: libc::c_int = 0;
    let mut fd: libc::c_int = openat(
        fd_cwd,
        dir,
        0 as libc::c_int | 0o200000 as libc::c_int | 0o400 as libc::c_int
            | 0o400000 as libc::c_int | 0o4000 as libc::c_int,
    );
    if fd < 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    dirp = fdopendir(fd);
    if dirp.is_null() {
        close(fd);
        return 0 as libc::c_int != 0;
    }
    *__errno_location() = 0 as libc::c_int;
    dp = readdir_ignoring_dot_and_dotdot(dirp);
    saved_errno = *__errno_location();
    closedir(dirp);
    *__errno_location() = saved_errno;
    if !dp.is_null() {
        return 0 as libc::c_int != 0;
    }
    return if saved_errno == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0;
}
#[no_mangle]
pub unsafe extern "C" fn rm(
    mut file: *const *mut libc::c_char,
    mut x: *const rm_options,
) -> RM_status {
    let mut rm_status: RM_status = RM_OK;
    if !(*file).is_null() {
        let mut bit_flags: libc::c_int = 0x200 as libc::c_int | 0x8 as libc::c_int
            | 0x10 as libc::c_int;
        if (*x).one_file_system {
            bit_flags |= 0x40 as libc::c_int;
        }
        let mut fts: *mut FTS = xfts_open(file, bit_flags, None);
        loop {
            let mut ent: *mut FTSENT = 0 as *mut FTSENT;
            ent = rpl_fts_read(fts);
            if ent.is_null() {
                if *__errno_location() != 0 as libc::c_int {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        gettext(b"fts_read failed\0" as *const u8 as *const libc::c_char),
                    );
                    rm_status = RM_ERROR;
                }
                break;
            } else {
                let mut s: RM_status = rm_fts(fts, ent, x);
                if s as libc::c_uint == RM_OK as libc::c_int as libc::c_uint
                    || s as libc::c_uint
                        == RM_USER_DECLINED as libc::c_int as libc::c_uint
                    || s as libc::c_uint == RM_ERROR as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"VALID_STATUS (s)\0" as *const u8 as *const libc::c_char,
                        b"src/remove.c\0" as *const u8 as *const libc::c_char,
                        610 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 60],
                            &[libc::c_char; 60],
                        >(
                            b"enum RM_status rm(char *const *, const struct rm_options *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_8757: {
                    if s as libc::c_uint == RM_OK as libc::c_int as libc::c_uint
                        || s as libc::c_uint
                            == RM_USER_DECLINED as libc::c_int as libc::c_uint
                        || s as libc::c_uint == RM_ERROR as libc::c_int as libc::c_uint
                    {} else {
                        __assert_fail(
                            b"VALID_STATUS (s)\0" as *const u8 as *const libc::c_char,
                            b"src/remove.c\0" as *const u8 as *const libc::c_char,
                            610 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 60],
                                &[libc::c_char; 60],
                            >(
                                b"enum RM_status rm(char *const *, const struct rm_options *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                if s as libc::c_uint == RM_ERROR as libc::c_int as libc::c_uint
                    || s as libc::c_uint
                        == RM_USER_DECLINED as libc::c_int as libc::c_uint
                        && rm_status as libc::c_uint
                            == RM_OK as libc::c_int as libc::c_uint
                {
                    rm_status = s;
                }
            }
        }
        if rpl_fts_close(fts) != 0 as libc::c_int {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(b"fts_close failed\0" as *const u8 as *const libc::c_char),
            );
            rm_status = RM_ERROR;
        }
    }
    return rm_status;
}
unsafe extern "C" fn cache_fstatat(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut st: *mut stat,
    mut flag: libc::c_int,
) -> libc::c_int {
    if (*st).st_size == -(1 as libc::c_int) as libc::c_long
        && fstatat(fd, file, st, flag) != 0 as libc::c_int
    {
        (*st).st_size = -(2 as libc::c_int) as __off_t;
        (*st).st_ino = *__errno_location() as __ino_t;
    }
    if 0 as libc::c_int as libc::c_long <= (*st).st_size {
        return 0 as libc::c_int;
    }
    *__errno_location() = (*st).st_ino as libc::c_int;
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn cache_stat_init(mut st: *mut stat) -> *mut stat {
    (*st).st_size = -(1 as libc::c_int) as __off_t;
    return st;
}
unsafe extern "C" fn write_protected_non_symlink(
    mut fd_cwd: libc::c_int,
    mut file: *const libc::c_char,
    mut buf: *mut stat,
) -> libc::c_int {
    if can_write_any_file() {
        return 0 as libc::c_int;
    }
    if cache_fstatat(fd_cwd, file, buf, 0x100 as libc::c_int) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*buf).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o120000 as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if faccessat(fd_cwd, file, 2 as libc::c_int, 0x200 as libc::c_int)
        == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return if *__errno_location() == 13 as libc::c_int {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
unsafe extern "C" fn prompt(
    mut fts: *const FTS,
    mut ent: *const FTSENT,
    mut is_dir: bool,
    mut x: *const rm_options,
    mut mode: Prompt_action,
    mut is_empty_p: *mut Ternary,
) -> RM_status {
    let mut fd_cwd: libc::c_int = (*fts).fts_cwd_fd;
    let mut full_name: *const libc::c_char = (*ent).fts_path;
    let mut filename: *const libc::c_char = (*ent).fts_accpath;
    if !is_empty_p.is_null() {
        *is_empty_p = T_UNKNOWN;
    }
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
    let mut sbuf: *mut stat = &mut st;
    cache_stat_init(sbuf);
    let mut dirent_type: libc::c_int = if is_dir as libc::c_int != 0 {
        DT_DIR as libc::c_int
    } else {
        DT_UNKNOWN as libc::c_int
    };
    let mut write_protected: libc::c_int = 0 as libc::c_int;
    let mut is_empty: bool = 0 as libc::c_int != 0;
    if !is_empty_p.is_null() {
        is_empty = is_empty_dir(fd_cwd, filename);
        *is_empty_p = (if is_empty as libc::c_int != 0 {
            T_YES as libc::c_int
        } else {
            T_NO as libc::c_int
        }) as Ternary;
    }
    if (*ent).fts_number != 0 {
        return RM_USER_DECLINED;
    }
    if (*x).interactive as libc::c_uint == RMI_NEVER as libc::c_int as libc::c_uint {
        return RM_OK;
    }
    let mut wp_errno: libc::c_int = 0 as libc::c_int;
    if !(*x).ignore_missing_files
        && ((*x).interactive as libc::c_uint == RMI_ALWAYS as libc::c_int as libc::c_uint
            || (*x).stdin_tty as libc::c_int != 0)
        && dirent_type != DT_LNK as libc::c_int
    {
        write_protected = write_protected_non_symlink(fd_cwd, filename, sbuf);
        wp_errno = *__errno_location();
    }
    if write_protected != 0
        || (*x).interactive as libc::c_uint == RMI_ALWAYS as libc::c_int as libc::c_uint
    {
        if 0 as libc::c_int <= write_protected
            && dirent_type == DT_UNKNOWN as libc::c_int
        {
            if cache_fstatat(fd_cwd, filename, sbuf, 0x100 as libc::c_int)
                == 0 as libc::c_int
            {
                if (*sbuf).st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o120000 as libc::c_int as libc::c_uint
                {
                    dirent_type = DT_LNK as libc::c_int;
                } else if (*sbuf).st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o40000 as libc::c_int as libc::c_uint
                {
                    dirent_type = DT_DIR as libc::c_int;
                }
            } else {
                write_protected = -(1 as libc::c_int);
                wp_errno = *__errno_location();
            }
        }
        if 0 as libc::c_int <= write_protected {
            match dirent_type {
                10 => {
                    if (*x).interactive as libc::c_uint
                        != RMI_ALWAYS as libc::c_int as libc::c_uint
                    {
                        return RM_OK;
                    }
                }
                4 => {
                    if !((*x).recursive as libc::c_int != 0
                        || (*x).remove_empty_directories as libc::c_int != 0
                            && is_empty as libc::c_int != 0)
                    {
                        write_protected = -(1 as libc::c_int);
                        wp_errno = 21 as libc::c_int;
                    }
                }
                _ => {}
            }
        }
        let mut quoted_name: *const libc::c_char = quotearg_style(
            shell_escape_always_quoting_style,
            full_name,
        );
        if write_protected < 0 as libc::c_int {
            error(
                0 as libc::c_int,
                wp_errno,
                gettext(b"cannot remove %s\0" as *const u8 as *const libc::c_char),
                quoted_name,
            );
            return RM_ERROR;
        }
        if dirent_type == DT_DIR as libc::c_int
            && mode as libc::c_uint == PA_DESCEND_INTO_DIR as libc::c_int as libc::c_uint
            && !is_empty
        {
            fprintf(
                stderr,
                if write_protected != 0 {
                    gettext(
                        b"%s: descend into write-protected directory %s? \0" as *const u8
                            as *const libc::c_char,
                    )
                } else {
                    gettext(
                        b"%s: descend into directory %s? \0" as *const u8
                            as *const libc::c_char,
                    )
                },
                program_name,
                quoted_name,
            );
        } else {
            if cache_fstatat(fd_cwd, filename, sbuf, 0x100 as libc::c_int)
                != 0 as libc::c_int
            {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(b"cannot remove %s\0" as *const u8 as *const libc::c_char),
                    quoted_name,
                );
                return RM_ERROR;
            }
            fprintf(
                stderr,
                if write_protected != 0 {
                    gettext(
                        b"%s: remove write-protected %s %s? \0" as *const u8
                            as *const libc::c_char,
                    )
                } else {
                    gettext(b"%s: remove %s %s? \0" as *const u8 as *const libc::c_char)
                },
                program_name,
                file_type(sbuf),
                quoted_name,
            );
        }
        if !yesno() {
            return RM_USER_DECLINED;
        }
    }
    return RM_OK;
}
#[inline]
unsafe extern "C" fn nonexistent_file_errno(mut errnum: libc::c_int) -> bool {
    match errnum {
        84 | 22 | 2 | 20 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn ignorable_missing(
    mut x: *const rm_options,
    mut errnum: libc::c_int,
) -> bool {
    return (*x).ignore_missing_files as libc::c_int != 0
        && nonexistent_file_errno(errnum) as libc::c_int != 0;
}
unsafe extern "C" fn fts_skip_tree(mut fts: *mut FTS, mut ent: *mut FTSENT) {
    rpl_fts_set(fts, ent, 4 as libc::c_int);
    rpl_fts_read(fts);
}
unsafe extern "C" fn mark_ancestor_dirs(mut ent: *mut FTSENT) {
    let mut p: *mut FTSENT = 0 as *mut FTSENT;
    p = (*ent).fts_parent;
    while 0 as libc::c_int as libc::c_long <= (*p).fts_level {
        if (*p).fts_number != 0 {
            break;
        }
        (*p).fts_number = 1 as libc::c_int as libc::c_long;
        p = (*p).fts_parent;
    }
}
unsafe extern "C" fn excise(
    mut fts: *mut FTS,
    mut ent: *mut FTSENT,
    mut x: *const rm_options,
    mut is_dir: bool,
) -> RM_status {
    let mut flag: libc::c_int = if is_dir as libc::c_int != 0 {
        0x200 as libc::c_int
    } else {
        0 as libc::c_int
    };
    if unlinkat((*fts).fts_cwd_fd, (*ent).fts_accpath, flag) == 0 as libc::c_int {
        if (*x).verbose {
            printf(
                if is_dir as libc::c_int != 0 {
                    gettext(
                        b"removed directory %s\n\0" as *const u8 as *const libc::c_char,
                    )
                } else {
                    gettext(b"removed %s\n\0" as *const u8 as *const libc::c_char)
                },
                quotearg_style(shell_escape_always_quoting_style, (*ent).fts_path),
            );
        }
        return RM_OK;
    }
    if *__errno_location() == 30 as libc::c_int {
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
        if !(lstatat((*fts).fts_cwd_fd, (*ent).fts_accpath, &mut st) != 0
            && *__errno_location() == 2 as libc::c_int)
        {
            *__errno_location() = 30 as libc::c_int;
        }
    }
    if ignorable_missing(x, *__errno_location()) {
        return RM_OK;
    }
    if (*ent).fts_info as libc::c_int == 4 as libc::c_int
        && (*__errno_location() == 39 as libc::c_int
            || *__errno_location() == 21 as libc::c_int
            || *__errno_location() == 20 as libc::c_int
            || *__errno_location() == 17 as libc::c_int)
        && ((*ent).fts_errno == 1 as libc::c_int
            || (*ent).fts_errno == 13 as libc::c_int)
    {
        *__errno_location() = (*ent).fts_errno;
    }
    error(
        0 as libc::c_int,
        *__errno_location(),
        gettext(b"cannot remove %s\0" as *const u8 as *const libc::c_char),
        quotearg_style(shell_escape_always_quoting_style, (*ent).fts_path),
    );
    mark_ancestor_dirs(ent);
    return RM_ERROR;
}
unsafe extern "C" fn rm_fts(
    mut fts: *mut FTS,
    mut ent: *mut FTSENT,
    mut x: *const rm_options,
) -> RM_status {
    match (*ent).fts_info as libc::c_int {
        1 => {
            if !(*x).recursive
                && !((*x).remove_empty_directories as libc::c_int != 0
                    && is_empty_dir((*fts).fts_cwd_fd, (*ent).fts_accpath) as libc::c_int
                        != 0)
            {
                let mut err: libc::c_int = if (*x).remove_empty_directories
                    as libc::c_int != 0
                {
                    39 as libc::c_int
                } else {
                    21 as libc::c_int
                };
                error(
                    0 as libc::c_int,
                    err,
                    gettext(b"cannot remove %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, (*ent).fts_path),
                );
                mark_ancestor_dirs(ent);
                fts_skip_tree(fts, ent);
                return RM_ERROR;
            }
            if (*ent).fts_level == 0 as libc::c_int as libc::c_long {
                if dot_or_dotdot(last_component((*ent).fts_accpath)) {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"refusing to remove %s or %s directory: skipping %s\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quotearg_n_style(
                            0 as libc::c_int,
                            shell_escape_always_quoting_style,
                            b".\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_n_style(
                            1 as libc::c_int,
                            shell_escape_always_quoting_style,
                            b"..\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_n_style(
                            2 as libc::c_int,
                            shell_escape_always_quoting_style,
                            (*ent).fts_path,
                        ),
                    );
                    fts_skip_tree(fts, ent);
                    return RM_ERROR;
                }
                if !((*x).root_dev_ino).is_null()
                    && ((*((*ent).fts_statp).as_mut_ptr()).st_ino
                        == (*(*x).root_dev_ino).st_ino
                        && (*((*ent).fts_statp).as_mut_ptr()).st_dev
                            == (*(*x).root_dev_ino).st_dev)
                {
                    if strcmp(
                        (*ent).fts_path,
                        b"/\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"it is dangerous to operate recursively on %s\0"
                                    as *const u8 as *const libc::c_char,
                            ),
                            quotearg_style(
                                shell_escape_always_quoting_style,
                                (*ent).fts_path,
                            ),
                        );
                    } else {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"it is dangerous to operate recursively on %s (same as %s)\0"
                                    as *const u8 as *const libc::c_char,
                            ),
                            quotearg_n_style(
                                0 as libc::c_int,
                                shell_escape_always_quoting_style,
                                (*ent).fts_path,
                            ),
                            quotearg_n_style(
                                1 as libc::c_int,
                                shell_escape_always_quoting_style,
                                b"/\0" as *const u8 as *const libc::c_char,
                            ),
                        );
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"use --no-preserve-root to override this failsafe\0"
                                as *const u8 as *const libc::c_char,
                        ),
                    );
                    fts_skip_tree(fts, ent);
                    return RM_ERROR;
                }
                if (*x).preserve_all_root {
                    let mut failed: bool = 0 as libc::c_int != 0;
                    let mut parent: *mut libc::c_char = file_name_concat(
                        (*ent).fts_accpath,
                        b"..\0" as *const u8 as *const libc::c_char,
                        0 as *mut *mut libc::c_char,
                    );
                    let mut statbuf: stat = stat {
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
                    if parent.is_null() || lstat(parent, &mut statbuf) != 0 {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"failed to stat %s: skipping %s\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            quotearg_n_style(
                                0 as libc::c_int,
                                shell_escape_always_quoting_style,
                                parent,
                            ),
                            quotearg_n_style(
                                1 as libc::c_int,
                                shell_escape_always_quoting_style,
                                (*ent).fts_accpath,
                            ),
                        );
                        failed = 1 as libc::c_int != 0;
                    }
                    free(parent as *mut libc::c_void);
                    if failed as libc::c_int != 0 || (*fts).fts_dev != statbuf.st_dev {
                        if !failed {
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                gettext(
                                    b"skipping %s, since it's on a different device\0"
                                        as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(
                                    shell_escape_always_quoting_style,
                                    (*ent).fts_path,
                                ),
                            );
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                gettext(
                                    b"and --preserve-root=all is in effect\0" as *const u8
                                        as *const libc::c_char,
                                ),
                            );
                        }
                        fts_skip_tree(fts, ent);
                        return RM_ERROR;
                    }
                }
            }
            let mut is_empty_directory: Ternary = 0 as Ternary;
            let mut s: RM_status = prompt(
                fts,
                ent,
                1 as libc::c_int != 0,
                x,
                PA_DESCEND_INTO_DIR,
                &mut is_empty_directory,
            );
            if s as libc::c_uint == RM_OK as libc::c_int as libc::c_uint
                && is_empty_directory as libc::c_uint
                    == T_YES as libc::c_int as libc::c_uint
            {
                s = excise(fts, ent, x, 1 as libc::c_int != 0);
                if s as libc::c_uint == RM_OK as libc::c_int as libc::c_uint {
                    fts_skip_tree(fts, ent);
                }
            }
            if s as libc::c_uint != RM_OK as libc::c_int as libc::c_uint {
                mark_ancestor_dirs(ent);
                fts_skip_tree(fts, ent);
            }
            return s;
        }
        8 | 10 | 12 | 13 | 6 | 4 | 11 | 3 => {
            if (*ent).fts_info as libc::c_int == 6 as libc::c_int
                && (*x).one_file_system as libc::c_int != 0
                && (0 as libc::c_int as libc::c_long) < (*ent).fts_level
                && (*((*ent).fts_statp).as_mut_ptr()).st_dev != (*fts).fts_dev
            {
                mark_ancestor_dirs(ent);
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"skipping %s, since it's on a different device\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, (*ent).fts_path),
                );
                return RM_ERROR;
            }
            let mut is_dir: bool = (*ent).fts_info as libc::c_int == 6 as libc::c_int
                || (*ent).fts_info as libc::c_int == 4 as libc::c_int;
            let mut s_0: RM_status = prompt(
                fts,
                ent,
                is_dir,
                x,
                PA_REMOVE_DIR,
                0 as *mut Ternary,
            );
            if s_0 as libc::c_uint != RM_OK as libc::c_int as libc::c_uint {
                return s_0;
            }
            return excise(fts, ent, x, is_dir);
        }
        2 => {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"WARNING: Circular directory structure.\nThis almost certainly means that you have a corrupted file system.\nNOTIFY YOUR SYSTEM MANAGER.\nThe following directory is part of the cycle:\n  %s\n\0"
                        as *const u8 as *const libc::c_char,
                ),
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    (*ent).fts_path,
                ),
            );
            fts_skip_tree(fts, ent);
            return RM_ERROR;
        }
        7 => {
            error(
                0 as libc::c_int,
                (*ent).fts_errno,
                gettext(b"traversal failed: %s\0" as *const u8 as *const libc::c_char),
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    (*ent).fts_path,
                ),
            );
            fts_skip_tree(fts, ent);
            return RM_ERROR;
        }
        _ => {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"unexpected failure: fts_info=%d: %s\nplease report to %s\0"
                        as *const u8 as *const libc::c_char,
                ),
                (*ent).fts_info as libc::c_int,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    (*ent).fts_path,
                ),
                b"bug-coreutils@gnu.org\0" as *const u8 as *const libc::c_char,
            );
            abort();
        }
    };
}
