use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn savewd_chdir(
        wd: *mut savewd,
        dir: *const libc::c_char,
        options: libc::c_int,
        open_result: *mut libc::c_int,
    ) -> libc::c_int;
}
pub type ptrdiff_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct savewd {
    pub state: C2RustUnnamed_0,
    pub val: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub fd: libc::c_int,
    pub errnum: libc::c_int,
    pub child: pid_t,
}
pub type pid_t = __pid_t;
pub type __pid_t = libc::c_int;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const FINAL_STATE: C2RustUnnamed_0 = 5;
pub const ERROR_STATE: C2RustUnnamed_0 = 4;
pub const FORKING_STATE: C2RustUnnamed_0 = 3;
pub const FD_POST_CHDIR_STATE: C2RustUnnamed_0 = 2;
pub const FD_STATE: C2RustUnnamed_0 = 1;
pub const INITIAL_STATE: C2RustUnnamed_0 = 0;
pub const SAVEWD_CHDIR_NOFOLLOW: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const SAVEWD_CHDIR_SKIP_READABLE: C2RustUnnamed_1 = 2;
#[no_mangle]
pub unsafe extern "C" fn mkancesdirs(
    mut file: *mut libc::c_char,
    mut wd: *mut savewd,
    mut make_dir: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    mut make_dir_arg: *mut libc::c_void,
) -> ptrdiff_t {
    let mut sep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut component: *mut libc::c_char = file;
    let mut p: *mut libc::c_char = file.offset(0 as libc::c_int as isize);
    let mut c: libc::c_char = 0;
    let mut made_dir: bool = 0 as libc::c_int != 0;
    loop {
        let fresh0 = p;
        p = p.offset(1);
        c = *fresh0;
        if !(c != 0) {
            break;
        }
        if *p as libc::c_int == '/' as i32 {
            if !(c as libc::c_int == '/' as i32) {
                sep = p;
            }
        } else if c as libc::c_int == '/' as i32 && *p as libc::c_int != 0
            && !sep.is_null()
        {
            if !(sep.offset_from(component) as libc::c_long
                == 1 as libc::c_int as libc::c_long
                && *component.offset(0 as libc::c_int as isize) as libc::c_int
                    == '.' as i32)
            {
                let mut make_dir_errno: libc::c_int = 0 as libc::c_int;
                let mut savewd_chdir_options: libc::c_int = 0 as libc::c_int;
                let mut chdir_result: libc::c_int = 0;
                *sep = '\0' as i32 as libc::c_char;
                if sep.offset_from(component) as libc::c_long
                    == 2 as libc::c_int as libc::c_long
                    && *component.offset(0 as libc::c_int as isize) as libc::c_int
                        == '.' as i32
                    && *component.offset(1 as libc::c_int as isize) as libc::c_int
                        == '.' as i32
                {
                    made_dir = 0 as libc::c_int != 0;
                } else if make_dir
                    .expect("non-null function pointer")(file, component, make_dir_arg)
                    < 0 as libc::c_int
                {
                    make_dir_errno = *__errno_location();
                } else {
                    made_dir = 1 as libc::c_int != 0;
                }
                if made_dir {
                    savewd_chdir_options |= SAVEWD_CHDIR_NOFOLLOW as libc::c_int;
                }
                chdir_result = savewd_chdir(
                    wd,
                    component,
                    savewd_chdir_options,
                    0 as *mut libc::c_int,
                );
                if chdir_result != -(1 as libc::c_int) {
                    *sep = '/' as i32 as libc::c_char;
                }
                if chdir_result != 0 as libc::c_int {
                    if make_dir_errno != 0 as libc::c_int
                        && *__errno_location() == 2 as libc::c_int
                    {
                        *__errno_location() = make_dir_errno;
                    }
                    return chdir_result as ptrdiff_t;
                }
            }
            component = p;
        }
    }
    return component.offset_from(file) as libc::c_long;
}
