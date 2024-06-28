use ::libc;
extern "C" {
    fn SHA224_Init(c: *mut SHA256_CTX) -> libc::c_int;
    fn SHA224_Final(md: *mut libc::c_uchar, c: *mut SHA256_CTX) -> libc::c_int;
    fn SHA224(
        d: *const libc::c_uchar,
        n: size_t,
        md: *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
    fn SHA256_Init(c: *mut SHA256_CTX) -> libc::c_int;
    fn SHA256_Update(
        c: *mut SHA256_CTX,
        data: *const libc::c_void,
        len: size_t,
    ) -> libc::c_int;
    fn SHA256_Final(md: *mut libc::c_uchar, c: *mut SHA256_CTX) -> libc::c_int;
    fn SHA256(
        d: *const libc::c_uchar,
        n: size_t,
        md: *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA256state_st {
    pub h: [libc::c_uint; 8],
    pub Nl: libc::c_uint,
    pub Nh: libc::c_uint,
    pub data: [libc::c_uint; 16],
    pub num: libc::c_uint,
    pub md_len: libc::c_uint,
}
pub type SHA256_CTX = SHA256state_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha256_ctx {
    pub CTX: SHA256_CTX,
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha256_read_ctx(
    mut ctx: *const sha256_ctx,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut tmp_ctx: SHA256_CTX = *(ctx as *mut SHA256_CTX);
    SHA256_Final(res as *mut libc::c_uchar, &mut tmp_ctx);
    return res;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha256_buffer(
    mut buf: *const libc::c_char,
    mut len: size_t,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    return SHA256(buf as *const libc::c_uchar, len, res as *mut libc::c_uchar)
        as *mut libc::c_void;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha256_finish_ctx(
    mut ctx: *mut sha256_ctx,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    SHA256_Final(res as *mut libc::c_uchar, ctx as *mut SHA256_CTX);
    return res;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha256_process_block(
    mut buf: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut sha256_ctx,
) {
    sha256_process_bytes(buf, len, ctx);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha256_process_bytes(
    mut buf: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut sha256_ctx,
) {
    SHA256_Update(ctx as *mut SHA256_CTX, buf, len);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha256_init_ctx(mut ctx: *mut sha256_ctx) {
    SHA256_Init(ctx as *mut SHA256_CTX);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha224_read_ctx(
    mut ctx: *const sha256_ctx,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut tmp_ctx: SHA256_CTX = *(ctx as *mut SHA256_CTX);
    SHA224_Final(res as *mut libc::c_uchar, &mut tmp_ctx);
    return res;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha224_buffer(
    mut buf: *const libc::c_char,
    mut len: size_t,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    return SHA224(buf as *const libc::c_uchar, len, res as *mut libc::c_uchar)
        as *mut libc::c_void;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha224_finish_ctx(
    mut ctx: *mut sha256_ctx,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    SHA224_Final(res as *mut libc::c_uchar, ctx as *mut SHA256_CTX);
    return res;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha224_init_ctx(mut ctx: *mut sha256_ctx) {
    SHA224_Init(ctx as *mut SHA256_CTX);
}
