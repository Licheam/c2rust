use ::libc;
extern "C" {
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn getgroups(__size: libc::c_int, __list: *mut __gid_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn getgrouplist(
        __user: *const libc::c_char,
        __group: __gid_t,
        __groups: *mut __gid_t,
        __ngroups: *mut libc::c_int,
    ) -> libc::c_int;
    fn getugroups(
        maxcount: libc::c_int,
        grouplist: *mut gid_t,
        username: *const libc::c_char,
        gid: gid_t,
    ) -> libc::c_int;
}
pub type __gid_t = libc::c_uint;
pub type gid_t = __gid_t;
pub type size_t = libc::c_ulong;
pub const N_GROUPS_INIT: C2RustUnnamed = 10;
pub type C2RustUnnamed = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn mgetgroups(
    mut username: *const libc::c_char,
    mut gid: gid_t,
    mut groups: *mut *mut gid_t,
) -> libc::c_int {
    let mut max_n_groups: libc::c_int = 0;
    let mut ng: libc::c_int = 0;
    let mut g: *mut gid_t = 0 as *mut gid_t;
    if !username.is_null() {
        max_n_groups = N_GROUPS_INIT as libc::c_int;
        g = realloc_groupbuf(0 as *mut gid_t, max_n_groups as size_t);
        if g.is_null() {
            return -(1 as libc::c_int);
        }
        loop {
            let mut h: *mut gid_t = 0 as *mut gid_t;
            let mut last_n_groups: libc::c_int = max_n_groups;
            ng = getgrouplist(username, gid, g, &mut max_n_groups);
            if ng < 0 as libc::c_int && last_n_groups == max_n_groups {
                max_n_groups *= 2 as libc::c_int;
            }
            h = realloc_groupbuf(g, max_n_groups as size_t);
            if h.is_null() {
                free(g as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            g = h;
            if 0 as libc::c_int <= ng {
                *groups = g;
                return max_n_groups;
            }
        }
    }
    max_n_groups = if !username.is_null() {
        getugroups(0 as libc::c_int, 0 as *mut gid_t, username, gid)
    } else {
        getgroups(0 as libc::c_int, 0 as *mut __gid_t)
    };
    if max_n_groups < 0 as libc::c_int {
        if *__errno_location() == 38 as libc::c_int
            && {
                g = realloc_groupbuf(0 as *mut gid_t, 1 as libc::c_int as size_t);
                !g.is_null()
            }
        {
            *groups = g;
            *g = gid;
            return (gid != -(1 as libc::c_int) as gid_t) as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    if max_n_groups == 0 as libc::c_int
        || username.is_null() && gid != -(1 as libc::c_int) as gid_t
    {
        max_n_groups += 1;
        max_n_groups;
    }
    g = realloc_groupbuf(0 as *mut gid_t, max_n_groups as size_t);
    if g.is_null() {
        return -(1 as libc::c_int);
    }
    ng = if !username.is_null() {
        getugroups(max_n_groups, g, username, gid)
    } else {
        getgroups(
            max_n_groups - (gid != -(1 as libc::c_int) as gid_t) as libc::c_int,
            g.offset((gid != -(1 as libc::c_int) as gid_t) as libc::c_int as isize),
        )
    };
    if ng < 0 as libc::c_int {
        free(g as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    if username.is_null() && gid != -(1 as libc::c_int) as gid_t {
        *g = gid;
        ng += 1;
        ng;
    }
    *groups = g;
    if (1 as libc::c_int) < ng {
        let mut first: gid_t = *g;
        let mut next: *mut gid_t = 0 as *mut gid_t;
        let mut groups_end: *mut gid_t = g.offset(ng as isize);
        next = g.offset(1 as libc::c_int as isize);
        while next < groups_end {
            if *next == first || *next == *g {
                ng -= 1;
                ng;
            } else {
                g = g.offset(1);
                *g = *next;
            }
            next = next.offset(1);
            next;
        }
    }
    return ng;
}
unsafe extern "C" fn realloc_groupbuf(mut g: *mut gid_t, mut num: size_t) -> *mut gid_t {
    if ::core::mem::size_of::<gid_t>() as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
        && (if (9223372036854775807 as libc::c_long as libc::c_ulong)
            < 18446744073709551615 as libc::c_ulong
        {
            9223372036854775807 as libc::c_long as libc::c_ulong
        } else {
            (18446744073709551615 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        })
            .wrapping_div(::core::mem::size_of::<gid_t>() as libc::c_ulong) < num
    {
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut gid_t;
    }
    return realloc(
        g as *mut libc::c_void,
        num.wrapping_mul(::core::mem::size_of::<gid_t>() as libc::c_ulong),
    ) as *mut gid_t;
}
