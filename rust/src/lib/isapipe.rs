use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
}
pub type __mode_t = libc::c_uint;
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
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __time_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __gid_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __ino_t = libc::c_ulong;
pub type nlink_t = __nlink_t;
#[no_mangle]
pub unsafe extern "C" fn isapipe(mut fd: libc::c_int) -> libc::c_int {
    let mut pipe_link_count_max: nlink_t = 1 as libc::c_int as nlink_t;
    let mut check_for_fifo: bool = 1 as libc::c_int == 1 as libc::c_int;
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
    let mut fstat_result: libc::c_int = fstat(fd, &mut st);
    if fstat_result != 0 as libc::c_int {
        return fstat_result;
    }
    if !((1 as libc::c_int == 0 as libc::c_int || 1 as libc::c_int == 1 as libc::c_int)
        && 1 as libc::c_int as libc::c_ulong != -(1 as libc::c_int) as nlink_t)
        && (st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o10000 as libc::c_int as libc::c_uint) as libc::c_int
            | (st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o140000 as libc::c_int as libc::c_uint) as libc::c_int != 0
    {
        let mut fd_pair: [libc::c_int; 2] = [0; 2];
        let mut pipe_result: libc::c_int = pipe(fd_pair.as_mut_ptr());
        if pipe_result != 0 as libc::c_int {
            return pipe_result
        } else {
            let mut pipe_st: stat = stat {
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
            let mut fstat_pipe_result: libc::c_int = fstat(
                fd_pair[0 as libc::c_int as usize],
                &mut pipe_st,
            );
            let mut fstat_pipe_errno: libc::c_int = *__errno_location();
            close(fd_pair[0 as libc::c_int as usize]);
            close(fd_pair[1 as libc::c_int as usize]);
            if fstat_pipe_result != 0 as libc::c_int {
                *__errno_location() = fstat_pipe_errno;
                return fstat_pipe_result;
            }
            check_for_fifo = (pipe_st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o10000 as libc::c_int as libc::c_uint) as libc::c_int
                != 0 as libc::c_int;
            pipe_link_count_max = pipe_st.st_nlink;
        }
    }
    return (st.st_nlink <= pipe_link_count_max
        && (if check_for_fifo as libc::c_int != 0 {
            (st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o10000 as libc::c_int as libc::c_uint) as libc::c_int
        } else {
            (st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o140000 as libc::c_int as libc::c_uint) as libc::c_int
        }) != 0) as libc::c_int;
}
