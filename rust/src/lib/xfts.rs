use ::libc;
extern "C" {
    pub type __dirstream;
    pub type cycle_check_state;
    pub type hash_table;
    fn __errno_location() -> *mut libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn xalloc_die();
    fn rpl_fts_open(
        _: *const *mut libc::c_char,
        _: libc::c_int,
        _: Option::<
            unsafe extern "C" fn(*mut *const FTSENT, *mut *const FTSENT) -> libc::c_int,
        >,
    ) -> *mut FTS;
}
pub type size_t = libc::c_ulong;
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
pub type dev_t = __dev_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type ptrdiff_t = libc::c_long;
pub type DIR = __dirstream;
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
pub struct I_ring {
    pub ir_data: [libc::c_int; 4],
    pub ir_default_val: libc::c_int,
    pub ir_front: libc::c_uint,
    pub ir_back: libc::c_uint,
    pub ir_empty: bool,
}
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
    pub fts_cycle: C2RustUnnamed,
    pub fts_fd_ring: I_ring,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
#[no_mangle]
pub unsafe extern "C" fn xfts_open(
    mut argv: *const *mut libc::c_char,
    mut options: libc::c_int,
    mut compar: Option::<
        unsafe extern "C" fn(*mut *const FTSENT, *mut *const FTSENT) -> libc::c_int,
    >,
) -> *mut FTS {
    let mut fts: *mut FTS = rpl_fts_open(argv, options | 0x200 as libc::c_int, compar);
    if fts.is_null() {
        if *__errno_location() != 22 as libc::c_int {} else {
            __assert_fail(
                b"errno != EINVAL\0" as *const u8 as *const libc::c_char,
                b"lib/xfts.c\0" as *const u8 as *const libc::c_char,
                41 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 78],
                    &[libc::c_char; 78],
                >(
                    b"FTS *xfts_open(char *const *, int, int (*)(const FTSENT **, const FTSENT **))\0",
                ))
                    .as_ptr(),
            );
        }
        'c_2194: {
            if *__errno_location() != 22 as libc::c_int {} else {
                __assert_fail(
                    b"errno != EINVAL\0" as *const u8 as *const libc::c_char,
                    b"lib/xfts.c\0" as *const u8 as *const libc::c_char,
                    41 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 78],
                        &[libc::c_char; 78],
                    >(
                        b"FTS *xfts_open(char *const *, int, int (*)(const FTSENT **, const FTSENT **))\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        xalloc_die();
    }
    return fts;
}
#[no_mangle]
pub unsafe extern "C" fn cycle_warning_required(
    mut fts: *const FTS,
    mut ent: *const FTSENT,
) -> bool {
    return (*fts).fts_options & 0x10 as libc::c_int != 0
        && (*fts).fts_options & 0x1 as libc::c_int == 0
        || (*fts).fts_options & 0x10 as libc::c_int != 0
            && (*fts).fts_options & 0x1 as libc::c_int != 0
            && (*ent).fts_level != 0 as libc::c_int as libc::c_long;
}
