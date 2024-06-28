use ::libc;
extern "C" {
    pub type hash_table;
    fn fstatat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn utimensat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __times: *const timespec,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn hash_insert(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_lookup(
        table: *const Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
}
pub type __uintmax_t = libc::c_ulong;
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
pub type time_t = __time_t;
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
pub type C2RustUnnamed = libc::c_uint;
pub const UTIMECMP_TRUNCATE_SOURCE: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fs_res {
    pub dev: dev_t,
    pub resolution: libc::c_int,
    pub exact: bool,
}
pub const SYSCALL_RESOLUTION: C2RustUnnamed_0 = 1;
pub type Hash_table = hash_table;
pub type uintmax_t = __uintmax_t;
pub type Hash_tuning = hash_tuning;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_tuning {
    pub shrink_threshold: libc::c_float,
    pub shrink_factor: libc::c_float,
    pub growth_threshold: libc::c_float,
    pub growth_factor: libc::c_float,
    pub is_n_buckets: bool,
}
pub type Hash_data_freer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type Hash_comparator = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_hasher = Option::<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
pub type C2RustUnnamed_0 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn utimecmpat(
    mut dfd: libc::c_int,
    mut dst_name: *const libc::c_char,
    mut dst_stat: *const stat,
    mut src_stat: *const stat,
    mut options: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut dst_s: time_t = (*dst_stat).st_mtim.tv_sec;
    let mut src_s: time_t = (*src_stat).st_mtim.tv_sec;
    let mut dst_ns: libc::c_int = get_stat_mtime_ns(dst_stat) as libc::c_int;
    let mut src_ns: libc::c_int = get_stat_mtime_ns(src_stat) as libc::c_int;
    if options & UTIMECMP_TRUNCATE_SOURCE as libc::c_int != 0 {
        static mut ht: *mut Hash_table = 0 as *const Hash_table as *mut Hash_table;
        static mut new_dst_res: *mut fs_res = 0 as *const fs_res as *mut fs_res;
        let mut dst_res: *mut fs_res = 0 as *mut fs_res;
        let mut tmp_dst_res: fs_res = fs_res {
            dev: 0,
            resolution: 0,
            exact: false,
        };
        let mut res: libc::c_int = 0;
        if dst_s == src_s && dst_ns == src_ns {
            return 0 as libc::c_int;
        }
        if dst_s <= src_s - 2 as libc::c_int as libc::c_long {
            return -(1 as libc::c_int);
        }
        if src_s <= dst_s - 2 as libc::c_int as libc::c_long {
            return 1 as libc::c_int;
        }
        if ht.is_null() {
            ht = hash_initialize(
                16 as libc::c_int as size_t,
                0 as *const Hash_tuning,
                Some(
                    dev_info_hash
                        as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
                ),
                Some(
                    dev_info_compare
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> bool,
                ),
                Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            );
        }
        if !ht.is_null() {
            if new_dst_res.is_null() {
                new_dst_res = malloc(::core::mem::size_of::<fs_res>() as libc::c_ulong)
                    as *mut fs_res;
                if new_dst_res.is_null() {
                    current_block = 9522824044611290678;
                } else {
                    (*new_dst_res)
                        .resolution = 2 as libc::c_int
                        * (1000 as libc::c_int * 1000 as libc::c_int
                            * 1000 as libc::c_int);
                    (*new_dst_res).exact = 0 as libc::c_int != 0;
                    current_block = 7976072742316086414;
                }
            } else {
                current_block = 7976072742316086414;
            }
            match current_block {
                9522824044611290678 => {}
                _ => {
                    (*new_dst_res).dev = (*dst_stat).st_dev;
                    dst_res = hash_insert(ht, new_dst_res as *const libc::c_void)
                        as *mut fs_res;
                    if dst_res.is_null() {
                        current_block = 9522824044611290678;
                    } else {
                        if dst_res == new_dst_res {
                            new_dst_res = 0 as *mut fs_res;
                        }
                        current_block = 14648156034262866959;
                    }
                }
            }
        } else {
            current_block = 9522824044611290678;
        }
        match current_block {
            9522824044611290678 => {
                if !ht.is_null() {
                    tmp_dst_res.dev = (*dst_stat).st_dev;
                    dst_res = hash_lookup(
                        ht,
                        &mut tmp_dst_res as *mut fs_res as *const libc::c_void,
                    ) as *mut fs_res;
                }
                if dst_res.is_null() {
                    dst_res = &mut tmp_dst_res;
                    (*dst_res)
                        .resolution = 2 as libc::c_int
                        * (1000 as libc::c_int * 1000 as libc::c_int
                            * 1000 as libc::c_int);
                    (*dst_res).exact = 0 as libc::c_int != 0;
                }
            }
            _ => {}
        }
        res = (*dst_res).resolution;
        if !(*dst_res).exact {
            let mut dst_a_s: time_t = (*dst_stat).st_atim.tv_sec;
            let mut dst_c_s: time_t = (*dst_stat).st_ctim.tv_sec;
            let mut dst_m_s: time_t = dst_s;
            let mut dst_a_ns: libc::c_int = get_stat_atime_ns(dst_stat) as libc::c_int;
            let mut dst_c_ns: libc::c_int = get_stat_ctime_ns(dst_stat) as libc::c_int;
            let mut dst_m_ns: libc::c_int = dst_ns;
            let mut odd_second: bool = (dst_a_s | dst_c_s | dst_m_s)
                & 1 as libc::c_int as libc::c_long != 0;
            if SYSCALL_RESOLUTION as libc::c_int
                == 1000 as libc::c_int * 1000 as libc::c_int * 1000 as libc::c_int
            {
                if odd_second as libc::c_int | dst_a_ns | dst_c_ns | dst_m_ns != 0 {
                    res = 1000 as libc::c_int * 1000 as libc::c_int
                        * 1000 as libc::c_int;
                }
            } else {
                let mut a: libc::c_int = dst_a_ns;
                let mut c: libc::c_int = dst_c_ns;
                let mut m: libc::c_int = dst_m_ns;
                let mut SR10: libc::c_int = SYSCALL_RESOLUTION as libc::c_int;
                SR10 *= 10 as libc::c_int;
                if a % SR10 | c % SR10 | m % SR10 != 0 as libc::c_int {
                    res = SYSCALL_RESOLUTION as libc::c_int;
                } else {
                    res = SR10;
                    a /= SR10;
                    c /= SR10;
                    m /= SR10;
                    while res < (*dst_res).resolution
                        && a % 10 as libc::c_int | c % 10 as libc::c_int
                            | m % 10 as libc::c_int == 0 as libc::c_int
                    {
                        if res
                            == 1000 as libc::c_int * 1000 as libc::c_int
                                * 1000 as libc::c_int
                        {
                            if !odd_second {
                                res *= 2 as libc::c_int;
                            }
                            break;
                        } else {
                            res *= 10 as libc::c_int;
                            a /= 10 as libc::c_int;
                            c /= 10 as libc::c_int;
                            m /= 10 as libc::c_int;
                        }
                    }
                }
            }
            (*dst_res).resolution = res;
            if (SYSCALL_RESOLUTION as libc::c_int) < res {
                let mut timespec: [timespec; 2] = [timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                }; 2];
                let mut dst_status: stat = stat {
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
                src_ns -= src_ns % SYSCALL_RESOLUTION as libc::c_int;
                let mut s: time_t = src_s
                    & !(if res
                        == 2 as libc::c_int
                            * (1000 as libc::c_int * 1000 as libc::c_int
                                * 1000 as libc::c_int)
                    {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as libc::c_long;
                if src_s < dst_s || src_s == dst_s && src_ns <= dst_ns {
                    return 1 as libc::c_int;
                }
                if dst_s < s || dst_s == s && dst_ns < src_ns - src_ns % res {
                    return -(1 as libc::c_int);
                }
                timespec[0 as libc::c_int as usize].tv_sec = dst_a_s;
                timespec[0 as libc::c_int as usize]
                    .tv_nsec = dst_a_ns as __syscall_slong_t;
                timespec[1 as libc::c_int as usize]
                    .tv_sec = dst_m_s
                    | (res
                        == 2 as libc::c_int
                            * (1000 as libc::c_int * 1000 as libc::c_int
                                * 1000 as libc::c_int)) as libc::c_int as libc::c_long;
                timespec[1 as libc::c_int as usize]
                    .tv_nsec = (dst_m_ns + res / 9 as libc::c_int) as __syscall_slong_t;
                if utimensat(
                    dfd,
                    dst_name,
                    timespec.as_mut_ptr() as *const timespec,
                    0x100 as libc::c_int,
                ) != 0
                {
                    return -(2 as libc::c_int);
                }
                let mut stat_result: libc::c_int = fstatat(
                    dfd,
                    dst_name,
                    &mut dst_status,
                    0x100 as libc::c_int,
                );
                if stat_result as libc::c_long | dst_status.st_mtim.tv_sec ^ dst_m_s
                    | get_stat_mtime_ns(&mut dst_status) ^ dst_m_ns as libc::c_long != 0
                {
                    timespec[1 as libc::c_int as usize].tv_sec = dst_m_s;
                    timespec[1 as libc::c_int as usize]
                        .tv_nsec = dst_m_ns as __syscall_slong_t;
                    utimensat(
                        dfd,
                        dst_name,
                        timespec.as_mut_ptr() as *const timespec,
                        0x100 as libc::c_int,
                    );
                }
                if stat_result != 0 as libc::c_int {
                    return -(2 as libc::c_int);
                }
                let mut old_res: libc::c_int = res;
                let mut a_0: libc::c_int = ((1000 as libc::c_int * 1000 as libc::c_int
                    * 1000 as libc::c_int) as libc::c_long
                    * (dst_status.st_mtim.tv_sec & 1 as libc::c_int as libc::c_long)
                    + get_stat_mtime_ns(&mut dst_status)) as libc::c_int;
                res = SYSCALL_RESOLUTION as libc::c_int;
                a_0 /= res;
                while a_0 % 10 as libc::c_int == 0 as libc::c_int {
                    if res
                        == 1000 as libc::c_int * 1000 as libc::c_int
                            * 1000 as libc::c_int
                    {
                        res *= 2 as libc::c_int;
                        break;
                    } else {
                        res *= 10 as libc::c_int;
                        if res == old_res {
                            break;
                        }
                        a_0 /= 10 as libc::c_int;
                    }
                }
            }
            (*dst_res).resolution = res;
            (*dst_res).exact = 1 as libc::c_int != 0;
        }
        src_s
            &= !if res
                == 2 as libc::c_int
                    * (1000 as libc::c_int * 1000 as libc::c_int * 1000 as libc::c_int)
            {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } as libc::c_long;
        src_ns -= src_ns % res;
    }
    return (dst_s > src_s) as libc::c_int - (dst_s < src_s) as libc::c_int
        + ((if dst_s == src_s { !(0 as libc::c_int) } else { 0 as libc::c_int })
            & (dst_ns > src_ns) as libc::c_int - (dst_ns < src_ns) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn utimecmp(
    mut dst_name: *const libc::c_char,
    mut dst_stat: *const stat,
    mut src_stat: *const stat,
    mut options: libc::c_int,
) -> libc::c_int {
    return utimecmpat(-(100 as libc::c_int), dst_name, dst_stat, src_stat, options);
}
#[inline]
unsafe extern "C" fn get_stat_ctime_ns(mut st: *const stat) -> libc::c_long {
    return (*st).st_ctim.tv_nsec;
}
#[inline]
unsafe extern "C" fn get_stat_atime_ns(mut st: *const stat) -> libc::c_long {
    return (*st).st_atim.tv_nsec;
}
#[inline]
unsafe extern "C" fn get_stat_mtime_ns(mut st: *const stat) -> libc::c_long {
    return (*st).st_mtim.tv_nsec;
}
unsafe extern "C" fn dev_info_hash(
    mut x: *const libc::c_void,
    mut table_size: size_t,
) -> size_t {
    let mut p: *const fs_res = x as *const fs_res;
    if !((0 as libc::c_int as dev_t) < -(1 as libc::c_int) as dev_t)
        && (18446744073709551615 as libc::c_ulong)
            < (if 2147483647 as libc::c_int as libc::c_ulong
                > (if (0 as libc::c_int as dev_t) < -(1 as libc::c_int) as dev_t {
                    -(1 as libc::c_int) as dev_t
                } else {
                    ((1 as libc::c_int as dev_t)
                        << (::core::mem::size_of::<dev_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                })
            {
                2147483647 as libc::c_int as libc::c_ulong
            } else {
                (if (0 as libc::c_int as dev_t) < -(1 as libc::c_int) as dev_t {
                    -(1 as libc::c_int) as dev_t
                } else {
                    ((1 as libc::c_int as dev_t)
                        << (::core::mem::size_of::<dev_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                })
            })
    {
        let mut dev: uintmax_t = (*p).dev;
        return dev.wrapping_rem(table_size);
    }
    return ((*p).dev).wrapping_rem(table_size);
}
unsafe extern "C" fn dev_info_compare(
    mut x: *const libc::c_void,
    mut y: *const libc::c_void,
) -> bool {
    let mut a: *const fs_res = x as *const fs_res;
    let mut b: *const fs_res = y as *const fs_res;
    return (*a).dev == (*b).dev;
}
