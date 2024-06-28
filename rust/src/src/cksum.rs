use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use core::arch::asm;
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
    fn __errno_location() -> *mut libc::c_int;
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    static mut cksum_debug: bool;
    fn cksum_pclmul(
        fp: *mut FILE,
        crc_out: *mut uint_fast32_t,
        length_out: *mut uintmax_t,
    ) -> bool;
    static crctab: [[uint_fast32_t; 256]; 8];
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
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
pub type uint32_t = __uint32_t;
pub type uint_fast32_t = libc::c_ulong;
pub type uintmax_t = __uintmax_t;
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn crc_sum_stream(
    mut stream: *mut FILE,
    mut resstream: *mut libc::c_void,
    mut length: *mut uintmax_t,
) -> libc::c_int {
    let mut total_bytes: uintmax_t = 0 as libc::c_int as uintmax_t;
    let mut crc: uint_fast32_t = 0 as libc::c_int as uint_fast32_t;
    if cksum_fp.is_none() {
        if pclmul_supported() {
            cksum_fp = Some(
                cksum_pclmul
                    as unsafe extern "C" fn(
                        *mut FILE,
                        *mut uint_fast32_t,
                        *mut uintmax_t,
                    ) -> bool,
            );
        } else {
            cksum_fp = Some(
                cksum_slice8
                    as unsafe extern "C" fn(
                        *mut FILE,
                        *mut uint_fast32_t,
                        *mut uintmax_t,
                    ) -> bool,
            );
        }
    }
    if !cksum_fp.expect("non-null function pointer")(stream, &mut crc, &mut total_bytes)
    {
        return -(1 as libc::c_int);
    }
    *length = total_bytes;
    while total_bytes != 0 {
        crc = crc << 8 as libc::c_int
            ^ crctab[0 as libc::c_int
                as usize][((crc >> 24 as libc::c_int ^ total_bytes)
                & 0xff as libc::c_int as libc::c_ulong) as usize];
        total_bytes >>= 8 as libc::c_int;
    }
    crc = !crc & 0xffffffff as libc::c_uint as libc::c_ulong;
    let mut crc_out: libc::c_uint = crc as libc::c_uint;
    memcpy(
        resstream,
        &mut crc_out as *mut libc::c_uint as *const libc::c_void,
        ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn output_crc(
    mut file: *const libc::c_char,
    mut binary_file: libc::c_int,
    mut digest: *const libc::c_void,
    mut tagged: bool,
    mut delim: libc::c_uchar,
    mut args: bool,
    mut length: uintmax_t,
) {
    let mut length_buf: [libc::c_char; 21] = [0; 21];
    printf(
        b"%u %s\0" as *const u8 as *const libc::c_char,
        *(digest as *mut libc::c_uint),
        umaxtostr(length, length_buf.as_mut_ptr()),
    );
    if args {
        printf(b" %s\0" as *const u8 as *const libc::c_char, file);
    }
    putchar_unlocked(delim as libc::c_int);
}
#[inline]
unsafe extern "C" fn __get_cpuid_max(
    mut __leaf: libc::c_uint,
    mut __sig: *mut libc::c_uint,
) -> libc::c_int {
    let mut __eax: libc::c_uint = 0;
    let mut __ebx: libc::c_uint = 0;
    let mut __ecx: libc::c_uint = 0;
    let mut __edx: libc::c_uint = 0;
    let fresh0 = &mut __eax;
    let fresh1;
    let fresh2 = __leaf;
    asm!(
        "  xchgq  %rbx,{1:r}\n  cpuid\n  xchgq  %rbx,{1:r}", inlateout("ax")
        c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2) => fresh1, lateout(reg) __ebx,
        lateout("cx") __ecx, lateout("dx") __edx, options(preserves_flags, pure,
        readonly, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
    if !__sig.is_null() {
        *__sig = __ebx;
    }
    return __eax as libc::c_int;
}
#[inline]
unsafe extern "C" fn __get_cpuid(
    mut __leaf: libc::c_uint,
    mut __eax: *mut libc::c_uint,
    mut __ebx: *mut libc::c_uint,
    mut __ecx: *mut libc::c_uint,
    mut __edx: *mut libc::c_uint,
) -> libc::c_int {
    let mut __max_leaf: libc::c_uint = __get_cpuid_max(
        __leaf & 0x80000000 as libc::c_uint,
        0 as *mut libc::c_uint,
    ) as libc::c_uint;
    if __max_leaf == 0 as libc::c_int as libc::c_uint || __max_leaf < __leaf {
        return 0 as libc::c_int;
    }
    let fresh3 = &mut *__eax;
    let fresh4;
    let fresh5 = __leaf;
    asm!(
        "  xchgq  %rbx,{1:r}\n  cpuid\n  xchgq  %rbx,{1:r}", inlateout("ax")
        c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5) => fresh4, lateout(reg) *
        __ebx, lateout("cx") * __ecx, lateout("dx") * __edx, options(preserves_flags,
        pure, readonly, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
    return 1 as libc::c_int;
}
unsafe extern "C" fn cksum_slice8(
    mut fp: *mut FILE,
    mut crc_out: *mut uint_fast32_t,
    mut length_out: *mut uintmax_t,
) -> bool {
    let mut buf: [uint32_t; 16384] = [0; 16384];
    let mut crc: uint_fast32_t = 0 as libc::c_int as uint_fast32_t;
    let mut length: uintmax_t = 0 as libc::c_int as uintmax_t;
    let mut bytes_read: size_t = 0;
    if fp.is_null() || crc_out.is_null() || length_out.is_null() {
        return 0 as libc::c_int != 0;
    }
    loop {
        bytes_read = fread_unlocked(
            buf.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as size_t,
            ((1 as libc::c_int) << 16 as libc::c_int) as size_t,
            fp,
        );
        if !(bytes_read > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        let mut datap: *mut uint32_t = 0 as *mut uint32_t;
        if length.wrapping_add(bytes_read) < length {
            *__errno_location() = 75 as libc::c_int;
            return 0 as libc::c_int != 0;
        }
        length = (length as libc::c_ulong).wrapping_add(bytes_read) as uintmax_t
            as uintmax_t;
        if bytes_read == 0 as libc::c_int as libc::c_ulong {
            if ferror_unlocked(fp) != 0 {
                return 0 as libc::c_int != 0;
            }
        }
        datap = buf.as_mut_ptr();
        while bytes_read >= 8 as libc::c_int as libc::c_ulong {
            let fresh6 = datap;
            datap = datap.offset(1);
            let mut first: uint32_t = *fresh6;
            let fresh7 = datap;
            datap = datap.offset(1);
            let mut second: uint32_t = *fresh7;
            crc ^= __bswap_32(first) as libc::c_ulong;
            second = __bswap_32(second);
            crc = crctab[7 as libc::c_int
                as usize][(crc >> 24 as libc::c_int
                & 0xff as libc::c_int as libc::c_ulong) as usize]
                ^ crctab[6 as libc::c_int
                    as usize][(crc >> 16 as libc::c_int
                    & 0xff as libc::c_int as libc::c_ulong) as usize]
                ^ crctab[5 as libc::c_int
                    as usize][(crc >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_ulong) as usize]
                ^ crctab[4 as libc::c_int
                    as usize][(crc & 0xff as libc::c_int as libc::c_ulong) as usize]
                ^ crctab[3 as libc::c_int
                    as usize][(second >> 24 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as usize]
                ^ crctab[2 as libc::c_int
                    as usize][(second >> 16 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as usize]
                ^ crctab[1 as libc::c_int
                    as usize][(second >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as usize]
                ^ crctab[0 as libc::c_int
                    as usize][(second & 0xff as libc::c_int as libc::c_uint) as usize];
            bytes_read = (bytes_read as libc::c_ulong)
                .wrapping_sub(8 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        let mut cp: *mut libc::c_uchar = datap as *mut libc::c_uchar;
        loop {
            let fresh8 = bytes_read;
            bytes_read = bytes_read.wrapping_sub(1);
            if !(fresh8 != 0) {
                break;
            }
            let fresh9 = cp;
            cp = cp.offset(1);
            crc = crc << 8 as libc::c_int
                ^ crctab[0 as libc::c_int
                    as usize][((crc >> 24 as libc::c_int ^ *fresh9 as libc::c_ulong)
                    & 0xff as libc::c_int as libc::c_ulong) as usize];
        }
        if feof_unlocked(fp) != 0 {
            break;
        }
    }
    *crc_out = crc;
    *length_out = length;
    return 1 as libc::c_int != 0;
}
static mut cksum_fp: Option::<
    unsafe extern "C" fn(*mut FILE, *mut uint_fast32_t, *mut uintmax_t) -> bool,
> = None;
unsafe extern "C" fn pclmul_supported() -> bool {
    let mut eax: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut ebx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut ecx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut edx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if __get_cpuid(
        1 as libc::c_int as libc::c_uint,
        &mut eax,
        &mut ebx,
        &mut ecx,
        &mut edx,
    ) == 0
    {
        if cksum_debug {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                gettext(b"failed to get cpuid\0" as *const u8 as *const libc::c_char),
            );
        }
        return 0 as libc::c_int != 0;
    }
    if ecx & 0x2 as libc::c_int as libc::c_uint == 0
        || ecx & 0x10000000 as libc::c_int as libc::c_uint == 0
    {
        if cksum_debug {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                gettext(
                    b"pclmul support not detected\0" as *const u8 as *const libc::c_char,
                ),
            );
        }
        return 0 as libc::c_int != 0;
    }
    if cksum_debug {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            gettext(
                b"using pclmul hardware support\0" as *const u8 as *const libc::c_char,
            ),
        );
    }
    return 1 as libc::c_int != 0;
}
