use ::libc;
extern "C" {
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
}
pub type __socklen_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type sa_family_t = libc::c_ushort;
pub type socklen_t = __socklen_t;
#[no_mangle]
pub unsafe extern "C" fn ch_strerror() -> *const libc::c_char {
    return gai_strerror(last_cherror);
}
#[no_mangle]
pub unsafe extern "C" fn canon_host_r(
    mut host: *const libc::c_char,
    mut cherror: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut retval: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *const sockaddr as *mut sockaddr,
        ai_canonname: 0 as *const libc::c_char as *mut libc::c_char,
        ai_next: 0 as *const addrinfo as *mut addrinfo,
    };
    let mut res: *mut addrinfo = 0 as *mut addrinfo;
    let mut status: libc::c_int = 0;
    hints.ai_flags = 0x2 as libc::c_int;
    status = getaddrinfo(host, 0 as *const libc::c_char, &mut hints, &mut res);
    if status == 0 {
        retval = strdup(
            if !((*res).ai_canonname).is_null() {
                (*res).ai_canonname as *const libc::c_char
            } else {
                host
            },
        );
        if retval.is_null() && !cherror.is_null() {
            *cherror = -(10 as libc::c_int);
        }
        freeaddrinfo(res);
    } else if !cherror.is_null() {
        *cherror = status;
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn canon_host(mut host: *const libc::c_char) -> *mut libc::c_char {
    return canon_host_r(host, &mut last_cherror);
}
static mut last_cherror: libc::c_int = 0;
