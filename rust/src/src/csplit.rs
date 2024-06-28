use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn sigismember(__set: *const sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn rawmemchr(__s: *const libc::c_void, __c: libc::c_int) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
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
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    static mut stdout: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn ferror_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn x2nrealloc(p: *mut libc::c_void, pn: *mut size_t, s: size_t) -> *mut libc::c_void;
    fn xmemdup(p: *const libc::c_void, s: size_t) -> *mut libc::c_void;
    static mut stderr: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
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
    fn fd_reopen(
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: mode_t,
    ) -> libc::c_int;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn fopen_safer(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn xdectoimax(
        n_str: *const libc::c_char,
        min: intmax_t,
        max: intmax_t,
        suffixes: *const libc::c_char,
        err: *const libc::c_char,
        err_exit: libc::c_int,
    ) -> intmax_t;
    fn xstrtoimax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut intmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
    fn xstrtoumax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut uintmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type __uint32_t = libc::c_uint;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
pub type ptrdiff_t = libc::c_long;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type C2RustUnnamed_10 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_10 = 8;
pub const _ISpunct: C2RustUnnamed_10 = 4;
pub const _IScntrl: C2RustUnnamed_10 = 2;
pub const _ISblank: C2RustUnnamed_10 = 1;
pub const _ISgraph: C2RustUnnamed_10 = 32768;
pub const _ISprint: C2RustUnnamed_10 = 16384;
pub const _ISspace: C2RustUnnamed_10 = 8192;
pub const _ISxdigit: C2RustUnnamed_10 = 4096;
pub const _ISdigit: C2RustUnnamed_10 = 2048;
pub const _ISalpha: C2RustUnnamed_10 = 1024;
pub const _ISlower: C2RustUnnamed_10 = 512;
pub const _ISupper: C2RustUnnamed_10 = 256;
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
pub type FILE = _IO_FILE;
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
pub type C2RustUnnamed_11 = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed_11 = -131;
pub const GETOPT_HELP_CHAR: C2RustUnnamed_11 = -130;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct infomap {
    pub program: *const libc::c_char,
    pub node: *const libc::c_char,
}
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
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct control {
    pub offset: intmax_t,
    pub lines_required: uintmax_t,
    pub repeat: uintmax_t,
    pub argnum: libc::c_int,
    pub repeat_forever: bool,
    pub ignore: bool,
    pub regexpr: bool,
    pub re_compiled: re_pattern_buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cstring {
    pub len: size_t,
    pub str_0: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line {
    pub used: size_t,
    pub insert_index: size_t,
    pub retrieve_index: size_t,
    pub starts: [cstring; 80],
    pub next: *mut line,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer_record {
    pub bytes_alloc: size_t,
    pub bytes_used: size_t,
    pub start_line: uintmax_t,
    pub first_available: uintmax_t,
    pub num_lines: size_t,
    pub buffer: *mut libc::c_char,
    pub line_start: *mut line,
    pub curr_line: *mut line,
    pub next: *mut buffer_record,
}
pub type C2RustUnnamed_12 = libc::c_uint;
pub const SUPPRESS_MATCHED_OPTION: C2RustUnnamed_12 = 128;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub _gl_dummy: libc::c_int,
}
pub type C2RustUnnamed_23 = libc::c_uint;
pub const FLAG_ALTERNATIVE: C2RustUnnamed_23 = 2;
pub const FLAG_THOUSANDS: C2RustUnnamed_23 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub _gl_dummy: libc::c_int,
}
pub const nsigs: C2RustUnnamed_30 = 11;
pub type C2RustUnnamed_30 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn xalloc_die() {
    error(
        0 as libc::c_int,
        0 as libc::c_int,
        b"%s\0" as *const u8 as *const libc::c_char,
        gettext(b"memory exhausted\0" as *const u8 as *const libc::c_char),
    );
    cleanup_fatal();
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
                b"Usage: %s [OPTION]... FILE PATTERN...\n\0" as *const u8
                    as *const libc::c_char,
            ),
            program_name,
        );
        fputs_unlocked(
            gettext(
                b"Output pieces of FILE separated by PATTERN(s) to files 'xx00', 'xx01', ...,\nand output byte counts of each piece to standard output.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"\nRead standard input if FILE is -\n\0" as *const u8
                    as *const libc::c_char,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            gettext(
                b"  -b, --suffix-format=FORMAT  use sprintf FORMAT instead of %02d\n  -f, --prefix=PREFIX        use PREFIX instead of 'xx'\n  -k, --keep-files           do not remove output files on errors\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"      --suppress-matched     suppress the lines matching PATTERN\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -n, --digits=DIGITS        use specified number of digits instead of 2\n  -s, --quiet, --silent      do not print counts of output file sizes\n  -z, --elide-empty-files    remove empty output files\n\0"
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
                b"\nEach PATTERN may be:\n  INTEGER            copy up to but not including specified line number\n  /REGEXP/[OFFSET]   copy up to but not including a matching line\n  %REGEXP%[OFFSET]   skip to, but not including a matching line\n  {INTEGER}          repeat the previous pattern specified number of times\n  {*}                repeat the previous pattern as many times as possible\n\nA line OFFSET is a required '+' or '-' followed by a positive integer.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_ancillary_info(b"csplit\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn close_output_file() {
    if !output_stream.is_null() {
        if ferror_unlocked(output_stream) != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(b"write error for %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, output_filename),
            );
            output_stream = 0 as *mut FILE;
            cleanup_fatal();
        }
        if rpl_fclose(output_stream) != 0 as libc::c_int {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    output_filename,
                ),
            );
            output_stream = 0 as *mut FILE;
            cleanup_fatal();
        }
        if bytes_written == 0 as libc::c_int as libc::c_ulong
            && elide_empty_files as libc::c_int != 0
        {
            let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
            let mut unlink_ok: bool = false;
            let mut unlink_errno: libc::c_int = 0;
            sigprocmask(0 as libc::c_int, &mut caught_signals, &mut oldset);
            unlink_ok = unlink(output_filename) == 0 as libc::c_int;
            unlink_errno = *__errno_location();
            ::core::ptr::write_volatile(
                &mut files_created as *mut libc::c_uint,
                (::core::ptr::read_volatile::<
                    libc::c_uint,
                >(&files_created as *const libc::c_uint))
                    .wrapping_sub(unlink_ok as libc::c_uint),
            );
            sigprocmask(2 as libc::c_int, &mut oldset, 0 as *mut sigset_t);
            if !unlink_ok {
                error(
                    0 as libc::c_int,
                    unlink_errno,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        output_filename,
                    ),
                );
            }
        } else if !suppress_count {
            let mut buf: [libc::c_char; 21] = [0; 21];
            fprintf(
                stdout,
                b"%s\n\0" as *const u8 as *const libc::c_char,
                umaxtostr(bytes_written, buf.as_mut_ptr()),
            );
        }
        output_stream = 0 as *mut FILE;
    }
}
unsafe extern "C" fn create_output_file() {
    let mut fopen_ok: bool = false;
    let mut fopen_errno: libc::c_int = 0;
    output_filename = make_filename(files_created);
    if files_created
        == (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint)
    {
        fopen_ok = 0 as libc::c_int != 0;
        fopen_errno = 75 as libc::c_int;
    } else {
        let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
        sigprocmask(0 as libc::c_int, &mut caught_signals, &mut oldset);
        output_stream = fopen_safer(
            output_filename,
            b"w\0" as *const u8 as *const libc::c_char,
        );
        fopen_ok = !output_stream.is_null();
        fopen_errno = *__errno_location();
        ::core::ptr::write_volatile(
            &mut files_created as *mut libc::c_uint,
            (::core::ptr::read_volatile::<
                libc::c_uint,
            >(&files_created as *const libc::c_uint))
                .wrapping_add(fopen_ok as libc::c_uint),
        );
        sigprocmask(2 as libc::c_int, &mut oldset, 0 as *mut sigset_t);
    }
    if !fopen_ok {
        error(
            0 as libc::c_int,
            fopen_errno,
            b"%s\0" as *const u8 as *const libc::c_char,
            quotearg_n_style_colon(
                0 as libc::c_int,
                shell_escape_quoting_style,
                output_filename,
            ),
        );
        cleanup_fatal();
    }
    bytes_written = 0 as libc::c_int as uintmax_t;
}
unsafe extern "C" fn delete_all_files(mut in_signal_handler: bool) {
    if !remove_files {
        return;
    }
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < files_created {
        let mut name: *const libc::c_char = make_filename(i);
        if unlink(name) != 0 as libc::c_int && !in_signal_handler {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    name,
                ),
            );
        }
        i = i.wrapping_add(1);
        i;
    }
    ::core::ptr::write_volatile(
        &mut files_created as *mut libc::c_uint,
        0 as libc::c_int as libc::c_uint,
    );
}
unsafe extern "C" fn save_line_to_file(mut line: *const cstring) {
    let mut l: size_t = fwrite_unlocked(
        (*line).str_0 as *const libc::c_void,
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        (*line).len,
        output_stream,
    );
    if l != (*line).len {
        error(
            0 as libc::c_int,
            *__errno_location(),
            gettext(b"write error for %s\0" as *const u8 as *const libc::c_char),
            quotearg_style(shell_escape_always_quoting_style, output_filename),
        );
        output_stream = 0 as *mut FILE;
        cleanup_fatal();
    }
    bytes_written = (bytes_written as libc::c_ulong).wrapping_add((*line).len)
        as uintmax_t as uintmax_t;
}
static mut head: *mut buffer_record = 0 as *const buffer_record as *mut buffer_record;
static mut hold_area: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut hold_count: size_t = 0 as libc::c_int as size_t;
static mut last_line_number: uintmax_t = 0 as libc::c_int as uintmax_t;
static mut current_line: uintmax_t = 0 as libc::c_int as uintmax_t;
static mut have_read_eof: bool = 0 as libc::c_int != 0;
static mut filename_space: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut prefix: *const libc::c_char = 0 as *const libc::c_char;
static mut suffix: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut digits: libc::c_int = 2 as libc::c_int;
static mut files_created: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut bytes_written: uintmax_t = 0;
static mut output_stream: *mut FILE = 0 as *const FILE as *mut FILE;
static mut output_filename: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut global_argv: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut suppress_count: bool = false;
static mut remove_files: bool = false;
static mut elide_empty_files: bool = false;
static mut suppress_matched: bool = false;
static mut controls: *mut control = 0 as *const control as *mut control;
static mut control_used: size_t = 0;
static mut caught_signals: sigset_t = sigset_t { __val: [0; 16] };
static mut longopts: [option; 11] = [
    {
        let mut init = option {
            name: b"digits\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"quiet\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"silent\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"keep-files\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'k' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"elide-empty-files\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'z' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"prefix\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"suffix-format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"suppress-matched\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: SUPPRESS_MATCHED_OPTION as libc::c_int,
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
unsafe extern "C" fn cleanup() {
    let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
    close_output_file();
    sigprocmask(0 as libc::c_int, &mut caught_signals, &mut oldset);
    delete_all_files(0 as libc::c_int != 0);
    sigprocmask(2 as libc::c_int, &mut oldset, 0 as *mut sigset_t);
}
unsafe extern "C" fn cleanup_fatal() -> ! {
    cleanup();
    exit(1 as libc::c_int);
}
unsafe extern "C" fn interrupt_handler(mut sig: libc::c_int) {
    delete_all_files(1 as libc::c_int != 0);
    signal(sig, None);
    raise(sig);
}
unsafe extern "C" fn save_to_hold_area(mut start: *mut libc::c_char, mut num: size_t) {
    free(hold_area as *mut libc::c_void);
    hold_area = start;
    hold_count = num;
}
unsafe extern "C" fn read_input(
    mut dest: *mut libc::c_char,
    mut max_n_bytes: size_t,
) -> size_t {
    let mut bytes_read: size_t = 0;
    if max_n_bytes == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t;
    }
    bytes_read = safe_read(0 as libc::c_int, dest as *mut libc::c_void, max_n_bytes);
    if bytes_read == 0 as libc::c_int as libc::c_ulong {
        have_read_eof = 1 as libc::c_int != 0;
    }
    if bytes_read == -(1 as libc::c_int) as size_t {
        error(
            0 as libc::c_int,
            *__errno_location(),
            gettext(b"read error\0" as *const u8 as *const libc::c_char),
        );
        cleanup_fatal();
    }
    return bytes_read;
}
unsafe extern "C" fn clear_line_control(mut p: *mut line) {
    (*p).used = 0 as libc::c_int as size_t;
    (*p).insert_index = 0 as libc::c_int as size_t;
    (*p).retrieve_index = 0 as libc::c_int as size_t;
}
unsafe extern "C" fn new_line_control() -> *mut line {
    let mut p: *mut line = xmalloc(::core::mem::size_of::<line>() as libc::c_ulong)
        as *mut line;
    (*p).next = 0 as *mut line;
    clear_line_control(p);
    return p;
}
unsafe extern "C" fn keep_new_line(
    mut b: *mut buffer_record,
    mut line_start: *mut libc::c_char,
    mut line_len: size_t,
) {
    let mut l: *mut line = 0 as *mut line;
    if ((*b).line_start).is_null() {
        (*b).curr_line = new_line_control();
        (*b).line_start = (*b).curr_line;
    }
    if (*(*b).curr_line).used == 80 as libc::c_int as libc::c_ulong {
        (*(*b).curr_line).next = new_line_control();
        (*b).curr_line = (*(*b).curr_line).next;
    }
    l = (*b).curr_line;
    (*l).starts[(*l).insert_index as usize].str_0 = line_start;
    (*l).starts[(*l).insert_index as usize].len = line_len;
    (*l).used = ((*l).used).wrapping_add(1);
    (*l).used;
    (*l).insert_index = ((*l).insert_index).wrapping_add(1);
    (*l).insert_index;
}
unsafe extern "C" fn record_line_starts(mut b: *mut buffer_record) -> size_t {
    let mut line_start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lines: size_t = 0;
    let mut line_length: size_t = 0;
    if (*b).bytes_used == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t;
    }
    lines = 0 as libc::c_int as size_t;
    line_start = (*b).buffer;
    let mut buffer_end: *mut libc::c_char = line_start.offset((*b).bytes_used as isize);
    *buffer_end = '\n' as i32 as libc::c_char;
    loop {
        let mut line_end: *mut libc::c_char = rawmemchr(
            line_start as *const libc::c_void,
            '\n' as i32,
        ) as *mut libc::c_char;
        if line_end == buffer_end {
            break;
        }
        line_length = (line_end.offset_from(line_start) as libc::c_long
            + 1 as libc::c_int as libc::c_long) as size_t;
        keep_new_line(b, line_start, line_length);
        line_start = line_end.offset(1 as libc::c_int as isize);
        lines = lines.wrapping_add(1);
        lines;
    }
    let mut bytes_left: idx_t = buffer_end.offset_from(line_start) as libc::c_long;
    if bytes_left != 0 {
        if have_read_eof {
            keep_new_line(b, line_start, bytes_left as size_t);
            lines = lines.wrapping_add(1);
            lines;
        } else {
            save_to_hold_area(
                xmemdup(line_start as *const libc::c_void, bytes_left as size_t)
                    as *mut libc::c_char,
                bytes_left as size_t,
            );
        }
    }
    (*b).num_lines = lines;
    (*b).start_line = last_line_number.wrapping_add(1 as libc::c_int as libc::c_ulong);
    (*b).first_available = (*b).start_line;
    last_line_number = (last_line_number as libc::c_ulong).wrapping_add(lines)
        as uintmax_t as uintmax_t;
    return lines;
}
unsafe extern "C" fn create_new_buffer(mut size: size_t) -> *mut buffer_record {
    let mut new_buffer: *mut buffer_record = xmalloc(
        ::core::mem::size_of::<buffer_record>() as libc::c_ulong,
    ) as *mut buffer_record;
    (*new_buffer)
        .buffer = xmalloc(size.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    (*new_buffer).bytes_alloc = size;
    (*new_buffer).curr_line = 0 as *mut line;
    (*new_buffer).line_start = (*new_buffer).curr_line;
    return new_buffer;
}
unsafe extern "C" fn get_new_buffer(mut min_size: size_t) -> *mut buffer_record {
    let mut new_buffer: *mut buffer_record = 0 as *mut buffer_record;
    let mut alloc_size: size_t = 0;
    alloc_size = 8191 as libc::c_int as size_t;
    if alloc_size < min_size {
        let mut s: size_t = min_size
            .wrapping_sub(alloc_size)
            .wrapping_add(2048 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let (fresh0, fresh1) = alloc_size
            .overflowing_add(
                s.wrapping_sub(s.wrapping_rem(2048 as libc::c_int as libc::c_ulong)),
            );
        *(&mut alloc_size as *mut size_t) = fresh0;
        if fresh1 {
            xalloc_die();
        }
    }
    new_buffer = create_new_buffer(alloc_size);
    (*new_buffer).num_lines = 0 as libc::c_int as size_t;
    (*new_buffer).bytes_used = 0 as libc::c_int as size_t;
    (*new_buffer)
        .first_available = last_line_number
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    (*new_buffer).start_line = (*new_buffer).first_available;
    (*new_buffer).next = 0 as *mut buffer_record;
    return new_buffer;
}
unsafe extern "C" fn free_buffer(mut buf: *mut buffer_record) {
    let mut l: *mut line = 0 as *mut line;
    l = (*buf).line_start;
    while !l.is_null() {
        let mut n: *mut line = (*l).next;
        free(l as *mut libc::c_void);
        l = n;
    }
    (*buf).line_start = 0 as *mut line;
    free((*buf).buffer as *mut libc::c_void);
    (*buf).buffer = 0 as *mut libc::c_char;
}
unsafe extern "C" fn save_buffer(mut buf: *mut buffer_record) {
    let mut p: *mut buffer_record = 0 as *mut buffer_record;
    (*buf).next = 0 as *mut buffer_record;
    (*buf).curr_line = (*buf).line_start;
    if head.is_null() {
        head = buf;
    } else {
        p = head;
        while !((*p).next).is_null() {
            p = (*p).next;
        }
        (*p).next = buf;
    };
}
unsafe extern "C" fn load_buffer() -> bool {
    let mut b: *mut buffer_record = 0 as *mut buffer_record;
    let mut bytes_wanted: size_t = 8191 as libc::c_int as size_t;
    let mut bytes_avail: size_t = 0;
    let mut lines_found: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if have_read_eof {
        return 0 as libc::c_int != 0;
    }
    if bytes_wanted <= hold_count {
        bytes_wanted = hold_count.wrapping_add(1 as libc::c_int as libc::c_ulong);
    }
    loop {
        b = get_new_buffer(bytes_wanted);
        bytes_avail = (*b).bytes_alloc;
        p = (*b).buffer;
        if hold_count != 0 {
            memcpy(p as *mut libc::c_void, hold_area as *const libc::c_void, hold_count);
            p = p.offset(hold_count as isize);
            (*b)
                .bytes_used = ((*b).bytes_used as libc::c_ulong).wrapping_add(hold_count)
                as size_t as size_t;
            bytes_avail = (bytes_avail as libc::c_ulong).wrapping_sub(hold_count)
                as size_t as size_t;
            hold_count = 0 as libc::c_int as size_t;
        }
        (*b)
            .bytes_used = ((*b).bytes_used as libc::c_ulong)
            .wrapping_add(
                read_input(
                    p,
                    bytes_avail.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ),
            ) as size_t as size_t;
        lines_found = record_line_starts(b);
        if lines_found != 0 || have_read_eof as libc::c_int != 0 {
            break;
        }
        if if ::core::mem::size_of::<size_t>() as libc::c_ulong
            == ::core::mem::size_of::<libc::c_schar>() as libc::c_ulong
        {
            if !((0 as libc::c_int as size_t) < -(1 as libc::c_int) as size_t) {
                if if (2 as libc::c_int) < 0 as libc::c_int {
                    if (*b).bytes_alloc < 0 as libc::c_int as libc::c_ulong {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                127 as libc::c_int
                            }) + 2 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            ((*b).bytes_alloc
                                < (127 as libc::c_int / 2 as libc::c_int) as libc::c_ulong)
                                as libc::c_int
                        } else {
                            ((if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) + 1 as libc::c_int)
                                    << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) + 0 as libc::c_int
                            }) < 0 as libc::c_int
                            {
                                ((2 as libc::c_int)
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        2 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            2 as libc::c_int
                                        }) + 1 as libc::c_int)
                                            << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            2 as libc::c_int
                                        }) - 1 as libc::c_int
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int) < 2 as libc::c_int) as libc::c_int
                            }) != 0
                            {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) + 127 as libc::c_int
                                    >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            } else {
                                127 as libc::c_int / -(2 as libc::c_int)
                            }) as libc::c_ulong
                                <= (-(1 as libc::c_int) as libc::c_ulong)
                                    .wrapping_sub((*b).bytes_alloc)) as libc::c_int
                        }
                    } else if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                        }) + 1 as libc::c_int)
                            << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                        }) + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        2 as libc::c_int
                                    }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                }) + 1 as libc::c_int)
                                    << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        2 as libc::c_int
                                    }) + (-(127 as libc::c_int) - 1 as libc::c_int)
                                }) - 1 as libc::c_int
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + (-(127 as libc::c_int) - 1 as libc::c_int))
                            as libc::c_int
                    }) != 0 && 2 as libc::c_int == -(1 as libc::c_int)
                    {
                        if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            ((0 as libc::c_int as libc::c_ulong)
                                < ((*b).bytes_alloc)
                                    .wrapping_add(
                                        (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                    )) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_ulong) < (*b).bytes_alloc
                                && ((-(1 as libc::c_int)
                                    - (-(127 as libc::c_int) - 1 as libc::c_int))
                                    as libc::c_ulong)
                                    < ((*b).bytes_alloc)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                as libc::c_int
                        }
                    } else {
                        ((((-(127 as libc::c_int) - 1 as libc::c_int) / 2 as libc::c_int)
                            as libc::c_ulong) < (*b).bytes_alloc) as libc::c_int
                    }
                } else if 2 as libc::c_int == 0 as libc::c_int {
                    0 as libc::c_int
                } else if (*b).bytes_alloc < 0 as libc::c_int as libc::c_ulong {
                    if (if (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_add(
                                (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                            )
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        !((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (*b).bytes_alloc
                            })
                                .wrapping_add(
                                    (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                )
                        })
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            << (::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (*b).bytes_alloc
                            })
                                .wrapping_add(
                                    (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                )
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    }) < 0 as libc::c_int as libc::c_ulong
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_add(
                                (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                            )
                            < (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (*b).bytes_alloc
                                })
                                    .wrapping_add(
                                        (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                    )
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (*b).bytes_alloc
                                    })
                                        .wrapping_add(
                                            (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                        )
                                })
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    << (::core::mem::size_of::<libc::c_ulong>()
                                        as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (*b).bytes_alloc
                                    })
                                        .wrapping_add(
                                            (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                        )
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_neg()) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (*b).bytes_alloc
                            })
                                .wrapping_add(
                                    (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong,
                                )) as libc::c_int
                    }) != 0 && (*b).bytes_alloc == -(1 as libc::c_int) as libc::c_ulong
                    {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            ((0 as libc::c_int)
                                < 2 as libc::c_int
                                    + (-(127 as libc::c_int) - 1 as libc::c_int)) as libc::c_int
                        } else {
                            (-(1 as libc::c_int)
                                - (-(127 as libc::c_int) - 1 as libc::c_int)
                                < 2 as libc::c_int - 1 as libc::c_int) as libc::c_int
                        }
                    } else {
                        (((-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_div((*b).bytes_alloc)
                            < 2 as libc::c_int as libc::c_ulong) as libc::c_int
                    }
                } else {
                    (((127 as libc::c_int / 2 as libc::c_int) as libc::c_ulong)
                        < (*b).bytes_alloc) as libc::c_int
                } != 0
                {
                    bytes_wanted = ((*b).bytes_alloc as libc::c_uint)
                        .wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_schar
                        as size_t;
                    1 as libc::c_int
                } else {
                    bytes_wanted = ((*b).bytes_alloc as libc::c_uint)
                        .wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_schar
                        as size_t;
                    0 as libc::c_int
                }
            } else if if (2 as libc::c_int) < 0 as libc::c_int {
                if (*b).bytes_alloc < 0 as libc::c_int as libc::c_ulong {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                        }) + 2 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        ((*b).bytes_alloc
                            < ((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                / 2 as libc::c_int) as libc::c_ulong) as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + 1 as libc::c_int)
                                << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + 0 as libc::c_int
                        }) < 0 as libc::c_int
                        {
                            ((2 as libc::c_int)
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        2 as libc::c_int
                                    }) + 1 as libc::c_int)
                                        << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        2 as libc::c_int
                                    }) - 1 as libc::c_int
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int) < 2 as libc::c_int) as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            })
                                + (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                / -(2 as libc::c_int)
                        }) as libc::c_ulong
                            <= (-(1 as libc::c_int) as libc::c_ulong)
                                .wrapping_sub((*b).bytes_alloc)) as libc::c_int
                    }
                } else if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }) + 0 as libc::c_int
                }) - 1 as libc::c_int) < 0 as libc::c_int
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) + 0 as libc::c_int
                    }) + 1 as libc::c_int)
                        << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) + 0 as libc::c_int
                    }) + 0 as libc::c_int
                }) < 0 as libc::c_int
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }) + 0 as libc::c_int)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + 0 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) + 0 as libc::c_int
                            }) + 1 as libc::c_int)
                                << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) + 0 as libc::c_int
                            }) - 1 as libc::c_int
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) + 0 as libc::c_int) as libc::c_int
                }) != 0 && 2 as libc::c_int == -(1 as libc::c_int)
                {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (*b).bytes_alloc
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        ((0 as libc::c_int as libc::c_ulong)
                            < ((*b).bytes_alloc)
                                .wrapping_add(0 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong) < (*b).bytes_alloc
                            && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                as libc::c_ulong)
                                < ((*b).bytes_alloc)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    }
                } else {
                    (((0 as libc::c_int / 2 as libc::c_int) as libc::c_ulong)
                        < (*b).bytes_alloc) as libc::c_int
                }
            } else if 2 as libc::c_int == 0 as libc::c_int {
                0 as libc::c_int
            } else if (*b).bytes_alloc < 0 as libc::c_int as libc::c_ulong {
                if (if (if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (*b).bytes_alloc
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    !((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    })
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        << (::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                }) < 0 as libc::c_int as libc::c_ulong
                {
                    ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (*b).bytes_alloc
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        < (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (*b).bytes_alloc
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (*b).bytes_alloc
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                << (::core::mem::size_of::<libc::c_ulong>()
                                    as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (*b).bytes_alloc
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_neg()) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_ulong)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                }) != 0 && (*b).bytes_alloc == -(1 as libc::c_int) as libc::c_ulong
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        ((0 as libc::c_int) < 2 as libc::c_int + 0 as libc::c_int)
                            as libc::c_int
                    } else {
                        ((-(1 as libc::c_int) - 0 as libc::c_int)
                            < 2 as libc::c_int - 1 as libc::c_int) as libc::c_int
                    }
                } else {
                    ((0 as libc::c_int as libc::c_ulong).wrapping_div((*b).bytes_alloc)
                        < 2 as libc::c_int as libc::c_ulong) as libc::c_int
                }
            } else {
                ((((127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                    / 2 as libc::c_int) as libc::c_ulong) < (*b).bytes_alloc)
                    as libc::c_int
            } != 0
            {
                bytes_wanted = ((*b).bytes_alloc as libc::c_uint)
                    .wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_uchar
                    as size_t;
                1 as libc::c_int
            } else {
                bytes_wanted = ((*b).bytes_alloc as libc::c_uint)
                    .wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_uchar
                    as size_t;
                0 as libc::c_int
            }
        } else if ::core::mem::size_of::<size_t>() as libc::c_ulong
            == ::core::mem::size_of::<libc::c_short>() as libc::c_ulong
        {
            if !((0 as libc::c_int as size_t) < -(1 as libc::c_int) as size_t) {
                if if (2 as libc::c_int) < 0 as libc::c_int {
                    if (*b).bytes_alloc < 0 as libc::c_int as libc::c_ulong {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                32767 as libc::c_int
                            }) + 2 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            ((*b).bytes_alloc
                                < (32767 as libc::c_int / 2 as libc::c_int)
                                    as libc::c_ulong) as libc::c_int
                        } else {
                            ((if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) + 1 as libc::c_int)
                                    << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) + 0 as libc::c_int
                            }) < 0 as libc::c_int
                            {
                                ((2 as libc::c_int)
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        2 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            2 as libc::c_int
                                        }) + 1 as libc::c_int)
                                            << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            2 as libc::c_int
                                        }) - 1 as libc::c_int
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int) < 2 as libc::c_int) as libc::c_int
                            }) != 0
                            {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) + 32767 as libc::c_int
                                    >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            } else {
                                32767 as libc::c_int / -(2 as libc::c_int)
                            }) as libc::c_ulong
                                <= (-(1 as libc::c_int) as libc::c_ulong)
                                    .wrapping_sub((*b).bytes_alloc)) as libc::c_int
                        }
                    } else if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                        }) + 1 as libc::c_int)
                            << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                        }) + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        2 as libc::c_int
                                    }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                }) + 1 as libc::c_int)
                                    << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        2 as libc::c_int
                                    }) + (-(32767 as libc::c_int) - 1 as libc::c_int)
                                }) - 1 as libc::c_int
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + (-(32767 as libc::c_int) - 1 as libc::c_int))
                            as libc::c_int
                    }) != 0 && 2 as libc::c_int == -(1 as libc::c_int)
                    {
                        if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            ((0 as libc::c_int as libc::c_ulong)
                                < ((*b).bytes_alloc)
                                    .wrapping_add(
                                        (-(32767 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_ulong,
                                    )) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_ulong) < (*b).bytes_alloc
                                && ((-(1 as libc::c_int)
                                    - (-(32767 as libc::c_int) - 1 as libc::c_int))
                                    as libc::c_ulong)
                                    < ((*b).bytes_alloc)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                as libc::c_int
                        }
                    } else {
                        ((((-(32767 as libc::c_int) - 1 as libc::c_int)
                            / 2 as libc::c_int) as libc::c_ulong) < (*b).bytes_alloc)
                            as libc::c_int
                    }
                } else if 2 as libc::c_int == 0 as libc::c_int {
                    0 as libc::c_int
                } else if (*b).bytes_alloc < 0 as libc::c_int as libc::c_ulong {
                    if (if (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_add(
                                (-(32767 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_ulong,
                            )
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        !((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (*b).bytes_alloc
                            })
                                .wrapping_add(
                                    (-(32767 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_ulong,
                                )
                        })
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            << (::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (*b).bytes_alloc
                            })
                                .wrapping_add(
                                    (-(32767 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_ulong,
                                )
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    }) < 0 as libc::c_int as libc::c_ulong
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_add(
                                (-(32767 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_ulong,
                            )
                            < (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (*b).bytes_alloc
                                })
                                    .wrapping_add(
                                        (-(32767 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_ulong,
                                    )
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (*b).bytes_alloc
                                    })
                                        .wrapping_add(
                                            (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_ulong,
                                        )
                                })
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    << (::core::mem::size_of::<libc::c_ulong>()
                                        as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (*b).bytes_alloc
                                    })
                                        .wrapping_add(
                                            (-(32767 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_ulong,
                                        )
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_neg()) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (*b).bytes_alloc
                            })
                                .wrapping_add(
                                    (-(32767 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_ulong,
                                )) as libc::c_int
                    }) != 0 && (*b).bytes_alloc == -(1 as libc::c_int) as libc::c_ulong
                    {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            ((0 as libc::c_int)
                                < 2 as libc::c_int
                                    + (-(32767 as libc::c_int) - 1 as libc::c_int))
                                as libc::c_int
                        } else {
                            (-(1 as libc::c_int)
                                - (-(32767 as libc::c_int) - 1 as libc::c_int)
                                < 2 as libc::c_int - 1 as libc::c_int) as libc::c_int
                        }
                    } else {
                        (((-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_div((*b).bytes_alloc)
                            < 2 as libc::c_int as libc::c_ulong) as libc::c_int
                    }
                } else {
                    (((32767 as libc::c_int / 2 as libc::c_int) as libc::c_ulong)
                        < (*b).bytes_alloc) as libc::c_int
                } != 0
                {
                    bytes_wanted = ((*b).bytes_alloc as libc::c_uint)
                        .wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_short
                        as size_t;
                    1 as libc::c_int
                } else {
                    bytes_wanted = ((*b).bytes_alloc as libc::c_uint)
                        .wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_short
                        as size_t;
                    0 as libc::c_int
                }
            } else if if (2 as libc::c_int) < 0 as libc::c_int {
                if (*b).bytes_alloc < 0 as libc::c_int as libc::c_ulong {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                        }) + 2 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        ((*b).bytes_alloc
                            < ((32767 as libc::c_int * 2 as libc::c_int
                                + 1 as libc::c_int) / 2 as libc::c_int) as libc::c_ulong)
                            as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + 1 as libc::c_int)
                                << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + 0 as libc::c_int
                        }) < 0 as libc::c_int
                        {
                            ((2 as libc::c_int)
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        2 as libc::c_int
                                    }) + 1 as libc::c_int)
                                        << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        2 as libc::c_int
                                    }) - 1 as libc::c_int
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int) < 2 as libc::c_int) as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            })
                                + (32767 as libc::c_int * 2 as libc::c_int
                                    + 1 as libc::c_int)
                                >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                / -(2 as libc::c_int)
                        }) as libc::c_ulong
                            <= (-(1 as libc::c_int) as libc::c_ulong)
                                .wrapping_sub((*b).bytes_alloc)) as libc::c_int
                    }
                } else if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }) + 0 as libc::c_int
                }) - 1 as libc::c_int) < 0 as libc::c_int
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) + 0 as libc::c_int
                    }) + 1 as libc::c_int)
                        << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) + 0 as libc::c_int
                    }) + 0 as libc::c_int
                }) < 0 as libc::c_int
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }) + 0 as libc::c_int)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + 0 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) + 0 as libc::c_int
                            }) + 1 as libc::c_int)
                                << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) + 0 as libc::c_int
                            }) - 1 as libc::c_int
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) + 0 as libc::c_int) as libc::c_int
                }) != 0 && 2 as libc::c_int == -(1 as libc::c_int)
                {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (*b).bytes_alloc
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        ((0 as libc::c_int as libc::c_ulong)
                            < ((*b).bytes_alloc)
                                .wrapping_add(0 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong) < (*b).bytes_alloc
                            && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                as libc::c_ulong)
                                < ((*b).bytes_alloc)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    }
                } else {
                    (((0 as libc::c_int / 2 as libc::c_int) as libc::c_ulong)
                        < (*b).bytes_alloc) as libc::c_int
                }
            } else if 2 as libc::c_int == 0 as libc::c_int {
                0 as libc::c_int
            } else if (*b).bytes_alloc < 0 as libc::c_int as libc::c_ulong {
                if (if (if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (*b).bytes_alloc
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    !((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    })
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        << (::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                }) < 0 as libc::c_int as libc::c_ulong
                {
                    ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (*b).bytes_alloc
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        < (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (*b).bytes_alloc
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (*b).bytes_alloc
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                << (::core::mem::size_of::<libc::c_ulong>()
                                    as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (*b).bytes_alloc
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_neg()) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_ulong)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                }) != 0 && (*b).bytes_alloc == -(1 as libc::c_int) as libc::c_ulong
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        ((0 as libc::c_int) < 2 as libc::c_int + 0 as libc::c_int)
                            as libc::c_int
                    } else {
                        ((-(1 as libc::c_int) - 0 as libc::c_int)
                            < 2 as libc::c_int - 1 as libc::c_int) as libc::c_int
                    }
                } else {
                    ((0 as libc::c_int as libc::c_ulong).wrapping_div((*b).bytes_alloc)
                        < 2 as libc::c_int as libc::c_ulong) as libc::c_int
                }
            } else {
                ((((32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                    / 2 as libc::c_int) as libc::c_ulong) < (*b).bytes_alloc)
                    as libc::c_int
            } != 0
            {
                bytes_wanted = ((*b).bytes_alloc as libc::c_uint)
                    .wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_ushort
                    as size_t;
                1 as libc::c_int
            } else {
                bytes_wanted = ((*b).bytes_alloc as libc::c_uint)
                    .wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_ushort
                    as size_t;
                0 as libc::c_int
            }
        } else if ::core::mem::size_of::<size_t>() as libc::c_ulong
            == ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
        {
            if (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                bytes_wanted
            })
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                < 0 as libc::c_int as libc::c_ulong
            {
                if if (2 as libc::c_int) < 0 as libc::c_int {
                    if (*b).bytes_alloc < 0 as libc::c_int as libc::c_ulong {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2147483647 as libc::c_int
                            }) + 2 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            ((*b).bytes_alloc
                                < (2147483647 as libc::c_int / 2 as libc::c_int)
                                    as libc::c_ulong) as libc::c_int
                        } else {
                            ((if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) + 1 as libc::c_int)
                                    << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) + 0 as libc::c_int
                            }) < 0 as libc::c_int
                            {
                                ((2 as libc::c_int)
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        2 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            2 as libc::c_int
                                        }) + 1 as libc::c_int)
                                            << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            2 as libc::c_int
                                        }) - 1 as libc::c_int
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int) < 2 as libc::c_int) as libc::c_int
                            }) != 0
                            {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) + 2147483647 as libc::c_int
                                    >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            } else {
                                2147483647 as libc::c_int / -(2 as libc::c_int)
                            }) as libc::c_ulong
                                <= (-(1 as libc::c_int) as libc::c_ulong)
                                    .wrapping_sub((*b).bytes_alloc)) as libc::c_int
                        }
                    } else if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                        }) + 1 as libc::c_int)
                            << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                        }) + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        2 as libc::c_int
                                    }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                }) + 1 as libc::c_int)
                                    << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        2 as libc::c_int
                                    }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                }) - 1 as libc::c_int
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                            as libc::c_int
                    }) != 0 && 2 as libc::c_int == -(1 as libc::c_int)
                    {
                        if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            ((0 as libc::c_int as libc::c_ulong)
                                < ((*b).bytes_alloc)
                                    .wrapping_add(
                                        (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_ulong,
                                    )) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_ulong) < (*b).bytes_alloc
                                && ((-(1 as libc::c_int)
                                    - (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                    as libc::c_ulong)
                                    < ((*b).bytes_alloc)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                as libc::c_int
                        }
                    } else {
                        ((((-(2147483647 as libc::c_int) - 1 as libc::c_int)
                            / 2 as libc::c_int) as libc::c_ulong) < (*b).bytes_alloc)
                            as libc::c_int
                    }
                } else if 2 as libc::c_int == 0 as libc::c_int {
                    0 as libc::c_int
                } else if (*b).bytes_alloc < 0 as libc::c_int as libc::c_ulong {
                    if (if (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_add(
                                (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_ulong,
                            )
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        !((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (*b).bytes_alloc
                            })
                                .wrapping_add(
                                    (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_ulong,
                                )
                        })
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            << (::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (*b).bytes_alloc
                            })
                                .wrapping_add(
                                    (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_ulong,
                                )
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    }) < 0 as libc::c_int as libc::c_ulong
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_add(
                                (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_ulong,
                            )
                            < (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (*b).bytes_alloc
                                })
                                    .wrapping_add(
                                        (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_ulong,
                                    )
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (*b).bytes_alloc
                                    })
                                        .wrapping_add(
                                            (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_ulong,
                                        )
                                })
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    << (::core::mem::size_of::<libc::c_ulong>()
                                        as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (*b).bytes_alloc
                                    })
                                        .wrapping_add(
                                            (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                                as libc::c_ulong,
                                        )
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_neg()) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (*b).bytes_alloc
                            })
                                .wrapping_add(
                                    (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_ulong,
                                )) as libc::c_int
                    }) != 0 && (*b).bytes_alloc == -(1 as libc::c_int) as libc::c_ulong
                    {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            ((0 as libc::c_int)
                                < 2 as libc::c_int
                                    + (-(2147483647 as libc::c_int) - 1 as libc::c_int))
                                as libc::c_int
                        } else {
                            (-(1 as libc::c_int)
                                - (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                < 2 as libc::c_int - 1 as libc::c_int) as libc::c_int
                        }
                    } else {
                        (((-(2147483647 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_ulong)
                            .wrapping_div((*b).bytes_alloc)
                            < 2 as libc::c_int as libc::c_ulong) as libc::c_int
                    }
                } else {
                    (((2147483647 as libc::c_int / 2 as libc::c_int) as libc::c_ulong)
                        < (*b).bytes_alloc) as libc::c_int
                } != 0
                {
                    bytes_wanted = ((*b).bytes_alloc as libc::c_uint)
                        .wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_int
                        as size_t;
                    1 as libc::c_int
                } else {
                    bytes_wanted = ((*b).bytes_alloc as libc::c_uint)
                        .wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_int
                        as size_t;
                    0 as libc::c_int
                }
            } else if if (2 as libc::c_int) < 0 as libc::c_int {
                if (*b).bytes_alloc < 0 as libc::c_int as libc::c_ulong {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_uint
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_uint
                        } else {
                            (2147483647 as libc::c_int as libc::c_uint)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_add(1 as libc::c_uint)
                        })
                            .wrapping_add(2 as libc::c_int as libc::c_uint)
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        < 0 as libc::c_int as libc::c_uint
                    {
                        ((*b).bytes_alloc
                            < (2147483647 as libc::c_int as libc::c_uint)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_add(1 as libc::c_uint)
                                .wrapping_div(2 as libc::c_int as libc::c_uint)
                                as libc::c_ulong) as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + 1 as libc::c_int)
                                << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + 0 as libc::c_int
                        }) < 0 as libc::c_int
                        {
                            ((2 as libc::c_int)
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        2 as libc::c_int
                                    }) + 1 as libc::c_int)
                                        << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        2 as libc::c_int
                                    }) - 1 as libc::c_int
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int) < 2 as libc::c_int) as libc::c_int
                        }) != 0
                        {
                            ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) as libc::c_uint)
                                .wrapping_add(
                                    (2147483647 as libc::c_int as libc::c_uint)
                                        .wrapping_mul(2 as libc::c_uint)
                                        .wrapping_add(1 as libc::c_uint),
                                )
                                >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (2147483647 as libc::c_int as libc::c_uint)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_add(1 as libc::c_uint)
                                .wrapping_div(-(2 as libc::c_int) as libc::c_uint)
                        }) as libc::c_ulong
                            <= (-(1 as libc::c_int) as libc::c_ulong)
                                .wrapping_sub((*b).bytes_alloc)) as libc::c_int
                    }
                } else if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }) + 0 as libc::c_int
                }) - 1 as libc::c_int) < 0 as libc::c_int
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) + 0 as libc::c_int
                    }) + 1 as libc::c_int)
                        << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) + 0 as libc::c_int
                    }) + 0 as libc::c_int
                }) < 0 as libc::c_int
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }) + 0 as libc::c_int)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + 0 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) + 0 as libc::c_int
                            }) + 1 as libc::c_int)
                                << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) + 0 as libc::c_int
                            }) - 1 as libc::c_int
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) + 0 as libc::c_int) as libc::c_int
                }) != 0 && 2 as libc::c_int == -(1 as libc::c_int)
                {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (*b).bytes_alloc
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        ((0 as libc::c_int as libc::c_ulong)
                            < ((*b).bytes_alloc)
                                .wrapping_add(0 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong) < (*b).bytes_alloc
                            && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                as libc::c_ulong)
                                < ((*b).bytes_alloc)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    }
                } else {
                    (((0 as libc::c_int / 2 as libc::c_int) as libc::c_ulong)
                        < (*b).bytes_alloc) as libc::c_int
                }
            } else if 2 as libc::c_int == 0 as libc::c_int {
                0 as libc::c_int
            } else if (*b).bytes_alloc < 0 as libc::c_int as libc::c_ulong {
                if (if (if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (*b).bytes_alloc
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    !((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    })
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        << (::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                }) < 0 as libc::c_int as libc::c_ulong
                {
                    ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (*b).bytes_alloc
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        < (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (*b).bytes_alloc
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (*b).bytes_alloc
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                << (::core::mem::size_of::<libc::c_ulong>()
                                    as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (*b).bytes_alloc
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_neg()) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_ulong)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                }) != 0 && (*b).bytes_alloc == -(1 as libc::c_int) as libc::c_ulong
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        ((0 as libc::c_int) < 2 as libc::c_int + 0 as libc::c_int)
                            as libc::c_int
                    } else {
                        ((-(1 as libc::c_int) - 0 as libc::c_int)
                            < 2 as libc::c_int - 1 as libc::c_int) as libc::c_int
                    }
                } else {
                    ((0 as libc::c_int as libc::c_ulong).wrapping_div((*b).bytes_alloc)
                        < 2 as libc::c_int as libc::c_ulong) as libc::c_int
                }
            } else {
                (((2147483647 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_uint)
                    .wrapping_add(1 as libc::c_uint)
                    .wrapping_div(2 as libc::c_int as libc::c_uint) as libc::c_ulong)
                    < (*b).bytes_alloc) as libc::c_int
            } != 0
            {
                bytes_wanted = ((*b).bytes_alloc as libc::c_uint)
                    .wrapping_mul(2 as libc::c_int as libc::c_uint) as size_t;
                1 as libc::c_int
            } else {
                bytes_wanted = ((*b).bytes_alloc as libc::c_uint)
                    .wrapping_mul(2 as libc::c_int as libc::c_uint) as size_t;
                0 as libc::c_int
            }
        } else if ::core::mem::size_of::<size_t>() as libc::c_ulong
            == ::core::mem::size_of::<libc::c_long>() as libc::c_ulong
        {
            if (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                bytes_wanted
            })
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                < 0 as libc::c_int as libc::c_ulong
            {
                if if (2 as libc::c_int) < 0 as libc::c_int {
                    if (*b).bytes_alloc < 0 as libc::c_int as libc::c_ulong {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                9223372036854775807 as libc::c_long
                            }) + 2 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((*b).bytes_alloc
                                < (9223372036854775807 as libc::c_long
                                    / 2 as libc::c_int as libc::c_long) as libc::c_ulong)
                                as libc::c_int
                        } else {
                            ((if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) + 1 as libc::c_int)
                                    << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) + 0 as libc::c_int
                            }) < 0 as libc::c_int
                            {
                                ((2 as libc::c_int)
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        2 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            2 as libc::c_int
                                        }) + 1 as libc::c_int)
                                            << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            2 as libc::c_int
                                        }) - 1 as libc::c_int
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int) < 2 as libc::c_int) as libc::c_int
                            }) != 0
                            {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) as libc::c_long + 9223372036854775807 as libc::c_long
                                    >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            } else {
                                9223372036854775807 as libc::c_long
                                    / -(2 as libc::c_int) as libc::c_long
                            }) as libc::c_ulong
                                <= (-(1 as libc::c_int) as libc::c_ulong)
                                    .wrapping_sub((*b).bytes_alloc)) as libc::c_int
                        }
                    } else if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) as libc::c_long
                            + (-(9223372036854775807 as libc::c_long)
                                - 1 as libc::c_long)
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) as libc::c_long
                                + (-(9223372036854775807 as libc::c_long)
                                    - 1 as libc::c_long)
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) as libc::c_long
                                + (-(9223372036854775807 as libc::c_long)
                                    - 1 as libc::c_long)
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) as libc::c_long
                            + (-(9223372036854775807 as libc::c_long)
                                - 1 as libc::c_long)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) as libc::c_long
                                    + (-(9223372036854775807 as libc::c_long)
                                        - 1 as libc::c_long)
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        2 as libc::c_int
                                    }) as libc::c_long
                                        + (-(9223372036854775807 as libc::c_long)
                                            - 1 as libc::c_long)
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        2 as libc::c_int
                                    }) as libc::c_long
                                        + (-(9223372036854775807 as libc::c_long)
                                            - 1 as libc::c_long)
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) as libc::c_long
                                + (-(9223372036854775807 as libc::c_long)
                                    - 1 as libc::c_long)) as libc::c_int
                    }) != 0 && 2 as libc::c_int == -(1 as libc::c_int)
                    {
                        if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            ((0 as libc::c_int as libc::c_ulong)
                                < ((*b).bytes_alloc)
                                    .wrapping_add(
                                        (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                            as libc::c_ulong,
                                    )) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_ulong) < (*b).bytes_alloc
                                && ((-(1 as libc::c_int) as libc::c_long
                                    - (-(9223372036854775807 as libc::c_long)
                                        - 1 as libc::c_long)) as libc::c_ulong)
                                    < ((*b).bytes_alloc)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                as libc::c_int
                        }
                    } else {
                        ((((-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                            / 2 as libc::c_int as libc::c_long) as libc::c_ulong)
                            < (*b).bytes_alloc) as libc::c_int
                    }
                } else if 2 as libc::c_int == 0 as libc::c_int {
                    0 as libc::c_int
                } else if (*b).bytes_alloc < 0 as libc::c_int as libc::c_ulong {
                    if (if (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_add(
                                (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                    as libc::c_ulong,
                            )
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        !((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (*b).bytes_alloc
                            })
                                .wrapping_add(
                                    (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                        as libc::c_ulong,
                                )
                        })
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            << (::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (*b).bytes_alloc
                            })
                                .wrapping_add(
                                    (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                        as libc::c_ulong,
                                )
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    }) < 0 as libc::c_int as libc::c_ulong
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_add(
                                (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                    as libc::c_ulong,
                            )
                            < (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (*b).bytes_alloc
                                })
                                    .wrapping_add(
                                        (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                            as libc::c_ulong,
                                    )
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (*b).bytes_alloc
                                    })
                                        .wrapping_add(
                                            (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                as libc::c_ulong,
                                        )
                                })
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    << (::core::mem::size_of::<libc::c_ulong>()
                                        as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (*b).bytes_alloc
                                    })
                                        .wrapping_add(
                                            (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                                as libc::c_ulong,
                                        )
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_neg()) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (*b).bytes_alloc
                            })
                                .wrapping_add(
                                    (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                                        as libc::c_ulong,
                                )) as libc::c_int
                    }) != 0 && (*b).bytes_alloc == -(1 as libc::c_int) as libc::c_ulong
                    {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            ((0 as libc::c_int as libc::c_long)
                                < 2 as libc::c_int as libc::c_long
                                    + (-(9223372036854775807 as libc::c_long)
                                        - 1 as libc::c_long)) as libc::c_int
                        } else {
                            (-(1 as libc::c_int) as libc::c_long
                                - (-(9223372036854775807 as libc::c_long)
                                    - 1 as libc::c_long)
                                < (2 as libc::c_int - 1 as libc::c_int) as libc::c_long)
                                as libc::c_int
                        }
                    } else {
                        (((-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
                            as libc::c_ulong)
                            .wrapping_div((*b).bytes_alloc)
                            < 2 as libc::c_int as libc::c_ulong) as libc::c_int
                    }
                } else {
                    (((9223372036854775807 as libc::c_long
                        / 2 as libc::c_int as libc::c_long) as libc::c_ulong)
                        < (*b).bytes_alloc) as libc::c_int
                } != 0
                {
                    bytes_wanted = ((*b).bytes_alloc)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as libc::c_long
                        as size_t;
                    1 as libc::c_int
                } else {
                    bytes_wanted = ((*b).bytes_alloc)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as libc::c_long
                        as size_t;
                    0 as libc::c_int
                }
            } else if if (2 as libc::c_int) < 0 as libc::c_int {
                if (*b).bytes_alloc < 0 as libc::c_int as libc::c_ulong {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                        })
                            .wrapping_add(2 as libc::c_int as libc::c_ulong)
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        ((*b).bytes_alloc
                            < (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(2 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + 1 as libc::c_int)
                                << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + 0 as libc::c_int
                        }) < 0 as libc::c_int
                        {
                            ((2 as libc::c_int)
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        2 as libc::c_int
                                    }) + 1 as libc::c_int)
                                        << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        2 as libc::c_int
                                    }) - 1 as libc::c_int
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int) < 2 as libc::c_int) as libc::c_int
                        }) != 0
                        {
                            ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) as libc::c_ulong)
                                .wrapping_add(
                                    (9223372036854775807 as libc::c_long as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_ulong),
                                )
                                >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (9223372036854775807 as libc::c_long as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong)
                                .wrapping_add(1 as libc::c_ulong)
                                .wrapping_div(-(2 as libc::c_int) as libc::c_ulong)
                        })
                            <= (-(1 as libc::c_int) as libc::c_ulong)
                                .wrapping_sub((*b).bytes_alloc)) as libc::c_int
                    }
                } else if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }) + 0 as libc::c_int
                }) - 1 as libc::c_int) < 0 as libc::c_int
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) + 0 as libc::c_int
                    }) + 1 as libc::c_int)
                        << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) + 0 as libc::c_int
                    }) + 0 as libc::c_int
                }) < 0 as libc::c_int
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }) + 0 as libc::c_int)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + 0 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) + 0 as libc::c_int
                            }) + 1 as libc::c_int)
                                << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) + 0 as libc::c_int
                            }) - 1 as libc::c_int
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) + 0 as libc::c_int) as libc::c_int
                }) != 0 && 2 as libc::c_int == -(1 as libc::c_int)
                {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (*b).bytes_alloc
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        ((0 as libc::c_int as libc::c_ulong)
                            < ((*b).bytes_alloc)
                                .wrapping_add(0 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong) < (*b).bytes_alloc
                            && ((-(1 as libc::c_int) - 0 as libc::c_int)
                                as libc::c_ulong)
                                < ((*b).bytes_alloc)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    }
                } else {
                    (((0 as libc::c_int / 2 as libc::c_int) as libc::c_ulong)
                        < (*b).bytes_alloc) as libc::c_int
                }
            } else if 2 as libc::c_int == 0 as libc::c_int {
                0 as libc::c_int
            } else if (*b).bytes_alloc < 0 as libc::c_int as libc::c_ulong {
                if (if (if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (*b).bytes_alloc
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    !((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    })
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        << (::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                }) < 0 as libc::c_int as libc::c_ulong
                {
                    ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (*b).bytes_alloc
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        < (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (*b).bytes_alloc
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (*b).bytes_alloc
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                << (::core::mem::size_of::<libc::c_ulong>()
                                    as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (*b).bytes_alloc
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_neg()) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_ulong)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                }) != 0 && (*b).bytes_alloc == -(1 as libc::c_int) as libc::c_ulong
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        ((0 as libc::c_int) < 2 as libc::c_int + 0 as libc::c_int)
                            as libc::c_int
                    } else {
                        ((-(1 as libc::c_int) - 0 as libc::c_int)
                            < 2 as libc::c_int - 1 as libc::c_int) as libc::c_int
                    }
                } else {
                    ((0 as libc::c_int as libc::c_ulong).wrapping_div((*b).bytes_alloc)
                        < 2 as libc::c_int as libc::c_ulong) as libc::c_int
                }
            } else {
                ((9223372036854775807 as libc::c_long as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_ulong)
                    .wrapping_add(1 as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong) < (*b).bytes_alloc)
                    as libc::c_int
            } != 0
            {
                bytes_wanted = ((*b).bytes_alloc)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong);
                1 as libc::c_int
            } else {
                bytes_wanted = ((*b).bytes_alloc)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong);
                0 as libc::c_int
            }
        } else if (if 1 as libc::c_int != 0 {
            0 as libc::c_int as libc::c_ulong
        } else {
            bytes_wanted
        })
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            < 0 as libc::c_int as libc::c_ulong
        {
            if if (2 as libc::c_int) < 0 as libc::c_int {
                if (*b).bytes_alloc < 0 as libc::c_int as libc::c_ulong {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_longlong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_longlong
                        } else {
                            9223372036854775807 as libc::c_longlong
                        }) + 2 as libc::c_int as libc::c_longlong
                    }) - 1 as libc::c_int as libc::c_longlong)
                        < 0 as libc::c_int as libc::c_longlong
                    {
                        (((*b).bytes_alloc as libc::c_ulonglong)
                            < (9223372036854775807 as libc::c_longlong
                                / 2 as libc::c_int as libc::c_longlong)
                                as libc::c_ulonglong) as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + 1 as libc::c_int)
                                << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + 0 as libc::c_int
                        }) < 0 as libc::c_int
                        {
                            ((2 as libc::c_int)
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        2 as libc::c_int
                                    }) + 1 as libc::c_int)
                                        << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        2 as libc::c_int
                                    }) - 1 as libc::c_int
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int) < 2 as libc::c_int) as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) as libc::c_longlong
                                + 9223372036854775807 as libc::c_longlong
                                >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            9223372036854775807 as libc::c_longlong
                                / -(2 as libc::c_int) as libc::c_longlong
                        }) as libc::c_ulonglong
                            <= (-(1 as libc::c_int) as libc::c_ulong)
                                .wrapping_sub((*b).bytes_alloc) as libc::c_ulonglong)
                            as libc::c_int
                    }
                } else if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_longlong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }) as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)
                }) - 1 as libc::c_int as libc::c_longlong)
                    < 0 as libc::c_int as libc::c_longlong
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_longlong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) as libc::c_longlong
                            + (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)
                    }) + 1 as libc::c_int as libc::c_longlong)
                        << (::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_longlong)
                        * 2 as libc::c_int as libc::c_longlong
                        + 1 as libc::c_int as libc::c_longlong)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_longlong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) as libc::c_longlong
                            + (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)
                    }) + 0 as libc::c_int as libc::c_longlong
                }) < 0 as libc::c_int as libc::c_longlong
                {
                    ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }) as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_longlong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) as libc::c_longlong
                                + (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong)
                        }) - 1 as libc::c_int as libc::c_longlong)
                            < 0 as libc::c_int as libc::c_longlong
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_longlong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) as libc::c_longlong
                                    + (-(9223372036854775807 as libc::c_longlong)
                                        - 1 as libc::c_longlong)
                            }) + 1 as libc::c_int as libc::c_longlong)
                                << (::core::mem::size_of::<libc::c_longlong>()
                                    as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_longlong)
                                * 2 as libc::c_int as libc::c_longlong
                                + 1 as libc::c_int as libc::c_longlong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_longlong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) as libc::c_longlong
                                    + (-(9223372036854775807 as libc::c_longlong)
                                        - 1 as libc::c_longlong)
                            }) - 1 as libc::c_int as libc::c_longlong
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_longlong)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) as libc::c_longlong
                            + (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)) as libc::c_int
                }) != 0 && 2 as libc::c_int == -(1 as libc::c_int)
                {
                    if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (*b).bytes_alloc
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        ((0 as libc::c_int as libc::c_ulonglong)
                            < ((*b).bytes_alloc as libc::c_ulonglong)
                                .wrapping_add(
                                    (-(9223372036854775807 as libc::c_longlong)
                                        - 1 as libc::c_longlong) as libc::c_ulonglong,
                                )) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong) < (*b).bytes_alloc
                            && ((-(1 as libc::c_int) as libc::c_longlong
                                - (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong)) as libc::c_ulonglong)
                                < ((*b).bytes_alloc)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    as libc::c_ulonglong) as libc::c_int
                    }
                } else {
                    ((((-(9223372036854775807 as libc::c_longlong)
                        - 1 as libc::c_longlong) / 2 as libc::c_int as libc::c_longlong)
                        as libc::c_ulonglong) < (*b).bytes_alloc as libc::c_ulonglong)
                        as libc::c_int
                }
            } else if 2 as libc::c_int == 0 as libc::c_int {
                0 as libc::c_int
            } else if (*b).bytes_alloc < 0 as libc::c_int as libc::c_ulong {
                if (if (if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulonglong
                } else {
                    ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (*b).bytes_alloc
                    }) as libc::c_ulonglong)
                        .wrapping_add(
                            (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong) as libc::c_ulonglong,
                        )
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                    < 0 as libc::c_int as libc::c_ulonglong
                {
                    !((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulonglong
                    } else {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        }) as libc::c_ulonglong)
                            .wrapping_add(
                                (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong) as libc::c_ulonglong,
                            )
                    })
                        .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
                        << (::core::mem::size_of::<libc::c_ulonglong>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulonglong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulonglong
                    } else {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        }) as libc::c_ulonglong)
                            .wrapping_add(
                                (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong) as libc::c_ulonglong,
                            )
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulonglong)
                }) < 0 as libc::c_int as libc::c_ulonglong
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (*b).bytes_alloc
                    }) as libc::c_ulonglong)
                        .wrapping_add(
                            (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong) as libc::c_ulonglong,
                        )
                        < (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulonglong
                        } else {
                            ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (*b).bytes_alloc
                            }) as libc::c_ulonglong)
                                .wrapping_add(
                                    (-(9223372036854775807 as libc::c_longlong)
                                        - 1 as libc::c_longlong) as libc::c_ulonglong,
                                )
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                            < 0 as libc::c_int as libc::c_ulonglong
                        {
                            ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulonglong
                            } else {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (*b).bytes_alloc
                                }) as libc::c_ulonglong)
                                    .wrapping_add(
                                        (-(9223372036854775807 as libc::c_longlong)
                                            - 1 as libc::c_longlong) as libc::c_ulonglong,
                                    )
                            })
                                .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
                                << (::core::mem::size_of::<libc::c_ulonglong>()
                                    as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulonglong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulonglong
                            } else {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (*b).bytes_alloc
                                }) as libc::c_ulonglong)
                                    .wrapping_add(
                                        (-(9223372036854775807 as libc::c_longlong)
                                            - 1 as libc::c_longlong) as libc::c_ulonglong,
                                    )
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                        })
                            .wrapping_neg()) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_ulonglong)
                        < ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        }) as libc::c_ulonglong)
                            .wrapping_add(
                                (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong) as libc::c_ulonglong,
                            )) as libc::c_int
                }) != 0 && (*b).bytes_alloc == -(1 as libc::c_int) as libc::c_ulong
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        ((0 as libc::c_int as libc::c_longlong)
                            < 2 as libc::c_int as libc::c_longlong
                                + (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong)) as libc::c_int
                    } else {
                        (-(1 as libc::c_int) as libc::c_longlong
                            - (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)
                            < (2 as libc::c_int - 1 as libc::c_int) as libc::c_longlong)
                            as libc::c_int
                    }
                } else {
                    (((-(9223372036854775807 as libc::c_longlong)
                        - 1 as libc::c_longlong) as libc::c_ulonglong)
                        .wrapping_div((*b).bytes_alloc as libc::c_ulonglong)
                        < 2 as libc::c_int as libc::c_ulonglong) as libc::c_int
                }
            } else {
                (((9223372036854775807 as libc::c_longlong
                    / 2 as libc::c_int as libc::c_longlong) as libc::c_ulonglong)
                    < (*b).bytes_alloc as libc::c_ulonglong) as libc::c_int
            } != 0
            {
                bytes_wanted = ((*b).bytes_alloc as libc::c_ulonglong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulonglong)
                    as libc::c_longlong as size_t;
                1 as libc::c_int
            } else {
                bytes_wanted = ((*b).bytes_alloc as libc::c_ulonglong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulonglong)
                    as libc::c_longlong as size_t;
                0 as libc::c_int
            }
        } else if if (2 as libc::c_int) < 0 as libc::c_int {
            if (*b).bytes_alloc < 0 as libc::c_int as libc::c_ulong {
                if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulonglong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulonglong
                    } else {
                        (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                            .wrapping_mul(2 as libc::c_ulonglong)
                            .wrapping_add(1 as libc::c_ulonglong)
                    })
                        .wrapping_add(2 as libc::c_int as libc::c_ulonglong)
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
                    < 0 as libc::c_int as libc::c_ulonglong
                {
                    (((*b).bytes_alloc as libc::c_ulonglong)
                        < (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                            .wrapping_mul(2 as libc::c_ulonglong)
                            .wrapping_add(1 as libc::c_ulonglong)
                            .wrapping_div(2 as libc::c_int as libc::c_ulonglong))
                        as libc::c_int
                } else {
                    ((if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) + 1 as libc::c_int)
                            << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        ((2 as libc::c_int)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) + 1 as libc::c_int)
                                    << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    2 as libc::c_int
                                }) - 1 as libc::c_int
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int) < 2 as libc::c_int) as libc::c_int
                    }) != 0
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) as libc::c_ulonglong)
                            .wrapping_add(
                                (9223372036854775807 as libc::c_longlong
                                    as libc::c_ulonglong)
                                    .wrapping_mul(2 as libc::c_ulonglong)
                                    .wrapping_add(1 as libc::c_ulonglong),
                            )
                            >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                            .wrapping_mul(2 as libc::c_ulonglong)
                            .wrapping_add(1 as libc::c_ulonglong)
                            .wrapping_div(-(2 as libc::c_int) as libc::c_ulonglong)
                    })
                        <= (-(1 as libc::c_int) as libc::c_ulong)
                            .wrapping_sub((*b).bytes_alloc) as libc::c_ulonglong)
                        as libc::c_int
                }
            } else if (if (if ((if 1 as libc::c_int != 0 {
                0 as libc::c_int
            } else {
                (if 1 as libc::c_int != 0 { 0 as libc::c_int } else { 2 as libc::c_int })
                    + 0 as libc::c_int
            }) - 1 as libc::c_int) < 0 as libc::c_int
            {
                !(((((if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }) + 0 as libc::c_int
                }) + 1 as libc::c_int)
                    << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }) + 0 as libc::c_int
                }) + 0 as libc::c_int
            }) < 0 as libc::c_int
            {
                (((if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    2 as libc::c_int
                }) + 0 as libc::c_int)
                    < -(if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) + 0 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        ((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + 0 as libc::c_int
                        }) + 1 as libc::c_int)
                            << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }) + 0 as libc::c_int
                        }) - 1 as libc::c_int
                    })) as libc::c_int
            } else {
                ((0 as libc::c_int)
                    < (if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }) + 0 as libc::c_int) as libc::c_int
            }) != 0 && 2 as libc::c_int == -(1 as libc::c_int)
            {
                if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (*b).bytes_alloc
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    ((0 as libc::c_int as libc::c_ulong)
                        < ((*b).bytes_alloc)
                            .wrapping_add(0 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_ulong) < (*b).bytes_alloc
                        && ((-(1 as libc::c_int) - 0 as libc::c_int) as libc::c_ulong)
                            < ((*b).bytes_alloc)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                        as libc::c_int
                }
            } else {
                (((0 as libc::c_int / 2 as libc::c_int) as libc::c_ulong)
                    < (*b).bytes_alloc) as libc::c_int
            }
        } else if 2 as libc::c_int == 0 as libc::c_int {
            0 as libc::c_int
        } else if (*b).bytes_alloc < 0 as libc::c_int as libc::c_ulong {
            if (if (if (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (*b).bytes_alloc
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
            })
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                < 0 as libc::c_int as libc::c_ulong
            {
                !((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (*b).bytes_alloc
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                })
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    << (::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (*b).bytes_alloc
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
            }) < 0 as libc::c_int as libc::c_ulong
            {
                ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (*b).bytes_alloc
                })
                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    < (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (*b).bytes_alloc
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (*b).bytes_alloc
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            << (::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (*b).bytes_alloc
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    })
                        .wrapping_neg()) as libc::c_int
            } else {
                ((0 as libc::c_int as libc::c_ulong)
                    < (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (*b).bytes_alloc
                    })
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)) as libc::c_int
            }) != 0 && (*b).bytes_alloc == -(1 as libc::c_int) as libc::c_ulong
            {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    2 as libc::c_int
                }) - 1 as libc::c_int) < 0 as libc::c_int
                {
                    ((0 as libc::c_int) < 2 as libc::c_int + 0 as libc::c_int)
                        as libc::c_int
                } else {
                    ((-(1 as libc::c_int) - 0 as libc::c_int)
                        < 2 as libc::c_int - 1 as libc::c_int) as libc::c_int
                }
            } else {
                ((0 as libc::c_int as libc::c_ulong).wrapping_div((*b).bytes_alloc)
                    < 2 as libc::c_int as libc::c_ulong) as libc::c_int
            }
        } else {
            ((9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                .wrapping_mul(2 as libc::c_ulonglong)
                .wrapping_add(1 as libc::c_ulonglong)
                .wrapping_div(2 as libc::c_int as libc::c_ulonglong)
                < (*b).bytes_alloc as libc::c_ulonglong) as libc::c_int
        } != 0
        {
            bytes_wanted = ((*b).bytes_alloc as libc::c_ulonglong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulonglong) as size_t;
            1 as libc::c_int
        } else {
            bytes_wanted = ((*b).bytes_alloc as libc::c_ulonglong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulonglong) as size_t;
            0 as libc::c_int
        } != 0
        {
            xalloc_die();
        }
        free_buffer(b);
        free(b as *mut libc::c_void);
    }
    if lines_found != 0 {
        save_buffer(b);
    } else {
        free_buffer(b);
        free(b as *mut libc::c_void);
    }
    return lines_found != 0 as libc::c_int as libc::c_ulong;
}
unsafe extern "C" fn get_first_line_in_buffer() -> uintmax_t {
    if head.is_null() && !load_buffer() {
        if ::core::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                gettext(b"input disappeared\0" as *const u8 as *const libc::c_char),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                *__errno_location(),
                gettext(b"input disappeared\0" as *const u8 as *const libc::c_char),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    return (*head).first_available;
}
unsafe extern "C" fn remove_line() -> *mut cstring {
    static mut prev_buf: *mut buffer_record = 0 as *const buffer_record
        as *mut buffer_record;
    let mut line: *mut cstring = 0 as *mut cstring;
    let mut l: *mut line = 0 as *mut line;
    if !prev_buf.is_null() {
        free_buffer(prev_buf);
        free(prev_buf as *mut libc::c_void);
        prev_buf = 0 as *mut buffer_record;
    }
    if head.is_null() && !load_buffer() {
        return 0 as *mut cstring;
    }
    if current_line < (*head).first_available {
        current_line = (*head).first_available;
    }
    (*head).first_available = ((*head).first_available).wrapping_add(1);
    (*head).first_available;
    l = (*head).curr_line;
    line = &mut *((*l).starts).as_mut_ptr().offset((*l).retrieve_index as isize)
        as *mut cstring;
    (*l).retrieve_index = ((*l).retrieve_index).wrapping_add(1);
    if (*l).retrieve_index == (*l).used {
        (*head).curr_line = (*l).next;
        if ((*head).curr_line).is_null()
            || (*(*head).curr_line).used == 0 as libc::c_int as libc::c_ulong
        {
            prev_buf = head;
            head = (*head).next;
        }
    }
    return line;
}
unsafe extern "C" fn find_line(mut linenum: uintmax_t) -> *mut cstring {
    let mut b: *mut buffer_record = 0 as *mut buffer_record;
    if head.is_null() && !load_buffer() {
        return 0 as *mut cstring;
    }
    if linenum < (*head).start_line {
        return 0 as *mut cstring;
    }
    b = head;
    loop {
        if !b.is_null() {} else {
            __assert_fail(
                b"b\0" as *const u8 as *const libc::c_char,
                b"src/csplit.c\0" as *const u8 as *const libc::c_char,
                619 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"struct cstring *find_line(uintmax_t)\0"))
                    .as_ptr(),
            );
        }
        'c_19594: {
            if !b.is_null() {} else {
                __assert_fail(
                    b"b\0" as *const u8 as *const libc::c_char,
                    b"src/csplit.c\0" as *const u8 as *const libc::c_char,
                    619 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 37],
                        &[libc::c_char; 37],
                    >(b"struct cstring *find_line(uintmax_t)\0"))
                        .as_ptr(),
                );
            }
        };
        if linenum < ((*b).start_line).wrapping_add((*b).num_lines) {
            let mut l: *mut line = 0 as *mut line;
            let mut offset: size_t = 0;
            l = (*b).line_start;
            offset = linenum.wrapping_sub((*b).start_line);
            while offset >= 80 as libc::c_int as libc::c_ulong {
                l = (*l).next;
                offset = (offset as libc::c_ulong)
                    .wrapping_sub(80 as libc::c_int as libc::c_ulong) as size_t
                    as size_t;
            }
            return &mut *((*l).starts).as_mut_ptr().offset(offset as isize)
                as *mut cstring;
        }
        if ((*b).next).is_null() && !load_buffer() {
            return 0 as *mut cstring;
        }
        b = (*b).next;
    };
}
unsafe extern "C" fn no_more_lines() -> bool {
    return (find_line(current_line.wrapping_add(1 as libc::c_int as libc::c_ulong)))
        .is_null();
}
unsafe extern "C" fn set_input_file(mut name: *const libc::c_char) {
    if !(strcmp(name, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int)
        && fd_reopen(
            0 as libc::c_int,
            name,
            0 as libc::c_int,
            0 as libc::c_int as mode_t,
        ) < 0 as libc::c_int
    {
        if ::core::mem::size_of::<C2RustUnnamed_14>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                gettext(
                    b"cannot open %s for reading\0" as *const u8 as *const libc::c_char,
                ),
                quotearg_style(shell_escape_always_quoting_style, name),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                *__errno_location(),
                gettext(
                    b"cannot open %s for reading\0" as *const u8 as *const libc::c_char,
                ),
                quotearg_style(shell_escape_always_quoting_style, name),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
}
unsafe extern "C" fn write_to_file(
    mut last_line: uintmax_t,
    mut ignore: bool,
    mut argnum: libc::c_int,
) {
    let mut line: *mut cstring = 0 as *mut cstring;
    let mut first_line: uintmax_t = 0;
    let mut lines: uintmax_t = 0;
    let mut i: uintmax_t = 0;
    first_line = get_first_line_in_buffer();
    if first_line > last_line {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            gettext(
                b"%s: line number out of range\0" as *const u8 as *const libc::c_char,
            ),
            quote(*global_argv.offset(argnum as isize)),
        );
        cleanup_fatal();
    }
    lines = last_line.wrapping_sub(first_line);
    i = 0 as libc::c_int as uintmax_t;
    while i < lines {
        line = remove_line();
        if line.is_null() {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"%s: line number out of range\0" as *const u8 as *const libc::c_char,
                ),
                quote(*global_argv.offset(argnum as isize)),
            );
            cleanup_fatal();
        }
        if !ignore {
            save_line_to_file(line);
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn dump_rest_of_file() {
    let mut line: *mut cstring = 0 as *mut cstring;
    loop {
        line = remove_line();
        if line.is_null() {
            break;
        }
        save_line_to_file(line);
    };
}
unsafe extern "C" fn handle_line_error(
    mut p: *const control,
    mut repetition: uintmax_t,
) -> ! {
    let mut buf: [libc::c_char; 21] = [0; 21];
    fprintf(
        stderr,
        gettext(
            b"%s: %s: line number out of range\0" as *const u8 as *const libc::c_char,
        ),
        program_name,
        quote(umaxtostr((*p).lines_required, buf.as_mut_ptr())),
    );
    if repetition != 0 {
        fprintf(
            stderr,
            gettext(b" on repetition %s\n\0" as *const u8 as *const libc::c_char),
            umaxtostr(repetition, buf.as_mut_ptr()),
        );
    } else {
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
    cleanup_fatal();
}
unsafe extern "C" fn process_line_count(
    mut p: *const control,
    mut repetition: uintmax_t,
) {
    let mut linenum: uintmax_t = 0;
    let mut last_line_to_save: uintmax_t = ((*p).lines_required)
        .wrapping_mul(repetition.wrapping_add(1 as libc::c_int as libc::c_ulong));
    create_output_file();
    if no_more_lines() as libc::c_int != 0 && suppress_matched as libc::c_int != 0 {
        handle_line_error(p, repetition);
    }
    linenum = get_first_line_in_buffer();
    loop {
        let fresh2 = linenum;
        linenum = linenum.wrapping_add(1);
        if !(fresh2 < last_line_to_save) {
            break;
        }
        let mut line: *mut cstring = remove_line();
        if line.is_null() {
            handle_line_error(p, repetition);
        }
        save_line_to_file(line);
    }
    close_output_file();
    if suppress_matched {
        remove_line();
    }
    if no_more_lines() as libc::c_int != 0 && !suppress_matched {
        handle_line_error(p, repetition);
    }
}
unsafe extern "C" fn regexp_error(
    mut p: *mut control,
    mut repetition: uintmax_t,
    mut ignore: bool,
) -> ! {
    fprintf(
        stderr,
        gettext(b"%s: %s: match not found\0" as *const u8 as *const libc::c_char),
        program_name,
        quote(*global_argv.offset((*p).argnum as isize)),
    );
    if repetition != 0 {
        let mut buf: [libc::c_char; 21] = [0; 21];
        fprintf(
            stderr,
            gettext(b" on repetition %s\n\0" as *const u8 as *const libc::c_char),
            umaxtostr(repetition, buf.as_mut_ptr()),
        );
    } else {
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
    if !ignore {
        dump_rest_of_file();
        close_output_file();
    }
    cleanup_fatal();
}
unsafe extern "C" fn process_regexp(mut p: *mut control, mut repetition: uintmax_t) {
    let mut line: *mut cstring = 0 as *mut cstring;
    let mut line_len: size_t = 0;
    let mut break_line: uintmax_t = 0;
    let mut ignore: bool = (*p).ignore;
    let mut ret: regoff_t = 0;
    if !ignore {
        create_output_file();
    }
    if (*p).offset >= 0 as libc::c_int as libc::c_long {
        loop {
            current_line = current_line.wrapping_add(1);
            line = find_line(current_line);
            if line.is_null() {
                if (*p).repeat_forever {
                    if !ignore {
                        dump_rest_of_file();
                        close_output_file();
                    }
                    exit(0 as libc::c_int);
                } else {
                    regexp_error(p, repetition, ignore);
                }
            }
            line_len = (*line).len;
            if *((*line).str_0)
                .offset(
                    line_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_int == '\n' as i32
            {
                line_len = line_len.wrapping_sub(1);
                line_len;
            }
            ret = rpl_re_search(
                &mut (*p).re_compiled,
                (*line).str_0,
                line_len as regoff_t,
                0 as libc::c_int as regoff_t,
                line_len as regoff_t,
                0 as *mut re_registers,
            );
            if ret == -(2 as libc::c_int) as libc::c_long {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"error in regular expression search\0" as *const u8
                            as *const libc::c_char,
                    ),
                );
                cleanup_fatal();
            }
            if !(ret == -(1 as libc::c_int) as libc::c_long) {
                break;
            }
            line = remove_line();
            if !ignore {
                save_line_to_file(line);
            }
        }
    } else {
        loop {
            current_line = current_line.wrapping_add(1);
            line = find_line(current_line);
            if line.is_null() {
                if (*p).repeat_forever {
                    if !ignore {
                        dump_rest_of_file();
                        close_output_file();
                    }
                    exit(0 as libc::c_int);
                } else {
                    regexp_error(p, repetition, ignore);
                }
            }
            line_len = (*line).len;
            if *((*line).str_0)
                .offset(
                    line_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_int == '\n' as i32
            {
                line_len = line_len.wrapping_sub(1);
                line_len;
            }
            ret = rpl_re_search(
                &mut (*p).re_compiled,
                (*line).str_0,
                line_len as regoff_t,
                0 as libc::c_int as regoff_t,
                line_len as regoff_t,
                0 as *mut re_registers,
            );
            if ret == -(2 as libc::c_int) as libc::c_long {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"error in regular expression search\0" as *const u8
                            as *const libc::c_char,
                    ),
                );
                cleanup_fatal();
            }
            if ret != -(1 as libc::c_int) as libc::c_long {
                break;
            }
        }
    }
    break_line = current_line.wrapping_add((*p).offset as libc::c_ulong);
    write_to_file(break_line, ignore, (*p).argnum);
    if !ignore {
        close_output_file();
    }
    if (*p).offset > 0 as libc::c_int as libc::c_long {
        current_line = break_line;
    }
    if suppress_matched {
        remove_line();
    }
}
unsafe extern "C" fn split_file() {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < control_used {
        let mut j: uintmax_t = 0;
        if (*controls.offset(i as isize)).regexpr {
            j = 0 as libc::c_int as uintmax_t;
            while (*controls.offset(i as isize)).repeat_forever as libc::c_int != 0
                || j <= (*controls.offset(i as isize)).repeat
            {
                process_regexp(&mut *controls.offset(i as isize), j);
                j = j.wrapping_add(1);
                j;
            }
        } else {
            j = 0 as libc::c_int as uintmax_t;
            while (*controls.offset(i as isize)).repeat_forever as libc::c_int != 0
                || j <= (*controls.offset(i as isize)).repeat
            {
                process_line_count(&mut *controls.offset(i as isize), j);
                j = j.wrapping_add(1);
                j;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    create_output_file();
    dump_rest_of_file();
    close_output_file();
}
unsafe extern "C" fn make_filename(mut num: libc::c_uint) -> *mut libc::c_char {
    strcpy(filename_space, prefix);
    if !suffix.is_null() {
        sprintf(filename_space.offset(strlen(prefix) as isize), suffix, num);
    } else {
        sprintf(
            filename_space.offset(strlen(prefix) as isize),
            b"%0*u\0" as *const u8 as *const libc::c_char,
            digits,
            num,
        );
    }
    return filename_space;
}
unsafe extern "C" fn new_control_record() -> *mut control {
    static mut control_allocated: size_t = 0 as libc::c_int as size_t;
    let mut p: *mut control = 0 as *mut control;
    if control_used == control_allocated {
        controls = (if ::core::mem::size_of::<C2RustUnnamed_15>() as libc::c_ulong != 0 {
            x2nrealloc(
                controls as *mut libc::c_void,
                &mut control_allocated,
                ::core::mem::size_of::<control>() as libc::c_ulong,
            )
        } else {
            x2nrealloc(
                controls as *mut libc::c_void,
                &mut control_allocated,
                ::core::mem::size_of::<control>() as libc::c_ulong,
            )
        }) as *mut control;
    }
    let fresh3 = control_used;
    control_used = control_used.wrapping_add(1);
    p = &mut *controls.offset(fresh3 as isize) as *mut control;
    (*p).regexpr = 0 as libc::c_int != 0;
    (*p).repeat = 0 as libc::c_int as uintmax_t;
    (*p).repeat_forever = 0 as libc::c_int != 0;
    (*p).lines_required = 0 as libc::c_int as uintmax_t;
    (*p).offset = 0 as libc::c_int as intmax_t;
    return p;
}
unsafe extern "C" fn check_for_offset(
    mut p: *mut control,
    mut str: *const libc::c_char,
    mut num: *const libc::c_char,
) {
    if xstrtoimax(
        num,
        0 as *mut *mut libc::c_char,
        10 as libc::c_int,
        &mut (*p).offset,
        b"\0" as *const u8 as *const libc::c_char,
    ) as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint
    {
        if ::core::mem::size_of::<C2RustUnnamed_16>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"%s: integer expected after delimiter\0" as *const u8
                        as *const libc::c_char,
                ),
                quote(str),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"%s: integer expected after delimiter\0" as *const u8
                        as *const libc::c_char,
                ),
                quote(str),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
}
unsafe extern "C" fn parse_repeat_count(
    mut argnum: libc::c_int,
    mut p: *mut control,
    mut str: *mut libc::c_char,
) {
    let mut val: uintmax_t = 0;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    end = str.offset(strlen(str) as isize).offset(-(1 as libc::c_int as isize));
    if *end as libc::c_int != '}' as i32 {
        if ::core::mem::size_of::<C2RustUnnamed_18>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"%s: '}' is required in repeat count\0" as *const u8
                        as *const libc::c_char,
                ),
                quote(str),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"%s: '}' is required in repeat count\0" as *const u8
                        as *const libc::c_char,
                ),
                quote(str),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    *end = '\0' as i32 as libc::c_char;
    if str.offset(1 as libc::c_int as isize) == end.offset(-(1 as libc::c_int as isize))
        && *str.offset(1 as libc::c_int as isize) as libc::c_int == '*' as i32
    {
        (*p).repeat_forever = 1 as libc::c_int != 0;
    } else {
        if xstrtoumax(
            str.offset(1 as libc::c_int as isize),
            0 as *mut *mut libc::c_char,
            10 as libc::c_int,
            &mut val,
            b"\0" as *const u8 as *const libc::c_char,
        ) as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint
        {
            if ::core::mem::size_of::<C2RustUnnamed_17>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"%s}: integer required between '{' and '}'\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quote(*global_argv.offset(argnum as isize)),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"%s}: integer required between '{' and '}'\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quote(*global_argv.offset(argnum as isize)),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        (*p).repeat = val;
    }
    *end = '}' as i32 as libc::c_char;
}
unsafe extern "C" fn extract_regexp(
    mut argnum: libc::c_int,
    mut ignore: bool,
    mut str: *const libc::c_char,
) -> *mut control {
    let mut len: size_t = 0;
    let mut delim: libc::c_char = *str;
    let mut closing_delim: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *mut control = 0 as *mut control;
    let mut err: *const libc::c_char = 0 as *const libc::c_char;
    closing_delim = strrchr(str.offset(1 as libc::c_int as isize), delim as libc::c_int);
    if closing_delim.is_null() {
        if ::core::mem::size_of::<C2RustUnnamed_19>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"%s: closing delimiter '%c' missing\0" as *const u8
                        as *const libc::c_char,
                ),
                str,
                delim as libc::c_int,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"%s: closing delimiter '%c' missing\0" as *const u8
                        as *const libc::c_char,
                ),
                str,
                delim as libc::c_int,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    len = (closing_delim.offset_from(str) as libc::c_long
        - 1 as libc::c_int as libc::c_long) as size_t;
    p = new_control_record();
    (*p).argnum = argnum;
    (*p).ignore = ignore;
    (*p).regexpr = 1 as libc::c_int != 0;
    (*p).re_compiled.buffer = 0 as *mut re_dfa_t;
    (*p).re_compiled.allocated = 0 as libc::c_int as __re_long_size_t;
    (*p)
        .re_compiled
        .fastmap = xmalloc(
        (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int)
            as size_t,
    ) as *mut libc::c_char;
    (*p).re_compiled.translate = 0 as *mut libc::c_uchar;
    rpl_re_syntax_options = (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
        << 1 as libc::c_int
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
        | ((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int)
        & !(((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
        & !(((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int);
    err = rpl_re_compile_pattern(
        str.offset(1 as libc::c_int as isize),
        len,
        &mut (*p).re_compiled,
    );
    if !err.is_null() {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            gettext(
                b"%s: invalid regular expression: %s\0" as *const u8
                    as *const libc::c_char,
            ),
            quote(str),
            err,
        );
        cleanup_fatal();
    }
    if *closing_delim.offset(1 as libc::c_int as isize) != 0 {
        check_for_offset(p, str, closing_delim.offset(1 as libc::c_int as isize));
    }
    return p;
}
unsafe extern "C" fn parse_patterns(
    mut argc: libc::c_int,
    mut start: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut p: *mut control = 0 as *mut control;
    let mut val: uintmax_t = 0;
    static mut last_val: uintmax_t = 0 as libc::c_int as uintmax_t;
    let mut i: libc::c_int = start;
    while i < argc {
        if **argv.offset(i as isize) as libc::c_int == '/' as i32
            || **argv.offset(i as isize) as libc::c_int == '%' as i32
        {
            p = extract_regexp(
                i,
                **argv.offset(i as isize) as libc::c_int == '%' as i32,
                *argv.offset(i as isize),
            );
        } else {
            p = new_control_record();
            (*p).argnum = i;
            if xstrtoumax(
                *argv.offset(i as isize),
                0 as *mut *mut libc::c_char,
                10 as libc::c_int,
                &mut val,
                b"\0" as *const u8 as *const libc::c_char,
            ) as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint
            {
                if ::core::mem::size_of::<C2RustUnnamed_22>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"%s: invalid pattern\0" as *const u8 as *const libc::c_char,
                        ),
                        quote(*argv.offset(i as isize)),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"%s: invalid pattern\0" as *const u8 as *const libc::c_char,
                        ),
                        quote(*argv.offset(i as isize)),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            if val == 0 as libc::c_int as libc::c_ulong {
                if ::core::mem::size_of::<C2RustUnnamed_21>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"%s: line number must be greater than zero\0" as *const u8
                                as *const libc::c_char,
                        ),
                        *argv.offset(i as isize),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"%s: line number must be greater than zero\0" as *const u8
                                as *const libc::c_char,
                        ),
                        *argv.offset(i as isize),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            if val < last_val {
                let mut buf: [libc::c_char; 21] = [0; 21];
                if ::core::mem::size_of::<C2RustUnnamed_20>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"line number %s is smaller than preceding line number, %s\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quote(*argv.offset(i as isize)),
                        umaxtostr(last_val, buf.as_mut_ptr()),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"line number %s is smaller than preceding line number, %s\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quote(*argv.offset(i as isize)),
                        umaxtostr(last_val, buf.as_mut_ptr()),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            if val == last_val {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"warning: line number %s is the same as preceding line number\0"
                            as *const u8 as *const libc::c_char,
                    ),
                    quote(*argv.offset(i as isize)),
                );
            }
            last_val = val;
            (*p).lines_required = val;
        }
        if (i + 1 as libc::c_int) < argc
            && **argv.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                == '{' as i32
        {
            i += 1;
            i;
            parse_repeat_count(i, p, *argv.offset(i as isize));
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn get_format_flags(
    mut format: *const libc::c_char,
    mut flags_ptr: *mut libc::c_int,
) -> size_t {
    let mut flags: libc::c_int = 0 as libc::c_int;
    let mut count: size_t = 0 as libc::c_int as size_t;
    loop {
        match *format.offset(count as isize) as libc::c_int {
            45 | 48 => {}
            39 => {
                flags |= FLAG_THOUSANDS as libc::c_int;
            }
            35 => {
                flags |= FLAG_ALTERNATIVE as libc::c_int;
            }
            _ => {
                *flags_ptr = flags;
                return count;
            }
        }
        count = count.wrapping_add(1);
        count;
    };
}
unsafe extern "C" fn check_format_conv_type(
    mut format: *mut libc::c_char,
    mut flags: libc::c_int,
) {
    let mut ch: libc::c_uchar = *format as libc::c_uchar;
    let mut compatible_flags: libc::c_int = FLAG_THOUSANDS as libc::c_int;
    let mut current_block_5: u64;
    match ch as libc::c_int {
        100 | 105 => {
            *format = 'u' as i32 as libc::c_char;
            current_block_5 = 1917311967535052937;
        }
        117 => {
            current_block_5 = 1917311967535052937;
        }
        111 | 120 | 88 => {
            compatible_flags = FLAG_ALTERNATIVE as libc::c_int;
            current_block_5 = 1917311967535052937;
        }
        0 => {
            if ::core::mem::size_of::<C2RustUnnamed_27>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"missing conversion specifier in suffix\0" as *const u8
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
                        b"missing conversion specifier in suffix\0" as *const u8
                            as *const libc::c_char,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
            current_block_5 = 10352285264753461094;
        }
        _ => {
            current_block_5 = 10352285264753461094;
        }
    }
    match current_block_5 {
        10352285264753461094 => {
            if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as libc::c_int
                & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                if ::core::mem::size_of::<C2RustUnnamed_26>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"invalid conversion specifier in suffix: %c\0" as *const u8
                                as *const libc::c_char,
                        ),
                        ch as libc::c_int,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"invalid conversion specifier in suffix: %c\0" as *const u8
                                as *const libc::c_char,
                        ),
                        ch as libc::c_int,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            } else {
                if ::core::mem::size_of::<C2RustUnnamed_25>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"invalid conversion specifier in suffix: \\%.3o\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        ch as libc::c_int,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"invalid conversion specifier in suffix: \\%.3o\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        ch as libc::c_int,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
        }
        _ => {}
    }
    if flags & !compatible_flags != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_24>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"invalid flags in conversion specification: %%%c%c\0" as *const u8
                        as *const libc::c_char,
                ),
                (if flags & !compatible_flags & FLAG_ALTERNATIVE as libc::c_int != 0 {
                    '#' as i32
                } else {
                    '\'' as i32
                }),
                ch as libc::c_int,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"invalid flags in conversion specification: %%%c%c\0" as *const u8
                        as *const libc::c_char,
                ),
                (if flags & !compatible_flags & FLAG_ALTERNATIVE as libc::c_int != 0 {
                    '#' as i32
                } else {
                    '\'' as i32
                }),
                ch as libc::c_int,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
}
unsafe extern "C" fn max_out(mut format: *mut libc::c_char) -> size_t {
    let mut percent: bool = 0 as libc::c_int != 0;
    let mut f: *mut libc::c_char = format;
    while *f != 0 {
        if *f as libc::c_int == '%' as i32
            && {
                f = f.offset(1);
                *f as libc::c_int != '%' as i32
            }
        {
            if percent {
                if ::core::mem::size_of::<C2RustUnnamed_29>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"too many %% conversion specifications in suffix\0"
                                as *const u8 as *const libc::c_char,
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
                            b"too many %% conversion specifications in suffix\0"
                                as *const u8 as *const libc::c_char,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            percent = 1 as libc::c_int != 0;
            let mut flags: libc::c_int = 0;
            f = f.offset(get_format_flags(f, &mut flags) as isize);
            while (*f as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint
            {
                f = f.offset(1);
                f;
            }
            if *f as libc::c_int == '.' as i32 {
                loop {
                    f = f.offset(1);
                    if !((*f as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                        <= 9 as libc::c_int as libc::c_uint)
                    {
                        break;
                    }
                }
            }
            check_format_conv_type(f, flags);
        }
        f = f.offset(1);
        f;
    }
    if !percent {
        if ::core::mem::size_of::<C2RustUnnamed_28>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"missing %% conversion specification in suffix\0" as *const u8
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
                    b"missing %% conversion specification in suffix\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    let mut maxlen: libc::c_int = snprintf(
        0 as *mut libc::c_char,
        0 as libc::c_int as libc::c_ulong,
        format,
        (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint),
    );
    if !(0 as libc::c_int <= maxlen
        && maxlen as libc::c_ulong <= 18446744073709551615 as libc::c_ulong)
    {
        xalloc_die();
    }
    return maxlen as size_t;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut optc: libc::c_int = 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    global_argv = argv;
    controls = 0 as *mut control;
    control_used = 0 as libc::c_int as size_t;
    suppress_count = 0 as libc::c_int != 0;
    ::core::ptr::write_volatile(&mut remove_files as *mut bool, 1 as libc::c_int != 0);
    suppress_matched = 0 as libc::c_int != 0;
    ::core::ptr::write_volatile(
        &mut prefix as *mut *const libc::c_char,
        b"xx\0" as *const u8 as *const libc::c_char,
    );
    loop {
        optc = getopt_long(
            argc,
            argv,
            b"f:b:kn:sqz\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        match optc {
            102 => {
                ::core::ptr::write_volatile(
                    &mut prefix as *mut *const libc::c_char,
                    optarg,
                );
            }
            98 => {
                ::core::ptr::write_volatile(
                    &mut suffix as *mut *mut libc::c_char,
                    optarg,
                );
            }
            107 => {
                ::core::ptr::write_volatile(
                    &mut remove_files as *mut bool,
                    0 as libc::c_int != 0,
                );
            }
            110 => {
                ::core::ptr::write_volatile(
                    &mut digits as *mut libc::c_int,
                    xdectoimax(
                        optarg,
                        0 as libc::c_int as intmax_t,
                        (if (2147483647 as libc::c_int as libc::c_ulong)
                            < 18446744073709551615 as libc::c_ulong
                        {
                            2147483647 as libc::c_int as libc::c_ulong
                        } else {
                            18446744073709551615 as libc::c_ulong
                        }) as intmax_t,
                        b"\0" as *const u8 as *const libc::c_char,
                        gettext(b"invalid number\0" as *const u8 as *const libc::c_char),
                        0 as libc::c_int,
                    ) as libc::c_int,
                );
            }
            115 | 113 => {
                suppress_count = 1 as libc::c_int != 0;
            }
            122 => {
                elide_empty_files = 1 as libc::c_int != 0;
            }
            128 => {
                suppress_matched = 1 as libc::c_int != 0;
            }
            -130 => {
                usage(0 as libc::c_int);
            }
            -131 => {
                version_etc(
                    stdout,
                    b"csplit\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Stuart Kemp\0" as *const u8 as *const libc::c_char,
                    b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    if argc - optind < 2 as libc::c_int {
        if argc <= optind {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(b"missing operand\0" as *const u8 as *const libc::c_char),
            );
        } else {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"missing operand after %s\0" as *const u8 as *const libc::c_char,
                ),
                quote(*argv.offset((argc - 1 as libc::c_int) as isize)),
            );
        }
        usage(1 as libc::c_int);
    }
    let mut prefix_len: size_t = strlen(prefix);
    let mut max_digit_string_len: size_t = if !suffix.is_null() {
        max_out(suffix)
    } else if (::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_sub(
            !((0 as libc::c_int as libc::c_uint) < -(1 as libc::c_int) as libc::c_uint)
                as libc::c_int as libc::c_ulong,
        )
        .wrapping_mul(146 as libc::c_int as libc::c_ulong)
        .wrapping_add(484 as libc::c_int as libc::c_ulong)
        .wrapping_div(485 as libc::c_int as libc::c_ulong)
        .wrapping_add(
            !((0 as libc::c_int as libc::c_uint) < -(1 as libc::c_int) as libc::c_uint)
                as libc::c_int as libc::c_ulong,
        ) > digits as libc::c_ulong
    {
        (::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(
                !((0 as libc::c_int as libc::c_uint)
                    < -(1 as libc::c_int) as libc::c_uint) as libc::c_int
                    as libc::c_ulong,
            )
            .wrapping_mul(146 as libc::c_int as libc::c_ulong)
            .wrapping_add(484 as libc::c_int as libc::c_ulong)
            .wrapping_div(485 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                !((0 as libc::c_int as libc::c_uint)
                    < -(1 as libc::c_int) as libc::c_uint) as libc::c_int
                    as libc::c_ulong,
            )
    } else {
        digits as libc::c_ulong
    };
    if (18446744073709551615 as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_sub(prefix_len) < max_digit_string_len
    {
        xalloc_die();
    }
    ::core::ptr::write_volatile(
        &mut filename_space as *mut *mut libc::c_char,
        xmalloc(
            prefix_len
                .wrapping_add(max_digit_string_len)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char,
    );
    let fresh4 = optind;
    optind = optind + 1;
    set_input_file(*argv.offset(fresh4 as isize));
    parse_patterns(argc, optind, argv);
    let mut i: libc::c_int = 0;
    static mut sig: [libc::c_int; 11] = [
        14 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        13 as libc::c_int,
        3 as libc::c_int,
        15 as libc::c_int,
        29 as libc::c_int,
        27 as libc::c_int,
        26 as libc::c_int,
        24 as libc::c_int,
        25 as libc::c_int,
    ];
    let mut act: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sigemptyset(&mut caught_signals);
    i = 0 as libc::c_int;
    while i < nsigs as libc::c_int {
        sigaction(sig[i as usize], 0 as *const sigaction, &mut act);
        if act.__sigaction_handler.sa_handler
            != ::core::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t)
        {
            sigaddset(&mut caught_signals, sig[i as usize]);
        }
        i += 1;
        i;
    }
    act
        .__sigaction_handler
        .sa_handler = Some(interrupt_handler as unsafe extern "C" fn(libc::c_int) -> ());
    act.sa_mask = caught_signals;
    act.sa_flags = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nsigs as libc::c_int {
        if sigismember(&mut caught_signals, sig[i as usize]) != 0 {
            sigaction(sig[i as usize], &mut act, 0 as *mut sigaction);
        }
        i += 1;
        i;
    }
    split_file();
    if close(0 as libc::c_int) != 0 as libc::c_int {
        error(
            0 as libc::c_int,
            *__errno_location(),
            gettext(b"read error\0" as *const u8 as *const libc::c_char),
        );
        cleanup_fatal();
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
