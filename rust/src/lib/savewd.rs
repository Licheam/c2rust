use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn fchdir(__fd: libc::c_int) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn fork() -> __pid_t;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn open_safer(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
}
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const FINAL_STATE: C2RustUnnamed_0 = 5;
pub const ERROR_STATE: C2RustUnnamed_0 = 4;
pub const FORKING_STATE: C2RustUnnamed_0 = 3;
pub const FD_POST_CHDIR_STATE: C2RustUnnamed_0 = 2;
pub const FD_STATE: C2RustUnnamed_0 = 1;
pub const INITIAL_STATE: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const SAVEWD_CHDIR_SKIP_READABLE: C2RustUnnamed_1 = 2;
pub const SAVEWD_CHDIR_NOFOLLOW: C2RustUnnamed_1 = 1;
#[no_mangle]
pub unsafe extern "C" fn savewd_process_files(
    mut n_files: libc::c_int,
    mut file: *mut *mut libc::c_char,
    mut act: Option::<
        unsafe extern "C" fn(
            *mut libc::c_char,
            *mut savewd,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    mut options: *mut libc::c_void,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut last_relative: libc::c_int = 0;
    let mut exit_status: libc::c_int = 0 as libc::c_int;
    let mut wd: savewd = savewd {
        state: INITIAL_STATE,
        val: C2RustUnnamed { fd: 0 },
    };
    savewd_init(&mut wd);
    last_relative = n_files - 1 as libc::c_int;
    while 0 as libc::c_int <= last_relative {
        if !(*(*file.offset(last_relative as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int == '/' as i32)
        {
            break;
        }
        last_relative -= 1;
        last_relative;
    }
    while i < last_relative {
        if !savewd_delegating(&mut wd) {
            let mut s: libc::c_int = act
                .expect(
                    "non-null function pointer",
                )(*file.offset(i as isize), &mut wd, options);
            if exit_status < s {
                exit_status = s;
            }
        }
        if !(*(*file.offset((i + 1 as libc::c_int) as isize))
            .offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32)
        {
            let mut r: libc::c_int = savewd_restore(&mut wd, exit_status);
            if exit_status < r {
                exit_status = r;
            }
        }
        i += 1;
        i;
    }
    savewd_finish(&mut wd);
    while i < n_files {
        let mut s_0: libc::c_int = act
            .expect(
                "non-null function pointer",
            )(*file.offset(i as isize), &mut wd, options);
        if exit_status < s_0 {
            exit_status = s_0;
        }
        i += 1;
        i;
    }
    return exit_status;
}
#[no_mangle]
pub unsafe extern "C" fn savewd_finish(mut wd: *mut savewd) {
    match (*wd).state as libc::c_uint {
        0 | 4 => {}
        1 | 2 => {
            close((*wd).val.fd);
        }
        3 => {
            if (*wd).val.child < 0 as libc::c_int {} else {
                __assert_fail(
                    b"wd->val.child < 0\0" as *const u8 as *const libc::c_char,
                    b"lib/savewd.c\0" as *const u8 as *const libc::c_char,
                    240 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 36],
                        &[libc::c_char; 36],
                    >(b"void savewd_finish(struct savewd *)\0"))
                        .as_ptr(),
                );
            }
            'c_1556: {
                if (*wd).val.child < 0 as libc::c_int {} else {
                    __assert_fail(
                        b"wd->val.child < 0\0" as *const u8 as *const libc::c_char,
                        b"lib/savewd.c\0" as *const u8 as *const libc::c_char,
                        240 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 36],
                            &[libc::c_char; 36],
                        >(b"void savewd_finish(struct savewd *)\0"))
                            .as_ptr(),
                    );
                }
            };
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"lib/savewd.c\0" as *const u8 as *const libc::c_char,
                244 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void savewd_finish(struct savewd *)\0"))
                    .as_ptr(),
            );
            'c_1523: {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"lib/savewd.c\0" as *const u8 as *const libc::c_char,
                    244 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 36],
                        &[libc::c_char; 36],
                    >(b"void savewd_finish(struct savewd *)\0"))
                        .as_ptr(),
                );
            };
        }
    }
    (*wd).state = FINAL_STATE;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn savewd_errno(mut wd: *const savewd) -> libc::c_int {
    return if (*wd).state as libc::c_uint == ERROR_STATE as libc::c_int as libc::c_uint {
        (*wd).val.errnum
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn savewd_init(mut wd: *mut savewd) {
    (*wd).state = INITIAL_STATE;
}
#[no_mangle]
pub unsafe extern "C" fn savewd_chdir(
    mut wd: *mut savewd,
    mut dir: *const libc::c_char,
    mut options: libc::c_int,
    mut open_result: *mut libc::c_int,
) -> libc::c_int {
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut result: libc::c_int = 0 as libc::c_int;
    if !open_result.is_null()
        || options
            & (if 1 as libc::c_int != 0 {
                SAVEWD_CHDIR_NOFOLLOW as libc::c_int
            } else {
                0 as libc::c_int
            }) != 0
    {
        fd = open(
            dir,
            0 as libc::c_int | 0o200000 as libc::c_int | 0o400 as libc::c_int
                | 0o4000 as libc::c_int
                | (if options & SAVEWD_CHDIR_NOFOLLOW as libc::c_int != 0 {
                    0o400000 as libc::c_int
                } else {
                    0 as libc::c_int
                }),
        );
        if !open_result.is_null() {
            *open_result.offset(0 as libc::c_int as isize) = fd;
            *open_result.offset(1 as libc::c_int as isize) = *__errno_location();
        }
        if fd < 0 as libc::c_int && *__errno_location() != 13 as libc::c_int {
            result = -(1 as libc::c_int);
        }
    }
    if result == 0 as libc::c_int
        && !(0 as libc::c_int <= fd
            && options & SAVEWD_CHDIR_SKIP_READABLE as libc::c_int != 0)
    {
        if savewd_save(wd) {
            open_result = 0 as *mut libc::c_int;
            result = -(2 as libc::c_int);
        } else {
            result = if fd < 0 as libc::c_int { chdir(dir) } else { fchdir(fd) };
            if result == 0 as libc::c_int {
                match (*wd).state as libc::c_uint {
                    1 => {
                        (*wd).state = FD_POST_CHDIR_STATE;
                    }
                    4 | 2 | 5 => {}
                    3 => {
                        if (*wd).val.child == 0 as libc::c_int {} else {
                            __assert_fail(
                                b"wd->val.child == 0\0" as *const u8 as *const libc::c_char,
                                b"lib/savewd.c\0" as *const u8 as *const libc::c_char,
                                148 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 60],
                                    &[libc::c_char; 60],
                                >(
                                    b"int savewd_chdir(struct savewd *, const char *, int, int *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_747: {
                            if (*wd).val.child == 0 as libc::c_int {} else {
                                __assert_fail(
                                    b"wd->val.child == 0\0" as *const u8 as *const libc::c_char,
                                    b"lib/savewd.c\0" as *const u8 as *const libc::c_char,
                                    148 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 60],
                                        &[libc::c_char; 60],
                                    >(
                                        b"int savewd_chdir(struct savewd *, const char *, int, int *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                    }
                    _ => {
                        __assert_fail(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"lib/savewd.c\0" as *const u8 as *const libc::c_char,
                            152 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 60],
                                &[libc::c_char; 60],
                            >(
                                b"int savewd_chdir(struct savewd *, const char *, int, int *)\0",
                            ))
                                .as_ptr(),
                        );
                        'c_704: {
                            __assert_fail(
                                b"0\0" as *const u8 as *const libc::c_char,
                                b"lib/savewd.c\0" as *const u8 as *const libc::c_char,
                                152 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 60],
                                    &[libc::c_char; 60],
                                >(
                                    b"int savewd_chdir(struct savewd *, const char *, int, int *)\0",
                                ))
                                    .as_ptr(),
                            );
                        };
                    }
                }
            }
        }
    }
    if 0 as libc::c_int <= fd && open_result.is_null() {
        let mut e: libc::c_int = *__errno_location();
        close(fd);
        *__errno_location() = e;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn savewd_restore(
    mut wd: *mut savewd,
    mut status: libc::c_int,
) -> libc::c_int {
    let mut current_block_19: u64;
    match (*wd).state as libc::c_uint {
        0 | 1 => {
            current_block_19 = 26972500619410423;
        }
        2 => {
            if fchdir((*wd).val.fd) == 0 as libc::c_int {
                (*wd).state = FD_STATE;
                current_block_19 = 26972500619410423;
            } else {
                let mut chdir_errno: libc::c_int = *__errno_location();
                close((*wd).val.fd);
                (*wd).state = ERROR_STATE;
                (*wd).val.errnum = chdir_errno;
                current_block_19 = 11095089989504814202;
            }
        }
        4 => {
            current_block_19 = 11095089989504814202;
        }
        3 => {
            let mut child: pid_t = (*wd).val.child;
            if child == 0 as libc::c_int {
                _exit(status);
            }
            if (0 as libc::c_int) < child {
                let mut child_status: libc::c_int = 0;
                while waitpid(child, &mut child_status, 0 as libc::c_int)
                    < 0 as libc::c_int
                {
                    if *__errno_location() == 4 as libc::c_int {} else {
                        __assert_fail(
                            b"(*__errno_location ()) == 4\0" as *const u8
                                as *const libc::c_char,
                            b"lib/savewd.c\0" as *const u8 as *const libc::c_char,
                            209 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 41],
                                &[libc::c_char; 41],
                            >(b"int savewd_restore(struct savewd *, int)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_1310: {
                        if *__errno_location() == 4 as libc::c_int {} else {
                            __assert_fail(
                                b"(*__errno_location ()) == 4\0" as *const u8
                                    as *const libc::c_char,
                                b"lib/savewd.c\0" as *const u8 as *const libc::c_char,
                                209 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 41],
                                    &[libc::c_char; 41],
                                >(b"int savewd_restore(struct savewd *, int)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                }
                (*wd).val.child = -(1 as libc::c_int);
                if !(child_status & 0x7f as libc::c_int == 0 as libc::c_int) {
                    raise(child_status & 0x7f as libc::c_int);
                }
                return (child_status & 0xff00 as libc::c_int) >> 8 as libc::c_int;
            }
            current_block_19 = 26972500619410423;
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"lib/savewd.c\0" as *const u8 as *const libc::c_char,
                219 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"int savewd_restore(struct savewd *, int)\0"))
                    .as_ptr(),
            );
            'c_1220: {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"lib/savewd.c\0" as *const u8 as *const libc::c_char,
                    219 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 41],
                        &[libc::c_char; 41],
                    >(b"int savewd_restore(struct savewd *, int)\0"))
                        .as_ptr(),
                );
            };
            current_block_19 = 26972500619410423;
        }
    }
    match current_block_19 {
        26972500619410423 => {}
        _ => {
            *__errno_location() = (*wd).val.errnum;
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn savewd_save(mut wd: *mut savewd) -> bool {
    let mut current_block_16: u64;
    match (*wd).state as libc::c_uint {
        0 => {
            let mut fd: libc::c_int = open_safer(
                b".\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
            if 0 as libc::c_int <= fd {
                (*wd).state = FD_STATE;
                (*wd).val.fd = fd;
                current_block_16 = 13242334135786603907;
            } else if *__errno_location() != 13 as libc::c_int
                && *__errno_location() != 116 as libc::c_int
            {
                (*wd).state = ERROR_STATE;
                (*wd).val.errnum = *__errno_location();
                current_block_16 = 13242334135786603907;
            } else {
                (*wd).state = FORKING_STATE;
                (*wd).val.child = -(1 as libc::c_int);
                current_block_16 = 13895751885136125791;
            }
        }
        3 => {
            current_block_16 = 13895751885136125791;
        }
        1 | 2 | 4 | 5 => {
            current_block_16 = 13242334135786603907;
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"lib/savewd.c\0" as *const u8 as *const libc::c_char,
                92 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"_Bool savewd_save(struct savewd *)\0"))
                    .as_ptr(),
            );
            'c_884: {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"lib/savewd.c\0" as *const u8 as *const libc::c_char,
                    92 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 35],
                        &[libc::c_char; 35],
                    >(b"_Bool savewd_save(struct savewd *)\0"))
                        .as_ptr(),
                );
            };
            current_block_16 = 13242334135786603907;
        }
    }
    match current_block_16 {
        13895751885136125791 => {
            if (*wd).val.child < 0 as libc::c_int {
                (*wd).val.child = fork();
                if (*wd).val.child != 0 as libc::c_int {
                    if (0 as libc::c_int) < (*wd).val.child {
                        return 1 as libc::c_int != 0;
                    }
                    (*wd).state = ERROR_STATE;
                    (*wd).val.errnum = *__errno_location();
                }
            }
        }
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn savewd_delegating(mut wd: *const savewd) -> bool {
    return (*wd).state as libc::c_uint == FORKING_STATE as libc::c_int as libc::c_uint
        && (0 as libc::c_int) < (*wd).val.child;
}
