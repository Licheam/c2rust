use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn set_program_name(argv0: *const libc::c_char);
    static mut program_name: *const libc::c_char;
    fn version_etc(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        _: ...
    );
    fn close_stdout();
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
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct infomap {
    pub program: *const libc::c_char,
    pub node: *const libc::c_char,
}
pub const DEFAULT_ECHO_TO_XPG: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn usage(mut status: libc::c_int) -> ! {
    if status == 0 as libc::c_int {} else {
        __assert_fail(
            b"status == EXIT_SUCCESS\0" as *const u8 as *const libc::c_char,
            b"src/echo.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"void usage(int)\0"))
                .as_ptr(),
        );
    }
    'c_6504: {
        if status == 0 as libc::c_int {} else {
            __assert_fail(
                b"status == EXIT_SUCCESS\0" as *const u8 as *const libc::c_char,
                b"src/echo.c\0" as *const u8 as *const libc::c_char,
                40 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"void usage(int)\0"))
                    .as_ptr(),
            );
        }
    };
    printf(
        gettext(
            b"Usage: %s [SHORT-OPTION]... [STRING]...\n  or:  %s LONG-OPTION\n\0"
                as *const u8 as *const libc::c_char,
        ),
        program_name,
        program_name,
    );
    fputs_unlocked(
        gettext(
            b"Echo the STRING(s) to standard output.\n\n  -n             do not output the trailing newline\n\0"
                as *const u8 as *const libc::c_char,
        ),
        stdout,
    );
    fputs_unlocked(
        gettext(
            if DEFAULT_ECHO_TO_XPG as libc::c_int != 0 {
                b"  -e             enable interpretation of backslash escapes (default)\n  -E             disable interpretation of backslash escapes\n\0"
                    as *const u8 as *const libc::c_char
            } else {
                b"  -e             enable interpretation of backslash escapes\n  -E             disable interpretation of backslash escapes (default)\n\0"
                    as *const u8 as *const libc::c_char
            },
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
    fputs_unlocked(
        gettext(
            b"\nIf -e is in effect, the following sequences are recognized:\n\n\0"
                as *const u8 as *const libc::c_char,
        ),
        stdout,
    );
    fputs_unlocked(
        gettext(
            b"  \\\\      backslash\n  \\a      alert (BEL)\n  \\b      backspace\n  \\c      produce no further output\n  \\e      escape\n  \\f      form feed\n  \\n      new line\n  \\r      carriage return\n  \\t      horizontal tab\n  \\v      vertical tab\n\0"
                as *const u8 as *const libc::c_char,
        ),
        stdout,
    );
    fputs_unlocked(
        gettext(
            b"  \\0NNN   byte with octal value NNN (1 to 3 digits)\n  \\xHH    byte with hexadecimal value HH (1 to 2 digits)\n\0"
                as *const u8 as *const libc::c_char,
        ),
        stdout,
    );
    printf(
        gettext(
            b"\nNOTE: your shell may have its own version of %s, which usually supersedes\nthe version described here.  Please refer to your shell's documentation\nfor details about the options it supports.\n\0"
                as *const u8 as *const libc::c_char,
        ),
        b"echo\0" as *const u8 as *const libc::c_char,
    );
    emit_ancillary_info(b"echo\0" as *const u8 as *const libc::c_char);
    exit(status);
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
unsafe extern "C" fn hextobin(mut c: libc::c_uchar) -> libc::c_int {
    match c as libc::c_int {
        97 | 65 => return 10 as libc::c_int,
        98 | 66 => return 11 as libc::c_int,
        99 | 67 => return 12 as libc::c_int,
        100 | 68 => return 13 as libc::c_int,
        101 | 69 => return 14 as libc::c_int,
        102 | 70 => return 15 as libc::c_int,
        _ => return c as libc::c_int - '0' as i32,
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut display_return: bool = 1 as libc::c_int != 0;
    let mut posixly_correct: bool = !(getenv(
        b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char,
    ))
        .is_null();
    let mut allow_options: bool = !posixly_correct
        || DEFAULT_ECHO_TO_XPG as libc::c_int == 0 && (1 as libc::c_int) < argc
            && strcmp(
                *argv.offset(1 as libc::c_int as isize),
                b"-n\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int;
    let mut do_v9: bool = DEFAULT_ECHO_TO_XPG as libc::c_int != 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    if allow_options as libc::c_int != 0 && argc == 2 as libc::c_int {
        if strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"--help\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            usage(0 as libc::c_int);
        }
        if strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"--version\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            version_etc(
                stdout,
                b"echo\0" as *const u8 as *const libc::c_char,
                b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                Version,
                b"Brian Fox\0" as *const u8 as *const libc::c_char,
                b"Chet Ramey\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_char,
            );
            return 0 as libc::c_int;
        }
    }
    argc -= 1;
    argc;
    argv = argv.offset(1);
    argv;
    if allow_options {
        's_64: while argc > 0 as libc::c_int
            && **argv.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
        {
            let mut temp: *const libc::c_char = (*argv.offset(0 as libc::c_int as isize))
                .offset(1 as libc::c_int as isize);
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while *temp.offset(i as isize) != 0 {
                match *temp.offset(i as isize) as libc::c_int {
                    101 | 69 | 110 => {}
                    _ => {
                        break 's_64;
                    }
                }
                i = i.wrapping_add(1);
                i;
            }
            if i == 0 as libc::c_int as libc::c_ulong {
                break;
            }
            while *temp != 0 {
                let fresh0 = temp;
                temp = temp.offset(1);
                match *fresh0 as libc::c_int {
                    101 => {
                        do_v9 = 1 as libc::c_int != 0;
                    }
                    69 => {
                        do_v9 = 0 as libc::c_int != 0;
                    }
                    110 => {
                        display_return = 0 as libc::c_int != 0;
                    }
                    _ => {}
                }
            }
            argc -= 1;
            argc;
            argv = argv.offset(1);
            argv;
        }
    }
    if do_v9 as libc::c_int != 0 || posixly_correct as libc::c_int != 0 {
        while argc > 0 as libc::c_int {
            let mut s: *const libc::c_char = *argv.offset(0 as libc::c_int as isize);
            let mut c: libc::c_uchar = 0;
            loop {
                let fresh1 = s;
                s = s.offset(1);
                c = *fresh1 as libc::c_uchar;
                if !(c != 0) {
                    break;
                }
                if c as libc::c_int == '\\' as i32 && *s as libc::c_int != 0 {
                    let mut current_block_48: u64;
                    let fresh2 = s;
                    s = s.offset(1);
                    c = *fresh2 as libc::c_uchar;
                    match c as libc::c_int {
                        97 => {
                            c = '\u{7}' as i32 as libc::c_uchar;
                            current_block_48 = 317151059986244064;
                        }
                        98 => {
                            c = '\u{8}' as i32 as libc::c_uchar;
                            current_block_48 = 317151059986244064;
                        }
                        99 => return 0 as libc::c_int,
                        101 => {
                            c = '\u{1b}' as i32 as libc::c_uchar;
                            current_block_48 = 317151059986244064;
                        }
                        102 => {
                            c = '\u{c}' as i32 as libc::c_uchar;
                            current_block_48 = 317151059986244064;
                        }
                        110 => {
                            c = '\n' as i32 as libc::c_uchar;
                            current_block_48 = 317151059986244064;
                        }
                        114 => {
                            c = '\r' as i32 as libc::c_uchar;
                            current_block_48 = 317151059986244064;
                        }
                        116 => {
                            c = '\t' as i32 as libc::c_uchar;
                            current_block_48 = 317151059986244064;
                        }
                        118 => {
                            c = '\u{b}' as i32 as libc::c_uchar;
                            current_block_48 = 317151059986244064;
                        }
                        120 => {
                            let mut ch: libc::c_uchar = *s as libc::c_uchar;
                            if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                                as libc::c_int
                                & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                                == 0
                            {
                                current_block_48 = 10999470842310942890;
                            } else {
                                s = s.offset(1);
                                s;
                                c = hextobin(ch) as libc::c_uchar;
                                ch = *s as libc::c_uchar;
                                if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize)
                                    as libc::c_int
                                    & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                                {
                                    s = s.offset(1);
                                    s;
                                    c = (c as libc::c_int * 16 as libc::c_int + hextobin(ch))
                                        as libc::c_uchar;
                                }
                                current_block_48 = 317151059986244064;
                            }
                        }
                        48 => {
                            c = 0 as libc::c_int as libc::c_uchar;
                            if !('0' as i32 <= *s as libc::c_int
                                && *s as libc::c_int <= '7' as i32)
                            {
                                current_block_48 = 317151059986244064;
                            } else {
                                let fresh3 = s;
                                s = s.offset(1);
                                c = *fresh3 as libc::c_uchar;
                                current_block_48 = 14316407509428893691;
                            }
                        }
                        49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                            current_block_48 = 14316407509428893691;
                        }
                        92 => {
                            current_block_48 = 317151059986244064;
                        }
                        _ => {
                            current_block_48 = 10999470842310942890;
                        }
                    }
                    match current_block_48 {
                        14316407509428893691 => {
                            c = (c as libc::c_int - '0' as i32) as libc::c_uchar;
                            if '0' as i32 <= *s as libc::c_int
                                && *s as libc::c_int <= '7' as i32
                            {
                                let fresh4 = s;
                                s = s.offset(1);
                                c = (c as libc::c_int * 8 as libc::c_int
                                    + (*fresh4 as libc::c_int - '0' as i32)) as libc::c_uchar;
                            }
                            if '0' as i32 <= *s as libc::c_int
                                && *s as libc::c_int <= '7' as i32
                            {
                                let fresh5 = s;
                                s = s.offset(1);
                                c = (c as libc::c_int * 8 as libc::c_int
                                    + (*fresh5 as libc::c_int - '0' as i32)) as libc::c_uchar;
                            }
                        }
                        10999470842310942890 => {
                            putchar_unlocked('\\' as i32);
                        }
                        _ => {}
                    }
                }
                putchar_unlocked(c as libc::c_int);
            }
            argc -= 1;
            argc;
            argv = argv.offset(1);
            argv;
            if argc > 0 as libc::c_int {
                putchar_unlocked(' ' as i32);
            }
        }
    } else {
        while argc > 0 as libc::c_int {
            fputs_unlocked(*argv.offset(0 as libc::c_int as isize), stdout);
            argc -= 1;
            argc;
            argv = argv.offset(1);
            argv;
            if argc > 0 as libc::c_int {
                putchar_unlocked(' ' as i32);
            }
        }
    }
    if display_return {
        putchar_unlocked('\n' as i32);
    }
    return 0 as libc::c_int;
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
