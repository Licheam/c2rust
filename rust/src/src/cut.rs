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
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn clearerr_unlocked(__stream: *mut FILE);
    fn feof_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn ferror_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optind: libc::c_int;
    static mut optarg: *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
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
    fn xalloc_die();
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn fadvise(fp: *mut FILE, advice: fadvice_t);
    fn getndelim2(
        lineptr: *mut *mut libc::c_char,
        linesize: *mut size_t,
        offset: size_t,
        nmax: size_t,
        delim1: libc::c_int,
        delim2: libc::c_int,
        stream: *mut FILE,
    ) -> ssize_t;
    static mut frp: *mut field_range_pair;
    fn set_fields(fieldstr: *const libc::c_char, options: libc::c_uint);
}
pub type size_t = libc::c_ulong;
pub type __uintmax_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type uintmax_t = __uintmax_t;
pub type C2RustUnnamed = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed = -131;
pub const GETOPT_HELP_CHAR: C2RustUnnamed = -130;
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
pub struct field_range_pair {
    pub lo: uintmax_t,
    pub hi: uintmax_t,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const SETFLD_ERRMSG_USE_POS: C2RustUnnamed_0 = 4;
pub const SETFLD_COMPLEMENT: C2RustUnnamed_0 = 2;
pub const SETFLD_ALLOW_DASH: C2RustUnnamed_0 = 1;
pub type operating_mode = libc::c_uint;
pub const field_mode: operating_mode = 2;
pub const byte_mode: operating_mode = 1;
pub const undefined_mode: operating_mode = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const COMPLEMENT_OPTION: C2RustUnnamed_1 = 129;
pub const OUTPUT_DELIMITER_OPTION: C2RustUnnamed_1 = 128;
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
                b"Usage: %s OPTION... [FILE]...\n\0" as *const u8 as *const libc::c_char,
            ),
            program_name,
        );
        fputs_unlocked(
            gettext(
                b"Print selected parts of lines from each FILE to standard output.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_stdin_note();
        emit_mandatory_arg_note();
        fputs_unlocked(
            gettext(
                b"  -b, --bytes=LIST        select only these bytes\n  -c, --characters=LIST   select only these characters\n  -d, --delimiter=DELIM   use DELIM instead of TAB for field delimiter\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -f, --fields=LIST       select only these fields;  also print any line\n                            that contains no delimiter character, unless\n                            the -s option is specified\n  -n                      (ignored)\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"      --complement        complement the set of selected bytes, characters\n                            or fields\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -s, --only-delimited    do not print lines not containing delimiters\n      --output-delimiter=STRING  use STRING as the output delimiter\n                            the default is to use the input delimiter\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -z, --zero-terminated    line delimiter is NUL, not newline\n\0"
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
        fputs_unlocked(
            gettext(
                b"\nUse one, and only one of -b, -c or -f.  Each LIST is made up of one\nrange, or many ranges separated by commas.  Selected input is written\nin the same order that it is read, and is written exactly once.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"Each range is one of:\n\n  N     N'th byte, character or field, counted from 1\n  N-    from N'th byte, character or field, to end of line\n  N-M   from N'th to M'th (included) byte, character or field\n  -M    from first to M'th (included) byte, character or field\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_ancillary_info(b"cut\0" as *const u8 as *const libc::c_char);
    }
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
unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
}
static mut current_rp: *mut field_range_pair = 0 as *const field_range_pair
    as *mut field_range_pair;
static mut field_1_buffer: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut field_1_bufsize: size_t = 0;
static mut operating_mode: operating_mode = undefined_mode;
static mut suppress_non_delimited: bool = false;
static mut complement: bool = false;
static mut delim: libc::c_uchar = 0;
static mut line_delim: libc::c_uchar = '\n' as i32 as libc::c_uchar;
static mut output_delimiter_specified: bool = false;
static mut output_delimiter_length: size_t = 0;
static mut output_delimiter_string: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut have_read_stdin: bool = false;
static mut longopts: [option; 11] = [
    {
        let mut init = option {
            name: b"bytes\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"characters\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"fields\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"delimiter\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"only-delimited\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"output-delimiter\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OUTPUT_DELIMITER_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"complement\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: COMPLEMENT_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"zero-terminated\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'z' as i32,
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
#[inline]
unsafe extern "C" fn next_item(mut item_idx: *mut uintmax_t) {
    *item_idx = (*item_idx).wrapping_add(1);
    *item_idx;
    if *item_idx > (*current_rp).hi {
        current_rp = current_rp.offset(1);
        current_rp;
    }
}
#[inline]
unsafe extern "C" fn print_kth(mut k: uintmax_t) -> bool {
    return (*current_rp).lo <= k;
}
#[inline]
unsafe extern "C" fn is_range_start_index(mut k: uintmax_t) -> bool {
    return k == (*current_rp).lo;
}
unsafe extern "C" fn cut_bytes(mut stream: *mut FILE) {
    let mut byte_idx: uintmax_t = 0;
    let mut print_delimiter: bool = false;
    byte_idx = 0 as libc::c_int as uintmax_t;
    print_delimiter = 0 as libc::c_int != 0;
    current_rp = frp;
    loop {
        let mut c: libc::c_int = 0;
        c = getc_unlocked(stream);
        if c == line_delim as libc::c_int {
            putchar_unlocked(c);
            byte_idx = 0 as libc::c_int as uintmax_t;
            print_delimiter = 0 as libc::c_int != 0;
            current_rp = frp;
        } else if c == -(1 as libc::c_int) {
            if byte_idx > 0 as libc::c_int as libc::c_ulong {
                putchar_unlocked(line_delim as libc::c_int);
            }
            break;
        } else {
            next_item(&mut byte_idx);
            if print_kth(byte_idx) {
                if output_delimiter_specified {
                    if print_delimiter as libc::c_int != 0
                        && is_range_start_index(byte_idx) as libc::c_int != 0
                    {
                        fwrite_unlocked(
                            output_delimiter_string as *const libc::c_void,
                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            output_delimiter_length,
                            stdout,
                        );
                    }
                    print_delimiter = 1 as libc::c_int != 0;
                }
                putchar_unlocked(c);
            }
        }
    };
}
unsafe extern "C" fn cut_fields(mut stream: *mut FILE) {
    let mut c: libc::c_int = 0;
    let mut field_idx: uintmax_t = 1 as libc::c_int as uintmax_t;
    let mut found_any_selected_field: bool = 0 as libc::c_int != 0;
    let mut buffer_first_field: bool = false;
    current_rp = frp;
    c = getc_unlocked(stream);
    if c == -(1 as libc::c_int) {
        return;
    }
    ungetc(c, stream);
    c = 0 as libc::c_int;
    buffer_first_field = suppress_non_delimited as libc::c_int
        ^ !print_kth(1 as libc::c_int as uintmax_t) as libc::c_int != 0;
    loop {
        if field_idx == 1 as libc::c_int as libc::c_ulong
            && buffer_first_field as libc::c_int != 0
        {
            let mut len: ssize_t = 0;
            let mut n_bytes: size_t = 0;
            len = getndelim2(
                &mut field_1_buffer,
                &mut field_1_bufsize,
                0 as libc::c_int as size_t,
                -(1 as libc::c_int) as size_t,
                delim as libc::c_int,
                line_delim as libc::c_int,
                stream,
            );
            if len < 0 as libc::c_int as libc::c_long {
                free(field_1_buffer as *mut libc::c_void);
                field_1_buffer = 0 as *mut libc::c_char;
                if ferror_unlocked(stream) != 0 || feof_unlocked(stream) != 0 {
                    break;
                }
                xalloc_die();
            }
            n_bytes = len as size_t;
            if n_bytes != 0 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"n_bytes != 0\0" as *const u8 as *const libc::c_char,
                    b"src/cut.c\0" as *const u8 as *const libc::c_char,
                    328 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 24],
                        &[libc::c_char; 24],
                    >(b"void cut_fields(FILE *)\0"))
                        .as_ptr(),
                );
            }
            'c_7734: {
                if n_bytes != 0 as libc::c_int as libc::c_ulong {} else {
                    __assert_fail(
                        b"n_bytes != 0\0" as *const u8 as *const libc::c_char,
                        b"src/cut.c\0" as *const u8 as *const libc::c_char,
                        328 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 24],
                            &[libc::c_char; 24],
                        >(b"void cut_fields(FILE *)\0"))
                            .as_ptr(),
                    );
                }
            };
            c = 0 as libc::c_int;
            if to_uchar(
                *field_1_buffer
                    .offset(
                        n_bytes.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ),
            ) as libc::c_int != delim as libc::c_int
            {
                if !suppress_non_delimited {
                    fwrite_unlocked(
                        field_1_buffer as *const libc::c_void,
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        n_bytes,
                        stdout,
                    );
                    if *field_1_buffer
                        .offset(
                            n_bytes.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) as libc::c_int != line_delim as libc::c_int
                    {
                        putchar_unlocked(line_delim as libc::c_int);
                    }
                    c = line_delim as libc::c_int;
                }
                continue;
            } else {
                if print_kth(1 as libc::c_int as uintmax_t) {
                    fwrite_unlocked(
                        field_1_buffer as *const libc::c_void,
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        n_bytes.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        stdout,
                    );
                    if delim as libc::c_int == line_delim as libc::c_int {
                        let mut last_c: libc::c_int = getc_unlocked(stream);
                        if last_c != -(1 as libc::c_int) {
                            ungetc(last_c, stream);
                            found_any_selected_field = 1 as libc::c_int != 0;
                        }
                    } else {
                        found_any_selected_field = 1 as libc::c_int != 0;
                    }
                }
                next_item(&mut field_idx);
            }
        }
        let mut prev_c: libc::c_int = c;
        if print_kth(field_idx) {
            if found_any_selected_field {
                fwrite_unlocked(
                    output_delimiter_string as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    output_delimiter_length,
                    stdout,
                );
            }
            found_any_selected_field = 1 as libc::c_int != 0;
            loop {
                c = getc_unlocked(stream);
                if !(c != delim as libc::c_int && c != line_delim as libc::c_int
                    && c != -(1 as libc::c_int))
                {
                    break;
                }
                putchar_unlocked(c);
                prev_c = c;
            }
        } else {
            loop {
                c = getc_unlocked(stream);
                if !(c != delim as libc::c_int && c != line_delim as libc::c_int
                    && c != -(1 as libc::c_int))
                {
                    break;
                }
                prev_c = c;
            }
        }
        if delim as libc::c_int == line_delim as libc::c_int && c == delim as libc::c_int
        {
            let mut last_c_0: libc::c_int = getc_unlocked(stream);
            if last_c_0 != -(1 as libc::c_int) {
                ungetc(last_c_0, stream);
            } else {
                c = last_c_0;
            }
        }
        if c == delim as libc::c_int {
            next_item(&mut field_idx);
        } else {
            if !(c == line_delim as libc::c_int || c == -(1 as libc::c_int)) {
                continue;
            }
            if found_any_selected_field as libc::c_int != 0
                || !(suppress_non_delimited as libc::c_int != 0
                    && field_idx == 1 as libc::c_int as libc::c_ulong)
            {
                if c == line_delim as libc::c_int || prev_c != line_delim as libc::c_int
                    || delim as libc::c_int == line_delim as libc::c_int
                {
                    putchar_unlocked(line_delim as libc::c_int);
                }
            }
            if c == -(1 as libc::c_int) {
                break;
            }
            field_idx = 1 as libc::c_int as uintmax_t;
            current_rp = frp;
            found_any_selected_field = 0 as libc::c_int != 0;
        }
    };
}
unsafe extern "C" fn cut_stream(mut stream: *mut FILE) {
    if operating_mode as libc::c_uint == byte_mode as libc::c_int as libc::c_uint {
        cut_bytes(stream);
    } else {
        cut_fields(stream);
    };
}
unsafe extern "C" fn cut_file(mut file: *const libc::c_char) -> bool {
    let mut stream: *mut FILE = 0 as *mut FILE;
    if strcmp(file, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        have_read_stdin = 1 as libc::c_int != 0;
        stream = stdin;
    } else {
        stream = fopen(file, b"r\0" as *const u8 as *const libc::c_char);
        if stream.is_null() {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    file,
                ),
            );
            return 0 as libc::c_int != 0;
        }
    }
    fadvise(stream, FADVISE_SEQUENTIAL);
    cut_stream(stream);
    let mut err: libc::c_int = *__errno_location();
    if ferror_unlocked(stream) == 0 {
        err = 0 as libc::c_int;
    }
    if strcmp(file, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        clearerr_unlocked(stream);
    } else if rpl_fclose(stream) == -(1 as libc::c_int) {
        err = *__errno_location();
    }
    if err != 0 {
        error(
            0 as libc::c_int,
            err,
            b"%s\0" as *const u8 as *const libc::c_char,
            quotearg_n_style_colon(0 as libc::c_int, shell_escape_quoting_style, file),
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut optc: libc::c_int = 0;
    let mut ok: bool = false;
    let mut delim_specified: bool = 0 as libc::c_int != 0;
    let mut spec_list_string: *mut libc::c_char = 0 as *mut libc::c_char;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    operating_mode = undefined_mode;
    suppress_non_delimited = 0 as libc::c_int != 0;
    delim = '\0' as i32 as libc::c_uchar;
    have_read_stdin = 0 as libc::c_int != 0;
    loop {
        optc = getopt_long(
            argc,
            argv,
            b"b:c:d:f:nsz\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        match optc {
            98 | 99 => {
                if operating_mode as libc::c_uint
                    != undefined_mode as libc::c_int as libc::c_uint
                {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"only one type of list may be specified\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    usage(1 as libc::c_int);
                }
                operating_mode = byte_mode;
                spec_list_string = optarg;
            }
            102 => {
                if operating_mode as libc::c_uint
                    != undefined_mode as libc::c_int as libc::c_uint
                {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"only one type of list may be specified\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    usage(1 as libc::c_int);
                }
                operating_mode = field_mode;
                spec_list_string = optarg;
            }
            100 => {
                if *optarg.offset(0 as libc::c_int as isize) as libc::c_int
                    != '\0' as i32
                    && *optarg.offset(1 as libc::c_int as isize) as libc::c_int
                        != '\0' as i32
                {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"the delimiter must be a single character\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    usage(1 as libc::c_int);
                }
                delim = *optarg.offset(0 as libc::c_int as isize) as libc::c_uchar;
                delim_specified = 1 as libc::c_int != 0;
            }
            128 => {
                output_delimiter_specified = 1 as libc::c_int != 0;
                output_delimiter_length = if *optarg.offset(0 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                {
                    1 as libc::c_int as libc::c_ulong
                } else {
                    strlen(optarg)
                };
                output_delimiter_string = xstrdup(optarg);
            }
            110 => {}
            115 => {
                suppress_non_delimited = 1 as libc::c_int != 0;
            }
            122 => {
                line_delim = '\0' as i32 as libc::c_uchar;
            }
            129 => {
                complement = 1 as libc::c_int != 0;
            }
            -130 => {
                usage(0 as libc::c_int);
            }
            -131 => {
                version_etc(
                    stdout,
                    b"cut\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"David M. Ihnat\0" as *const u8 as *const libc::c_char,
                    b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                    b"Jim Meyering\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    if operating_mode as libc::c_uint == undefined_mode as libc::c_int as libc::c_uint {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            gettext(
                b"you must specify a list of bytes, characters, or fields\0" as *const u8
                    as *const libc::c_char,
            ),
        );
        usage(1 as libc::c_int);
    }
    if delim_specified as libc::c_int != 0
        && operating_mode as libc::c_uint != field_mode as libc::c_int as libc::c_uint
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            gettext(
                b"an input delimiter may be specified only when operating on fields\0"
                    as *const u8 as *const libc::c_char,
            ),
        );
        usage(1 as libc::c_int);
    }
    if suppress_non_delimited as libc::c_int != 0
        && operating_mode as libc::c_uint != field_mode as libc::c_int as libc::c_uint
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            gettext(
                b"suppressing non-delimited lines makes sense\n\tonly when operating on fields\0"
                    as *const u8 as *const libc::c_char,
            ),
        );
        usage(1 as libc::c_int);
    }
    set_fields(
        spec_list_string,
        ((if operating_mode as libc::c_uint == field_mode as libc::c_int as libc::c_uint
        {
            0 as libc::c_int
        } else {
            SETFLD_ERRMSG_USE_POS as libc::c_int
        })
            | (if complement as libc::c_int != 0 {
                SETFLD_COMPLEMENT as libc::c_int
            } else {
                0 as libc::c_int
            })) as libc::c_uint,
    );
    if !delim_specified {
        delim = '\t' as i32 as libc::c_uchar;
    }
    if output_delimiter_string.is_null() {
        static mut dummy: [libc::c_char; 2] = [0; 2];
        dummy[0 as libc::c_int as usize] = delim as libc::c_char;
        dummy[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        output_delimiter_string = dummy.as_mut_ptr();
        output_delimiter_length = 1 as libc::c_int as size_t;
    }
    if optind == argc {
        ok = cut_file(b"-\0" as *const u8 as *const libc::c_char);
    } else {
        ok = 1 as libc::c_int != 0;
        while optind < argc {
            ok = (ok as libc::c_int
                & cut_file(*argv.offset(optind as isize)) as libc::c_int) as bool;
            optind += 1;
            optind;
        }
    }
    if have_read_stdin as libc::c_int != 0 && rpl_fclose(stdin) == -(1 as libc::c_int) {
        error(
            0 as libc::c_int,
            *__errno_location(),
            b"-\0" as *const u8 as *const libc::c_char,
        );
        ok = 0 as libc::c_int != 0;
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
