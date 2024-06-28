use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: size_t,
    ) -> libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn ferror_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn fread_unlocked(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    static mut exit_failure: libc::c_int;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
    fn getrandom(
        __buffer: *mut libc::c_void,
        __length: size_t,
        __flags: libc::c_uint,
    ) -> ssize_t;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn isaac_seed(_: *mut isaac_state);
    fn isaac_refill(_: *mut isaac_state, _: *mut isaac_word);
    fn fopen_safer(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct randread_source {
    pub source: *mut FILE,
    pub handler: Option::<unsafe extern "C" fn(*const libc::c_void) -> ()>,
    pub handler_arg: *const libc::c_void,
    pub buf: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub c: [libc::c_char; 4096],
    pub isaac: isaac,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct isaac {
    pub buffered: size_t,
    pub state: isaac_state,
    pub data: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub w: [isaac_word; 256],
    pub b: [libc::c_uchar; 2048],
}
pub type isaac_word = uint_least64_t;
pub type uint_least64_t = __uint_least64_t;
pub type __uint_least64_t = __uint64_t;
pub type __uint64_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct isaac_state {
    pub m: [isaac_word; 256],
    pub a: isaac_word,
    pub b: isaac_word,
    pub c: isaac_word,
}
pub type FILE = _IO_FILE;
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
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type __ssize_t = libc::c_long;
#[no_mangle]
pub unsafe extern "C" fn randread_new(
    mut name: *const libc::c_char,
    mut bytes_bound: size_t,
) -> *mut randread_source {
    if bytes_bound == 0 as libc::c_int as libc::c_ulong {
        return simple_new(0 as *mut FILE, 0 as *const libc::c_void)
    } else {
        let mut source: *mut FILE = 0 as *mut FILE;
        let mut s: *mut randread_source = 0 as *mut randread_source;
        if !name.is_null() {
            source = fopen_safer(name, b"rb\0" as *const u8 as *const libc::c_char);
            if source.is_null() {
                return 0 as *mut randread_source;
            }
        }
        s = simple_new(source, name as *const libc::c_void);
        if !source.is_null() {
            setvbuf(
                source,
                ((*s).buf.c).as_mut_ptr(),
                0 as libc::c_int,
                if (::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
                    < bytes_bound
                {
                    ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                } else {
                    bytes_bound
                },
            );
        } else {
            (*s).buf.isaac.buffered = 0 as libc::c_int as size_t;
            if !get_nonce(
                ((*s).buf.isaac.state.m).as_mut_ptr() as *mut libc::c_void,
                if (::core::mem::size_of::<[isaac_word; 256]>() as libc::c_ulong)
                    < bytes_bound
                {
                    ::core::mem::size_of::<[isaac_word; 256]>() as libc::c_ulong
                } else {
                    bytes_bound
                },
            ) {
                let mut e: libc::c_int = *__errno_location();
                randread_free(s);
                *__errno_location() = e;
                return 0 as *mut randread_source;
            }
            isaac_seed(&mut (*s).buf.isaac.state);
        }
        return s;
    };
}
#[no_mangle]
pub unsafe extern "C" fn randread(
    mut s: *mut randread_source,
    mut buf: *mut libc::c_void,
    mut size: size_t,
) {
    if !((*s).source).is_null() {
        readsource(s, buf as *mut libc::c_uchar, size);
    } else {
        readisaac(&mut (*s).buf.isaac, buf, size);
    };
}
#[no_mangle]
pub unsafe extern "C" fn randread_set_handler(
    mut s: *mut randread_source,
    mut handler: Option::<unsafe extern "C" fn(*const libc::c_void) -> ()>,
) {
    (*s).handler = handler;
}
#[no_mangle]
pub unsafe extern "C" fn randread_set_handler_arg(
    mut s: *mut randread_source,
    mut handler_arg: *const libc::c_void,
) {
    (*s).handler_arg = handler_arg;
}
#[no_mangle]
pub unsafe extern "C" fn randread_free(mut s: *mut randread_source) -> libc::c_int {
    let mut source: *mut FILE = (*s).source;
    explicit_bzero(
        s as *mut libc::c_void,
        ::core::mem::size_of::<randread_source>() as libc::c_ulong,
    );
    free(s as *mut libc::c_void);
    return if !source.is_null() { rpl_fclose(source) } else { 0 as libc::c_int };
}
unsafe extern "C" fn randread_error(mut file_name: *const libc::c_void) -> ! {
    if !file_name.is_null() {
        error(
            exit_failure,
            *__errno_location(),
            if *__errno_location() == 0 as libc::c_int {
                gettext(b"%s: end of file\0" as *const u8 as *const libc::c_char)
            } else {
                gettext(b"%s: read error\0" as *const u8 as *const libc::c_char)
            },
            quote(file_name as *const libc::c_char),
        );
    }
    abort();
}
unsafe extern "C" fn simple_new(
    mut source: *mut FILE,
    mut handler_arg: *const libc::c_void,
) -> *mut randread_source {
    let mut s: *mut randread_source = xmalloc(
        ::core::mem::size_of::<randread_source>() as libc::c_ulong,
    ) as *mut randread_source;
    (*s).source = source;
    (*s)
        .handler = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*const libc::c_void) -> !>,
        Option::<unsafe extern "C" fn(*const libc::c_void) -> ()>,
    >(Some(randread_error as unsafe extern "C" fn(*const libc::c_void) -> !));
    (*s).handler_arg = handler_arg;
    return s;
}
unsafe extern "C" fn get_nonce(
    mut buffer: *mut libc::c_void,
    mut bufsize: size_t,
) -> bool {
    let mut buf: *mut libc::c_char = buffer as *mut libc::c_char;
    let mut buflim: *mut libc::c_char = buf.offset(bufsize as isize);
    while buf < buflim {
        let mut nbytes: ssize_t = getrandom(
            buf as *mut libc::c_void,
            buflim.offset_from(buf) as libc::c_long as size_t,
            0 as libc::c_int as libc::c_uint,
        );
        if 0 as libc::c_int as libc::c_long <= nbytes {
            buf = buf.offset(nbytes as isize);
        } else if *__errno_location() != 4 as libc::c_int {
            return 0 as libc::c_int != 0
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn readsource(
    mut s: *mut randread_source,
    mut p: *mut libc::c_uchar,
    mut size: size_t,
) {
    loop {
        let mut inbytes: size_t = fread_unlocked(
            p as *mut libc::c_void,
            ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
            size,
            (*s).source,
        );
        let mut fread_errno: libc::c_int = *__errno_location();
        p = p.offset(inbytes as isize);
        size = (size as libc::c_ulong).wrapping_sub(inbytes) as size_t as size_t;
        if size == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        *__errno_location() = if ferror_unlocked((*s).source) != 0 {
            fread_errno
        } else {
            0 as libc::c_int
        };
        ((*s).handler).expect("non-null function pointer")((*s).handler_arg);
    };
}
unsafe extern "C" fn readisaac(
    mut isaac: *mut isaac,
    mut p: *mut libc::c_void,
    mut size: size_t,
) {
    let mut inbytes: size_t = (*isaac).buffered;
    loop {
        let mut char_p: *mut libc::c_char = p as *mut libc::c_char;
        if size <= inbytes {
            memcpy(
                p,
                ((*isaac).data.b)
                    .as_mut_ptr()
                    .offset(
                        (((1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<isaac_word>() as libc::c_ulong,
                            ) as isize,
                    )
                    .offset(-(inbytes as isize)) as *const libc::c_void,
                size,
            );
            (*isaac).buffered = inbytes.wrapping_sub(size);
            return;
        }
        memcpy(
            p,
            ((*isaac).data.b)
                .as_mut_ptr()
                .offset(
                    (((1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<isaac_word>() as libc::c_ulong,
                        ) as isize,
                )
                .offset(-(inbytes as isize)) as *const libc::c_void,
            inbytes,
        );
        p = char_p.offset(inbytes as isize) as *mut libc::c_void;
        size = (size as libc::c_ulong).wrapping_sub(inbytes) as size_t as size_t;
        if (p as size_t)
            .wrapping_rem(::core::mem::align_of::<isaac_word>() as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
        {
            let mut wp: *mut isaac_word = p as *mut isaac_word;
            while (((1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<isaac_word>() as libc::c_ulong)
                <= size
            {
                isaac_refill(&mut (*isaac).state, wp);
                wp = wp.offset(((1 as libc::c_int) << 8 as libc::c_int) as isize);
                size = (size as libc::c_ulong)
                    .wrapping_sub(
                        (((1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<isaac_word>() as libc::c_ulong,
                            ),
                    ) as size_t as size_t;
                if size == 0 as libc::c_int as libc::c_ulong {
                    (*isaac).buffered = 0 as libc::c_int as size_t;
                    return;
                }
            }
            p = wp as *mut libc::c_void;
        }
        isaac_refill(&mut (*isaac).state, ((*isaac).data.w).as_mut_ptr());
        inbytes = (((1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<isaac_word>() as libc::c_ulong);
    };
}
