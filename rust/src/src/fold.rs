use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn clearerr_unlocked(__stream: *mut FILE);
    fn ferror_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    static mut Version: *const libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn x2realloc(p: *mut libc::c_void, ps: *mut size_t) -> *mut libc::c_void;
    fn close_stdout();
    fn version_etc(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        _: ...
    );
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn fadvise(fp: *mut FILE, advice: fadvice_t);
    fn xdectoumax(
        n_str: *const libc::c_char,
        min: uintmax_t,
        max: uintmax_t,
        suffixes: *const libc::c_char,
        err: *const libc::c_char,
        err_exit: libc::c_int,
    ) -> uintmax_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
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
pub type C2RustUnnamed_0 = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed_0 = -131;
pub const GETOPT_HELP_CHAR: C2RustUnnamed_0 = -130;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct infomap {
    pub program: *const libc::c_char,
    pub node: *const libc::c_char,
}
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
pub struct C2RustUnnamed_1 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub _gl_dummy: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn usage(mut status: libc::c_int) -> ! {
    if status != 0 as libc::c_int {
        fprintf(
            stderr,
            gettext(
                b"Try '%s --help' for more information.\n\0" as *const u8
                    as *const libc::c_char,
            ),
            program_name,
        );
    } else {
        printf(
            gettext(
                b"Usage: %s [OPTION]... [FILE]...\n\0" as *const u8
                    as *const libc::c_char,
            ),
            program_name,
        );
        fputs_unlocked(
            gettext(
                b"Wrap input lines in each FILE, writing to standard output.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_stdin_note();
        emit_mandatory_arg_note();
        fputs_unlocked(
            gettext(
                b"  -b, --bytes         count bytes rather than columns\n  -s, --spaces        break at spaces\n  -w, --width=WIDTH   use WIDTH columns instead of 80\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"      --help     display this help and exit\n\0" as *const u8
                    as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"      --version  output version information and exit\n\0" as *const u8
                    as *const libc::c_char,
            ),
            stdout,
        );
        emit_ancillary_info(b"fold\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
#[inline]
unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn emit_stdin_note() {
    fputs_unlocked(
        gettext(
            b"\nWith no FILE, or when FILE is -, read standard input.\n\0" as *const u8
                as *const libc::c_char,
        ),
        stdout,
    );
}
#[inline]
unsafe extern "C" fn emit_mandatory_arg_note() {
    fputs_unlocked(
        gettext(
            b"\nMandatory arguments to long options are mandatory for short options too.\n\0"
                as *const u8 as *const libc::c_char,
        ),
        stdout,
    );
}
#[inline]
unsafe extern "C" fn emit_ancillary_info(mut program: *const libc::c_char) {
    let infomap_0: [infomap; 7] = [
        {
            let mut init = infomap {
                program: b"[\0" as *const u8 as *const libc::c_char,
                node: b"test invocation\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = infomap {
                program: b"coreutils\0" as *const u8 as *const libc::c_char,
                node: b"Multi-call invocation\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = infomap {
                program: b"sha224sum\0" as *const u8 as *const libc::c_char,
                node: b"sha2 utilities\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = infomap {
                program: b"sha256sum\0" as *const u8 as *const libc::c_char,
                node: b"sha2 utilities\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = infomap {
                program: b"sha384sum\0" as *const u8 as *const libc::c_char,
                node: b"sha2 utilities\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = infomap {
                program: b"sha512sum\0" as *const u8 as *const libc::c_char,
                node: b"sha2 utilities\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = infomap {
                program: 0 as *const libc::c_char,
                node: 0 as *const libc::c_char,
            };
            init
        },
    ];
    let mut node: *const libc::c_char = program;
    let mut map_prog: *const infomap = infomap_0.as_ptr();
    while !((*map_prog).program).is_null()
        && !(strcmp(program, (*map_prog).program) == 0 as libc::c_int)
    {
        map_prog = map_prog.offset(1);
        map_prog;
    }
    if !((*map_prog).node).is_null() {
        node = (*map_prog).node;
    }
    printf(
        gettext(b"\n%s online help: <%s>\n\0" as *const u8 as *const libc::c_char),
        b"GNU coreutils\0" as *const u8 as *const libc::c_char,
        b"https://www.gnu.org/software/coreutils/\0" as *const u8 as *const libc::c_char,
    );
    let mut lc_messages: *const libc::c_char = setlocale(
        5 as libc::c_int,
        0 as *const libc::c_char,
    );
    if !lc_messages.is_null()
        && strncmp(
            lc_messages,
            b"en_\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) != 0
    {
        fputs_unlocked(
            gettext(
                b"Report any translation bugs to <https://translationproject.org/team/>\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
    }
    printf(
        gettext(b"Full documentation <%s%s>\n\0" as *const u8 as *const libc::c_char),
        b"https://www.gnu.org/software/coreutils/\0" as *const u8 as *const libc::c_char,
        program,
    );
    printf(
        gettext(
            b"or available locally via: info '(coreutils) %s%s'\n\0" as *const u8
                as *const libc::c_char,
        ),
        node,
        if node == program {
            b" invocation\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
}
static mut break_spaces: bool = false;
static mut count_bytes: bool = false;
static mut have_read_stdin: bool = false;
static mut shortopts: [libc::c_char; 35] = unsafe {
    *::core::mem::transmute::<
        &[u8; 35],
        &[libc::c_char; 35],
    >(b"bsw:0::1::2::3::4::5::6::7::8::9::\0")
};
static mut longopts: [option; 6] = [
    {
        let mut init = option {
            name: b"bytes\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"spaces\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"width\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'w' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: GETOPT_HELP_CHAR as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: GETOPT_VERSION_CHAR as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn adjust_column(mut column: size_t, mut c: libc::c_char) -> size_t {
    if !count_bytes {
        if c as libc::c_int == '\u{8}' as i32 {
            if column > 0 as libc::c_int as libc::c_ulong {
                column = column.wrapping_sub(1);
                column;
            }
        } else if c as libc::c_int == '\r' as i32 {
            column = 0 as libc::c_int as size_t;
        } else if c as libc::c_int == '\t' as i32 {
            column = (column as libc::c_ulong)
                .wrapping_add(
                    (8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(
                            column.wrapping_rem(8 as libc::c_int as libc::c_ulong),
                        ),
                ) as size_t as size_t;
        } else {
            column = column.wrapping_add(1);
            column;
        }
    } else {
        column = column.wrapping_add(1);
        column;
    }
    return column;
}
unsafe extern "C" fn fold_file(
    mut filename: *const libc::c_char,
    mut width: size_t,
) -> bool {
    let mut istream: *mut FILE = 0 as *mut FILE;
    let mut c: libc::c_int = 0;
    let mut column: size_t = 0 as libc::c_int as size_t;
    let mut offset_out: size_t = 0 as libc::c_int as size_t;
    static mut line_out: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut allocated_out: size_t = 0 as libc::c_int as size_t;
    let mut saved_errno: libc::c_int = 0;
    if strcmp(filename, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        istream = stdin;
        have_read_stdin = 1 as libc::c_int != 0;
    } else {
        istream = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    }
    if istream.is_null() {
        error(
            0 as libc::c_int,
            *__errno_location(),
            b"%s\0" as *const u8 as *const libc::c_char,
            quotearg_n_style_colon(
                0 as libc::c_int,
                shell_escape_quoting_style,
                filename,
            ),
        );
        return 0 as libc::c_int != 0;
    }
    fadvise(istream, FADVISE_SEQUENTIAL);
    loop {
        c = getc_unlocked(istream);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        if offset_out.wrapping_add(1 as libc::c_int as libc::c_ulong) >= allocated_out {
            line_out = (if ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong
                != 0
            {
                x2realloc(line_out as *mut libc::c_void, &mut allocated_out)
            } else {
                x2realloc(line_out as *mut libc::c_void, &mut allocated_out)
            }) as *mut libc::c_char;
        }
        if c == '\n' as i32 {
            let fresh0 = offset_out;
            offset_out = offset_out.wrapping_add(1);
            *line_out.offset(fresh0 as isize) = c as libc::c_char;
            fwrite_unlocked(
                line_out as *const libc::c_void,
                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                offset_out,
                stdout,
            );
            offset_out = 0 as libc::c_int as size_t;
            column = offset_out;
        } else {
            loop {
                column = adjust_column(column, c as libc::c_char);
                if column > width {
                    if break_spaces {
                        let mut found_blank: bool = 0 as libc::c_int != 0;
                        let mut logical_end: size_t = offset_out;
                        while logical_end != 0 {
                            logical_end = logical_end.wrapping_sub(1);
                            logical_end;
                            if !(*(*__ctype_b_loc())
                                .offset(
                                    to_uchar(*line_out.offset(logical_end as isize))
                                        as libc::c_int as isize,
                                ) as libc::c_int
                                & _ISblank as libc::c_int as libc::c_ushort as libc::c_int
                                != 0)
                            {
                                continue;
                            }
                            found_blank = 1 as libc::c_int != 0;
                            break;
                        }
                        if found_blank {
                            let mut i: size_t = 0;
                            logical_end = logical_end.wrapping_add(1);
                            logical_end;
                            fwrite_unlocked(
                                line_out as *const libc::c_void,
                                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                logical_end,
                                stdout,
                            );
                            putchar_unlocked('\n' as i32);
                            memmove(
                                line_out as *mut libc::c_void,
                                line_out.offset(logical_end as isize)
                                    as *const libc::c_void,
                                offset_out.wrapping_sub(logical_end),
                            );
                            offset_out = (offset_out as libc::c_ulong)
                                .wrapping_sub(logical_end) as size_t as size_t;
                            i = 0 as libc::c_int as size_t;
                            column = i;
                            while i < offset_out {
                                column = adjust_column(
                                    column,
                                    *line_out.offset(i as isize),
                                );
                                i = i.wrapping_add(1);
                                i;
                            }
                            continue;
                        }
                    }
                    if offset_out == 0 as libc::c_int as libc::c_ulong {
                        let fresh1 = offset_out;
                        offset_out = offset_out.wrapping_add(1);
                        *line_out.offset(fresh1 as isize) = c as libc::c_char;
                        break;
                    } else {
                        let fresh2 = offset_out;
                        offset_out = offset_out.wrapping_add(1);
                        *line_out.offset(fresh2 as isize) = '\n' as i32 as libc::c_char;
                        fwrite_unlocked(
                            line_out as *const libc::c_void,
                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            offset_out,
                            stdout,
                        );
                        offset_out = 0 as libc::c_int as size_t;
                        column = offset_out;
                    }
                } else {
                    let fresh3 = offset_out;
                    offset_out = offset_out.wrapping_add(1);
                    *line_out.offset(fresh3 as isize) = c as libc::c_char;
                    break;
                }
            }
        }
    }
    saved_errno = *__errno_location();
    if ferror_unlocked(istream) == 0 {
        saved_errno = 0 as libc::c_int;
    }
    if offset_out != 0 {
        fwrite_unlocked(
            line_out as *const libc::c_void,
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
            offset_out,
            stdout,
        );
    }
    if strcmp(filename, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        clearerr_unlocked(istream);
    } else if rpl_fclose(istream) != 0 as libc::c_int && saved_errno == 0 {
        saved_errno = *__errno_location();
    }
    if saved_errno != 0 {
        error(
            0 as libc::c_int,
            saved_errno,
            b"%s\0" as *const u8 as *const libc::c_char,
            quotearg_n_style_colon(
                0 as libc::c_int,
                shell_escape_quoting_style,
                filename,
            ),
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut width: size_t = 80 as libc::c_int as size_t;
    let mut i: libc::c_int = 0;
    let mut optc: libc::c_int = 0;
    let mut ok: bool = false;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    have_read_stdin = 0 as libc::c_int != 0;
    count_bytes = have_read_stdin;
    break_spaces = count_bytes;
    loop {
        optc = getopt_long(
            argc,
            argv,
            shortopts.as_ptr(),
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        let mut optargbuf: [libc::c_char; 2] = [0; 2];
        let mut current_block_22: u64;
        match optc {
            98 => {
                count_bytes = 1 as libc::c_int != 0;
                current_block_22 = 6669252993407410313;
            }
            115 => {
                break_spaces = 1 as libc::c_int != 0;
                current_block_22 = 6669252993407410313;
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                if !optarg.is_null() {
                    optarg = optarg.offset(-1);
                    optarg;
                } else {
                    optargbuf[0 as libc::c_int as usize] = optc as libc::c_char;
                    optargbuf[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                    optarg = optargbuf.as_mut_ptr();
                }
                current_block_22 = 12326535741508966465;
            }
            119 => {
                current_block_22 = 12326535741508966465;
            }
            -130 => {
                usage(0 as libc::c_int);
            }
            -131 => {
                version_etc(
                    stdout,
                    b"fold\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
        match current_block_22 {
            12326535741508966465 => {
                width = xdectoumax(
                    optarg,
                    1 as libc::c_int as uintmax_t,
                    (18446744073709551615 as libc::c_ulong)
                        .wrapping_sub(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    b"\0" as *const u8 as *const libc::c_char,
                    gettext(
                        b"invalid number of columns\0" as *const u8
                            as *const libc::c_char,
                    ),
                    0 as libc::c_int,
                );
            }
            _ => {}
        }
    }
    if argc == optind {
        ok = fold_file(b"-\0" as *const u8 as *const libc::c_char, width);
    } else {
        ok = 1 as libc::c_int != 0;
        i = optind;
        while i < argc {
            ok = (ok as libc::c_int
                & fold_file(*argv.offset(i as isize), width) as libc::c_int) as bool;
            i += 1;
            i;
        }
    }
    if have_read_stdin as libc::c_int != 0 && rpl_fclose(stdin) == -(1 as libc::c_int) {
        if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
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
    return if ok as libc::c_int != 0 { 0 as libc::c_int } else { 1 as libc::c_int };
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
