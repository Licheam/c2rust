use ::libc;
extern "C" {
    fn SHA384_Init(c: *mut SHA512_CTX) -> libc::c_int;
    fn SHA384_Final(md: *mut libc::c_uchar, c: *mut SHA512_CTX) -> libc::c_int;
    fn SHA384(
        d: *const libc::c_uchar,
        n: size_t,
        md: *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
    fn SHA512_Init(c: *mut SHA512_CTX) -> libc::c_int;
    fn SHA512_Update(
        c: *mut SHA512_CTX,
        data: *const libc::c_void,
        len: size_t,
    ) -> libc::c_int;
    fn SHA512_Final(md: *mut libc::c_uchar, c: *mut SHA512_CTX) -> libc::c_int;
    fn SHA512(
        d: *const libc::c_uchar,
        n: size_t,
        md: *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA512state_st {
    pub h: [libc::c_ulonglong; 8],
    pub Nl: libc::c_ulonglong,
    pub Nh: libc::c_ulonglong,
    pub u: C2RustUnnamed,
    pub num: libc::c_uint,
    pub md_len: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub d: [libc::c_ulonglong; 16],
    pub p: [libc::c_uchar; 128],
}
pub type SHA512_CTX = SHA512state_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha512_ctx {
    pub CTX: SHA512_CTX,
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha512_read_ctx(
    mut ctx: *const sha512_ctx,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut tmp_ctx: SHA512_CTX = *(ctx as *mut SHA512_CTX);
    SHA512_Final(res as *mut libc::c_uchar, &mut tmp_ctx);
    return res;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha512_buffer(
    mut buf: *const libc::c_char,
    mut len: size_t,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    return SHA512(buf as *const libc::c_uchar, len, res as *mut libc::c_uchar)
        as *mut libc::c_void;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha384_init_ctx(mut ctx: *mut sha512_ctx) {
    SHA384_Init(ctx as *mut SHA512_CTX);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha384_finish_ctx(
    mut ctx: *mut sha512_ctx,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    SHA384_Final(res as *mut libc::c_uchar, ctx as *mut SHA512_CTX);
    return res;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha384_buffer(
    mut buf: *const libc::c_char,
    mut len: size_t,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    return SHA384(buf as *const libc::c_uchar, len, res as *mut libc::c_uchar)
        as *mut libc::c_void;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha384_read_ctx(
    mut ctx: *const sha512_ctx,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut tmp_ctx: SHA512_CTX = *(ctx as *mut SHA512_CTX);
    SHA384_Final(res as *mut libc::c_uchar, &mut tmp_ctx);
    return res;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha512_init_ctx(mut ctx: *mut sha512_ctx) {
    SHA512_Init(ctx as *mut SHA512_CTX);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha512_process_bytes(
    mut buf: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut sha512_ctx,
) {
    SHA512_Update(ctx as *mut SHA512_CTX, buf, len);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha512_process_block(
    mut buf: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut sha512_ctx,
) {
    sha512_process_bytes(buf, len, ctx);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn sha512_finish_ctx(
    mut ctx: *mut sha512_ctx,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    SHA512_Final(res as *mut libc::c_uchar, ctx as *mut SHA512_CTX);
    return res;
}
