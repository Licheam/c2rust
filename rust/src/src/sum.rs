use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
    fn fread_unlocked(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn feof_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn ferror_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn human_readable(
        _: uintmax_t,
        _: *mut libc::c_char,
        _: libc::c_int,
        _: uintmax_t,
        _: uintmax_t,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uintmax_t = libc::c_ulong;
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
pub type uint8_t = __uint8_t;
pub type uintmax_t = __uintmax_t;
pub type C2RustUnnamed = libc::c_uint;
pub const human_B: C2RustUnnamed = 256;
pub const human_SI: C2RustUnnamed = 128;
pub const human_space_before_unit: C2RustUnnamed = 64;
pub const human_base_1024: C2RustUnnamed = 32;
pub const human_autoscale: C2RustUnnamed = 16;
pub const human_suppress_point_zero: C2RustUnnamed = 8;
pub const human_group_digits: C2RustUnnamed = 4;
pub const human_floor: C2RustUnnamed = 2;
pub const human_round_to_nearest: C2RustUnnamed = 1;
pub const human_ceiling: C2RustUnnamed = 0;
#[no_mangle]
pub unsafe extern "C" fn bsd_sum_stream(
    mut stream: *mut FILE,
    mut resstream: *mut libc::c_void,
    mut length: *mut uintmax_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut sum: size_t = 0;
    let mut n: size_t = 0;
    let mut checksum: libc::c_int = 0 as libc::c_int;
    let mut total_bytes: uintmax_t = 0 as libc::c_int as uintmax_t;
    static mut buffer_length: size_t = 32768 as libc::c_int as size_t;
    let mut buffer: *mut uint8_t = malloc(buffer_length) as *mut uint8_t;
    if buffer.is_null() {
        return -(1 as libc::c_int);
    }
    's_23: loop {
        sum = 0 as libc::c_int as size_t;
        loop {
            n = fread_unlocked(
                buffer.offset(sum as isize) as *mut libc::c_void,
                1 as libc::c_int as size_t,
                buffer_length.wrapping_sub(sum),
                stream,
            );
            sum = (sum as libc::c_ulong).wrapping_add(n) as size_t as size_t;
            if buffer_length == sum {
                break;
            }
            if n == 0 as libc::c_int as libc::c_ulong {
                if ferror_unlocked(stream) != 0 {
                    current_block = 13123974614121405167;
                    break 's_23;
                } else {
                    current_block = 5601891728916014340;
                    break 's_23;
                }
            } else if feof_unlocked(stream) != 0 {
                current_block = 5601891728916014340;
                break 's_23;
            }
        }
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < sum {
            checksum = (checksum >> 1 as libc::c_int)
                + ((checksum & 1 as libc::c_int) << 15 as libc::c_int);
            checksum += *buffer.offset(i as isize) as libc::c_int;
            checksum &= 0xffff as libc::c_int;
            i = i.wrapping_add(1);
            i;
        }
        if total_bytes.wrapping_add(sum) < total_bytes {
            *__errno_location() = 75 as libc::c_int;
            current_block = 13123974614121405167;
            break;
        } else {
            total_bytes = (total_bytes as libc::c_ulong).wrapping_add(sum) as uintmax_t
                as uintmax_t;
        }
    }
    match current_block {
        5601891728916014340 => {
            let mut i_0: size_t = 0 as libc::c_int as size_t;
            while i_0 < sum {
                checksum = (checksum >> 1 as libc::c_int)
                    + ((checksum & 1 as libc::c_int) << 15 as libc::c_int);
                checksum += *buffer.offset(i_0 as isize) as libc::c_int;
                checksum &= 0xffff as libc::c_int;
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
            if total_bytes.wrapping_add(sum) < total_bytes {
                *__errno_location() = 75 as libc::c_int;
            } else {
                total_bytes = (total_bytes as libc::c_ulong).wrapping_add(sum)
                    as uintmax_t as uintmax_t;
                memcpy(
                    resstream,
                    &mut checksum as *mut libc::c_int as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                );
                *length = total_bytes;
                ret = 0 as libc::c_int;
            }
        }
        _ => {}
    }
    free(buffer as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn sysv_sum_stream(
    mut stream: *mut FILE,
    mut resstream: *mut libc::c_void,
    mut length: *mut uintmax_t,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut checksum: libc::c_int = 0;
    let mut current_block: u64;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut sum: size_t = 0;
    let mut n: size_t = 0;
    let mut total_bytes: uintmax_t = 0 as libc::c_int as uintmax_t;
    static mut buffer_length: size_t = 32768 as libc::c_int as size_t;
    let mut buffer: *mut uint8_t = malloc(buffer_length) as *mut uint8_t;
    if buffer.is_null() {
        return -(1 as libc::c_int);
    }
    let mut s: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    's_23: loop {
        sum = 0 as libc::c_int as size_t;
        loop {
            n = fread_unlocked(
                buffer.offset(sum as isize) as *mut libc::c_void,
                1 as libc::c_int as size_t,
                buffer_length.wrapping_sub(sum),
                stream,
            );
            sum = (sum as libc::c_ulong).wrapping_add(n) as size_t as size_t;
            if buffer_length == sum {
                break;
            }
            if n == 0 as libc::c_int as libc::c_ulong {
                if ferror_unlocked(stream) != 0 {
                    current_block = 9808613889076251090;
                    break 's_23;
                } else {
                    current_block = 13242334135786603907;
                    break 's_23;
                }
            } else if feof_unlocked(stream) != 0 {
                current_block = 13242334135786603907;
                break 's_23;
            }
        }
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < sum {
            s = s.wrapping_add(*buffer.offset(i as isize) as libc::c_uint);
            i = i.wrapping_add(1);
            i;
        }
        if total_bytes.wrapping_add(sum) < total_bytes {
            *__errno_location() = 75 as libc::c_int;
            current_block = 9808613889076251090;
            break;
        } else {
            total_bytes = (total_bytes as libc::c_ulong).wrapping_add(sum) as uintmax_t
                as uintmax_t;
        }
    }
    match current_block {
        13242334135786603907 => {
            let mut i_0: size_t = 0 as libc::c_int as size_t;
            while i_0 < sum {
                s = s.wrapping_add(*buffer.offset(i_0 as isize) as libc::c_uint);
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
            if total_bytes.wrapping_add(sum) < total_bytes {
                *__errno_location() = 75 as libc::c_int;
            } else {
                total_bytes = (total_bytes as libc::c_ulong).wrapping_add(sum)
                    as uintmax_t as uintmax_t;
                r = (s & 0xffff as libc::c_int as libc::c_uint)
                    .wrapping_add((s & 0xffffffff as libc::c_uint) >> 16 as libc::c_int)
                    as libc::c_int;
                checksum = (r & 0xffff as libc::c_int) + (r >> 16 as libc::c_int);
                memcpy(
                    resstream,
                    &mut checksum as *mut libc::c_int as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                );
                *length = total_bytes;
                ret = 0 as libc::c_int;
            }
        }
        _ => {}
    }
    free(buffer as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn output_bsd(
    mut file: *const libc::c_char,
    mut binary_file: libc::c_int,
    mut digest: *const libc::c_void,
    mut tagged: bool,
    mut delim: libc::c_uchar,
    mut args: bool,
    mut length: uintmax_t,
) {
    let mut hbuf: [libc::c_char; 652] = [0; 652];
    printf(
        b"%05d %5s\0" as *const u8 as *const libc::c_char,
        *(digest as *mut libc::c_int),
        human_readable(
            length,
            hbuf.as_mut_ptr(),
            human_ceiling as libc::c_int,
            1 as libc::c_int as uintmax_t,
            1024 as libc::c_int as uintmax_t,
        ),
    );
    if args {
        printf(b" %s\0" as *const u8 as *const libc::c_char, file);
    }
    putchar_unlocked(delim as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn output_sysv(
    mut file: *const libc::c_char,
    mut binary_file: libc::c_int,
    mut digest: *const libc::c_void,
    mut tagged: bool,
    mut delim: libc::c_uchar,
    mut args: bool,
    mut length: uintmax_t,
) {
    let mut hbuf: [libc::c_char; 652] = [0; 652];
    printf(
        b"%d %s\0" as *const u8 as *const libc::c_char,
        *(digest as *mut libc::c_int),
        human_readable(
            length,
            hbuf.as_mut_ptr(),
            human_ceiling as libc::c_int,
            1 as libc::c_int as uintmax_t,
            512 as libc::c_int as uintmax_t,
        ),
    );
    if args {
        printf(b" %s\0" as *const u8 as *const libc::c_char, file);
    }
    putchar_unlocked(delim as libc::c_int);
}
