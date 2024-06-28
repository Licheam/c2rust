use ::libc;
use ::c2rust_bitfields;
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
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
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
pub type COST = libc::c_long;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Word {
    pub text: *const libc::c_char,
    pub length: libc::c_int,
    pub space: libc::c_int,
    #[bitfield(name = "paren", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "period", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "punct", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "final_0", ty = "libc::c_uint", bits = "3..=3")]
    pub paren_period_punct_final_0: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub line_length: libc::c_int,
    pub best_cost: COST,
    pub next_break: *mut WORD,
}
pub type WORD = Word;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
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
                b"Usage: %s [-WIDTH] [OPTION]... [FILE]...\n\0" as *const u8
                    as *const libc::c_char,
            ),
            program_name,
        );
        fputs_unlocked(
            gettext(
                b"Reformat each paragraph in the FILE(s), writing to standard output.\nThe option -WIDTH is an abbreviated form of --width=DIGITS.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_stdin_note();
        emit_mandatory_arg_note();
        fputs_unlocked(
            gettext(
                b"  -c, --crown-margin        preserve indentation of first two lines\n  -p, --prefix=STRING       reformat only lines beginning with STRING,\n                              reattaching the prefix to reformatted lines\n  -s, --split-only          split long lines, but do not refill\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -t, --tagged-paragraph    indentation of first line different from second\n  -u, --uniform-spacing     one space between words, two after sentences\n  -w, --width=WIDTH         maximum line width (default of 75 columns)\n  -g, --goal=WIDTH          goal width (default of 93% of width)\n\0"
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
        emit_ancillary_info(b"fmt\0" as *const u8 as *const libc::c_char);
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
unsafe extern "C" fn set_prefix(mut p: *mut libc::c_char) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    prefix_lead_space = 0 as libc::c_int;
    while *p as libc::c_int == ' ' as i32 {
        prefix_lead_space += 1;
        prefix_lead_space;
        p = p.offset(1);
        p;
    }
    prefix = p;
    prefix_full_length = strlen(p) as libc::c_int;
    s = p.offset(prefix_full_length as isize);
    while s > p && *s.offset(-(1 as libc::c_int) as isize) as libc::c_int == ' ' as i32 {
        s = s.offset(-1);
        s;
    }
    *s = '\0' as i32 as libc::c_char;
    prefix_length = s.offset_from(p) as libc::c_long as libc::c_int;
}
unsafe extern "C" fn fmt(mut f: *mut FILE, mut file: *const libc::c_char) -> bool {
    fadvise(f, FADVISE_SEQUENTIAL);
    tabs = 0 as libc::c_int != 0;
    other_indent = 0 as libc::c_int;
    next_char = get_prefix(f);
    while get_paragraph(f) {
        fmt_paragraph();
        put_paragraph(word_limit);
    }
    let mut err: libc::c_int = if ferror_unlocked(f) != 0 {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    if f == stdin {
        clearerr_unlocked(f);
    } else if rpl_fclose(f) != 0 as libc::c_int && err < 0 as libc::c_int {
        err = *__errno_location();
    }
    if 0 as libc::c_int <= err {
        error(
            0 as libc::c_int,
            err,
            if err != 0 {
                b"%s\0" as *const u8 as *const libc::c_char
            } else {
                gettext(b"read error\0" as *const u8 as *const libc::c_char)
                    as *const libc::c_char
            },
            quotearg_n_style_colon(0 as libc::c_int, shell_escape_quoting_style, file),
        );
    }
    return err < 0 as libc::c_int;
}
unsafe extern "C" fn get_paragraph(mut f: *mut FILE) -> bool {
    let mut c: libc::c_int = 0;
    last_line_length = 0 as libc::c_int;
    c = next_char;
    while c == '\n' as i32 || c == -(1 as libc::c_int)
        || next_prefix_indent < prefix_lead_space
        || in_column < next_prefix_indent + prefix_full_length
    {
        c = copy_rest(f, c);
        if c == -(1 as libc::c_int) {
            next_char = -(1 as libc::c_int);
            return 0 as libc::c_int != 0;
        }
        putchar_unlocked('\n' as i32);
        c = get_prefix(f);
    }
    prefix_indent = next_prefix_indent;
    first_indent = in_column;
    wptr = parabuf.as_mut_ptr();
    word_limit = unused_word_type.as_mut_ptr();
    c = get_line(f, c);
    set_other_indent(same_para(c));
    if !split {
        if crown {
            if same_para(c) {
                loop {
                    c = get_line(f, c);
                    if !(same_para(c) as libc::c_int != 0 && in_column == other_indent) {
                        break;
                    }
                }
            }
        } else if tagged {
            if same_para(c) as libc::c_int != 0 && in_column != first_indent {
                loop {
                    c = get_line(f, c);
                    if !(same_para(c) as libc::c_int != 0 && in_column == other_indent) {
                        break;
                    }
                }
            }
        } else {
            while same_para(c) as libc::c_int != 0 && in_column == other_indent {
                c = get_line(f, c);
            }
        }
    }
    if unused_word_type.as_mut_ptr() < word_limit {} else {
        __assert_fail(
            b"word < word_limit\0" as *const u8 as *const libc::c_char,
            b"src/fmt.c\0" as *const u8 as *const libc::c_char,
            623 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"_Bool get_paragraph(FILE *)\0"))
                .as_ptr(),
        );
    }
    'c_7874: {
        if unused_word_type.as_mut_ptr() < word_limit {} else {
            __assert_fail(
                b"word < word_limit\0" as *const u8 as *const libc::c_char,
                b"src/fmt.c\0" as *const u8 as *const libc::c_char,
                623 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"_Bool get_paragraph(FILE *)\0"))
                    .as_ptr(),
            );
        }
    };
    let ref mut fresh1 = *word_limit.offset(-(1 as libc::c_int as isize));
    let ref mut fresh0 = *word_limit.offset(-(1 as libc::c_int as isize));
    (*fresh0).set_final_0(1 as libc::c_int as libc::c_uint);
    (*fresh1).set_period((*fresh0).final_0());
    next_char = c;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn get_line(mut f: *mut FILE, mut c: libc::c_int) -> libc::c_int {
    let mut start: libc::c_int = 0;
    let mut end_of_parabuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end_of_word: *mut WORD = 0 as *mut WORD;
    end_of_parabuf = &mut *parabuf.as_mut_ptr().offset(5000 as libc::c_int as isize)
        as *mut libc::c_char;
    end_of_word = &mut *unused_word_type
        .as_mut_ptr()
        .offset((1000 as libc::c_int - 2 as libc::c_int) as isize) as *mut WORD;
    loop {
        (*word_limit).text = wptr;
        loop {
            if wptr == end_of_parabuf {
                set_other_indent(1 as libc::c_int != 0);
                flush_paragraph();
            }
            let fresh2 = wptr;
            wptr = wptr.offset(1);
            *fresh2 = c as libc::c_char;
            c = getc_unlocked(f);
            if !(c != -(1 as libc::c_int)
                && *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0)
            {
                break;
            }
        }
        (*word_limit)
            .length = wptr.offset_from((*word_limit).text) as libc::c_long
            as libc::c_int;
        in_column += (*word_limit).length;
        check_punctuation(word_limit);
        start = in_column;
        c = get_space(f, c);
        (*word_limit).space = in_column - start;
        (*word_limit)
            .set_final_0(
                (c == -(1 as libc::c_int)
                    || (*word_limit).period() as libc::c_int != 0
                        && (c == '\n' as i32 || (*word_limit).space > 1 as libc::c_int))
                    as libc::c_int as libc::c_uint,
            );
        if c == '\n' as i32 || c == -(1 as libc::c_int) || uniform as libc::c_int != 0 {
            (*word_limit)
                .space = if (*word_limit).final_0() as libc::c_int != 0 {
                2 as libc::c_int
            } else {
                1 as libc::c_int
            };
        }
        if word_limit == end_of_word {
            set_other_indent(1 as libc::c_int != 0);
            flush_paragraph();
        }
        word_limit = word_limit.offset(1);
        word_limit;
        if !(c != '\n' as i32 && c != -(1 as libc::c_int)) {
            break;
        }
    }
    return get_prefix(f);
}
unsafe extern "C" fn get_prefix(mut f: *mut FILE) -> libc::c_int {
    let mut c: libc::c_int = 0;
    in_column = 0 as libc::c_int;
    c = get_space(f, getc_unlocked(f));
    if prefix_length == 0 as libc::c_int {
        next_prefix_indent = if prefix_lead_space < in_column {
            prefix_lead_space
        } else {
            in_column
        };
    } else {
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        next_prefix_indent = in_column;
        p = prefix;
        while *p as libc::c_int != '\0' as i32 {
            let mut pc: libc::c_uchar = *p as libc::c_uchar;
            if c != pc as libc::c_int {
                return c;
            }
            in_column += 1;
            in_column;
            c = getc_unlocked(f);
            p = p.offset(1);
            p;
        }
        c = get_space(f, c);
    }
    return c;
}
unsafe extern "C" fn get_space(mut f: *mut FILE, mut c: libc::c_int) -> libc::c_int {
    loop {
        if c == ' ' as i32 {
            in_column += 1;
            in_column;
        } else if c == '\t' as i32 {
            tabs = 1 as libc::c_int != 0;
            in_column = (in_column / 8 as libc::c_int + 1 as libc::c_int)
                * 8 as libc::c_int;
        } else {
            return c
        }
        c = getc_unlocked(f);
    };
}
unsafe extern "C" fn copy_rest(mut f: *mut FILE, mut c: libc::c_int) -> libc::c_int {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    out_column = 0 as libc::c_int;
    if in_column > next_prefix_indent || c != '\n' as i32 && c != -(1 as libc::c_int) {
        put_space(next_prefix_indent);
        s = prefix;
        while out_column != in_column && *s as libc::c_int != 0 {
            let fresh3 = s;
            s = s.offset(1);
            putchar_unlocked(*fresh3 as libc::c_int);
            out_column += 1;
            out_column;
        }
        if c != -(1 as libc::c_int) && c != '\n' as i32 {
            put_space(in_column - out_column);
        }
        if c == -(1 as libc::c_int) && in_column >= next_prefix_indent + prefix_length {
            putchar_unlocked('\n' as i32);
        }
    }
    while c != '\n' as i32 && c != -(1 as libc::c_int) {
        putchar_unlocked(c);
        c = getc_unlocked(f);
    }
    return c;
}
unsafe extern "C" fn same_para(mut c: libc::c_int) -> bool {
    return next_prefix_indent == prefix_indent
        && in_column >= next_prefix_indent + prefix_full_length && c != '\n' as i32
        && c != -(1 as libc::c_int);
}
unsafe extern "C" fn flush_paragraph() {
    let mut split_point: *mut WORD = 0 as *mut WORD;
    let mut w: *mut WORD = 0 as *mut WORD;
    let mut shift: libc::c_int = 0;
    let mut best_break: COST = 0;
    if word_limit == unused_word_type.as_mut_ptr() {
        fwrite_unlocked(
            parabuf.as_mut_ptr() as *const libc::c_void,
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
            wptr.offset_from(parabuf.as_mut_ptr()) as libc::c_long as size_t,
            stdout,
        );
        wptr = parabuf.as_mut_ptr();
        return;
    }
    fmt_paragraph();
    split_point = word_limit;
    best_break = if (0 as libc::c_int as COST) < -(1 as libc::c_int) as COST {
        -(1 as libc::c_int) as COST
    } else {
        (((1 as libc::c_int as COST)
            << (::core::mem::size_of::<COST>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
            - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
            + 1 as libc::c_int as libc::c_long
    };
    w = (*unused_word_type.as_mut_ptr()).next_break;
    while w != word_limit {
        if (*w).best_cost - (*(*w).next_break).best_cost < best_break {
            split_point = w;
            best_break = (*w).best_cost - (*(*w).next_break).best_cost;
        }
        if best_break
            <= (if (0 as libc::c_int as COST) < -(1 as libc::c_int) as COST {
                -(1 as libc::c_int) as COST
            } else {
                (((1 as libc::c_int as COST)
                    << (::core::mem::size_of::<COST>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            }) - 3 as libc::c_int as COST * 3 as libc::c_int as COST
        {
            best_break += 3 as libc::c_int as COST * 3 as libc::c_int as COST;
        }
        w = (*w).next_break;
    }
    put_paragraph(split_point);
    memmove(
        parabuf.as_mut_ptr() as *mut libc::c_void,
        (*split_point).text as *const libc::c_void,
        wptr.offset_from((*split_point).text) as libc::c_long as libc::c_ulong,
    );
    shift = ((*split_point).text).offset_from(parabuf.as_mut_ptr()) as libc::c_long
        as libc::c_int;
    wptr = wptr.offset(-(shift as isize));
    w = split_point;
    while w <= word_limit {
        (*w).text = ((*w).text).offset(-(shift as isize));
        w = w.offset(1);
        w;
    }
    memmove(
        unused_word_type.as_mut_ptr() as *mut libc::c_void,
        split_point as *const libc::c_void,
        ((word_limit.offset_from(split_point) as libc::c_long
            + 1 as libc::c_int as libc::c_long) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<WORD>() as libc::c_ulong),
    );
    word_limit = word_limit
        .offset(
            -(split_point.offset_from(unused_word_type.as_mut_ptr()) as libc::c_long
                as isize),
        );
}
unsafe extern "C" fn fmt_paragraph() {
    let mut start: *mut WORD = 0 as *mut WORD;
    let mut w: *mut WORD = 0 as *mut WORD;
    let mut len: libc::c_int = 0;
    let mut wcost: COST = 0;
    let mut best: COST = 0;
    let mut saved_length: libc::c_int = 0;
    (*word_limit).best_cost = 0 as libc::c_int as COST;
    saved_length = (*word_limit).length;
    (*word_limit).length = max_width;
    start = word_limit.offset(-(1 as libc::c_int as isize));
    while start >= unused_word_type.as_mut_ptr() {
        best = if (0 as libc::c_int as COST) < -(1 as libc::c_int) as COST {
            -(1 as libc::c_int) as COST
        } else {
            (((1 as libc::c_int as COST)
                << (::core::mem::size_of::<COST>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        };
        len = if start == unused_word_type.as_mut_ptr() {
            first_indent
        } else {
            other_indent
        };
        w = start;
        len += (*w).length;
        loop {
            w = w.offset(1);
            w;
            wcost = line_cost(w, len) + (*w).best_cost;
            if start == unused_word_type.as_mut_ptr()
                && last_line_length > 0 as libc::c_int
            {
                wcost
                    += ((len - last_line_length) * 10 as libc::c_int) as COST
                        * ((len - last_line_length) * 10 as libc::c_int) as COST
                        / 2 as libc::c_int as libc::c_long;
            }
            if wcost < best {
                best = wcost;
                (*start).next_break = w;
                (*start).line_length = len;
            }
            if w == word_limit {
                break;
            }
            len += (*w.offset(-(1 as libc::c_int as isize))).space + (*w).length;
            if !(len < max_width) {
                break;
            }
        }
        (*start).best_cost = best + base_cost(start);
        start = start.offset(-1);
        start;
    }
    (*word_limit).length = saved_length;
}
unsafe extern "C" fn check_punctuation(mut w: *mut WORD) {
    let mut start: *const libc::c_char = (*w).text;
    let mut finish: *const libc::c_char = start
        .offset(((*w).length - 1 as libc::c_int) as isize);
    let mut fin: libc::c_uchar = *finish as libc::c_uchar;
    (*w)
        .set_paren(
            (strchr(
                b"(['`\"\0" as *const u8 as *const libc::c_char,
                *start as libc::c_int,
            ) != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
                as libc::c_uint,
        );
    (*w)
        .set_punct(
            (*(*__ctype_b_loc()).offset(fin as libc::c_int as isize) as libc::c_int
                & _ISpunct as libc::c_int as libc::c_ushort as libc::c_int != 0)
                as libc::c_int as libc::c_uint,
        );
    while start < finish
        && !(strchr(
            b")]'\"\0" as *const u8 as *const libc::c_char,
            *finish as libc::c_int,
        ))
            .is_null()
    {
        finish = finish.offset(-1);
        finish;
    }
    (*w)
        .set_period(
            (strchr(b".?!\0" as *const u8 as *const libc::c_char, *finish as libc::c_int)
                != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
                as libc::c_uint,
        );
}
unsafe extern "C" fn base_cost(mut this: *mut WORD) -> COST {
    let mut cost: COST = 0;
    cost = 70 as libc::c_int as COST * 70 as libc::c_int as COST;
    if this > unused_word_type.as_mut_ptr() {
        if (*this.offset(-(1 as libc::c_int as isize))).period() != 0 {
            if (*this.offset(-(1 as libc::c_int as isize))).final_0() != 0 {
                cost -= 50 as libc::c_int as COST * 50 as libc::c_int as COST;
            } else {
                cost += 600 as libc::c_int as COST * 600 as libc::c_int as COST;
            }
        } else if (*this.offset(-(1 as libc::c_int as isize))).punct() != 0 {
            cost -= 40 as libc::c_int as COST * 40 as libc::c_int as COST;
        } else if this > unused_word_type.as_mut_ptr().offset(1 as libc::c_int as isize)
            && (*this.offset(-(2 as libc::c_int as isize))).final_0() as libc::c_int != 0
        {
            cost
                += 200 as libc::c_int as COST * 200 as libc::c_int as COST
                    / ((*this.offset(-(1 as libc::c_int as isize))).length
                        + 2 as libc::c_int) as libc::c_long;
        }
    }
    if (*this).paren() != 0 {
        cost -= 40 as libc::c_int as COST * 40 as libc::c_int as COST;
    } else if (*this).final_0() != 0 {
        cost
            += 150 as libc::c_int as COST * 150 as libc::c_int as COST
                / ((*this).length + 2 as libc::c_int) as libc::c_long;
    }
    return cost;
}
unsafe extern "C" fn line_cost(mut next: *mut WORD, mut len: libc::c_int) -> COST {
    let mut n: libc::c_int = 0;
    let mut cost: COST = 0;
    if next == word_limit {
        return 0 as libc::c_int as COST;
    }
    n = goal_width - len;
    cost = (n * 10 as libc::c_int) as COST * (n * 10 as libc::c_int) as COST;
    if (*next).next_break != word_limit {
        n = len - (*next).line_length;
        cost
            += (n * 10 as libc::c_int) as COST * (n * 10 as libc::c_int) as COST
                / 2 as libc::c_int as libc::c_long;
    }
    return cost;
}
unsafe extern "C" fn put_paragraph(mut finish: *mut WORD) {
    let mut w: *mut WORD = 0 as *mut WORD;
    put_line(unused_word_type.as_mut_ptr(), first_indent);
    w = (*unused_word_type.as_mut_ptr()).next_break;
    while w != finish {
        put_line(w, other_indent);
        w = (*w).next_break;
    }
}
unsafe extern "C" fn put_line(mut w: *mut WORD, mut indent: libc::c_int) {
    let mut endline: *mut WORD = 0 as *mut WORD;
    out_column = 0 as libc::c_int;
    put_space(prefix_indent);
    fputs_unlocked(prefix, stdout);
    out_column += prefix_length;
    put_space(indent - out_column);
    endline = ((*w).next_break).offset(-(1 as libc::c_int as isize));
    while w != endline {
        put_word(w);
        put_space((*w).space);
        w = w.offset(1);
        w;
    }
    put_word(w);
    last_line_length = out_column;
    putchar_unlocked('\n' as i32);
}
unsafe extern "C" fn put_word(mut w: *mut WORD) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: libc::c_int = 0;
    s = (*w).text;
    n = (*w).length;
    while n != 0 as libc::c_int {
        let fresh4 = s;
        s = s.offset(1);
        putchar_unlocked(*fresh4 as libc::c_int);
        n -= 1;
        n;
    }
    out_column += (*w).length;
}
unsafe extern "C" fn put_space(mut space: libc::c_int) {
    let mut space_target: libc::c_int = 0;
    let mut tab_target: libc::c_int = 0;
    space_target = out_column + space;
    if tabs {
        tab_target = space_target / 8 as libc::c_int * 8 as libc::c_int;
        if (out_column + 1 as libc::c_int) < tab_target {
            while out_column < tab_target {
                putchar_unlocked('\t' as i32);
                out_column = (out_column / 8 as libc::c_int + 1 as libc::c_int)
                    * 8 as libc::c_int;
            }
        }
    }
    while out_column < space_target {
        putchar_unlocked(' ' as i32);
        out_column += 1;
        out_column;
    }
}
static mut crown: bool = false;
static mut tagged: bool = false;
static mut split: bool = false;
static mut uniform: bool = false;
static mut prefix: *const libc::c_char = 0 as *const libc::c_char;
static mut max_width: libc::c_int = 0;
static mut prefix_full_length: libc::c_int = 0;
static mut prefix_lead_space: libc::c_int = 0;
static mut prefix_length: libc::c_int = 0;
static mut goal_width: libc::c_int = 0;
static mut in_column: libc::c_int = 0;
static mut out_column: libc::c_int = 0;
static mut parabuf: [libc::c_char; 5000] = [0; 5000];
static mut wptr: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut unused_word_type: [WORD; 1000] = [WORD {
    text: 0 as *const libc::c_char,
    length: 0,
    space: 0,
    paren_period_punct_final_0: [0; 1],
    c2rust_padding: [0; 3],
    line_length: 0,
    best_cost: 0,
    next_break: 0 as *const WORD as *mut WORD,
}; 1000];
static mut word_limit: *mut WORD = 0 as *const WORD as *mut WORD;
static mut tabs: bool = false;
static mut prefix_indent: libc::c_int = 0;
static mut first_indent: libc::c_int = 0;
static mut other_indent: libc::c_int = 0;
static mut next_char: libc::c_int = 0;
static mut next_prefix_indent: libc::c_int = 0;
static mut last_line_length: libc::c_int = 0;
static mut long_options: [option; 10] = [
    {
        let mut init = option {
            name: b"crown-margin\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"prefix\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"split-only\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"tagged-paragraph\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"uniform-spacing\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'u' as i32,
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
            name: b"goal\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'g' as i32,
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
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut optchar: libc::c_int = 0;
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut max_width_option: *const libc::c_char = 0 as *const libc::c_char;
    let mut goal_width_option: *const libc::c_char = 0 as *const libc::c_char;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    uniform = 0 as libc::c_int != 0;
    split = uniform;
    tagged = split;
    crown = tagged;
    max_width = 75 as libc::c_int;
    prefix = b"\0" as *const u8 as *const libc::c_char;
    prefix_full_length = 0 as libc::c_int;
    prefix_lead_space = prefix_full_length;
    prefix_length = prefix_lead_space;
    if argc > 1 as libc::c_int
        && *(*argv.offset(1 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int == '-' as i32
        && (*(*argv.offset(1 as libc::c_int as isize)).offset(1 as libc::c_int as isize)
            as libc::c_uint)
            .wrapping_sub('0' as i32 as libc::c_uint) <= 9 as libc::c_int as libc::c_uint
    {
        max_width_option = (*argv.offset(1 as libc::c_int as isize))
            .offset(1 as libc::c_int as isize);
        let ref mut fresh5 = *argv.offset(1 as libc::c_int as isize);
        *fresh5 = *argv.offset(0 as libc::c_int as isize);
        argv = argv.offset(1);
        argv;
        argc -= 1;
        argc;
    }
    loop {
        optchar = getopt_long(
            argc,
            argv,
            b"0123456789cstuw:p:g:\0" as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optchar != -(1 as libc::c_int)) {
            break;
        }
        match optchar {
            99 => {
                crown = 1 as libc::c_int != 0;
            }
            115 => {
                split = 1 as libc::c_int != 0;
            }
            116 => {
                tagged = 1 as libc::c_int != 0;
            }
            117 => {
                uniform = 1 as libc::c_int != 0;
            }
            119 => {
                max_width_option = optarg;
            }
            103 => {
                goal_width_option = optarg;
            }
            112 => {
                set_prefix(optarg);
            }
            -130 => {
                usage(0 as libc::c_int);
            }
            -131 => {
                version_etc(
                    stdout,
                    b"fmt\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Ross Paterson\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                if (optchar as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                    <= 9 as libc::c_int as libc::c_uint
                {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"invalid option -- %c; -WIDTH is recognized only when it is the first\noption; use -w N instead\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        optchar,
                    );
                }
                usage(1 as libc::c_int);
            }
        }
    }
    if !max_width_option.is_null() {
        max_width = xdectoumax(
            max_width_option,
            0 as libc::c_int as uintmax_t,
            (5000 as libc::c_int / 2 as libc::c_int) as uintmax_t,
            b"\0" as *const u8 as *const libc::c_char,
            gettext(b"invalid width\0" as *const u8 as *const libc::c_char),
            0 as libc::c_int,
        ) as libc::c_int;
    }
    if !goal_width_option.is_null() {
        goal_width = xdectoumax(
            goal_width_option,
            0 as libc::c_int as uintmax_t,
            max_width as uintmax_t,
            b"\0" as *const u8 as *const libc::c_char,
            gettext(b"invalid width\0" as *const u8 as *const libc::c_char),
            0 as libc::c_int,
        ) as libc::c_int;
        if max_width_option.is_null() {
            max_width = goal_width + 10 as libc::c_int;
        }
    } else {
        goal_width = max_width
            * (2 as libc::c_int * (100 as libc::c_int - 7 as libc::c_int)
                + 1 as libc::c_int) / 200 as libc::c_int;
    }
    let mut have_read_stdin: bool = 0 as libc::c_int != 0;
    if optind == argc {
        have_read_stdin = 1 as libc::c_int != 0;
        ok = fmt(stdin, b"-\0" as *const u8 as *const libc::c_char);
    } else {
        while optind < argc {
            let mut file: *mut libc::c_char = *argv.offset(optind as isize);
            if strcmp(file, b"-\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                ok = (ok as libc::c_int & fmt(stdin, file) as libc::c_int) as bool;
                have_read_stdin = 1 as libc::c_int != 0;
            } else {
                let mut in_stream: *mut FILE = 0 as *mut FILE;
                in_stream = fopen(file, b"r\0" as *const u8 as *const libc::c_char);
                if !in_stream.is_null() {
                    ok = (ok as libc::c_int & fmt(in_stream, file) as libc::c_int)
                        as bool;
                } else {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        gettext(
                            b"cannot open %s for reading\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, file),
                    );
                    ok = 0 as libc::c_int != 0;
                }
            }
            optind += 1;
            optind;
        }
    }
    if have_read_stdin as libc::c_int != 0 && rpl_fclose(stdin) != 0 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                gettext(b"closing standard input\0" as *const u8 as *const libc::c_char),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                gettext(b"closing standard input\0" as *const u8 as *const libc::c_char),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    return if ok as libc::c_int != 0 { 0 as libc::c_int } else { 1 as libc::c_int };
}
unsafe extern "C" fn set_other_indent(mut same_paragraph: bool) {
    if split {
        other_indent = first_indent;
    } else if crown {
        other_indent = if same_paragraph as libc::c_int != 0 {
            in_column
        } else {
            first_indent
        };
    } else if tagged {
        if same_paragraph as libc::c_int != 0 && in_column != first_indent {
            other_indent = in_column;
        } else if other_indent == first_indent {
            other_indent = if first_indent == 0 as libc::c_int {
                3 as libc::c_int
            } else {
                0 as libc::c_int
            };
        }
    } else {
        other_indent = first_indent;
    };
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
