use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn clearerr_unlocked(__stream: *mut FILE);
    fn ferror_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn x2nrealloc(p: *mut libc::c_void, pn: *mut size_t, s: size_t) -> *mut libc::c_void;
    fn ximemdup0(p: *const libc::c_void, s: idx_t) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn fadvise(fp: *mut FILE, advice: fadvice_t);
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uintmax_t = libc::c_ulong;
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
pub type ptrdiff_t = libc::c_long;
pub type uintmax_t = __uintmax_t;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type idx_t = ptrdiff_t;
pub type quoting_style = libc::c_uint;
pub const custom_quoting_style: quoting_style = 10;
pub const clocale_quoting_style: quoting_style = 9;
pub const locale_quoting_style: quoting_style = 8;
pub const escape_quoting_style: quoting_style = 7;
pub const c_maybe_quoting_style: quoting_style = 6;
pub const c_quoting_style: quoting_style = 5;
pub const shell_escape_always_quoting_style: quoting_style = 4;
pub const shell_escape_quoting_style: quoting_style = 3;
pub const shell_always_quoting_style: quoting_style = 2;
pub const shell_quoting_style: quoting_style = 1;
pub const literal_quoting_style: quoting_style = 0;
pub type fadvice_t = libc::c_uint;
pub const FADVISE_RANDOM: fadvice_t = 1;
pub const FADVISE_WILLNEED: fadvice_t = 3;
pub const FADVISE_DONTNEED: fadvice_t = 4;
pub const FADVISE_NOREUSE: fadvice_t = 5;
pub const FADVISE_SEQUENTIAL: fadvice_t = 2;
pub const FADVISE_NORMAL: fadvice_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub _gl_dummy: libc::c_int,
}
#[inline]
unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
}
#[no_mangle]
pub static mut convert_entire_line: bool = 0 as libc::c_int != 0;
#[no_mangle]
pub static mut max_column_width: size_t = 0;
#[no_mangle]
pub static mut exit_status: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn add_tab_stop(mut tabval: uintmax_t) {
    let mut prev_column: uintmax_t = if first_free_tab != 0 {
        *tab_list
            .offset(
                first_free_tab.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            )
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let mut column_width: uintmax_t = if prev_column <= tabval {
        tabval.wrapping_sub(prev_column)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    if first_free_tab == n_tabs_allocated {
        tab_list = (if ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
            x2nrealloc(
                tab_list as *mut libc::c_void,
                &mut n_tabs_allocated,
                ::core::mem::size_of::<uintmax_t>() as libc::c_ulong,
            )
        } else {
            x2nrealloc(
                tab_list as *mut libc::c_void,
                &mut n_tabs_allocated,
                ::core::mem::size_of::<uintmax_t>() as libc::c_ulong,
            )
        }) as *mut uintmax_t;
    }
    let fresh0 = first_free_tab;
    first_free_tab = first_free_tab.wrapping_add(1);
    *tab_list.offset(fresh0 as isize) = tabval;
    if max_column_width < column_width {
        if (18446744073709551615 as libc::c_ulong) < column_width {
            if ::core::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"tabs are too far apart\0" as *const u8 as *const libc::c_char,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"tabs are too far apart\0" as *const u8 as *const libc::c_char,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        max_column_width = column_width;
    }
}
#[no_mangle]
pub unsafe extern "C" fn parse_tab_stops(mut stops: *const libc::c_char) {
    let mut have_tabval: bool = 0 as libc::c_int != 0;
    let mut tabval: uintmax_t = 0 as libc::c_int as uintmax_t;
    let mut extend_tabval: bool = 0 as libc::c_int != 0;
    let mut increment_tabval: bool = 0 as libc::c_int != 0;
    let mut num_start: *const libc::c_char = 0 as *const libc::c_char;
    let mut ok: bool = 1 as libc::c_int != 0;
    while *stops != 0 {
        if *stops as libc::c_int == ',' as i32
            || *(*__ctype_b_loc()).offset(to_uchar(*stops) as libc::c_int as isize)
                as libc::c_int & _ISblank as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            if have_tabval {
                if extend_tabval {
                    if !set_extend_size(tabval) {
                        ok = 0 as libc::c_int != 0;
                        break;
                    }
                } else if increment_tabval {
                    if !set_increment_size(tabval) {
                        ok = 0 as libc::c_int != 0;
                        break;
                    }
                } else {
                    add_tab_stop(tabval);
                }
            }
            have_tabval = 0 as libc::c_int != 0;
        } else if *stops as libc::c_int == '/' as i32 {
            if have_tabval {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"'/' specifier not at start of number: %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quote(stops),
                );
                ok = 0 as libc::c_int != 0;
            }
            extend_tabval = 1 as libc::c_int != 0;
            increment_tabval = 0 as libc::c_int != 0;
        } else if *stops as libc::c_int == '+' as i32 {
            if have_tabval {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"'+' specifier not at start of number: %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quote(stops),
                );
                ok = 0 as libc::c_int != 0;
            }
            increment_tabval = 1 as libc::c_int != 0;
            extend_tabval = 0 as libc::c_int != 0;
        } else if (*stops as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
            <= 9 as libc::c_int as libc::c_uint
        {
            if !have_tabval {
                tabval = 0 as libc::c_int as uintmax_t;
                have_tabval = 1 as libc::c_int != 0;
                num_start = stops;
            }
            &mut tabval as *mut uintmax_t;
            if (if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
                (if (-(1 as libc::c_int) as uintmax_t)
                    .wrapping_div(10 as libc::c_int as libc::c_ulong) < tabval
                    || tabval
                        .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            (*stops as libc::c_int - '0' as i32) as libc::c_ulong,
                        ) < tabval
                {
                    0 as libc::c_int
                } else {
                    tabval = tabval
                        .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            (*stops as libc::c_int - '0' as i32) as libc::c_ulong,
                        );
                    1 as libc::c_int
                })
            } else {
                (if (-(1 as libc::c_int) as uintmax_t)
                    .wrapping_div(10 as libc::c_int as libc::c_ulong) < tabval
                    || tabval
                        .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            (*stops as libc::c_int - '0' as i32) as libc::c_ulong,
                        ) < tabval
                {
                    0 as libc::c_int
                } else {
                    tabval = tabval
                        .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            (*stops as libc::c_int - '0' as i32) as libc::c_ulong,
                        );
                    1 as libc::c_int
                })
            }) == 0
            {
                let mut len: size_t = strspn(
                    num_start,
                    b"0123456789\0" as *const u8 as *const libc::c_char,
                );
                let mut bad_num: *mut libc::c_char = ximemdup0(
                    num_start as *const libc::c_void,
                    len as idx_t,
                );
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"tab stop is too large %s\0" as *const u8 as *const libc::c_char,
                    ),
                    quote(bad_num),
                );
                free(bad_num as *mut libc::c_void);
                ok = 0 as libc::c_int != 0;
                stops = num_start
                    .offset(len as isize)
                    .offset(-(1 as libc::c_int as isize));
            }
        } else {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"tab size contains invalid character(s): %s\0" as *const u8
                        as *const libc::c_char,
                ),
                quote(stops),
            );
            ok = 0 as libc::c_int != 0;
            break;
        }
        stops = stops.offset(1);
        stops;
    }
    if ok as libc::c_int != 0 && have_tabval as libc::c_int != 0 {
        if extend_tabval {
            ok = (ok as libc::c_int & set_extend_size(tabval) as libc::c_int) as bool;
        } else if increment_tabval {
            ok = (ok as libc::c_int & set_increment_size(tabval) as libc::c_int) as bool;
        } else {
            add_tab_stop(tabval);
        }
    }
    if !ok {
        exit(1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_next_tab_column(
    column: uintmax_t,
    mut tab_index: *mut size_t,
    mut last_tab: *mut bool,
) -> uintmax_t {
    *last_tab = 0 as libc::c_int != 0;
    if tab_size != 0 {
        return column.wrapping_add(tab_size.wrapping_sub(column.wrapping_rem(tab_size)));
    }
    while *tab_index < first_free_tab {
        let mut tab: uintmax_t = *tab_list.offset(*tab_index as isize);
        if column < tab {
            return tab;
        }
        *tab_index = (*tab_index).wrapping_add(1);
        *tab_index;
    }
    if extend_size != 0 {
        return column
            .wrapping_add(extend_size.wrapping_sub(column.wrapping_rem(extend_size)));
    }
    if increment_size != 0 {
        let mut end_tab: uintmax_t = *tab_list
            .offset(
                first_free_tab.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            );
        return column
            .wrapping_add(
                increment_size
                    .wrapping_sub(
                        column.wrapping_sub(end_tab).wrapping_rem(increment_size),
                    ),
            );
    }
    *last_tab = 1 as libc::c_int != 0;
    return 0 as libc::c_int as uintmax_t;
}
#[no_mangle]
pub unsafe extern "C" fn finalize_tab_stops() {
    validate_tab_stops(tab_list, first_free_tab);
    if first_free_tab == 0 as libc::c_int as libc::c_ulong {
        max_column_width = if extend_size != 0 {
            extend_size
        } else if increment_size != 0 {
            increment_size
        } else {
            8 as libc::c_int as libc::c_ulong
        };
        tab_size = max_column_width;
    } else if first_free_tab == 1 as libc::c_int as libc::c_ulong && extend_size == 0
        && increment_size == 0
    {
        tab_size = *tab_list.offset(0 as libc::c_int as isize);
    } else {
        tab_size = 0 as libc::c_int as uintmax_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn set_file_list(mut list: *mut *mut libc::c_char) {
    have_read_stdin = 0 as libc::c_int != 0;
    if list.is_null() {
        file_list = stdin_argv.as_mut_ptr();
    } else {
        file_list = list;
    };
}
#[no_mangle]
pub unsafe extern "C" fn next_file(mut fp: *mut FILE) -> *mut FILE {
    static mut prev_file: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    let mut file: *mut libc::c_char = 0 as *mut libc::c_char;
    if !fp.is_null() {
        if !prev_file.is_null() {} else {
            __assert_fail(
                b"prev_file\0" as *const u8 as *const libc::c_char,
                b"src/expand-common.c\0" as *const u8 as *const libc::c_char,
                340 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 24],
                    &[libc::c_char; 24],
                >(b"FILE *next_file(FILE *)\0"))
                    .as_ptr(),
            );
        }
        'c_8099: {
            if !prev_file.is_null() {} else {
                __assert_fail(
                    b"prev_file\0" as *const u8 as *const libc::c_char,
                    b"src/expand-common.c\0" as *const u8 as *const libc::c_char,
                    340 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 24],
                        &[libc::c_char; 24],
                    >(b"FILE *next_file(FILE *)\0"))
                        .as_ptr(),
                );
            }
        };
        let mut err: libc::c_int = *__errno_location();
        if ferror_unlocked(fp) == 0 {
            err = 0 as libc::c_int;
        }
        if strcmp(prev_file, b"-\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            clearerr_unlocked(fp);
        } else if rpl_fclose(fp) != 0 as libc::c_int {
            err = *__errno_location();
        }
        if err != 0 {
            error(
                0 as libc::c_int,
                err,
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    prev_file,
                ),
            );
            exit_status = 1 as libc::c_int;
        }
    }
    loop {
        let fresh1 = file_list;
        file_list = file_list.offset(1);
        file = *fresh1;
        if file.is_null() {
            break;
        }
        if strcmp(file, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            have_read_stdin = 1 as libc::c_int != 0;
            fp = stdin;
        } else {
            fp = fopen(file, b"r\0" as *const u8 as *const libc::c_char);
        }
        if !fp.is_null() {
            prev_file = file;
            fadvise(fp, FADVISE_SEQUENTIAL);
            return fp;
        }
        error(
            0 as libc::c_int,
            *__errno_location(),
            b"%s\0" as *const u8 as *const libc::c_char,
            quotearg_n_style_colon(0 as libc::c_int, shell_escape_quoting_style, file),
        );
        exit_status = 1 as libc::c_int;
    }
    return 0 as *mut FILE;
}
#[no_mangle]
pub unsafe extern "C" fn cleanup_file_list_stdin() {
    if have_read_stdin as libc::c_int != 0 && rpl_fclose(stdin) != 0 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"-\0" as *const u8 as *const libc::c_char,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"-\0" as *const u8 as *const libc::c_char,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn emit_tab_list_info() {
    fputs_unlocked(
        gettext(
            b"  -t, --tabs=LIST  use comma separated list of tab positions.\n\0"
                as *const u8 as *const libc::c_char,
        ),
        stdout,
    );
    fputs_unlocked(
        gettext(
            b"                     The last specified position can be prefixed with '/'\n                     to specify a tab size to use after the last\n                     explicitly specified tab stop.  Also a prefix of '+'\n                     can be used to align remaining tab stops relative to\n                     the last specified tab stop instead of the first column\n\0"
                as *const u8 as *const libc::c_char,
        ),
        stdout,
    );
}
static mut tab_size: uintmax_t = 0 as libc::c_int as uintmax_t;
static mut extend_size: uintmax_t = 0 as libc::c_int as uintmax_t;
static mut increment_size: uintmax_t = 0 as libc::c_int as uintmax_t;
static mut tab_list: *mut uintmax_t = 0 as *const uintmax_t as *mut uintmax_t;
static mut n_tabs_allocated: size_t = 0 as libc::c_int as size_t;
static mut first_free_tab: size_t = 0 as libc::c_int as size_t;
static mut file_list: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut stdin_argv: [*mut libc::c_char; 2] = [
    b"-\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
static mut have_read_stdin: bool = 0 as libc::c_int != 0;
unsafe extern "C" fn set_extend_size(mut tabval: uintmax_t) -> bool {
    let mut ok: bool = 1 as libc::c_int != 0;
    if extend_size != 0 {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            gettext(
                b"'/' specifier only allowed with the last value\0" as *const u8
                    as *const libc::c_char,
            ),
        );
        ok = 0 as libc::c_int != 0;
    }
    extend_size = tabval;
    return ok;
}
unsafe extern "C" fn set_increment_size(mut tabval: uintmax_t) -> bool {
    let mut ok: bool = 1 as libc::c_int != 0;
    if increment_size != 0 {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            gettext(
                b"'+' specifier only allowed with the last value\0" as *const u8
                    as *const libc::c_char,
            ),
        );
        ok = 0 as libc::c_int != 0;
    }
    increment_size = tabval;
    return ok;
}
unsafe extern "C" fn validate_tab_stops(
    mut tabs: *const uintmax_t,
    mut entries: size_t,
) {
    let mut prev_tab: uintmax_t = 0 as libc::c_int as uintmax_t;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < entries {
        if *tabs.offset(i as isize) == 0 as libc::c_int as libc::c_ulong {
            if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"tab size cannot be 0\0" as *const u8 as *const libc::c_char,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"tab size cannot be 0\0" as *const u8 as *const libc::c_char,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if *tabs.offset(i as isize) <= prev_tab {
            if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"tab sizes must be ascending\0" as *const u8
                            as *const libc::c_char,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"tab sizes must be ascending\0" as *const u8
                            as *const libc::c_char,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        prev_tab = *tabs.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    if increment_size != 0 && extend_size != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"'/' specifier is mutually exclusive with '+'\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"'/' specifier is mutually exclusive with '+'\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
}
