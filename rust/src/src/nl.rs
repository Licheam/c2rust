use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
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
    fn xmalloc(s: size_t) -> *mut libc::c_void;
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
    fn exit(_: libc::c_int) -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    static mut rpl_re_syntax_options: reg_syntax_t;
    fn rpl_re_compile_pattern(
        __pattern: *const libc::c_char,
        __length: size_t,
        __buffer: *mut re_pattern_buffer,
    ) -> *const libc::c_char;
    fn rpl_re_search(
        __buffer: *mut re_pattern_buffer,
        __String: *const libc::c_char,
        __length: regoff_t,
        __start: regoff_t,
        __range: regoff_t,
        __regs: *mut re_registers,
    ) -> regoff_t;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn fadvise(fp: *mut FILE, advice: fadvice_t);
    fn initbuffer(linebuffer: *mut linebuffer);
    fn readlinebuffer(linebuffer: *mut linebuffer, stream: *mut FILE) -> *mut linebuffer;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn xdectoimax(
        n_str: *const libc::c_char,
        min: intmax_t,
        max: intmax_t,
        suffixes: *const libc::c_char,
        err: *const libc::c_char,
        err_exit: libc::c_int,
    ) -> intmax_t;
}
pub type size_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
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
pub type ptrdiff_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type intmax_t = __intmax_t;
pub type idx_t = ptrdiff_t;
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
pub type __re_size_t = size_t;
pub type __re_long_size_t = size_t;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regoff_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
pub type fadvice_t = libc::c_uint;
pub const FADVISE_RANDOM: fadvice_t = 1;
pub const FADVISE_WILLNEED: fadvice_t = 3;
pub const FADVISE_DONTNEED: fadvice_t = 4;
pub const FADVISE_NOREUSE: fadvice_t = 5;
pub const FADVISE_SEQUENTIAL: fadvice_t = 2;
pub const FADVISE_NORMAL: fadvice_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linebuffer {
    pub size: idx_t,
    pub length: idx_t,
    pub buffer: *mut libc::c_char,
}
pub type section = libc::c_uint;
pub const Text: section = 3;
pub const Footer: section = 2;
pub const Body: section = 1;
pub const Header: section = 0;
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
                b"Write each FILE to standard output, with line numbers added.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_stdin_note();
        emit_mandatory_arg_note();
        fputs_unlocked(
            gettext(
                b"  -b, --body-numbering=STYLE      use STYLE for numbering body lines\n  -d, --section-delimiter=CC      use CC for logical page delimiters\n  -f, --footer-numbering=STYLE    use STYLE for numbering footer lines\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -h, --header-numbering=STYLE    use STYLE for numbering header lines\n  -i, --line-increment=NUMBER     line number increment at each line\n  -l, --join-blank-lines=NUMBER   group of NUMBER empty lines counted as one\n  -n, --number-format=FORMAT      insert line numbers according to FORMAT\n  -p, --no-renumber               do not reset line numbers for each section\n  -s, --number-separator=STRING   add STRING after (possible) line number\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -v, --starting-line-number=NUMBER  first line number for each section\n  -w, --number-width=NUMBER       use NUMBER columns for line numbers\n\0"
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
                b"\nDefault options are: -bt -d'\\:' -fn -hn -i1 -l1 -n'rn' -s<TAB> -v1 -w6\n\nCC are two delimiter characters used to construct logical page delimiters;\na missing second character implies ':'.  As a GNU extension one can specify\nmore than two characters, and also specifying the empty string (-d '')\ndisables section matching.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"\nSTYLE is one of:\n\n  a      number all lines\n  t      number only nonempty lines\n  n      number no lines\n  pBRE   number only lines that contain a match for the basic regular\n         expression, BRE\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"\nFORMAT is one of:\n\n  ln     left justified, no leading zeros\n  rn     right justified, no leading zeros\n  rz     right justified, leading zeros\n\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_ancillary_info(b"nl\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
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
static mut FORMAT_RIGHT_NOLZ: [libc::c_char; 7] = unsafe {
    *::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"%*ld%s\0")
};
static mut FORMAT_RIGHT_LZ: [libc::c_char; 8] = unsafe {
    *::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"%0*ld%s\0")
};
static mut FORMAT_LEFT: [libc::c_char; 8] = unsafe {
    *::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"%-*ld%s\0")
};
static mut DEFAULT_SECTION_DELIMITERS: [libc::c_char; 3] = unsafe {
    *::core::mem::transmute::<&[u8; 3], &mut [libc::c_char; 3]>(b"\\:\0")
};
static mut body_type: *const libc::c_char = b"t\0" as *const u8 as *const libc::c_char;
static mut header_type: *const libc::c_char = b"n\0" as *const u8 as *const libc::c_char;
static mut footer_type: *const libc::c_char = b"n\0" as *const u8 as *const libc::c_char;
static mut current_type: *const libc::c_char = 0 as *const libc::c_char;
static mut body_regex: re_pattern_buffer = re_pattern_buffer {
    buffer: 0 as *const re_dfa_t as *mut re_dfa_t,
    allocated: 0,
    used: 0,
    syntax: 0,
    fastmap: 0 as *const libc::c_char as *mut libc::c_char,
    translate: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    re_nsub: 0,
    can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
    c2rust_padding: [0; 7],
};
static mut header_regex: re_pattern_buffer = re_pattern_buffer {
    buffer: 0 as *const re_dfa_t as *mut re_dfa_t,
    allocated: 0,
    used: 0,
    syntax: 0,
    fastmap: 0 as *const libc::c_char as *mut libc::c_char,
    translate: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    re_nsub: 0,
    can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
    c2rust_padding: [0; 7],
};
static mut footer_regex: re_pattern_buffer = re_pattern_buffer {
    buffer: 0 as *const re_dfa_t as *mut re_dfa_t,
    allocated: 0,
    used: 0,
    syntax: 0,
    fastmap: 0 as *const libc::c_char as *mut libc::c_char,
    translate: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    re_nsub: 0,
    can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
    c2rust_padding: [0; 7],
};
static mut body_fastmap: [libc::c_char; 256] = [0; 256];
static mut header_fastmap: [libc::c_char; 256] = [0; 256];
static mut footer_fastmap: [libc::c_char; 256] = [0; 256];
static mut current_regex: *mut re_pattern_buffer = 0 as *const re_pattern_buffer
    as *mut re_pattern_buffer;
