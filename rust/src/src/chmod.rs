use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    pub type mode_change;
    pub type cycle_check_state;
    pub type hash_table;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optind: libc::c_int;
    fn fstatat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn fchmodat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __mode: __mode_t,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn memcpy(
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn quotearg_n_style(
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
    fn abort() -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn strmode(mode: mode_t, str: *mut libc::c_char);
    fn mode_compile(_: *const libc::c_char) -> *mut mode_change;
    fn mode_create_from_ref(_: *const libc::c_char) -> *mut mode_change;
    fn mode_adjust(
        _: mode_t,
        _: bool,
        _: mode_t,
        _: *const mode_change,
        _: *mut mode_t,
    ) -> mode_t;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn get_root_dev_ino(root_d_i: *mut dev_ino) -> *mut dev_ino;
    fn cycle_warning_required(fts: *const FTS, ent: *const FTSENT) -> bool;
    fn xfts_open(
        _: *const *mut libc::c_char,
        options: libc::c_int,
        _: Option::<
            unsafe extern "C" fn(*mut *const FTSENT, *mut *const FTSENT) -> libc::c_int,
        >,
    ) -> *mut FTS;
    fn rpl_fts_read(_: *mut FTS) -> *mut FTSENT;
    fn rpl_fts_close(_: *mut FTS) -> libc::c_int;
    fn rpl_fts_set(_: *mut FTS, _: *mut FTSENT, _: libc::c_int) -> libc::c_int;
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
pub type ptrdiff_t = libc::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
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
pub type DIR = __dirstream;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dev_ino {
    pub st_ino: ino_t,
    pub st_dev: dev_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct I_ring {
    pub ir_data: [libc::c_int; 4],
    pub ir_default_val: libc::c_int,
    pub ir_front: libc::c_uint,
    pub ir_back: libc::c_uint,
    pub ir_empty: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FTS {
    pub fts_cur: *mut _ftsent,
    pub fts_child: *mut _ftsent,
    pub fts_array: *mut *mut _ftsent,
    pub fts_dev: dev_t,
    pub fts_path: *mut libc::c_char,
    pub fts_rfd: libc::c_int,
    pub fts_cwd_fd: libc::c_int,
    pub fts_pathlen: size_t,
    pub fts_nitems: size_t,
    pub fts_compar: Option::<
        unsafe extern "C" fn(*mut *const _ftsent, *mut *const _ftsent) -> libc::c_int,
    >,
    pub fts_options: libc::c_int,
    pub fts_leaf_optimization_works_ht: *mut hash_table,
    pub fts_cycle: C2RustUnnamed_0,
    pub fts_fd_ring: I_ring,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ht: *mut hash_table,
    pub state: *mut cycle_check_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ftsent {
    pub fts_cycle: *mut _ftsent,
    pub fts_parent: *mut _ftsent,
    pub fts_link: *mut _ftsent,
    pub fts_dirp: *mut DIR,
    pub fts_number: libc::c_long,
    pub fts_pointer: *mut libc::c_void,
    pub fts_accpath: *mut libc::c_char,
    pub fts_path: *mut libc::c_char,
    pub fts_errno: libc::c_int,
    pub fts_symfd: libc::c_int,
    pub fts_pathlen: size_t,
    pub fts_fts: *mut FTS,
    pub fts_level: ptrdiff_t,
    pub fts_namelen: size_t,
    pub fts_info: libc::c_ushort,
    pub fts_flags: libc::c_ushort,
    pub fts_instr: libc::c_ushort,
    pub fts_statp: [stat; 1],
    pub fts_name: [libc::c_char; 0],
}
pub type FTSENT = _ftsent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct change_status {
    pub status: C2RustUnnamed_1,
    pub old_mode: mode_t,
    pub new_mode: mode_t,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const CH_SUCCEEDED: C2RustUnnamed_1 = 4;
pub const CH_NO_CHANGE_REQUESTED: C2RustUnnamed_1 = 3;
pub const CH_FAILED: C2RustUnnamed_1 = 2;
pub const CH_NOT_APPLIED: C2RustUnnamed_1 = 1;
pub const CH_NO_STAT: C2RustUnnamed_1 = 0;
pub type Verbosity = libc::c_uint;
pub const V_off: Verbosity = 2;
pub const V_changes_only: Verbosity = 1;
pub const V_high: Verbosity = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const REFERENCE_FILE_OPTION: C2RustUnnamed_2 = 130;
pub const PRESERVE_ROOT: C2RustUnnamed_2 = 129;
pub const NO_PRESERVE_ROOT: C2RustUnnamed_2 = 128;
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
                b"Usage: %s [OPTION]... MODE[,MODE]... FILE...\n  or:  %s [OPTION]... OCTAL-MODE FILE...\n  or:  %s [OPTION]... --reference=RFILE FILE...\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            program_name,
            program_name,
            program_name,
        );
        fputs_unlocked(
            gettext(
                b"Change the mode of each FILE to MODE.\nWith --reference, change the mode of each FILE to that of RFILE.\n\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -c, --changes          like verbose but report only when a change is made\n  -f, --silent, --quiet  suppress most error messages\n  -v, --verbose          output a diagnostic for every file processed\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"      --no-preserve-root  do not treat '/' specially (the default)\n      --preserve-root    fail to operate recursively on '/'\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"      --reference=RFILE  use RFILE's mode instead of MODE values\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -R, --recursive        change files and directories recursively\n\0"
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
                b"\nEach MODE is of the form '[ugoa]*([-+=]([rwxXst]*|[ugo]))+|[-+=][0-7]+'.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_ancillary_info(b"chmod\0" as *const u8 as *const libc::c_char);
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
unsafe extern "C" fn chmodat(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    return fchmodat(fd, file, mode, 0 as libc::c_int);
}
static mut change: *mut mode_change = 0 as *const mode_change as *mut mode_change;
static mut umask_value: mode_t = 0;
static mut recurse: bool = false;
static mut force_silent: bool = false;
static mut diagnose_surprises: bool = false;
static mut verbosity: Verbosity = V_off;
static mut root_dev_ino: *mut dev_ino = 0 as *const dev_ino as *mut dev_ino;
static mut long_options: [option; 11] = [
    {
        let mut init = option {
            name: b"changes\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"recursive\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'R' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-preserve-root\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: NO_PRESERVE_ROOT as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"preserve-root\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: PRESERVE_ROOT as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"quiet\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"reference\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: REFERENCE_FILE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"silent\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
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
unsafe extern "C" fn mode_changed(
    mut dir_fd: libc::c_int,
    mut file: *const libc::c_char,
    mut file_full_name: *const libc::c_char,
    mut old_mode: mode_t,
    mut new_mode: mode_t,
) -> bool {
    if new_mode
        & (0o4000 as libc::c_int | 0o2000 as libc::c_int | 0o1000 as libc::c_int)
            as libc::c_uint != 0
    {
        let mut new_stats: stat = stat {
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
        if fstatat(dir_fd, file, &mut new_stats, 0 as libc::c_int) != 0 as libc::c_int {
            if !force_silent {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"getting new attributes of %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, file_full_name),
                );
            }
            return 0 as libc::c_int != 0;
        }
        new_mode = new_stats.st_mode;
    }
    return (old_mode ^ new_mode)
        & (0o4000 as libc::c_int | 0o2000 as libc::c_int | 0o1000 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_uint
        != 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn describe_change(
    mut file: *const libc::c_char,
    mut ch: *const change_status,
) {
    let mut perms: [libc::c_char; 12] = [0; 12];
    let mut old_perms: [libc::c_char; 12] = [0; 12];
    let mut fmt: *const libc::c_char = 0 as *const libc::c_char;
    let mut quoted_file: *const libc::c_char = quotearg_style(
        shell_escape_always_quoting_style,
        file,
    );
    match (*ch).status as libc::c_uint {
        1 => {
            printf(
                gettext(
                    b"neither symbolic link %s nor referent has been changed\n\0"
                        as *const u8 as *const libc::c_char,
                ),
                quoted_file,
            );
            return;
        }
        0 => {
            printf(
                gettext(
                    b"%s could not be accessed\n\0" as *const u8 as *const libc::c_char,
                ),
                quoted_file,
            );
            return;
        }
        _ => {}
    }
    let mut old_m: libc::c_ulong = ((*ch).old_mode
        & (0o4000 as libc::c_int | 0o2000 as libc::c_int | 0o1000 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_uint)
        as libc::c_ulong;
    let mut m: libc::c_ulong = ((*ch).new_mode
        & (0o4000 as libc::c_int | 0o2000 as libc::c_int | 0o1000 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_uint)
        as libc::c_ulong;
    strmode((*ch).new_mode, perms.as_mut_ptr());
    perms[10 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    strmode((*ch).old_mode, old_perms.as_mut_ptr());
    old_perms[10 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    match (*ch).status as libc::c_uint {
        4 => {
            fmt = gettext(
                b"mode of %s changed from %04lo (%s) to %04lo (%s)\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        2 => {
            fmt = gettext(
                b"failed to change mode of %s from %04lo (%s) to %04lo (%s)\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        3 => {
            fmt = gettext(
                b"mode of %s retained as %04lo (%s)\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(
                fmt,
                quoted_file,
                m,
                &mut *perms.as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *mut libc::c_char,
            );
            return;
        }
        _ => {
            abort();
        }
    }
    printf(
        fmt,
        quoted_file,
        old_m,
        &mut *old_perms.as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut libc::c_char,
        m,
        &mut *perms.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut libc::c_char,
    );
}
unsafe extern "C" fn process_file(mut fts: *mut FTS, mut ent: *mut FTSENT) -> bool {
    let mut file_full_name: *const libc::c_char = (*ent).fts_path;
    let mut file: *const libc::c_char = (*ent).fts_accpath;
    let mut file_stats: *const stat = ((*ent).fts_statp).as_mut_ptr();
    let mut ch: change_status = {
        let mut init = change_status {
            status: CH_NO_STAT,
            old_mode: 0,
            new_mode: 0,
        };
        init
    };
    ch.status = CH_NO_STAT;
    let mut current_block_22: u64;
    match (*ent).fts_info as libc::c_int {
        6 => return 1 as libc::c_int != 0,
        10 => {
            if (*ent).fts_level == 0 as libc::c_int as libc::c_long
                && (*ent).fts_number == 0 as libc::c_int as libc::c_long
            {
                (*ent).fts_number = 1 as libc::c_int as libc::c_long;
                rpl_fts_set(fts, ent, 1 as libc::c_int);
                return 1 as libc::c_int != 0;
            }
            if !force_silent {
                error(
                    0 as libc::c_int,
                    (*ent).fts_errno,
                    gettext(b"cannot access %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, file_full_name),
                );
            }
            current_block_22 = 11307063007268554308;
        }
        7 => {
            if !force_silent {
                error(
                    0 as libc::c_int,
                    (*ent).fts_errno,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        file_full_name,
                    ),
                );
            }
            current_block_22 = 11307063007268554308;
        }
        4 => {
            if !force_silent {
                error(
                    0 as libc::c_int,
                    (*ent).fts_errno,
                    gettext(
                        b"cannot read directory %s\0" as *const u8 as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, file_full_name),
                );
            }
            current_block_22 = 11307063007268554308;
        }
        13 => {
            if !force_silent {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"cannot operate on dangling symlink %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, file_full_name),
                );
            }
            current_block_22 = 11307063007268554308;
        }
        2 => {
            if cycle_warning_required(fts, ent) {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"WARNING: Circular directory structure.\nThis almost certainly means that you have a corrupted file system.\nNOTIFY YOUR SYSTEM MANAGER.\nThe following directory is part of the cycle:\n  %s\n\0"
                            as *const u8 as *const libc::c_char,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        file_full_name,
                    ),
                );
                return 0 as libc::c_int != 0;
            }
            current_block_22 = 14625374194610218897;
        }
        _ => {
            current_block_22 = 14625374194610218897;
        }
    }
    match current_block_22 {
        14625374194610218897 => {
            ch.status = CH_NOT_APPLIED;
        }
        _ => {}
    }
    if ch.status as libc::c_uint == CH_NOT_APPLIED as libc::c_int as libc::c_uint
        && (!root_dev_ino.is_null()
            && ((*file_stats).st_ino == (*root_dev_ino).st_ino
                && (*file_stats).st_dev == (*root_dev_ino).st_dev))
    {
        if strcmp(file_full_name, b"/\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"it is dangerous to operate recursively on %s\0" as *const u8
                        as *const libc::c_char,
                ),
                quotearg_style(shell_escape_always_quoting_style, file_full_name),
            );
        } else {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"it is dangerous to operate recursively on %s (same as %s)\0"
                        as *const u8 as *const libc::c_char,
                ),
                quotearg_n_style(
                    0 as libc::c_int,
                    shell_escape_always_quoting_style,
                    file_full_name,
                ),
                quotearg_n_style(
                    1 as libc::c_int,
                    shell_escape_always_quoting_style,
                    b"/\0" as *const u8 as *const libc::c_char,
                ),
            );
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            gettext(
                b"use --no-preserve-root to override this failsafe\0" as *const u8
                    as *const libc::c_char,
            ),
        );
        rpl_fts_set(fts, ent, 4 as libc::c_int);
        rpl_fts_read(fts);
        return 0 as libc::c_int != 0;
    }
    if ch.status as libc::c_uint == CH_NOT_APPLIED as libc::c_int as libc::c_uint
        && !((*file_stats).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint)
    {
        ch.old_mode = (*file_stats).st_mode;
        ch
            .new_mode = mode_adjust(
            ch.old_mode,
            (ch.old_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int
                != 0 as libc::c_int,
            umask_value,
            change,
            0 as *mut mode_t,
        );
        if chmodat((*fts).fts_cwd_fd, file, ch.new_mode) == 0 as libc::c_int {
            ch.status = CH_SUCCEEDED;
        } else {
            if !force_silent {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"changing permissions of %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, file_full_name),
                );
            }
            ch.status = CH_FAILED;
        }
    }
    if verbosity as libc::c_uint != V_off as libc::c_int as libc::c_uint {
        if ch.status as libc::c_uint == CH_SUCCEEDED as libc::c_int as libc::c_uint
            && !mode_changed(
                (*fts).fts_cwd_fd,
                file,
                file_full_name,
                ch.old_mode,
                ch.new_mode,
            )
        {
            ch.status = CH_NO_CHANGE_REQUESTED;
        }
        if ch.status as libc::c_uint == CH_SUCCEEDED as libc::c_int as libc::c_uint
            || verbosity as libc::c_uint == V_high as libc::c_int as libc::c_uint
        {
            describe_change(file_full_name, &mut ch);
        }
    }
    if CH_NO_CHANGE_REQUESTED as libc::c_int as libc::c_uint <= ch.status as libc::c_uint
        && diagnose_surprises as libc::c_int != 0
    {
        let mut naively_expected_mode: mode_t = mode_adjust(
            ch.old_mode,
            (ch.old_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int
                != 0 as libc::c_int,
            0 as libc::c_int as mode_t,
            change,
            0 as *mut mode_t,
        );
        if ch.new_mode & !naively_expected_mode != 0 {
            let mut new_perms: [libc::c_char; 12] = [0; 12];
            let mut naively_expected_perms: [libc::c_char; 12] = [0; 12];
            strmode(ch.new_mode, new_perms.as_mut_ptr());
            strmode(naively_expected_mode, naively_expected_perms.as_mut_ptr());
            naively_expected_perms[10 as libc::c_int
                as usize] = '\0' as i32 as libc::c_char;
            new_perms[10 as libc::c_int
                as usize] = naively_expected_perms[10 as libc::c_int as usize];
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"%s: new permissions are %s, not %s\0" as *const u8
                        as *const libc::c_char,
                ),
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    file_full_name,
                ),
                new_perms.as_mut_ptr().offset(1 as libc::c_int as isize),
                naively_expected_perms.as_mut_ptr().offset(1 as libc::c_int as isize),
            );
            ch.status = CH_FAILED;
        }
    }
    if !recurse {
        rpl_fts_set(fts, ent, 4 as libc::c_int);
    }
    return CH_NO_CHANGE_REQUESTED as libc::c_int as libc::c_uint
        <= ch.status as libc::c_uint;
}
unsafe extern "C" fn process_files(
    mut files: *mut *mut libc::c_char,
    mut bit_flags: libc::c_int,
) -> bool {
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut fts: *mut FTS = xfts_open(files, bit_flags, None);
    loop {
        let mut ent: *mut FTSENT = 0 as *mut FTSENT;
        ent = rpl_fts_read(fts);
        if ent.is_null() {
            if *__errno_location() != 0 as libc::c_int {
                if !force_silent {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        gettext(b"fts_read failed\0" as *const u8 as *const libc::c_char),
                    );
                }
                ok = 0 as libc::c_int != 0;
            }
            break;
        } else {
            ok = (ok as libc::c_int & process_file(fts, ent) as libc::c_int) as bool;
        }
    }
    if rpl_fts_close(fts) != 0 as libc::c_int {
        error(
            0 as libc::c_int,
            *__errno_location(),
            gettext(b"fts_close failed\0" as *const u8 as *const libc::c_char),
        );
        ok = 0 as libc::c_int != 0;
    }
    return ok;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut mode: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mode_len: size_t = 0 as libc::c_int as size_t;
    let mut mode_alloc: size_t = 0 as libc::c_int as size_t;
    let mut ok: bool = false;
    let mut preserve_root: bool = 0 as libc::c_int != 0;
    let mut reference_file: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: libc::c_int = 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    diagnose_surprises = 0 as libc::c_int != 0;
    force_silent = diagnose_surprises;
    recurse = force_silent;
    loop {
        c = getopt_long(
            argc,
            argv,
            b"Rcfvr::w::x::X::s::t::u::g::o::a::,::+::=::0::1::2::3::4::5::6::7::\0"
                as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            114 | 119 | 120 | 88 | 115 | 116 | 117 | 103 | 111 | 97 | 44 | 43 | 61 | 48
            | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                let mut arg: *const libc::c_char = *argv
                    .offset((optind - 1 as libc::c_int) as isize);
                let mut arg_len: size_t = strlen(arg);
                let mut mode_comma_len: size_t = mode_len
                    .wrapping_add((mode_len != 0) as libc::c_int as libc::c_ulong);
                let mut new_mode_len: size_t = mode_comma_len.wrapping_add(arg_len);
                if mode_alloc <= new_mode_len {
                    mode_alloc = new_mode_len
                        .wrapping_add(1 as libc::c_int as libc::c_ulong);
                    mode = (if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong
                        != 0
                    {
                        x2realloc(mode as *mut libc::c_void, &mut mode_alloc)
                    } else {
                        x2realloc(mode as *mut libc::c_void, &mut mode_alloc)
                    }) as *mut libc::c_char;
                }
                *mode.offset(mode_len as isize) = ',' as i32 as libc::c_char;
                memcpy(
                    mode.offset(mode_comma_len as isize) as *mut libc::c_void,
                    arg as *const libc::c_void,
                    arg_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                mode_len = new_mode_len;
                diagnose_surprises = 1 as libc::c_int != 0;
            }
            128 => {
                preserve_root = 0 as libc::c_int != 0;
            }
            129 => {
                preserve_root = 1 as libc::c_int != 0;
            }
            130 => {
                reference_file = optarg;
            }
            82 => {
                recurse = 1 as libc::c_int != 0;
            }
            99 => {
                verbosity = V_changes_only;
            }
            102 => {
                force_silent = 1 as libc::c_int != 0;
            }
            118 => {
                verbosity = V_high;
            }
            -130 => {
                usage(0 as libc::c_int);
            }
            -131 => {
                version_etc(
                    stdout,
                    b"chmod\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
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
    if !reference_file.is_null() {
        if !mode.is_null() {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"cannot combine mode and --reference options\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            usage(1 as libc::c_int);
        }
    } else if mode.is_null() {
        let fresh0 = optind;
        optind = optind + 1;
        mode = *argv.offset(fresh0 as isize);
    }
    if optind >= argc {
        if mode.is_null() || mode != *argv.offset((optind - 1 as libc::c_int) as isize) {
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
    if !reference_file.is_null() {
        change = mode_create_from_ref(reference_file);
        if change.is_null() {
            if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"failed to get attributes of %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, reference_file),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"failed to get attributes of %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, reference_file),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    } else {
        change = mode_compile(mode);
        if change.is_null() {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(b"invalid mode: %s\0" as *const u8 as *const libc::c_char),
                quote(mode),
            );
            usage(1 as libc::c_int);
        }
        umask_value = umask(0 as libc::c_int as __mode_t);
    }
    if recurse as libc::c_int != 0 && preserve_root as libc::c_int != 0 {
        static mut dev_ino_buf: dev_ino = dev_ino { st_ino: 0, st_dev: 0 };
        root_dev_ino = get_root_dev_ino(&mut dev_ino_buf);
        if root_dev_ino.is_null() {
            if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"failed to get attributes of %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(
                        shell_escape_always_quoting_style,
                        b"/\0" as *const u8 as *const libc::c_char,
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
                        b"failed to get attributes of %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(
                        shell_escape_always_quoting_style,
                        b"/\0" as *const u8 as *const libc::c_char,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    } else {
        root_dev_ino = 0 as *mut dev_ino;
    }
    ok = process_files(
        argv.offset(optind as isize),
        0x1 as libc::c_int | 0x10 as libc::c_int | 0x400 as libc::c_int,
    );
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
