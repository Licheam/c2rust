use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn iconv_open(
        __tocode: *const libc::c_char,
        __fromcode: *const libc::c_char,
    ) -> iconv_t;
    fn iconv(
        __cd: iconv_t,
        __inbuf: *mut *mut libc::c_char,
        __inbytesleft: *mut size_t,
        __outbuf: *mut *mut libc::c_char,
        __outbytesleft: *mut size_t,
    ) -> size_t;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn locale_charset() -> *const libc::c_char;
    fn u8_uctomb(s: *mut uint8_t, uc: ucs4_t, n: ptrdiff_t) -> libc::c_int;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
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
pub type iconv_t = *mut libc::c_void;
pub type ucs4_t = uint32_t;
pub type uint32_t = __uint32_t;
pub type uint8_t = __uint8_t;
#[no_mangle]
pub unsafe extern "C" fn fwrite_success_callback(
    mut buf: *const libc::c_char,
    mut buflen: size_t,
    mut callback_arg: *mut libc::c_void,
) -> libc::c_long {
    let mut stream: *mut FILE = callback_arg as *mut FILE;
    fwrite(
        buf as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        buflen,
        stream,
    );
    return 0 as libc::c_int as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn unicode_to_mb(
    mut code: libc::c_uint,
    mut success: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            size_t,
            *mut libc::c_void,
        ) -> libc::c_long,
    >,
    mut failure: Option::<
        unsafe extern "C" fn(
            libc::c_uint,
            *const libc::c_char,
            *mut libc::c_void,
        ) -> libc::c_long,
    >,
    mut callback_arg: *mut libc::c_void,
) -> libc::c_long {
    static mut initialized: libc::c_int = 0;
    static mut is_utf8: libc::c_int = 0;
    static mut utf8_to_local: iconv_t = 0 as *const libc::c_void as *mut libc::c_void;
    let mut inbuf: [libc::c_char; 6] = [0; 6];
    let mut count: libc::c_int = 0;
    if initialized == 0 {
        let mut charset: *const libc::c_char = locale_charset();
        is_utf8 = (strcmp(charset, b"UTF-8\0" as *const u8 as *const libc::c_char) == 0)
            as libc::c_int;
        if is_utf8 == 0 {
            utf8_to_local = iconv_open(
                charset,
                b"UTF-8\0" as *const u8 as *const libc::c_char,
            );
            if utf8_to_local == -(1 as libc::c_int) as iconv_t {
                utf8_to_local = iconv_open(
                    b"ASCII\0" as *const u8 as *const libc::c_char,
                    b"UTF-8\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        initialized = 1 as libc::c_int;
    }
    if is_utf8 == 0 {
        if utf8_to_local == -(1 as libc::c_int) as iconv_t {
            return failure
                .expect(
                    "non-null function pointer",
                )(
                code,
                b"iconv function not usable\0" as *const u8 as *const libc::c_char,
                callback_arg,
            );
        }
    }
    count = u8_uctomb(
        inbuf.as_mut_ptr() as *mut libc::c_uchar,
        code,
        ::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as ptrdiff_t,
    );
    if count < 0 as libc::c_int {
        return failure
            .expect(
                "non-null function pointer",
            )(
            code,
            b"character out of range\0" as *const u8 as *const libc::c_char,
            callback_arg,
        );
    }
    if is_utf8 == 0 {
        let mut outbuf: [libc::c_char; 25] = [0; 25];
        let mut inptr: *const libc::c_char = 0 as *const libc::c_char;
        let mut inbytesleft: size_t = 0;
        let mut outptr: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut outbytesleft: size_t = 0;
        let mut res: size_t = 0;
        inptr = inbuf.as_mut_ptr();
        inbytesleft = count as size_t;
        outptr = outbuf.as_mut_ptr();
        outbytesleft = ::core::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong;
        res = iconv(
            utf8_to_local,
            &mut inptr as *mut *const libc::c_char as *mut *mut libc::c_char,
            &mut inbytesleft,
            &mut outptr,
            &mut outbytesleft,
        );
        if inbytesleft > 0 as libc::c_int as libc::c_ulong
            || res == -(1 as libc::c_int) as size_t
            || res > 0 as libc::c_int as libc::c_ulong
                && outptr.offset_from(outbuf.as_mut_ptr()) as libc::c_long
                    == 1 as libc::c_int as libc::c_long
                && *outbuf.as_mut_ptr() as libc::c_int == '?' as i32
        {
            return failure
                .expect(
                    "non-null function pointer",
                )(code, 0 as *const libc::c_char, callback_arg);
        }
        res = iconv(
            utf8_to_local,
            0 as *mut *mut libc::c_char,
            0 as *mut size_t,
            &mut outptr,
            &mut outbytesleft,
        );
        if res == -(1 as libc::c_int) as size_t {
            return failure
                .expect(
                    "non-null function pointer",
                )(code, 0 as *const libc::c_char, callback_arg);
        }
        return success
            .expect(
                "non-null function pointer",
            )(
            outbuf.as_mut_ptr(),
            outptr.offset_from(outbuf.as_mut_ptr()) as libc::c_long as size_t,
            callback_arg,
        );
    }
    return success
        .expect(
            "non-null function pointer",
        )(inbuf.as_mut_ptr(), count as size_t, callback_arg);
}
#[no_mangle]
pub unsafe extern "C" fn print_unicode_char(
    mut stream: *mut FILE,
    mut code: libc::c_uint,
    mut exit_on_error: libc::c_int,
) {
    unicode_to_mb(
        code,
        Some(
            fwrite_success_callback
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    size_t,
                    *mut libc::c_void,
                ) -> libc::c_long,
        ),
        if exit_on_error != 0 {
            Some(
                exit_failure_callback
                    as unsafe extern "C" fn(
                        libc::c_uint,
                        *const libc::c_char,
                        *mut libc::c_void,
                    ) -> libc::c_long,
            )
        } else {
            Some(
                fallback_failure_callback
                    as unsafe extern "C" fn(
                        libc::c_uint,
                        *const libc::c_char,
                        *mut libc::c_void,
                    ) -> libc::c_long,
            )
        },
        stream as *mut libc::c_void,
    );
}
unsafe extern "C" fn exit_failure_callback(
    mut code: libc::c_uint,
    mut msg: *const libc::c_char,
    mut callback_arg: *mut libc::c_void,
) -> libc::c_long {
    if msg.is_null() {
        error(
            1 as libc::c_int,
            0 as libc::c_int,
            gettext(
                b"cannot convert U+%04X to local character set\0" as *const u8
                    as *const libc::c_char,
            ),
            code,
        );
    } else {
        error(
            1 as libc::c_int,
            0 as libc::c_int,
            gettext(
                b"cannot convert U+%04X to local character set: %s\0" as *const u8
                    as *const libc::c_char,
            ),
            code,
            gettext(msg),
        );
    }
    return -(1 as libc::c_int) as libc::c_long;
}
unsafe extern "C" fn fallback_failure_callback(
    mut code: libc::c_uint,
    mut msg: *const libc::c_char,
    mut callback_arg: *mut libc::c_void,
) -> libc::c_long {
    let mut stream: *mut FILE = callback_arg as *mut FILE;
    if code < 0x10000 as libc::c_int as libc::c_uint {
        fprintf(stream, b"\\u%04X\0" as *const u8 as *const libc::c_char, code);
    } else {
        fprintf(stream, b"\\U%08X\0" as *const u8 as *const libc::c_char, code);
    }
    return -(1 as libc::c_int) as libc::c_long;
}
