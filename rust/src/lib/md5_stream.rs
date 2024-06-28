use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fread_unlocked(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn feof_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn ferror_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn MD5_Init(c: *mut MD5_CTX) -> libc::c_int;
    fn MD5_Update(
        c: *mut MD5_CTX,
        data: *const libc::c_void,
        len: size_t,
    ) -> libc::c_int;
    fn MD5_Final(md: *mut libc::c_uchar, c: *mut MD5_CTX) -> libc::c_int;
    fn MD5(
        d: *const libc::c_uchar,
        n: size_t,
        md: *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5state_st {
    pub A: libc::c_uint,
    pub B: libc::c_uint,
    pub C: libc::c_uint,
    pub D: libc::c_uint,
    pub Nl: libc::c_uint,
    pub Nh: libc::c_uint,
    pub data: [libc::c_uint; 16],
    pub num: libc::c_uint,
}
pub type MD5_CTX = MD5state_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md5_ctx {
    pub CTX: MD5_CTX,
}
#[no_mangle]
pub unsafe extern "C" fn md5_stream(
    mut stream: *mut FILE,
    mut resblock: *mut libc::c_void,
) -> libc::c_int {
    match afalg_stream(
        stream,
        b"md5\0" as *const u8 as *const libc::c_char,
        resblock,
        16 as libc::c_int as ssize_t,
    ) {
        0 => return 0 as libc::c_int,
        -5 => return 1 as libc::c_int,
        _ => {}
    }
    let mut buffer: *mut libc::c_char = malloc(
        (32768 as libc::c_int + 72 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_char;
    if buffer.is_null() {
        return 1 as libc::c_int;
    }
    let mut ctx: md5_ctx = md5_ctx {
        CTX: MD5_CTX {
            A: 0,
            B: 0,
            C: 0,
            D: 0,
            Nl: 0,
            Nh: 0,
            data: [0; 16],
            num: 0,
        },
    };
    md5_init_ctx(&mut ctx);
    let mut sum: size_t = 0;
    's_34: loop {
        let mut n: size_t = 0;
        sum = 0 as libc::c_int as size_t;
        loop {
            if feof_unlocked(stream) != 0 {
                break 's_34;
            }
            n = fread_unlocked(
                buffer.offset(sum as isize) as *mut libc::c_void,
                1 as libc::c_int as size_t,
                (32768 as libc::c_int as libc::c_ulong).wrapping_sub(sum),
                stream,
            );
            sum = (sum as libc::c_ulong).wrapping_add(n) as size_t as size_t;
            if sum == 32768 as libc::c_int as libc::c_ulong {
                break;
            }
            if !(n == 0 as libc::c_int as libc::c_ulong) {
                continue;
            }
            if ferror_unlocked(stream) != 0 {
                free(buffer as *mut libc::c_void);
                return 1 as libc::c_int;
            }
            break 's_34;
        }
        md5_process_block(
            buffer as *const libc::c_void,
            32768 as libc::c_int as size_t,
            &mut ctx,
        );
    }
    if sum > 0 as libc::c_int as libc::c_ulong {
        md5_process_bytes(buffer as *const libc::c_void, sum, &mut ctx);
    }
    md5_finish_ctx(&mut ctx, resblock);
    free(buffer as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn md5_init_ctx(mut ctx: *mut md5_ctx) {
    MD5_Init(ctx as *mut MD5_CTX);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn md5_process_bytes(
    mut buf: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut md5_ctx,
) {
    MD5_Update(ctx as *mut MD5_CTX, buf, len);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn md5_process_block(
    mut buf: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut md5_ctx,
) {
    md5_process_bytes(buf, len, ctx);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn md5_finish_ctx(
    mut ctx: *mut md5_ctx,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    MD5_Final(res as *mut libc::c_uchar, ctx as *mut MD5_CTX);
    return res;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn md5_buffer(
    mut buf: *const libc::c_char,
    mut len: size_t,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    return MD5(buf as *const libc::c_uchar, len, res as *mut libc::c_uchar)
        as *mut libc::c_void;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn md5_read_ctx(
    mut ctx: *const md5_ctx,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut tmp_ctx: MD5_CTX = *(ctx as *mut MD5_CTX);
    MD5_Final(res as *mut libc::c_uchar, &mut tmp_ctx);
    return res;
}
#[inline]
unsafe extern "C" fn afalg_stream(
    mut stream: *mut FILE,
    mut alg: *const libc::c_char,
    mut resblock: *mut libc::c_void,
    mut hashlen: ssize_t,
) -> libc::c_int {
    return -(97 as libc::c_int);
}
