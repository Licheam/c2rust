use ::libc;
extern "C" {
    pub type selabel_handle;
    pub type __dirstream;
    pub type cycle_check_state;
    pub type hash_table;
    fn freecon(con: *mut libc::c_char);
    fn getcon(con: *mut *mut libc::c_char) -> libc::c_int;
    fn getfscreatecon(con: *mut *mut libc::c_char) -> libc::c_int;
    fn setfscreatecon(context: *const libc::c_char) -> libc::c_int;
    fn rpl_lgetfilecon(
        path: *const libc::c_char,
        con: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn rpl_fgetfilecon(fd: libc::c_int, con: *mut *mut libc::c_char) -> libc::c_int;
    fn lsetfilecon(path: *const libc::c_char, con: *const libc::c_char) -> libc::c_int;
    fn fsetfilecon(fd: libc::c_int, con: *const libc::c_char) -> libc::c_int;
    fn security_compute_create(
        scon: *const libc::c_char,
        tcon: *const libc::c_char,
        tclass: security_class_t,
        newcon: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn mode_to_security_class(mode: mode_t) -> security_class_t;
    fn selabel_lookup(
        handle: *mut selabel_handle,
        con: *mut *mut libc::c_char,
        key: *const libc::c_char,
        type_0: libc::c_int,
    ) -> libc::c_int;
    fn rpl_getfilecon(
        path: *const libc::c_char,
        con: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn context_new(_: *const libc::c_char) -> context_t;
    fn context_str(_: context_t) -> *mut libc::c_char;
    fn context_free(_: context_t);
    fn context_type_get(_: context_t) -> *const libc::c_char;
    fn context_type_set(_: context_t, _: *const libc::c_char) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn dir_name(file: *const libc::c_char) -> *mut libc::c_char;
    fn canonicalize_filename_mode(
        _: *const libc::c_char,
        _: canonicalize_mode_t,
    ) -> *mut libc::c_char;
    fn rpl_fts_close(_: *mut FTS) -> libc::c_int;
    fn rpl_fts_read(_: *mut FTS) -> *mut FTSENT;
    fn xfts_open(
        _: *const *mut libc::c_char,
        options: libc::c_int,
        _: Option::<
            unsafe extern "C" fn(*mut *const FTSENT, *mut *const FTSENT) -> libc::c_int,
        >,
    ) -> *mut FTS;
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
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type security_class_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct context_s_t {
    pub ptr: *mut libc::c_void,
}
pub type context_t = *mut context_s_t;
pub type ptrdiff_t = libc::c_long;
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
pub type DIR = __dirstream;
pub type canonicalize_mode_t = libc::c_uint;
pub const CAN_NOLINKS: canonicalize_mode_t = 4;
pub const CAN_MISSING: canonicalize_mode_t = 2;
pub const CAN_ALL_BUT_LAST: canonicalize_mode_t = 1;
pub const CAN_EXISTING: canonicalize_mode_t = 0;
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
pub unsafe extern "C" fn restorecon(
    mut selabel_handle: *mut selabel_handle,
    mut path: *const libc::c_char,
    mut recurse: bool,
) -> bool {
    let mut newpath: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(*path.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32) {
        newpath = canonicalize_filename_mode(path, CAN_MISSING);
        if newpath.is_null() {
            return 0 as libc::c_int != 0;
        }
        path = newpath;
    }
    if !recurse {
        let mut ok: bool = restorecon_private(selabel_handle, path)
            != -(1 as libc::c_int);
        let mut err: libc::c_int = *__errno_location();
        free(newpath as *mut libc::c_void);
        *__errno_location() = err;
        return ok;
    }
    let mut ftspath: [*const libc::c_char; 2] = [path, 0 as *const libc::c_char];
    let mut fts: *mut FTS = xfts_open(
        ftspath.as_mut_ptr() as *const *mut libc::c_char,
        0x10 as libc::c_int,
        None,
    );
    let mut err_0: libc::c_int = 0 as libc::c_int;
    let mut ent: *mut FTSENT = 0 as *mut FTSENT;
    loop {
        ent = rpl_fts_read(fts);
        if ent.is_null() {
            break;
        }
        if restorecon_private(selabel_handle, (*fts).fts_path) < 0 as libc::c_int {
            err_0 = *__errno_location();
        }
    }
    if *__errno_location() != 0 as libc::c_int {
        err_0 = *__errno_location();
    }
    if rpl_fts_close(fts) != 0 as libc::c_int {
        err_0 = *__errno_location();
    }
    free(newpath as *mut libc::c_void);
    return err_0 == 0;
}
#[no_mangle]
pub unsafe extern "C" fn defaultcon(
    mut selabel_handle: *mut selabel_handle,
    mut path: *const libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = -(1 as libc::c_int);
    let mut scon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tcon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut scontext: context_t = 0 as context_t;
    let mut tcontext: context_t = 0 as context_t;
    let mut contype: *const libc::c_char = 0 as *const libc::c_char;
    let mut constr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newpath: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(*path.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32) {
        newpath = canonicalize_filename_mode(path, CAN_MISSING);
        if newpath.is_null() {
            current_block = 5143058163439228106;
        } else {
            path = newpath;
            current_block = 15619007995458559411;
        }
    } else {
        current_block = 15619007995458559411;
    }
    match current_block {
        15619007995458559411 => {
            if selabel_lookup(selabel_handle, &mut scon, path, mode as libc::c_int)
                < 0 as libc::c_int
            {
                if *__errno_location() == 2 as libc::c_int {
                    *__errno_location() = 61 as libc::c_int;
                }
            } else if !(computecon(path, mode, &mut tcon) < 0 as libc::c_int) {
                scontext = context_new(scon);
                if !scontext.is_null() {
                    tcontext = context_new(tcon);
                    if !tcontext.is_null() {
                        contype = context_type_get(scontext);
                        if !contype.is_null() {
                            if !(context_type_set(tcontext, contype) != 0) {
                                constr = context_str(tcontext);
                                if !constr.is_null() {
                                    rc = setfscreatecon(constr);
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    let mut err: libc::c_int = *__errno_location();
    context_free(scontext);
    context_free(tcontext);
    freecon(scon);
    freecon(tcon);
    free(newpath as *mut libc::c_void);
    *__errno_location() = err;
    return rc;
}
unsafe extern "C" fn computecon(
    mut path: *const libc::c_char,
    mut mode: mode_t,
    mut con: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut scon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tcon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tclass: security_class_t = 0;
    let mut rc: libc::c_int = -(1 as libc::c_int);
    let mut dir: *mut libc::c_char = dir_name(path);
    if !dir.is_null() {
        if !(getcon(&mut scon) < 0 as libc::c_int) {
            if !(rpl_getfilecon(dir, &mut tcon) < 0 as libc::c_int) {
                tclass = mode_to_security_class(mode);
                if !(tclass == 0) {
                    rc = security_compute_create(scon, tcon, tclass, con);
                }
            }
        }
    }
    let mut err: libc::c_int = *__errno_location();
    free(dir as *mut libc::c_void);
    freecon(scon);
    freecon(tcon);
    *__errno_location() = err;
    return rc;
}
unsafe extern "C" fn restorecon_private(
    mut selabel_handle: *mut selabel_handle,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = -(1 as libc::c_int);
    let mut sb: stat = stat {
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
    let mut scon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tcon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut scontext: context_t = 0 as context_t;
    let mut tcontext: context_t = 0 as context_t;
    let mut contype: *const libc::c_char = 0 as *const libc::c_char;
    let mut constr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fd: libc::c_int = 0;
    if selabel_handle.is_null() {
        if getfscreatecon(&mut tcon) < 0 as libc::c_int {
            return rc;
        }
        if tcon.is_null() {
            *__errno_location() = 61 as libc::c_int;
            return rc;
        }
        rc = lsetfilecon(path, tcon);
        let mut err: libc::c_int = *__errno_location();
        freecon(tcon);
        *__errno_location() = err;
        return rc;
    }
    fd = open(path, 0 as libc::c_int | 0o400000 as libc::c_int);
    if !(fd == -(1 as libc::c_int) && *__errno_location() != 40 as libc::c_int) {
        if fd != -(1 as libc::c_int) {
            if fstat(fd, &mut sb) < 0 as libc::c_int {
                current_block = 18377268871191777778;
            } else {
                current_block = 15976848397966268834;
            }
        } else if lstat(path, &mut sb) < 0 as libc::c_int {
            current_block = 18377268871191777778;
        } else {
            current_block = 15976848397966268834;
        }
        match current_block {
            18377268871191777778 => {}
            _ => {
                if selabel_lookup(
                    selabel_handle,
                    &mut scon,
                    path,
                    sb.st_mode as libc::c_int,
                ) < 0 as libc::c_int
                {
                    if *__errno_location() == 2 as libc::c_int {
                        *__errno_location() = 61 as libc::c_int;
                    }
                } else {
                    scontext = context_new(scon);
                    if !scontext.is_null() {
                        if fd != -(1 as libc::c_int) {
                            if rpl_fgetfilecon(fd, &mut tcon) < 0 as libc::c_int {
                                current_block = 18377268871191777778;
                            } else {
                                current_block = 9828876828309294594;
                            }
                        } else if rpl_lgetfilecon(path, &mut tcon) < 0 as libc::c_int {
                            current_block = 18377268871191777778;
                        } else {
                            current_block = 9828876828309294594;
                        }
                        match current_block {
                            18377268871191777778 => {}
                            _ => {
                                tcontext = context_new(tcon);
                                if !tcontext.is_null() {
                                    contype = context_type_get(scontext);
                                    if !contype.is_null() {
                                        if !(context_type_set(tcontext, contype) != 0) {
                                            constr = context_str(tcontext);
                                            if !constr.is_null() {
                                                if fd != -(1 as libc::c_int) {
                                                    rc = fsetfilecon(fd, constr);
                                                } else {
                                                    rc = lsetfilecon(path, constr);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let mut err_0: libc::c_int = *__errno_location();
    if fd != -(1 as libc::c_int) {
        close(fd);
    }
    context_free(scontext);
    context_free(tcontext);
    freecon(scon);
    freecon(tcon);
    *__errno_location() = err_0;
    return rc;
}
