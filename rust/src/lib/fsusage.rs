use ::libc;
extern "C" {
    fn statvfs(__file: *const libc::c_char, __buf: *mut statvfs) -> libc::c_int;
    fn strverscmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn uname(__name: *mut utsname) -> libc::c_int;
    fn statfs(__file: *const libc::c_char, __buf: *mut statfs) -> libc::c_int;
}
pub type __uintmax_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __fsid_t {
    pub __val: [libc::c_int; 2],
}
pub type __fsblkcnt_t = libc::c_ulong;
pub type __fsfilcnt_t = libc::c_ulong;
pub type __fsword_t = libc::c_long;
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fs_usage {
    pub fsu_blocksize: uintmax_t,
    pub fsu_blocks: uintmax_t,
    pub fsu_bfree: uintmax_t,
    pub fsu_bavail: uintmax_t,
    pub fsu_bavail_top_bit_set: bool,
    pub fsu_files: uintmax_t,
    pub fsu_ffree: uintmax_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statfs {
    pub f_type: __fsword_t,
    pub f_bsize: __fsword_t,
    pub f_blocks: __fsblkcnt_t,
    pub f_bfree: __fsblkcnt_t,
    pub f_bavail: __fsblkcnt_t,
    pub f_files: __fsfilcnt_t,
    pub f_ffree: __fsfilcnt_t,
    pub f_fsid: __fsid_t,
    pub f_namelen: __fsword_t,
    pub f_frsize: __fsword_t,
    pub f_flags: __fsword_t,
    pub f_spare: [__fsword_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statvfs {
    pub f_bsize: libc::c_ulong,
    pub f_frsize: libc::c_ulong,
    pub f_blocks: __fsblkcnt_t,
    pub f_bfree: __fsblkcnt_t,
    pub f_bavail: __fsblkcnt_t,
    pub f_files: __fsfilcnt_t,
    pub f_ffree: __fsfilcnt_t,
    pub f_favail: __fsfilcnt_t,
    pub f_fsid: libc::c_ulong,
    pub f_flag: libc::c_ulong,
    pub f_namemax: libc::c_ulong,
    pub __f_spare: [libc::c_int; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [libc::c_char; 65],
    pub nodename: [libc::c_char; 65],
    pub release: [libc::c_char; 65],
    pub version: [libc::c_char; 65],
    pub machine: [libc::c_char; 65],
    pub domainname: [libc::c_char; 65],
}
#[no_mangle]
pub unsafe extern "C" fn get_fs_usage(
    mut file: *const libc::c_char,
    mut disk: *const libc::c_char,
    mut fsp: *mut fs_usage,
) -> libc::c_int {
    if statvfs_works() != 0 {
        let mut vfsd: statvfs = statvfs {
            f_bsize: 0,
            f_frsize: 0,
            f_blocks: 0,
            f_bfree: 0,
            f_bavail: 0,
            f_files: 0,
            f_ffree: 0,
            f_favail: 0,
            f_fsid: 0,
            f_flag: 0,
            f_namemax: 0,
            __f_spare: [0; 6],
        };
        if statvfs(file, &mut vfsd) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        (*fsp)
            .fsu_blocksize = if vfsd.f_frsize != 0 {
            if (::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                < ::core::mem::size_of::<uintmax_t>() as libc::c_ulong
                && !vfsd.f_frsize
                    == (if (::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                        < ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
                    {
                        -((1 as libc::c_int)
                            << (::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong))
                    } else {
                        0 as libc::c_int
                    }) as libc::c_ulong
            {
                18446744073709551615 as libc::c_ulong
            } else {
                vfsd.f_frsize
            }
        } else if (::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
            < ::core::mem::size_of::<uintmax_t>() as libc::c_ulong
            && !vfsd.f_bsize
                == (if (::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                    < ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
                {
                    -((1 as libc::c_int)
                        << (::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong))
                } else {
                    0 as libc::c_int
                }) as libc::c_ulong
        {
            18446744073709551615 as libc::c_ulong
        } else {
            vfsd.f_bsize
        };
        (*fsp)
            .fsu_blocks = if (::core::mem::size_of::<__fsblkcnt_t>() as libc::c_ulong)
            < ::core::mem::size_of::<uintmax_t>() as libc::c_ulong
            && !vfsd.f_blocks
                == (if (::core::mem::size_of::<__fsblkcnt_t>() as libc::c_ulong)
                    < ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
                {
                    -((1 as libc::c_int)
                        << (::core::mem::size_of::<__fsblkcnt_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong))
                } else {
                    0 as libc::c_int
                }) as libc::c_ulong
        {
            18446744073709551615 as libc::c_ulong
        } else {
            vfsd.f_blocks
        };
        (*fsp)
            .fsu_bfree = if (::core::mem::size_of::<__fsblkcnt_t>() as libc::c_ulong)
            < ::core::mem::size_of::<uintmax_t>() as libc::c_ulong
            && !vfsd.f_bfree
                == (if (::core::mem::size_of::<__fsblkcnt_t>() as libc::c_ulong)
                    < ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
                {
                    -((1 as libc::c_int)
                        << (::core::mem::size_of::<__fsblkcnt_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong))
                } else {
                    0 as libc::c_int
                }) as libc::c_ulong
        {
            18446744073709551615 as libc::c_ulong
        } else {
            vfsd.f_bfree
        };
        (*fsp)
            .fsu_bavail = vfsd.f_bavail
            | !(vfsd.f_bavail
                & (1 as libc::c_int as uintmax_t)
                    << (::core::mem::size_of::<__fsblkcnt_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        (*fsp)
            .fsu_bavail_top_bit_set = vfsd.f_bavail
            & (1 as libc::c_int as uintmax_t)
                << (::core::mem::size_of::<__fsblkcnt_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            != 0 as libc::c_int as libc::c_ulong;
        (*fsp)
            .fsu_files = if (::core::mem::size_of::<__fsfilcnt_t>() as libc::c_ulong)
            < ::core::mem::size_of::<uintmax_t>() as libc::c_ulong
            && !vfsd.f_files
                == (if (::core::mem::size_of::<__fsfilcnt_t>() as libc::c_ulong)
                    < ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
                {
                    -((1 as libc::c_int)
                        << (::core::mem::size_of::<__fsfilcnt_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong))
                } else {
                    0 as libc::c_int
                }) as libc::c_ulong
        {
            18446744073709551615 as libc::c_ulong
        } else {
            vfsd.f_files
        };
        (*fsp)
            .fsu_ffree = if (::core::mem::size_of::<__fsfilcnt_t>() as libc::c_ulong)
            < ::core::mem::size_of::<uintmax_t>() as libc::c_ulong
            && !vfsd.f_ffree
                == (if (::core::mem::size_of::<__fsfilcnt_t>() as libc::c_ulong)
                    < ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
                {
                    -((1 as libc::c_int)
                        << (::core::mem::size_of::<__fsfilcnt_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong))
                } else {
                    0 as libc::c_int
                }) as libc::c_ulong
        {
            18446744073709551615 as libc::c_ulong
        } else {
            vfsd.f_ffree
        };
        return 0 as libc::c_int;
    }
    let mut fsd: statfs = statfs {
        f_type: 0,
        f_bsize: 0,
        f_blocks: 0,
        f_bfree: 0,
        f_bavail: 0,
        f_files: 0,
        f_ffree: 0,
        f_fsid: __fsid_t { __val: [0; 2] },
        f_namelen: 0,
        f_frsize: 0,
        f_flags: 0,
        f_spare: [0; 4],
    };
    if statfs(file, &mut fsd) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*fsp)
        .fsu_blocksize = if (::core::mem::size_of::<__fsword_t>() as libc::c_ulong)
        < ::core::mem::size_of::<uintmax_t>() as libc::c_ulong
        && !fsd.f_frsize
            == (if (::core::mem::size_of::<__fsword_t>() as libc::c_ulong)
                < ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
            {
                -((1 as libc::c_int)
                    << (::core::mem::size_of::<__fsword_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong))
            } else {
                0 as libc::c_int
            }) as libc::c_long
    {
        18446744073709551615 as libc::c_ulong
    } else {
        fsd.f_frsize as uintmax_t
    };
    (*fsp)
        .fsu_blocks = if (::core::mem::size_of::<__fsblkcnt_t>() as libc::c_ulong)
        < ::core::mem::size_of::<uintmax_t>() as libc::c_ulong
        && !fsd.f_blocks
            == (if (::core::mem::size_of::<__fsblkcnt_t>() as libc::c_ulong)
                < ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
            {
                -((1 as libc::c_int)
                    << (::core::mem::size_of::<__fsblkcnt_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong))
            } else {
                0 as libc::c_int
            }) as libc::c_ulong
    {
        18446744073709551615 as libc::c_ulong
    } else {
        fsd.f_blocks
    };
    (*fsp)
        .fsu_bfree = if (::core::mem::size_of::<__fsblkcnt_t>() as libc::c_ulong)
        < ::core::mem::size_of::<uintmax_t>() as libc::c_ulong
        && !fsd.f_bfree
            == (if (::core::mem::size_of::<__fsblkcnt_t>() as libc::c_ulong)
                < ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
            {
                -((1 as libc::c_int)
                    << (::core::mem::size_of::<__fsblkcnt_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong))
            } else {
                0 as libc::c_int
            }) as libc::c_ulong
    {
        18446744073709551615 as libc::c_ulong
    } else {
        fsd.f_bfree
    };
    (*fsp)
        .fsu_bavail = fsd.f_bavail
        | !(fsd.f_bavail
            & (1 as libc::c_int as uintmax_t)
                << (::core::mem::size_of::<__fsblkcnt_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    (*fsp)
        .fsu_bavail_top_bit_set = fsd.f_bavail
        & (1 as libc::c_int as uintmax_t)
            << (::core::mem::size_of::<__fsblkcnt_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        != 0 as libc::c_int as libc::c_ulong;
    (*fsp)
        .fsu_files = if (::core::mem::size_of::<__fsfilcnt_t>() as libc::c_ulong)
        < ::core::mem::size_of::<uintmax_t>() as libc::c_ulong
        && !fsd.f_files
            == (if (::core::mem::size_of::<__fsfilcnt_t>() as libc::c_ulong)
                < ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
            {
                -((1 as libc::c_int)
                    << (::core::mem::size_of::<__fsfilcnt_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong))
            } else {
                0 as libc::c_int
            }) as libc::c_ulong
    {
        18446744073709551615 as libc::c_ulong
    } else {
        fsd.f_files
    };
    (*fsp)
        .fsu_ffree = if (::core::mem::size_of::<__fsfilcnt_t>() as libc::c_ulong)
        < ::core::mem::size_of::<uintmax_t>() as libc::c_ulong
        && !fsd.f_ffree
            == (if (::core::mem::size_of::<__fsfilcnt_t>() as libc::c_ulong)
                < ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
            {
                -((1 as libc::c_int)
                    << (::core::mem::size_of::<__fsfilcnt_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong))
            } else {
                0 as libc::c_int
            }) as libc::c_ulong
    {
        18446744073709551615 as libc::c_ulong
    } else {
        fsd.f_ffree
    };
    return 0 as libc::c_int;
}
unsafe extern "C" fn statvfs_works() -> libc::c_int {
    static mut statvfs_works_cache: libc::c_int = -(1 as libc::c_int);
    let mut name: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };
    if statvfs_works_cache < 0 as libc::c_int {
        statvfs_works_cache = (uname(&mut name) == 0 as libc::c_int
            && 0 as libc::c_int
                <= strverscmp(
                    (name.release).as_mut_ptr(),
                    b"2.6.36\0" as *const u8 as *const libc::c_char,
                )) as libc::c_int;
    }
    return statvfs_works_cache;
}
