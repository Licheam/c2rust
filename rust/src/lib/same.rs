use ::libc;
extern "C" {
    fn free(_: *mut libc::c_void);
    fn fstatat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn dir_name(file: *const libc::c_char) -> *mut libc::c_char;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn base_len(filename: *const libc::c_char) -> size_t;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
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
#[no_mangle]
pub unsafe extern "C" fn same_name(
    mut source: *const libc::c_char,
    mut dest: *const libc::c_char,
) -> bool {
    return same_nameat(-(100 as libc::c_int), source, -(100 as libc::c_int), dest);
}
#[no_mangle]
pub unsafe extern "C" fn same_nameat(
    mut source_dfd: libc::c_int,
    mut source: *const libc::c_char,
    mut dest_dfd: libc::c_int,
    mut dest: *const libc::c_char,
) -> bool {
    let mut source_basename: *const libc::c_char = last_component(source);
    let mut dest_basename: *const libc::c_char = last_component(dest);
    let mut source_baselen: size_t = base_len(source_basename);
    let mut dest_baselen: size_t = base_len(dest_basename);
    let mut identical_basenames: bool = source_baselen == dest_baselen
        && memcmp(
            source_basename as *const libc::c_void,
            dest_basename as *const libc::c_void,
            dest_baselen,
        ) == 0 as libc::c_int;
    let mut compare_dirs: bool = identical_basenames;
    let mut same: bool = 0 as libc::c_int != 0;
    if compare_dirs {
        let mut source_dir_stats: stat = stat {
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
        let mut dest_dir_stats: stat = stat {
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
        let mut source_dirname: *mut libc::c_char = dir_name(source);
        let mut flags: libc::c_int = 0x100 as libc::c_int;
        if fstatat(source_dfd, source_dirname, &mut source_dir_stats, flags)
            != 0 as libc::c_int
        {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                source_dirname,
            );
        }
        free(source_dirname as *mut libc::c_void);
        let mut dest_dirname: *mut libc::c_char = dir_name(dest);
        if fstatat(dest_dfd, dest_dirname, &mut dest_dir_stats, flags)
            != 0 as libc::c_int
        {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                dest_dirname,
            );
        }
        same = source_dir_stats.st_ino == dest_dir_stats.st_ino
            && source_dir_stats.st_dev == dest_dir_stats.st_dev;
        free(dest_dirname as *mut libc::c_void);
    }
    return same;
}
