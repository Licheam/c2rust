use ::libc;
extern "C" {
    fn endpwent();
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn endgrent();
    fn getgrgid(__gid: __gid_t) -> *mut group;
    fn getgrnam(__name: *const libc::c_char) -> *mut group;
    fn free(_: *mut libc::c_void);
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn ximemdup(p: *const libc::c_void, s: idx_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn xstrtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut libc::c_ulong,
        _: *const libc::c_char,
    ) -> strtol_error;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
}
pub type __uintmax_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
pub const LONGINT_OK: strtol_error = 0;
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub type uintmax_t = __uintmax_t;
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
pub type idx_t = ptrdiff_t;
pub type ptrdiff_t = libc::c_long;
#[no_mangle]
pub unsafe extern "C" fn parse_user_spec(
    mut spec: *const libc::c_char,
    mut uid: *mut uid_t,
    mut gid: *mut gid_t,
    mut username: *mut *mut libc::c_char,
    mut groupname: *mut *mut libc::c_char,
) -> *const libc::c_char {
    let mut colon: *const libc::c_char = if !gid.is_null() {
        strchr(spec, ':' as i32)
    } else {
        0 as *mut libc::c_char
    };
    let mut error_msg: *const libc::c_char = parse_with_separator(
        spec,
        colon,
        uid,
        gid,
        username,
        groupname,
    );
    if !gid.is_null() && colon.is_null() && !error_msg.is_null() {
        let mut dot: *const libc::c_char = strchr(spec, '.' as i32);
        if !dot.is_null()
            && (parse_with_separator(spec, dot, uid, gid, username, groupname)).is_null()
        {
            error_msg = 0 as *const libc::c_char;
        }
    }
    return error_msg;
}
unsafe extern "C" fn parse_with_separator(
    mut spec: *const libc::c_char,
    mut separator: *const libc::c_char,
    mut uid: *mut uid_t,
    mut gid: *mut gid_t,
    mut username: *mut *mut libc::c_char,
    mut groupname: *mut *mut libc::c_char,
) -> *const libc::c_char {
    static mut E_invalid_user: *const libc::c_char = b"invalid user\0" as *const u8
        as *const libc::c_char;
    static mut E_invalid_group: *const libc::c_char = b"invalid group\0" as *const u8
        as *const libc::c_char;
    static mut E_bad_spec: *const libc::c_char = b"invalid spec\0" as *const u8
        as *const libc::c_char;
    let mut error_msg: *const libc::c_char = 0 as *const libc::c_char;
    let mut pwd: *mut passwd = 0 as *mut passwd;
    let mut grp: *mut group = 0 as *mut group;
    let mut u: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut g: *const libc::c_char = 0 as *const libc::c_char;
    let mut gname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut unum: uid_t = *uid;
    let mut gnum: gid_t = if !gid.is_null() {
        *gid
    } else {
        -(1 as libc::c_int) as libc::c_uint
    };
    error_msg = 0 as *const libc::c_char;
    if !username.is_null() {
        *username = 0 as *mut libc::c_char;
    }
    if !groupname.is_null() {
        *groupname = 0 as *mut libc::c_char;
    }
    u = 0 as *mut libc::c_char;
    if separator.is_null() {
        if *spec != 0 {
            u = xstrdup(spec);
        }
    } else {
        let mut ulen: idx_t = separator.offset_from(spec) as libc::c_long;
        if ulen != 0 as libc::c_int as libc::c_long {
            u = ximemdup(
                spec as *const libc::c_void,
                ulen + 1 as libc::c_int as libc::c_long,
            ) as *mut libc::c_char;
            *u.offset(ulen as isize) = '\0' as i32 as libc::c_char;
        }
    }
    g = if separator.is_null()
        || *separator.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        0 as *const libc::c_char
    } else {
        separator.offset(1 as libc::c_int as isize)
    };
    if !u.is_null() {
        pwd = if *u as libc::c_int == '+' as i32 {
            0 as *mut passwd
        } else {
            getpwnam(u)
        };
        if pwd.is_null() {
            let mut use_login_group: bool = !separator.is_null() && g.is_null();
            if use_login_group {
                error_msg = E_bad_spec;
            } else {
                let mut tmp: libc::c_ulong = 0;
                if xstrtoul(
                    u,
                    0 as *mut *mut libc::c_char,
                    10 as libc::c_int,
                    &mut tmp,
                    b"\0" as *const u8 as *const libc::c_char,
                ) as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint
                    && tmp
                        <= (if (0 as libc::c_int as uid_t) < -(1 as libc::c_int) as uid_t
                        {
                            -(1 as libc::c_int) as uid_t
                        } else {
                            ((1 as libc::c_int as uid_t)
                                << (::core::mem::size_of::<uid_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                                .wrapping_add(1 as libc::c_int as libc::c_uint)
                        }) as libc::c_ulong
                    && tmp as uid_t != -(1 as libc::c_int) as uid_t
                {
                    unum = tmp as uid_t;
                } else {
                    error_msg = E_invalid_user;
                }
            }
        } else {
            unum = (*pwd).pw_uid;
            if g.is_null() && !separator.is_null() {
                let mut buf: [libc::c_char; 21] = [0; 21];
                gnum = (*pwd).pw_gid;
                grp = getgrgid(gnum);
                gname = xstrdup(
                    if !grp.is_null() {
                        (*grp).gr_name
                    } else {
                        umaxtostr(gnum as uintmax_t, buf.as_mut_ptr())
                    },
                );
                endgrent();
            }
        }
        endpwent();
    }
    if !g.is_null() && error_msg.is_null() {
        grp = if *g as libc::c_int == '+' as i32 {
            0 as *mut group
        } else {
            getgrnam(g)
        };
        if grp.is_null() {
            let mut tmp_0: libc::c_ulong = 0;
            if xstrtoul(
                g,
                0 as *mut *mut libc::c_char,
                10 as libc::c_int,
                &mut tmp_0,
                b"\0" as *const u8 as *const libc::c_char,
            ) as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint
                && tmp_0
                    <= (if (0 as libc::c_int as gid_t) < -(1 as libc::c_int) as gid_t {
                        -(1 as libc::c_int) as gid_t
                    } else {
                        ((1 as libc::c_int as gid_t)
                            << (::core::mem::size_of::<gid_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_int as libc::c_uint)
                            .wrapping_add(1 as libc::c_int as libc::c_uint)
                    }) as libc::c_ulong && tmp_0 as gid_t != -(1 as libc::c_int) as gid_t
            {
                gnum = tmp_0 as gid_t;
            } else {
                error_msg = E_invalid_group;
            }
        } else {
            gnum = (*grp).gr_gid;
        }
        endgrent();
        gname = xstrdup(g);
    }
    if error_msg.is_null() {
        *uid = unum;
        if !gid.is_null() {
            *gid = gnum;
        }
        if !username.is_null() {
            *username = u;
            u = 0 as *mut libc::c_char;
        }
        if !groupname.is_null() {
            *groupname = gname;
            gname = 0 as *mut libc::c_char;
        }
    }
    free(u as *mut libc::c_void);
    free(gname as *mut libc::c_void);
    return if !error_msg.is_null() {
        gettext(error_msg)
    } else {
        0 as *mut libc::c_char
    };
}
