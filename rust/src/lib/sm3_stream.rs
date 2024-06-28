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
    fn sm3_init_ctx(ctx: *mut sm3_ctx);
    fn sm3_process_block(buffer: *const libc::c_void, len: size_t, ctx: *mut sm3_ctx);
    fn sm3_process_bytes(buffer: *const libc::c_void, len: size_t, ctx: *mut sm3_ctx);
    fn sm3_finish_ctx(ctx: *mut sm3_ctx, resbuf: *mut libc::c_void) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
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
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sm3_ctx {
    pub state: [uint32_t; 8],
    pub total: [uint32_t; 2],
    pub buflen: size_t,
    pub buffer: [uint32_t; 32],
}
#[no_mangle]
pub unsafe extern "C" fn sm3_stream(
    mut stream: *mut FILE,
    mut resblock: *mut libc::c_void,
) -> libc::c_int {
    let mut ctx: sm3_ctx = sm3_ctx {
        state: [0; 8],
        total: [0; 2],
        buflen: 0,
        buffer: [0; 32],
    };
    let mut sum: size_t = 0;
    let mut buffer: *mut libc::c_char = malloc(
        (32768 as libc::c_int + 72 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_char;
    if buffer.is_null() {
        return 1 as libc::c_int;
    }
    sm3_init_ctx(&mut ctx);
    's_20: loop {
        let mut n: size_t = 0;
        sum = 0 as libc::c_int as size_t;
        loop {
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
            if n == 0 as libc::c_int as libc::c_ulong {
                if ferror_unlocked(stream) != 0 {
                    free(buffer as *mut libc::c_void);
                    return 1 as libc::c_int;
                }
                break 's_20;
            } else if feof_unlocked(stream) != 0 {
                break 's_20;
            }
        }
        sm3_process_block(
            buffer as *const libc::c_void,
            32768 as libc::c_int as size_t,
            &mut ctx,
        );
    }
    if sum > 0 as libc::c_int as libc::c_ulong {
        sm3_process_bytes(buffer as *const libc::c_void, sum, &mut ctx);
    }
    sm3_finish_ctx(&mut ctx, resblock);
    free(buffer as *mut libc::c_void);
    return 0 as libc::c_int;
}