static mut separator_str: *const libc::c_char = b"\t\0" as *const u8
    as *const libc::c_char;
static mut section_del: *mut libc::c_char = unsafe {
    DEFAULT_SECTION_DELIMITERS.as_ptr() as *mut _
};
static mut header_del: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut header_del_len: size_t = 0;
static mut body_del: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut body_del_len: size_t = 0;
static mut footer_del: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut footer_del_len: size_t = 0;
static mut line_buf: linebuffer = linebuffer {
    size: 0,
    length: 0,
    buffer: 0 as *const libc::c_char as *mut libc::c_char,
};
static mut print_no_line_fmt: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut starting_line_number: intmax_t = 1 as libc::c_int as intmax_t;
static mut page_incr: intmax_t = 1 as libc::c_int as intmax_t;
static mut reset_numbers: bool = 1 as libc::c_int != 0;
static mut blank_join: intmax_t = 1 as libc::c_int as intmax_t;
static mut lineno_width: libc::c_int = 6 as libc::c_int;
static mut lineno_format: *const libc::c_char = unsafe { FORMAT_RIGHT_NOLZ.as_ptr() };
static mut line_no: intmax_t = 0;
static mut line_no_overflow: bool = false;
static mut have_read_stdin: bool = false;
static mut longopts: [option; 14] = [
    {
        let mut init = option {
            name: b"header-numbering\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"body-numbering\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"footer-numbering\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"starting-line-number\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"line-increment\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-renumber\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"join-blank-lines\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"number-separator\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"number-width\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'w' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"number-format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"section-delimiter\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
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
unsafe extern "C" fn build_type_arg(
    mut typep: *mut *const libc::c_char,
    mut regexp: *mut re_pattern_buffer,
    mut fastmap: *mut libc::c_char,
) -> bool {
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    let mut rval: bool = 1 as libc::c_int != 0;
    match *optarg as libc::c_int {
        97 | 116 | 110 => {
            *typep = optarg;
        }
        112 => {
            let fresh0 = optarg;
            optarg = optarg.offset(1);
            *typep = fresh0;
            (*regexp).buffer = 0 as *mut re_dfa_t;
            (*regexp).allocated = 0 as libc::c_int as __re_long_size_t;
            (*regexp).fastmap = fastmap;
            (*regexp).translate = 0 as *mut libc::c_uchar;
            rpl_re_syntax_options = (((1 as libc::c_int as libc::c_ulong)
                << 1 as libc::c_int) << 1 as libc::c_int
                | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | ((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int
                | ((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                & !(((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                & !(((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int);
            errmsg = rpl_re_compile_pattern(optarg, strlen(optarg), regexp);
            if !errmsg.is_null() {
                if ::core::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        errmsg,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        errmsg,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
        }
        _ => {
            rval = 0 as libc::c_int != 0;
        }
    }
    return rval;
}
unsafe extern "C" fn print_lineno() {
    if line_no_overflow {
        if ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(b"line number overflow\0" as *const u8 as *const libc::c_char),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(b"line number overflow\0" as *const u8 as *const libc::c_char),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    printf(lineno_format, lineno_width, line_no, separator_str);
    let (fresh1, fresh2) = line_no.overflowing_add(page_incr);
    *(&mut line_no as *mut intmax_t) = fresh1;
    if fresh2 {
        line_no_overflow = 1 as libc::c_int != 0;
    }
}
unsafe extern "C" fn reset_lineno() {
    if reset_numbers {
        line_no = starting_line_number;
        line_no_overflow = 0 as libc::c_int != 0;
    }
}
unsafe extern "C" fn proc_header() {
    current_type = header_type;
    current_regex = &mut header_regex;
    reset_lineno();
    putchar_unlocked('\n' as i32);
}
unsafe extern "C" fn proc_body() {
    current_type = body_type;
    current_regex = &mut body_regex;
    reset_lineno();
    putchar_unlocked('\n' as i32);
}
unsafe extern "C" fn proc_footer() {
    current_type = footer_type;
    current_regex = &mut footer_regex;
    reset_lineno();
    putchar_unlocked('\n' as i32);
}
unsafe extern "C" fn proc_text() {
    static mut blank_lines: intmax_t = 0 as libc::c_int as intmax_t;
    match *current_type as libc::c_int {
        97 => {
            if blank_join > 1 as libc::c_int as libc::c_long {
                if (1 as libc::c_int as libc::c_long) < line_buf.length
                    || {
                        blank_lines += 1;
                        blank_lines == blank_join
                    }
                {
                    print_lineno();
                    blank_lines = 0 as libc::c_int as intmax_t;
                } else {
                    fputs_unlocked(print_no_line_fmt, stdout);
                }
            } else {
                print_lineno();
            }
        }
        116 => {
            if (1 as libc::c_int as libc::c_long) < line_buf.length {
                print_lineno();
            } else {
                fputs_unlocked(print_no_line_fmt, stdout);
            }
        }
        110 => {
            fputs_unlocked(print_no_line_fmt, stdout);
        }
        112 => {
            let mut current_block_14: u64;
            match rpl_re_search(
                current_regex,
                line_buf.buffer,
                line_buf.length - 1 as libc::c_int as libc::c_long,
                0 as libc::c_int as regoff_t,
                line_buf.length - 1 as libc::c_int as libc::c_long,
                0 as *mut re_registers,
            ) {
                -2 => {
                    if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            gettext(
                                b"error in regular expression search\0" as *const u8
                                    as *const libc::c_char,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            gettext(
                                b"error in regular expression search\0" as *const u8
                                    as *const libc::c_char,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                    current_block_14 = 4950872042656359461;
                }
                -1 => {
                    current_block_14 = 4950872042656359461;
                }
                _ => {
                    print_lineno();
                    current_block_14 = 4956146061682418353;
                }
            }
            match current_block_14 {
                4950872042656359461 => {
                    fputs_unlocked(print_no_line_fmt, stdout);
                }
                _ => {}
            }
        }
        _ => {}
    }
    fwrite_unlocked(
        line_buf.buffer as *const libc::c_void,
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        line_buf.length as size_t,
        stdout,
    );
}
unsafe extern "C" fn check_section() -> section {
    let mut len: size_t = (line_buf.length - 1 as libc::c_int as libc::c_long) as size_t;
    if len < 2 as libc::c_int as libc::c_ulong
        || footer_del_len < 2 as libc::c_int as libc::c_ulong
        || memcmp(
            line_buf.buffer as *const libc::c_void,
            section_del as *const libc::c_void,
            2 as libc::c_int as libc::c_ulong,
        ) != 0
    {
        return Text;
    }
    if len == header_del_len
        && memcmp(
            line_buf.buffer as *const libc::c_void,
            header_del as *const libc::c_void,
            header_del_len,
        ) == 0
    {
        return Header;
    }
    if len == body_del_len
        && memcmp(
            line_buf.buffer as *const libc::c_void,
            body_del as *const libc::c_void,
            body_del_len,
        ) == 0
    {
        return Body;
    }
    if len == footer_del_len
        && memcmp(
            line_buf.buffer as *const libc::c_void,
            footer_del as *const libc::c_void,
            footer_del_len,
        ) == 0
    {
        return Footer;
    }
    return Text;
}
unsafe extern "C" fn process_file(mut fp: *mut FILE) {
    while !(readlinebuffer(&mut line_buf, fp)).is_null() {
        match check_section() as libc::c_uint {
            0 => {
                proc_header();
            }
            1 => {
                proc_body();
            }
            2 => {
                proc_footer();
            }
            3 => {
                proc_text();
            }
            _ => {}
        }
    }
}
unsafe extern "C" fn nl_file(mut file: *const libc::c_char) -> bool {
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
    process_file(stream);
    let mut err: libc::c_int = *__errno_location();
    if ferror_unlocked(stream) == 0 {
        err = 0 as libc::c_int;
    }
    if strcmp(file, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        clearerr_unlocked(stream);
    } else if rpl_fclose(stream) != 0 as libc::c_int && err == 0 {
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
    let mut c: libc::c_int = 0;
    let mut len: size_t = 0;
    let mut ok: bool = 1 as libc::c_int != 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    have_read_stdin = 0 as libc::c_int != 0;
    loop {
        c = getopt_long(
            argc,
            argv,
            b"h:b:f:v:i:pl:s:w:n:d:\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            104 => {
                if !build_type_arg(
                    &mut header_type,
                    &mut header_regex,
                    header_fastmap.as_mut_ptr(),
                ) {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"invalid header numbering style: %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quote(optarg),
                    );
                    ok = 0 as libc::c_int != 0;
                }
            }
            98 => {
                if !build_type_arg(
                    &mut body_type,
                    &mut body_regex,
                    body_fastmap.as_mut_ptr(),
                ) {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"invalid body numbering style: %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quote(optarg),
                    );
                    ok = 0 as libc::c_int != 0;
                }
            }
            102 => {
                if !build_type_arg(
                    &mut footer_type,
                    &mut footer_regex,
                    footer_fastmap.as_mut_ptr(),
                ) {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"invalid footer numbering style: %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quote(optarg),
                    );
                    ok = 0 as libc::c_int != 0;
                }
            }
            118 => {
                starting_line_number = xdectoimax(
                    optarg,
                    -(9223372036854775807 as libc::c_long)
                        - 1 as libc::c_int as libc::c_long,
                    9223372036854775807 as libc::c_long,
                    b"\0" as *const u8 as *const libc::c_char,
                    gettext(
                        b"invalid starting line number\0" as *const u8
                            as *const libc::c_char,
                    ),
                    0 as libc::c_int,
                );
            }
            105 => {
                page_incr = xdectoimax(
                    optarg,
                    -(9223372036854775807 as libc::c_long)
                        - 1 as libc::c_int as libc::c_long,
                    9223372036854775807 as libc::c_long,
                    b"\0" as *const u8 as *const libc::c_char,
                    gettext(
                        b"invalid line number increment\0" as *const u8
                            as *const libc::c_char,
                    ),
                    0 as libc::c_int,
                );
            }
            112 => {
                reset_numbers = 0 as libc::c_int != 0;
            }
            108 => {
                blank_join = xdectoimax(
                    optarg,
                    1 as libc::c_int as intmax_t,
                    9223372036854775807 as libc::c_long,
                    b"\0" as *const u8 as *const libc::c_char,
                    gettext(
                        b"invalid line number of blank lines\0" as *const u8
                            as *const libc::c_char,
                    ),
                    0 as libc::c_int,
                );
            }
            115 => {
                separator_str = optarg;
            }
            119 => {
                lineno_width = xdectoimax(
                    optarg,
                    1 as libc::c_int as intmax_t,
                    2147483647 as libc::c_int as intmax_t,
                    b"\0" as *const u8 as *const libc::c_char,
                    gettext(
                        b"invalid line number field width\0" as *const u8
                            as *const libc::c_char,
                    ),
                    0 as libc::c_int,
                ) as libc::c_int;
            }
            110 => {
                if strcmp(optarg, b"ln\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    lineno_format = FORMAT_LEFT.as_ptr();
                } else if strcmp(optarg, b"rn\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    lineno_format = FORMAT_RIGHT_NOLZ.as_ptr();
                } else if strcmp(optarg, b"rz\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    lineno_format = FORMAT_RIGHT_LZ.as_ptr();
                } else {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"invalid line numbering format: %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quote(optarg),
                    );
                    ok = 0 as libc::c_int != 0;
                }
            }
            100 => {
                len = strlen(optarg);
                if len == 1 as libc::c_int as libc::c_ulong
                    || len == 2 as libc::c_int as libc::c_ulong
                {
                    let mut p: *mut libc::c_char = section_del;
                    while *optarg != 0 {
                        let fresh3 = optarg;
                        optarg = optarg.offset(1);
                        let fresh4 = p;
                        p = p.offset(1);
                        *fresh4 = *fresh3;
                    }
                } else {
                    section_del = optarg;
                }
            }
            -130 => {
                usage(0 as libc::c_int);
            }
            -131 => {
                version_etc(
                    stdout,
                    b"nl\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Scott Bartram\0" as *const u8 as *const libc::c_char,
                    b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                ok = 0 as libc::c_int != 0;
            }
        }
    }
    if !ok {
        usage(1 as libc::c_int);
    }
    len = strlen(section_del);
    header_del_len = len.wrapping_mul(3 as libc::c_int as libc::c_ulong);
    header_del = xmalloc(header_del_len.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    stpcpy(stpcpy(stpcpy(header_del, section_del), section_del), section_del);
    body_del_len = len.wrapping_mul(2 as libc::c_int as libc::c_ulong);
    body_del = header_del.offset(len as isize);
    footer_del_len = len;
    footer_del = body_del.offset(len as isize);
    initbuffer(&mut line_buf);
    len = strlen(separator_str);
    print_no_line_fmt = xmalloc(
        (lineno_width as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    memset(
        print_no_line_fmt as *mut libc::c_void,
        ' ' as i32,
        (lineno_width as libc::c_ulong).wrapping_add(len),
    );
    *print_no_line_fmt
        .offset(
            (lineno_width as libc::c_ulong).wrapping_add(len) as isize,
        ) = '\0' as i32 as libc::c_char;
    line_no = starting_line_number;
    current_type = body_type;
    current_regex = &mut body_regex;
    if optind == argc {
        ok = nl_file(b"-\0" as *const u8 as *const libc::c_char);
    } else {
        while optind < argc {
            ok = (ok as libc::c_int
                & nl_file(*argv.offset(optind as isize)) as libc::c_int) as bool;
            optind += 1;
            optind;
        }
    }
    if have_read_stdin as libc::c_int != 0 && rpl_fclose(stdin) == -(1 as libc::c_int) {
        if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
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
