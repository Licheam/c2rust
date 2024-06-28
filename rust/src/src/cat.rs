use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn getpagesize() -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
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
    fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    static mut Version: *const libc::c_char;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
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
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn fdadvise(fd: libc::c_int, offset: off_t, len: off_t, advice: fadvice_t);
    fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> size_t;
    fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type off_t = __off_t;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IO_BUFSIZE: C2RustUnnamed_0 = 131072;
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
                b"Concatenate FILE(s) to standard output.\n\0" as *const u8
                    as *const libc::c_char,
            ),
            stdout,
        );
        emit_stdin_note();
        fputs_unlocked(
            gettext(
                b"\n  -A, --show-all           equivalent to -vET\n  -b, --number-nonblank    number nonempty output lines, overrides -n\n  -e                       equivalent to -vE\n  -E, --show-ends          display $ at end of each line\n  -n, --number             number all output lines\n  -s, --squeeze-blank      suppress repeated empty output lines\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -t                       equivalent to -vT\n  -T, --show-tabs          display TAB characters as ^I\n  -u                       (ignored)\n  -v, --show-nonprinting   use ^ and M- notation, except for LFD and TAB\n\0"
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
        printf(
            gettext(
                b"\nExamples:\n  %s f - g  Output f's contents, then standard input, then g's contents.\n  %s        Copy standard input to standard output.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            program_name,
            program_name,
        );
        emit_ancillary_info(b"cat\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
#[inline]
unsafe extern "C" fn ptr_align(
    mut ptr: *const libc::c_void,
    mut alignment: size_t,
) -> *mut libc::c_void {
    let mut p0: *const libc::c_char = ptr as *const libc::c_char;
    let mut p1: *const libc::c_char = p0
        .offset(alignment as isize)
        .offset(-(1 as libc::c_int as isize));
    return p1.offset(-((p1 as size_t).wrapping_rem(alignment) as isize))
        as *mut libc::c_void;
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
unsafe extern "C" fn io_blksize(mut sb: stat) -> size_t {
    return (if IO_BUFSIZE as libc::c_int as libc::c_long
        > (if (0 as libc::c_int as libc::c_long) < sb.st_blksize
            && sb.st_blksize as libc::c_ulong
                <= (-(1 as libc::c_int) as size_t)
                    .wrapping_div(8 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            sb.st_blksize
        } else {
            512 as libc::c_int as libc::c_long
        })
    {
        IO_BUFSIZE as libc::c_int as libc::c_long
    } else if (0 as libc::c_int as libc::c_long) < sb.st_blksize
        && sb.st_blksize as libc::c_ulong
            <= (-(1 as libc::c_int) as size_t)
                .wrapping_div(8 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        sb.st_blksize
    } else {
        512 as libc::c_int as libc::c_long
    }) as size_t;
}
#[inline]
unsafe extern "C" fn __gl_setmode(
    mut fd: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn set_binary_mode(
    mut fd: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    return __gl_setmode(fd, mode);
}
#[inline]
unsafe extern "C" fn xset_binary_mode_error() {}
#[inline]
unsafe extern "C" fn xset_binary_mode(mut fd: libc::c_int, mut mode: libc::c_int) {
    if set_binary_mode(fd, mode) < 0 as libc::c_int {
        xset_binary_mode_error();
    }
}
static mut infile: *const libc::c_char = 0 as *const libc::c_char;
static mut input_desc: libc::c_int = 0;
static mut line_buf: [libc::c_char; 20] = [
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '\t' as i32 as libc::c_char,
    '\0' as i32 as libc::c_char,
];
static mut line_num_print: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut line_num_start: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut line_num_end: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut newlines2: libc::c_int = 0 as libc::c_int;
static mut pending_cr: bool = 0 as libc::c_int != 0;
unsafe extern "C" fn next_line_num() {
    let mut endp: *mut libc::c_char = line_num_end;
    loop {
        let fresh0 = *endp;
        *endp = *endp + 1;
        if (fresh0 as libc::c_int) < '9' as i32 {
            return;
        }
        let fresh1 = endp;
        endp = endp.offset(-1);
        *fresh1 = '0' as i32 as libc::c_char;
        if !(endp >= line_num_start) {
            break;
        }
    }
    if line_num_start > line_buf.as_mut_ptr() {
        line_num_start = line_num_start.offset(-1);
        *line_num_start = '1' as i32 as libc::c_char;
    } else {
        *line_buf.as_mut_ptr() = '>' as i32 as libc::c_char;
    }
    if line_num_start < line_num_print {
        line_num_print = line_num_print.offset(-1);
        line_num_print;
    }
}
unsafe extern "C" fn simple_cat(
    mut buf: *mut libc::c_char,
    mut bufsize: size_t,
) -> bool {
    let mut n_read: size_t = 0;
    loop {
        n_read = safe_read(input_desc, buf as *mut libc::c_void, bufsize);
        if n_read == -(1 as libc::c_int) as size_t {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    infile,
                ),
            );
            return 0 as libc::c_int != 0;
        }
        if n_read == 0 as libc::c_int as libc::c_ulong {
            return 1 as libc::c_int != 0;
        }
        let mut n: size_t = n_read;
        if full_write(1 as libc::c_int, buf as *const libc::c_void, n) != n {
            if ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(b"write error\0" as *const u8 as *const libc::c_char),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(b"write error\0" as *const u8 as *const libc::c_char),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    };
}
#[inline]
unsafe extern "C" fn write_pending(
    mut outbuf: *mut libc::c_char,
    mut bpout: *mut *mut libc::c_char,
) {
    let mut n_write: size_t = (*bpout).offset_from(outbuf) as libc::c_long as size_t;
    if (0 as libc::c_int as libc::c_ulong) < n_write {
        if full_write(1 as libc::c_int, outbuf as *const libc::c_void, n_write)
            != n_write
        {
            if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(b"write error\0" as *const u8 as *const libc::c_char),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(b"write error\0" as *const u8 as *const libc::c_char),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        *bpout = outbuf;
    }
}
unsafe extern "C" fn cat(
    mut inbuf: *mut libc::c_char,
    mut insize: size_t,
    mut outbuf: *mut libc::c_char,
    mut outsize: size_t,
    mut show_nonprinting: bool,
    mut show_tabs: bool,
    mut number: bool,
    mut number_nonblank: bool,
    mut show_ends: bool,
    mut squeeze_blank: bool,
) -> bool {
    let mut ch: libc::c_uchar = 0;
    let mut bpin: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut eob: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bpout: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n_read: size_t = 0;
    let mut newlines: libc::c_int = newlines2;
    let mut use_fionread: bool = 1 as libc::c_int != 0;
    eob = inbuf;
    bpin = eob.offset(1 as libc::c_int as isize);
    bpout = outbuf;
    loop {
        let mut current_block_56: u64;
        loop {
            if outbuf.offset(outsize as isize) <= bpout {
                let mut wp: *mut libc::c_char = outbuf;
                let mut remaining_bytes: size_t = 0;
                loop {
                    if full_write(1 as libc::c_int, wp as *const libc::c_void, outsize)
                        != outsize
                    {
                        if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong
                            != 0
                        {
                            error(
                                1 as libc::c_int,
                                *__errno_location(),
                                gettext(
                                    b"write error\0" as *const u8 as *const libc::c_char,
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
                                    b"write error\0" as *const u8 as *const libc::c_char,
                                ),
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        };
                    }
                    wp = wp.offset(outsize as isize);
                    remaining_bytes = bpout.offset_from(wp) as libc::c_long as size_t;
                    if !(outsize <= remaining_bytes) {
                        break;
                    }
                }
                memmove(
                    outbuf as *mut libc::c_void,
                    wp as *const libc::c_void,
                    remaining_bytes,
                );
                bpout = outbuf.offset(remaining_bytes as isize);
            }
            if bpin > eob {
                let mut input_pending: bool = 0 as libc::c_int != 0;
                let mut n_to_read: libc::c_int = 0 as libc::c_int;
                if use_fionread as libc::c_int != 0
                    && ioctl(
                        input_desc,
                        0x541b as libc::c_int as libc::c_ulong,
                        &mut n_to_read as *mut libc::c_int,
                    ) < 0 as libc::c_int
                {
                    if *__errno_location() == 95 as libc::c_int
                        || *__errno_location() == 25 as libc::c_int
                        || *__errno_location() == 22 as libc::c_int
                        || *__errno_location() == 19 as libc::c_int
                        || *__errno_location() == 38 as libc::c_int
                    {
                        use_fionread = 0 as libc::c_int != 0;
                    } else {
                        error(
                            0 as libc::c_int,
                            *__errno_location(),
                            gettext(
                                b"cannot do ioctl on %s\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            quotearg_style(shell_escape_always_quoting_style, infile),
                        );
                        newlines2 = newlines;
                        return 0 as libc::c_int != 0;
                    }
                }
                if n_to_read != 0 as libc::c_int {
                    input_pending = 1 as libc::c_int != 0;
                }
                if !input_pending {
                    write_pending(outbuf, &mut bpout);
                }
                n_read = safe_read(input_desc, inbuf as *mut libc::c_void, insize);
                if n_read == -(1 as libc::c_int) as size_t {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            infile,
                        ),
                    );
                    write_pending(outbuf, &mut bpout);
                    newlines2 = newlines;
                    return 0 as libc::c_int != 0;
                }
                if n_read == 0 as libc::c_int as libc::c_ulong {
                    write_pending(outbuf, &mut bpout);
                    newlines2 = newlines;
                    return 1 as libc::c_int != 0;
                }
                bpin = inbuf;
                eob = bpin.offset(n_read as isize);
                *eob = '\n' as i32 as libc::c_char;
                current_block_56 = 9627623479216730126;
            } else {
                newlines += 1;
                if newlines > 0 as libc::c_int {
                    if newlines >= 2 as libc::c_int {
                        newlines = 2 as libc::c_int;
                        if squeeze_blank {
                            let fresh2 = bpin;
                            bpin = bpin.offset(1);
                            ch = *fresh2 as libc::c_uchar;
                            current_block_56 = 15427931788582360902;
                        } else {
                            current_block_56 = 15090052786889560393;
                        }
                    } else {
                        current_block_56 = 15090052786889560393;
                    }
                    match current_block_56 {
                        15427931788582360902 => {}
                        _ => {
                            if number as libc::c_int != 0 && !number_nonblank {
                                next_line_num();
                                bpout = stpcpy(bpout, line_num_print);
                            }
                            current_block_56 = 721385680381463314;
                        }
                    }
                } else {
                    current_block_56 = 721385680381463314;
                }
                match current_block_56 {
                    15427931788582360902 => {}
                    _ => {
                        if show_ends {
                            if pending_cr {
                                let fresh3 = bpout;
                                bpout = bpout.offset(1);
                                *fresh3 = '^' as i32 as libc::c_char;
                                let fresh4 = bpout;
                                bpout = bpout.offset(1);
                                *fresh4 = 'M' as i32 as libc::c_char;
                                pending_cr = 0 as libc::c_int != 0;
                            }
                            let fresh5 = bpout;
                            bpout = bpout.offset(1);
                            *fresh5 = '$' as i32 as libc::c_char;
                        }
                        let fresh6 = bpout;
                        bpout = bpout.offset(1);
                        *fresh6 = '\n' as i32 as libc::c_char;
                        current_block_56 = 9627623479216730126;
                    }
                }
            }
            match current_block_56 {
                9627623479216730126 => {
                    let fresh7 = bpin;
                    bpin = bpin.offset(1);
                    ch = *fresh7 as libc::c_uchar;
                }
                _ => {}
            }
            if !(ch as libc::c_int == '\n' as i32) {
                break;
            }
        }
        if pending_cr {
            let fresh8 = bpout;
            bpout = bpout.offset(1);
            *fresh8 = '\r' as i32 as libc::c_char;
            pending_cr = 0 as libc::c_int != 0;
        }
        if newlines >= 0 as libc::c_int && number as libc::c_int != 0 {
            next_line_num();
            bpout = stpcpy(bpout, line_num_print);
        }
        if show_nonprinting {
            loop {
                if ch as libc::c_int >= 32 as libc::c_int {
                    if (ch as libc::c_int) < 127 as libc::c_int {
                        let fresh9 = bpout;
                        bpout = bpout.offset(1);
                        *fresh9 = ch as libc::c_char;
                    } else if ch as libc::c_int == 127 as libc::c_int {
                        let fresh10 = bpout;
                        bpout = bpout.offset(1);
                        *fresh10 = '^' as i32 as libc::c_char;
                        let fresh11 = bpout;
                        bpout = bpout.offset(1);
                        *fresh11 = '?' as i32 as libc::c_char;
                    } else {
                        let fresh12 = bpout;
                        bpout = bpout.offset(1);
                        *fresh12 = 'M' as i32 as libc::c_char;
                        let fresh13 = bpout;
                        bpout = bpout.offset(1);
                        *fresh13 = '-' as i32 as libc::c_char;
                        if ch as libc::c_int >= 128 as libc::c_int + 32 as libc::c_int {
                            if (ch as libc::c_int)
                                < 128 as libc::c_int + 127 as libc::c_int
                            {
                                let fresh14 = bpout;
                                bpout = bpout.offset(1);
                                *fresh14 = (ch as libc::c_int - 128 as libc::c_int)
                                    as libc::c_char;
                            } else {
                                let fresh15 = bpout;
                                bpout = bpout.offset(1);
                                *fresh15 = '^' as i32 as libc::c_char;
                                let fresh16 = bpout;
                                bpout = bpout.offset(1);
                                *fresh16 = '?' as i32 as libc::c_char;
                            }
                        } else {
                            let fresh17 = bpout;
                            bpout = bpout.offset(1);
                            *fresh17 = '^' as i32 as libc::c_char;
                            let fresh18 = bpout;
                            bpout = bpout.offset(1);
                            *fresh18 = (ch as libc::c_int - 128 as libc::c_int
                                + 64 as libc::c_int) as libc::c_char;
                        }
                    }
                } else if ch as libc::c_int == '\t' as i32 && !show_tabs {
                    let fresh19 = bpout;
                    bpout = bpout.offset(1);
                    *fresh19 = '\t' as i32 as libc::c_char;
                } else if ch as libc::c_int == '\n' as i32 {
                    newlines = -(1 as libc::c_int);
                    break;
                } else {
                    let fresh20 = bpout;
                    bpout = bpout.offset(1);
                    *fresh20 = '^' as i32 as libc::c_char;
                    let fresh21 = bpout;
                    bpout = bpout.offset(1);
                    *fresh21 = (ch as libc::c_int + 64 as libc::c_int) as libc::c_char;
                }
                let fresh22 = bpin;
                bpin = bpin.offset(1);
                ch = *fresh22 as libc::c_uchar;
            }
        } else {
            loop {
                if ch as libc::c_int == '\t' as i32 && show_tabs as libc::c_int != 0 {
                    let fresh23 = bpout;
                    bpout = bpout.offset(1);
                    *fresh23 = '^' as i32 as libc::c_char;
                    let fresh24 = bpout;
                    bpout = bpout.offset(1);
                    *fresh24 = (ch as libc::c_int + 64 as libc::c_int) as libc::c_char;
                } else if ch as libc::c_int != '\n' as i32 {
                    if ch as libc::c_int == '\r' as i32
                        && *bpin as libc::c_int == '\n' as i32
                        && show_ends as libc::c_int != 0
                    {
                        if bpin == eob {
                            pending_cr = 1 as libc::c_int != 0;
                        } else {
                            let fresh25 = bpout;
                            bpout = bpout.offset(1);
                            *fresh25 = '^' as i32 as libc::c_char;
                            let fresh26 = bpout;
                            bpout = bpout.offset(1);
                            *fresh26 = 'M' as i32 as libc::c_char;
                        }
                    } else {
                        let fresh27 = bpout;
                        bpout = bpout.offset(1);
                        *fresh27 = ch as libc::c_char;
                    }
                } else {
                    newlines = -(1 as libc::c_int);
                    break;
                }
                let fresh28 = bpin;
                bpin = bpin.offset(1);
                ch = *fresh28 as libc::c_uchar;
            }
        }
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut outsize: size_t = 0;
    let mut insize: size_t = 0;
    let mut page_size: size_t = getpagesize() as size_t;
    let mut inbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut c: libc::c_int = 0;
    let mut argind: libc::c_int = 0;
    let mut out_dev: dev_t = 0;
    let mut out_ino: ino_t = 0;
    let mut out_isreg: bool = false;
    let mut have_read_stdin: bool = 0 as libc::c_int != 0;
    let mut stat_buf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut number: bool = 0 as libc::c_int != 0;
    let mut number_nonblank: bool = 0 as libc::c_int != 0;
    let mut squeeze_blank: bool = 0 as libc::c_int != 0;
    let mut show_ends: bool = 0 as libc::c_int != 0;
    let mut show_nonprinting: bool = 0 as libc::c_int != 0;
    let mut show_tabs: bool = 0 as libc::c_int != 0;
    let mut file_open_mode: libc::c_int = 0 as libc::c_int;
    static mut long_options: [option; 10] = [
        {
            let mut init = option {
                name: b"number-nonblank\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'b' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"number\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'n' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"squeeze-blank\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 's' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"show-nonprinting\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'v' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"show-ends\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'E' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"show-tabs\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'T' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"show-all\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'A' as i32,
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
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    loop {
        c = getopt_long(
            argc,
            argv,
            b"benstuvAET\0" as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            98 => {
                number = 1 as libc::c_int != 0;
                number_nonblank = 1 as libc::c_int != 0;
            }
            101 => {
                show_ends = 1 as libc::c_int != 0;
                show_nonprinting = 1 as libc::c_int != 0;
            }
            110 => {
                number = 1 as libc::c_int != 0;
            }
            115 => {
                squeeze_blank = 1 as libc::c_int != 0;
            }
            116 => {
                show_tabs = 1 as libc::c_int != 0;
                show_nonprinting = 1 as libc::c_int != 0;
            }
            117 => {}
            118 => {
                show_nonprinting = 1 as libc::c_int != 0;
            }
            65 => {
                show_nonprinting = 1 as libc::c_int != 0;
                show_ends = 1 as libc::c_int != 0;
                show_tabs = 1 as libc::c_int != 0;
            }
            69 => {
                show_ends = 1 as libc::c_int != 0;
            }
            84 => {
                show_tabs = 1 as libc::c_int != 0;
            }
            -130 => {
                usage(0 as libc::c_int);
            }
            -131 => {
                version_etc(
                    stdout,
                    b"cat\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Torbjorn Granlund\0" as *const u8 as *const libc::c_char,
                    b"Richard M. Stallman\0" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    if fstat(1 as libc::c_int, &mut stat_buf) < 0 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                gettext(b"standard output\0" as *const u8 as *const libc::c_char),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                *__errno_location(),
                gettext(b"standard output\0" as *const u8 as *const libc::c_char),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    outsize = io_blksize(stat_buf);
    out_dev = stat_buf.st_dev;
    out_ino = stat_buf.st_ino;
    out_isreg = (stat_buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint) as libc::c_int != 0 as libc::c_int;
    if !(number as libc::c_int != 0 || show_ends as libc::c_int != 0
        || squeeze_blank as libc::c_int != 0)
    {
        file_open_mode |= 0 as libc::c_int;
        xset_binary_mode(1 as libc::c_int, 0 as libc::c_int);
    }
    infile = b"-\0" as *const u8 as *const libc::c_char;
    argind = optind;
    loop {
        if argind < argc {
            infile = *argv.offset(argind as isize);
        }
        if strcmp(infile, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            have_read_stdin = 1 as libc::c_int != 0;
            input_desc = 0 as libc::c_int;
            if file_open_mode & 0 as libc::c_int != 0 {
                xset_binary_mode(0 as libc::c_int, 0 as libc::c_int);
            }
            current_block = 2500484646272006982;
        } else {
            input_desc = open(infile, file_open_mode);
            if input_desc < 0 as libc::c_int {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        infile,
                    ),
                );
                ok = 0 as libc::c_int != 0;
                current_block = 9441801433784995173;
            } else {
                current_block = 2500484646272006982;
            }
        }
        match current_block {
            2500484646272006982 => {
                if fstat(input_desc, &mut stat_buf) < 0 as libc::c_int {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            infile,
                        ),
                    );
                    ok = 0 as libc::c_int != 0;
                } else {
                    insize = io_blksize(stat_buf);
                    fdadvise(
                        input_desc,
                        0 as libc::c_int as off_t,
                        0 as libc::c_int as off_t,
                        FADVISE_SEQUENTIAL,
                    );
                    if out_isreg as libc::c_int != 0 && stat_buf.st_dev == out_dev
                        && stat_buf.st_ino == out_ino
                        && lseek(
                            input_desc,
                            0 as libc::c_int as __off_t,
                            1 as libc::c_int,
                        ) < stat_buf.st_size
                    {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"%s: input file is output file\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                infile,
                            ),
                        );
                        ok = 0 as libc::c_int != 0;
                    } else {
                        if !(number as libc::c_int != 0 || show_ends as libc::c_int != 0
                            || show_nonprinting as libc::c_int != 0
                            || show_tabs as libc::c_int != 0
                            || squeeze_blank as libc::c_int != 0)
                        {
                            insize = if insize > outsize { insize } else { outsize };
                            inbuf = xmalloc(
                                insize
                                    .wrapping_add(page_size)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            ) as *mut libc::c_char;
                            ok = (ok as libc::c_int
                                & simple_cat(
                                    ptr_align(inbuf as *const libc::c_void, page_size)
                                        as *mut libc::c_char,
                                    insize,
                                ) as libc::c_int) as bool;
                        } else {
                            inbuf = xmalloc(
                                insize
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(page_size)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            ) as *mut libc::c_char;
                            outbuf = xmalloc(
                                outsize
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(
                                        insize.wrapping_mul(4 as libc::c_int as libc::c_ulong),
                                    )
                                    .wrapping_add(20 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(page_size)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            ) as *mut libc::c_char;
                            ok = (ok as libc::c_int
                                & cat(
                                    ptr_align(inbuf as *const libc::c_void, page_size)
                                        as *mut libc::c_char,
                                    insize,
                                    ptr_align(outbuf as *const libc::c_void, page_size)
                                        as *mut libc::c_char,
                                    outsize,
                                    show_nonprinting,
                                    show_tabs,
                                    number,
                                    number_nonblank,
                                    show_ends,
                                    squeeze_blank,
                                ) as libc::c_int) as bool;
                            free(outbuf as *mut libc::c_void);
                        }
                        free(inbuf as *mut libc::c_void);
                    }
                }
                if !(strcmp(infile, b"-\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int) && close(input_desc) < 0 as libc::c_int
                {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            infile,
                        ),
                    );
                    ok = 0 as libc::c_int != 0;
                }
            }
            _ => {}
        }
        argind += 1;
        if !(argind < argc) {
            break;
        }
    }
    if pending_cr {
        if full_write(
            1 as libc::c_int,
            b"\r\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        ) != 1 as libc::c_int as libc::c_ulong
        {
            if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(b"write error\0" as *const u8 as *const libc::c_char),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(b"write error\0" as *const u8 as *const libc::c_char),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    if have_read_stdin as libc::c_int != 0 && close(0 as libc::c_int) < 0 as libc::c_int
    {
        if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                gettext(b"closing standard input\0" as *const u8 as *const libc::c_char),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                *__errno_location(),
                gettext(b"closing standard input\0" as *const u8 as *const libc::c_char),
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
unsafe extern "C" fn run_static_initializers() {
    line_num_print = line_buf
        .as_mut_ptr()
        .offset(20 as libc::c_int as isize)
        .offset(-(8 as libc::c_int as isize));
    line_num_start = line_buf
        .as_mut_ptr()
        .offset(20 as libc::c_int as isize)
        .offset(-(3 as libc::c_int as isize));
    line_num_end = line_buf
        .as_mut_ptr()
        .offset(20 as libc::c_int as isize)
        .offset(-(3 as libc::c_int as isize));
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
