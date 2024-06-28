use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut optind: libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    static mut program_name: *const libc::c_char;
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
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn parse_gnu_standard_options_only(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        scan_all: bool,
        usage_func: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        _: ...
    );
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn extract_trimmed_name(ut: *const STRUCT_UTMP) -> *mut libc::c_char;
    fn read_utmp(
        file: *const libc::c_char,
        n_entries: *mut size_t,
        utmp_buf: *mut *mut STRUCT_UTMP,
        options: libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __exit_status {
    pub e_termination: libc::c_short,
    pub e_exit: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utmpx {
    pub ut_type: libc::c_short,
    pub ut_pid: __pid_t,
    pub ut_line: [libc::c_char; 32],
    pub ut_id: [libc::c_char; 4],
    pub ut_user: [libc::c_char; 32],
    pub ut_host: [libc::c_char; 256],
    pub ut_exit: __exit_status,
    pub ut_session: __int32_t,
    pub ut_tv: C2RustUnnamed,
    pub ut_addr_v6: [__int32_t; 4],
    pub __glibc_reserved: [libc::c_char; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub tv_sec: __int32_t,
    pub tv_usec: __int32_t,
}
pub type STRUCT_UTMP = utmpx;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const READ_UTMP_USER_PROCESS: C2RustUnnamed_0 = 2;
pub const READ_UTMP_CHECK_PIDS: C2RustUnnamed_0 = 1;
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
                b"Usage: %s [OPTION]... [FILE]\n\0" as *const u8 as *const libc::c_char,
            ),
            program_name,
        );
        printf(
            gettext(
                b"Output who is currently logged in according to FILE.\nIf FILE is not specified, use %s.  %s as FILE is common.\n\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            b"/var/run/utmp\0" as *const u8 as *const libc::c_char,
            b"/var/log/wtmp\0" as *const u8 as *const libc::c_char,
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
        emit_ancillary_info(b"users\0" as *const u8 as *const libc::c_char);
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
unsafe extern "C" fn userid_compare(
    mut v_a: *const libc::c_void,
    mut v_b: *const libc::c_void,
) -> libc::c_int {
    let mut a: *mut *mut libc::c_char = v_a as *mut *mut libc::c_char;
    let mut b: *mut *mut libc::c_char = v_b as *mut *mut libc::c_char;
    return strcmp(*a, *b);
}
unsafe extern "C" fn list_entries_users(mut n: size_t, mut this: *const STRUCT_UTMP) {
    let mut u: *mut *mut libc::c_char = xnmalloc(
        n,
        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    let mut i: size_t = 0;
    let mut n_entries: size_t = 0 as libc::c_int as size_t;
    loop {
        let fresh0 = n;
        n = n.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        if (*this).ut_user[0 as libc::c_int as usize] as libc::c_int != 0
            && ((*this).ut_type as libc::c_int == 7 as libc::c_int
                || 0 as libc::c_int != 0 && (*this).ut_tv.tv_sec != 0 as libc::c_int)
        {
            let mut trimmed_name: *mut libc::c_char = 0 as *mut libc::c_char;
            trimmed_name = extract_trimmed_name(this);
            let ref mut fresh1 = *u.offset(n_entries as isize);
            *fresh1 = trimmed_name;
            n_entries = n_entries.wrapping_add(1);
            n_entries;
        }
        this = this.offset(1);
        this;
    }
    qsort(
        u as *mut libc::c_void,
        n_entries,
        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        Some(
            userid_compare
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int as size_t;
    while i < n_entries {
        let mut c: libc::c_char = (if i
            < n_entries.wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            ' ' as i32
        } else {
            '\n' as i32
        }) as libc::c_char;
        fputs_unlocked(*u.offset(i as isize), stdout);
        putchar_unlocked(c as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < n_entries {
        free(*u.offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1);
        i;
    }
    free(u as *mut libc::c_void);
}
unsafe extern "C" fn users(mut filename: *const libc::c_char, mut options: libc::c_int) {
    let mut n_users: size_t = 0;
    let mut utmp_buf: *mut STRUCT_UTMP = 0 as *mut STRUCT_UTMP;
    if read_utmp(filename, &mut n_users, &mut utmp_buf, options) != 0 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    filename,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    filename,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    list_entries_users(n_users, utmp_buf);
    free(utmp_buf as *mut libc::c_void);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    parse_gnu_standard_options_only(
        argc,
        argv,
        b"users\0" as *const u8 as *const libc::c_char,
        b"GNU coreutils\0" as *const u8 as *const libc::c_char,
        Version,
        1 as libc::c_int != 0,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int) -> !>,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(Some(usage as unsafe extern "C" fn(libc::c_int) -> !)),
        b"Joseph Arceneaux\0" as *const u8 as *const libc::c_char,
        b"David MacKenzie\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    match argc - optind {
        0 => {
            users(
                b"/var/run/utmp\0" as *const u8 as *const libc::c_char,
                READ_UTMP_CHECK_PIDS as libc::c_int,
            );
        }
        1 => {
            users(*argv.offset(optind as isize), 0 as libc::c_int);
        }
        _ => {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(b"extra operand %s\0" as *const u8 as *const libc::c_char),
                quote(*argv.offset((optind + 1 as libc::c_int) as isize)),
            );
            usage(1 as libc::c_int);
        }
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
