use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getgrgid(__gid: __gid_t) -> *mut group;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xgetgroups(
        username: *const libc::c_char,
        gid: gid_t,
        groups: *mut *mut gid_t,
    ) -> libc::c_int;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uintmax_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
pub type uintmax_t = __uintmax_t;
#[no_mangle]
pub unsafe extern "C" fn print_group(mut gid: gid_t, mut use_name: bool) -> bool {
    let mut grp: *mut group = 0 as *mut group;
    let mut ok: bool = 1 as libc::c_int != 0;
    if use_name {
        grp = getgrgid(gid);
        if grp.is_null() {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"cannot find name for group ID %lu\0" as *const u8
                        as *const libc::c_char,
                ),
                gid as libc::c_ulong,
            );
            ok = 0 as libc::c_int != 0;
        }
    }
    let mut s: *mut libc::c_char = if !grp.is_null() {
        (*grp).gr_name
    } else {
        gidtostr_ptr(&mut gid)
    };
    fputs_unlocked(s, stdout);
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn print_group_list(
    mut username: *const libc::c_char,
    mut ruid: uid_t,
    mut rgid: gid_t,
    mut egid: gid_t,
    mut use_names: bool,
    mut delim: libc::c_char,
) -> bool {
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut pwd: *mut passwd = 0 as *mut passwd;
    if !username.is_null() {
        pwd = getpwuid(ruid);
        if pwd.is_null() {
            ok = 0 as libc::c_int != 0;
        }
    }
    if !print_group(rgid, use_names) {
        ok = 0 as libc::c_int != 0;
    }
    if egid != rgid {
        putchar_unlocked(delim as libc::c_int);
        if !print_group(egid, use_names) {
            ok = 0 as libc::c_int != 0;
        }
    }
    let mut groups: *mut gid_t = 0 as *mut gid_t;
    let mut n_groups: libc::c_int = xgetgroups(
        username,
        if !pwd.is_null() { (*pwd).pw_gid } else { egid },
        &mut groups,
    );
    if n_groups < 0 as libc::c_int {
        if !username.is_null() {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(
                    b"failed to get groups for user %s\0" as *const u8
                        as *const libc::c_char,
                ),
                quote(username),
            );
        } else {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(
                    b"failed to get groups for the current process\0" as *const u8
                        as *const libc::c_char,
                ),
            );
        }
        return 0 as libc::c_int != 0;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n_groups {
        if *groups.offset(i as isize) != rgid && *groups.offset(i as isize) != egid {
            putchar_unlocked(delim as libc::c_int);
            if !print_group(*groups.offset(i as isize), use_names) {
                ok = 0 as libc::c_int != 0;
            }
        }
        i += 1;
        i;
    }
    free(groups as *mut libc::c_void);
    return ok;
}
unsafe extern "C" fn gidtostr_ptr(mut gid: *const gid_t) -> *mut libc::c_char {
    static mut buf: [libc::c_char; 21] = [0; 21];
    return umaxtostr(*gid as uintmax_t, buf.as_mut_ptr());
}
