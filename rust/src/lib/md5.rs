use ::libc;
extern "C" {
    fn MD5(
        d: *const libc::c_uchar,
        n: size_t,
        md: *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
    fn MD5_Final(md: *mut libc::c_uchar, c: *mut MD5_CTX) -> libc::c_int;
    fn MD5_Init(c: *mut MD5_CTX) -> libc::c_int;
    fn MD5_Update(
        c: *mut MD5_CTX,
        data: *const libc::c_void,
        len: size_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub unsafe extern "C" fn md5_init_ctx(mut ctx: *mut md5_ctx) {
    MD5_Init(ctx as *mut MD5_CTX);
}
