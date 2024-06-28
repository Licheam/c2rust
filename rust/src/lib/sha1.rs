use ::libc;
extern "C" {
    fn SHA1_Init(c: *mut SHA_CTX) -> libc::c_int;
    fn SHA1_Update(
        c: *mut SHA_CTX,
        data: *const libc::c_void,
        len: size_t,
    ) -> libc::c_int;
    fn SHA1_Final(md: *mut libc::c_uchar, c: *mut SHA_CTX) -> libc::c_int;
    fn SHA1(
        d: *const libc::c_uchar,
        n: size_t,
        md: *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHAstate_st {
    pub h0: libc::c_uint,
    pub h1: libc::c_uint,
    pub h2: libc::c_uint,
    pub h3: libc::c_uint,
    pub h4: libc::c_uint,
    pub Nl: libc::c_uint,
    pub Nh: libc::c_uint,
    pub data: [libc::c_uint; 16],
    pub num: libc::c_uint,
}
pub type SHA_CTX = SHAstate_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha1_ctx {
    pub CTX: SHA_CTX,
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha1_init_ctx(mut ctx: *mut sha1_ctx) {
    SHA1_Init(ctx as *mut SHA_CTX);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha1_process_bytes(
    mut buf: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut sha1_ctx,
) {
    SHA1_Update(ctx as *mut SHA_CTX, buf, len);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha1_process_block(
    mut buf: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut sha1_ctx,
) {
    sha1_process_bytes(buf, len, ctx);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha1_finish_ctx(
    mut ctx: *mut sha1_ctx,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    SHA1_Final(res as *mut libc::c_uchar, ctx as *mut SHA_CTX);
    return res;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha1_buffer(
    mut buf: *const libc::c_char,
    mut len: size_t,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    return SHA1(buf as *const libc::c_uchar, len, res as *mut libc::c_uchar)
        as *mut libc::c_void;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha1_read_ctx(
    mut ctx: *const sha1_ctx,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut tmp_ctx: SHA_CTX = *(ctx as *mut SHA_CTX);
    SHA1_Final(res as *mut libc::c_uchar, &mut tmp_ctx);
    return res;
}
