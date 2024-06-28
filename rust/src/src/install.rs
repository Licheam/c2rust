use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type selabel_handle;
    pub type hash_table;
    pub type mode_change;
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
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn lchown(
        __file: *const libc::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> libc::c_int;
    fn execlp(
        __file: *const libc::c_char,
        __arg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn getuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn fork() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn endpwent();
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn endgrent();
    fn getgrnam(__name: *const libc::c_char) -> *mut group;
    fn is_selinux_enabled() -> libc::c_int;
    fn freecon(con: *mut libc::c_char);
    fn setfscreatecon(context: *const libc::c_char) -> libc::c_int;
    fn rpl_getfilecon(
        path: *const libc::c_char,
        con: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn lsetfilecon(path: *const libc::c_char, con: *const libc::c_char) -> libc::c_int;
    fn selabel_lookup(
        handle: *mut selabel_handle,
        con: *mut *mut libc::c_char,
        key: *const libc::c_char,
        type_0: libc::c_int,
    ) -> libc::c_int;
    fn selabel_open(
        backend: libc::c_uint,
        opts: *const selinux_opt,
        nopts: libc::c_uint,
    ) -> *mut selabel_handle;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
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
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
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
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn close_stdin();
    fn version_etc(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        _: ...
    );
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn set_simple_backup_suffix(_: *const libc::c_char);
    fn xget_version(
        context: *const libc::c_char,
        arg: *const libc::c_char,
    ) -> backup_type;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn hash_init();
    fn copy(
        src_name: *const libc::c_char,
        dst_name: *const libc::c_char,
        nonexistent_dst: bool,
        options: *const cp_options,
        copy_into_self: *mut bool,
        rename_succeeded: *mut bool,
    ) -> bool;
    fn dest_info_init(_: *mut cp_options);
    fn cp_options_default(_: *mut cp_options);
    fn file_name_concat(
        dir: *const libc::c_char,
        base: *const libc::c_char,
        base_in_result: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn full_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn mkancesdirs(
        _: *mut libc::c_char,
        _: *mut savewd,
        _: Option::<
            unsafe extern "C" fn(
                *const libc::c_char,
                *const libc::c_char,
                *mut libc::c_void,
            ) -> libc::c_int,
        >,
        _: *mut libc::c_void,
    ) -> ptrdiff_t;
    fn make_dir_parents(
        dir: *mut libc::c_char,
        wd: *mut savewd,
        make_ancestor_0: Option::<
            unsafe extern "C" fn(
                *const libc::c_char,
                *const libc::c_char,
                *mut libc::c_void,
            ) -> libc::c_int,
        >,
        options: *mut libc::c_void,
        mode_0: mode_t,
        announce: Option::<
            unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> (),
        >,
        mode_bits: mode_t,
        owner: uid_t,
        group: gid_t,
        preserve_existing: bool,
    ) -> bool;
    fn mode_compile(_: *const libc::c_char) -> *mut mode_change;
    fn mode_adjust(
        _: mode_t,
        _: bool,
        _: mode_t,
        _: *const mode_change,
        _: *mut mode_t,
    ) -> mode_t;
    fn prog_fprintf(fp: *mut FILE, fmt: *const libc::c_char, _: ...);
    fn quote_n(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn savewd_restore(wd: *mut savewd, status: libc::c_int) -> libc::c_int;
    fn savewd_finish(wd: *mut savewd);
    fn savewd_process_files(
        n_files: libc::c_int,
        file: *mut *mut libc::c_char,
        act: Option::<
            unsafe extern "C" fn(
                *mut libc::c_char,
                *mut savewd,
                *mut libc::c_void,
            ) -> libc::c_int,
        >,
        options: *mut libc::c_void,
    ) -> libc::c_int;
    fn restorecon(
        selabel_handle: *mut selabel_handle,
        path: *const libc::c_char,
        recurse: bool,
    ) -> bool;
    fn defaultcon(
        selabel_handle: *mut selabel_handle,
        path: *const libc::c_char,
        mode_0: mode_t,
    ) -> libc::c_int;
    fn utimens(_: *const libc::c_char, _: *const timespec) -> libc::c_int;
    fn xstrtoumax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut uintmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
}
pub type size_t = libc::c_ulong;
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
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
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
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
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct selinux_opt {
    pub type_0: libc::c_int,
    pub value: *const libc::c_char,
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
pub type backup_type = libc::c_uint;
pub const numbered_backups: backup_type = 3;
pub const numbered_existing_backups: backup_type = 2;
pub const simple_backups: backup_type = 1;
pub const no_backups: backup_type = 0;
pub type Hash_table = hash_table;
pub type Sparse_type = libc::c_uint;
pub const SPARSE_ALWAYS: Sparse_type = 3;
pub const SPARSE_AUTO: Sparse_type = 2;
pub const SPARSE_NEVER: Sparse_type = 1;
pub const SPARSE_UNUSED: Sparse_type = 0;
pub type Reflink_type = libc::c_uint;
pub const REFLINK_ALWAYS: Reflink_type = 2;
pub const REFLINK_AUTO: Reflink_type = 1;
pub const REFLINK_NEVER: Reflink_type = 0;
pub type Interactive = libc::c_uint;
pub const I_UNSPECIFIED: Interactive = 4;
pub const I_ASK_USER: Interactive = 3;
pub const I_ALWAYS_NO: Interactive = 2;
pub const I_ALWAYS_YES: Interactive = 1;
pub type Dereference_symlink = libc::c_uint;
pub const DEREF_ALWAYS: Dereference_symlink = 4;
pub const DEREF_COMMAND_LINE_ARGUMENTS: Dereference_symlink = 3;
pub const DEREF_NEVER: Dereference_symlink = 2;
pub const DEREF_UNDEFINED: Dereference_symlink = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cp_options {
    pub backup_type: backup_type,
    pub dereference: Dereference_symlink,
    pub interactive: Interactive,
    pub sparse_mode: Sparse_type,
    pub mode: mode_t,
    pub copy_as_regular: bool,
    pub unlink_dest_before_opening: bool,
    pub unlink_dest_after_failed_open: bool,
    pub hard_link: bool,
    pub move_mode: bool,
    pub install_mode: bool,
    pub chown_privileges: bool,
    pub owner_privileges: bool,
    pub one_file_system: bool,
    pub preserve_ownership: bool,
    pub preserve_mode: bool,
    pub preserve_timestamps: bool,
    pub explicit_no_preserve_mode: bool,
    pub set_security_context: *mut selabel_handle,
    pub preserve_links: bool,
    pub data_copy_required: bool,
    pub require_preserve: bool,
    pub preserve_security_context: bool,
    pub require_preserve_context: bool,
    pub preserve_xattr: bool,
    pub require_preserve_xattr: bool,
    pub reduce_diagnostics: bool,
    pub recursive: bool,
    pub set_mode: bool,
    pub symbolic_link: bool,
    pub update: bool,
    pub verbose: bool,
    pub stdin_tty: bool,
    pub open_dangling_dest_symlink: bool,
    pub last_file: bool,
    pub rename_errno: libc::c_int,
    pub reflink_mode: Reflink_type,
    pub dest_info: *mut Hash_table,
    pub src_info: *mut Hash_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct savewd {
    pub state: C2RustUnnamed_1,
    pub val: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub fd: libc::c_int,
    pub errnum: libc::c_int,
    pub child: pid_t,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const FINAL_STATE: C2RustUnnamed_1 = 5;
pub const ERROR_STATE: C2RustUnnamed_1 = 4;
pub const FORKING_STATE: C2RustUnnamed_1 = 3;
pub const FD_POST_CHDIR_STATE: C2RustUnnamed_1 = 2;
pub const FD_STATE: C2RustUnnamed_1 = 1;
pub const INITIAL_STATE: C2RustUnnamed_1 = 0;
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const STRIP_PROGRAM_OPTION: C2RustUnnamed_2 = 129;
pub const PRESERVE_CONTEXT_OPTION: C2RustUnnamed_2 = 128;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const CMP_BLOCK_SIZE: C2RustUnnamed_3 = 4096;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub _gl_dummy: libc::c_int,
}
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
unsafe extern "C" fn emit_backup_suffix_note() {
    fputs_unlocked(
        gettext(
            b"\nThe backup suffix is '~', unless set with --suffix or SIMPLE_BACKUP_SUFFIX.\nThe version control method may be selected via the --backup option or through\nthe VERSION_CONTROL environment variable.  Here are the values:\n\n\0"
                as *const u8 as *const libc::c_char,
        ),
        stdout,
    );
    fputs_unlocked(
        gettext(
            b"  none, off       never make backups (even if --backup is given)\n  numbered, t     make numbered backups\n  existing, nil   numbered if numbered backups exist, simple otherwise\n  simple, never   always make simple backups\n\0"
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
                b"Usage: %s [OPTION]... [-T] SOURCE DEST\n  or:  %s [OPTION]... SOURCE... DIRECTORY\n  or:  %s [OPTION]... -t DIRECTORY SOURCE...\n  or:  %s [OPTION]... -d DIRECTORY...\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            program_name,
            program_name,
            program_name,
            program_name,
        );
        fputs_unlocked(
            gettext(
                b"\nThis install program copies files (often just compiled) into destination\nlocations you choose.  If you want to download and install a ready-to-use\npackage on a GNU/Linux system, you should instead be using a package manager\nlike yum(1) or apt-get(1).\n\nIn the first three forms, copy SOURCE to DEST or multiple SOURCE(s) to\nthe existing DIRECTORY, while setting permission modes and owner/group.\nIn the 4th form, create all components of the given DIRECTORY(ies).\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            gettext(
                b"      --backup[=CONTROL]  make a backup of each existing destination file\n  -b                  like --backup but does not accept an argument\n  -c                  (ignored)\n  -C, --compare       compare each pair of source and destination files, and\n                        in some cases, do not modify the destination at all\n  -d, --directory     treat all arguments as directory names; create all\n                        components of the specified directories\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -D                  create all leading components of DEST except the last,\n                        or all components of --target-directory,\n                        then copy SOURCE to DEST\n  -g, --group=GROUP   set group ownership, instead of process' current group\n  -m, --mode=MODE     set permission mode (as in chmod), instead of rwxr-xr-x\n  -o, --owner=OWNER   set ownership (super-user only)\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -p, --preserve-timestamps   apply access/modification times of SOURCE files\n                        to corresponding destination files\n  -s, --strip         strip symbol tables\n      --strip-program=PROGRAM  program used to strip binaries\n  -S, --suffix=SUFFIX  override the usual backup suffix\n  -t, --target-directory=DIRECTORY  copy all SOURCE arguments into DIRECTORY\n  -T, --no-target-directory  treat DEST as a normal file\n  -v, --verbose       print the name of each directory as it is created\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"      --preserve-context  preserve SELinux security context\n  -Z                      set SELinux security context of destination\n                            file and each created directory to default type\n      --context[=CTX]     like -Z, or if CTX is specified then set the\n                            SELinux or SMACK security context to CTX\n\0"
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
        emit_backup_suffix_note();
        emit_ancillary_info(b"install\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
#[inline]
unsafe extern "C" fn savewd_init(mut wd: *mut savewd) {
    (*wd).state = INITIAL_STATE;
}
#[inline]
unsafe extern "C" fn ignorable_ctx_err(mut err: libc::c_int) -> bool {
    return err == 95 as libc::c_int || err == 61 as libc::c_int;
}
#[inline]
unsafe extern "C" fn get_stat_atime(mut st: *const stat) -> timespec {
    return (*st).st_atim;
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
static mut selinux_enabled: libc::c_int = 0 as libc::c_int;
static mut use_default_selinux_context: bool = 1 as libc::c_int != 0;
static mut owner_name: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut owner_id: uid_t = 0;
static mut group_name: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut group_id: gid_t = 0;
static mut mode: mode_t = (0o400 as libc::c_int | 0o200 as libc::c_int
    | 0o100 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
    | 0o100 as libc::c_int >> 3 as libc::c_int
    | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
    | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as mode_t;
static mut dir_mode: mode_t = (0o400 as libc::c_int | 0o200 as libc::c_int
    | 0o100 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
    | 0o100 as libc::c_int >> 3 as libc::c_int
    | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
    | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as mode_t;
static mut dir_mode_bits: mode_t = (0o4000 as libc::c_int | 0o2000 as libc::c_int
    | 0o1000 as libc::c_int
    | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
    | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
        >> 3 as libc::c_int
    | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
        >> 3 as libc::c_int >> 3 as libc::c_int) as mode_t;
static mut copy_only_if_needed: bool = false;
static mut strip_files: bool = false;
static mut dir_arg: bool = false;
static mut strip_program: *const libc::c_char = b"strip\0" as *const u8
    as *const libc::c_char;
static mut long_options: [option; 18] = [
    {
        let mut init = option {
            name: b"backup\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"compare\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'C' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"context\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'Z' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"directory\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"group\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'g' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"mode\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'm' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-target-directory\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'T' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"owner\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"preserve-timestamps\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"preserve-context\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: PRESERVE_CONTEXT_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"strip\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"strip-program\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: STRIP_PROGRAM_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"suffix\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'S' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"target-directory\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
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
unsafe extern "C" fn have_same_content(
    mut a_fd: libc::c_int,
    mut b_fd: libc::c_int,
) -> bool {
    static mut a_buff: [libc::c_char; 4096] = [0; 4096];
    static mut b_buff: [libc::c_char; 4096] = [0; 4096];
    let mut size: size_t = 0;
    loop {
        size = full_read(
            a_fd,
            a_buff.as_mut_ptr() as *mut libc::c_void,
            ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        );
        if !((0 as libc::c_int as libc::c_ulong) < size) {
            break;
        }
        if size
            != full_read(
                b_fd,
                b_buff.as_mut_ptr() as *mut libc::c_void,
                ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
            )
        {
            return 0 as libc::c_int != 0;
        }
        if memcmp(
            a_buff.as_mut_ptr() as *const libc::c_void,
            b_buff.as_mut_ptr() as *const libc::c_void,
            size,
        ) != 0 as libc::c_int
        {
            return 0 as libc::c_int != 0;
        }
    }
    return size == 0 as libc::c_int as libc::c_ulong;
}
unsafe extern "C" fn extra_mode(mut input: mode_t) -> bool {
    let mut mask: mode_t = (0o400 as libc::c_int | 0o200 as libc::c_int
        | 0o100 as libc::c_int
        | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
            >> 3 as libc::c_int
        | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
            >> 3 as libc::c_int >> 3 as libc::c_int | 0o170000 as libc::c_int) as mode_t;
    return input & !mask != 0;
}
unsafe extern "C" fn need_copy(
    mut src_name: *const libc::c_char,
    mut dest_name: *const libc::c_char,
    mut x: *const cp_options,
) -> bool {
    let mut src_sb: stat = stat {
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
    let mut dest_sb: stat = stat {
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
    let mut src_fd: libc::c_int = 0;
    let mut dest_fd: libc::c_int = 0;
    let mut content_match: bool = false;
    if extra_mode(mode) {
        return 1 as libc::c_int != 0;
    }
    if lstat(src_name, &mut src_sb) != 0 as libc::c_int {
        return 1 as libc::c_int != 0;
    }
    if lstat(dest_name, &mut dest_sb) != 0 as libc::c_int {
        return 1 as libc::c_int != 0;
    }
    if !(src_sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint)
        || !(dest_sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint)
        || extra_mode(src_sb.st_mode) as libc::c_int != 0
        || extra_mode(dest_sb.st_mode) as libc::c_int != 0
    {
        return 1 as libc::c_int != 0;
    }
    if src_sb.st_size != dest_sb.st_size
        || dest_sb.st_mode
            & (0o4000 as libc::c_int | 0o2000 as libc::c_int | 0o1000 as libc::c_int
                | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                    >> 3 as libc::c_int
                | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                    >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_uint != mode
    {
        return 1 as libc::c_int != 0;
    }
    if owner_id == -(1 as libc::c_int) as uid_t {
        *__errno_location() = 0 as libc::c_int;
        let mut ruid: uid_t = getuid();
        if ruid == -(1 as libc::c_int) as uid_t && *__errno_location() != 0
            || dest_sb.st_uid != ruid
        {
            return 1 as libc::c_int != 0;
        }
    } else if dest_sb.st_uid != owner_id {
        return 1 as libc::c_int != 0
    }
    if group_id == -(1 as libc::c_int) as uid_t {
        *__errno_location() = 0 as libc::c_int;
        let mut rgid: gid_t = getgid();
        if rgid == -(1 as libc::c_int) as uid_t && *__errno_location() != 0
            || dest_sb.st_gid != rgid
        {
            return 1 as libc::c_int != 0;
        }
    } else if dest_sb.st_gid != group_id {
        return 1 as libc::c_int != 0
    }
    if selinux_enabled != 0 && (*x).preserve_security_context as libc::c_int != 0 {
        let mut file_scontext: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut to_scontext: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut scontext_match: bool = false;
        if rpl_getfilecon(src_name, &mut file_scontext) == -(1 as libc::c_int) {
            return 1 as libc::c_int != 0;
        }
        if rpl_getfilecon(dest_name, &mut to_scontext) == -(1 as libc::c_int) {
            freecon(file_scontext);
            return 1 as libc::c_int != 0;
        }
        scontext_match = strcmp(file_scontext, to_scontext) == 0 as libc::c_int;
        freecon(file_scontext);
        freecon(to_scontext);
        if !scontext_match {
            return 1 as libc::c_int != 0;
        }
    }
    src_fd = open(src_name, 0 as libc::c_int | 0 as libc::c_int);
    if src_fd < 0 as libc::c_int {
        return 1 as libc::c_int != 0;
    }
    dest_fd = open(dest_name, 0 as libc::c_int | 0 as libc::c_int);
    if dest_fd < 0 as libc::c_int {
        close(src_fd);
        return 1 as libc::c_int != 0;
    }
    content_match = have_same_content(src_fd, dest_fd);
    close(src_fd);
    close(dest_fd);
    return !content_match;
}
unsafe extern "C" fn cp_option_init(mut x: *mut cp_options) {
    cp_options_default(x);
    (*x).copy_as_regular = 1 as libc::c_int != 0;
    (*x).reflink_mode = REFLINK_AUTO;
    (*x).dereference = DEREF_ALWAYS;
    (*x).unlink_dest_before_opening = 1 as libc::c_int != 0;
    (*x).unlink_dest_after_failed_open = 0 as libc::c_int != 0;
    (*x).hard_link = 0 as libc::c_int != 0;
    (*x).interactive = I_UNSPECIFIED;
    (*x).move_mode = 0 as libc::c_int != 0;
    (*x).install_mode = 1 as libc::c_int != 0;
    (*x).one_file_system = 0 as libc::c_int != 0;
    (*x).preserve_ownership = 0 as libc::c_int != 0;
    (*x).preserve_links = 0 as libc::c_int != 0;
    (*x).preserve_mode = 0 as libc::c_int != 0;
    (*x).preserve_timestamps = 0 as libc::c_int != 0;
    (*x).explicit_no_preserve_mode = 0 as libc::c_int != 0;
    (*x).reduce_diagnostics = 0 as libc::c_int != 0;
    (*x).data_copy_required = 1 as libc::c_int != 0;
    (*x).require_preserve = 0 as libc::c_int != 0;
    (*x).require_preserve_xattr = 0 as libc::c_int != 0;
    (*x).recursive = 0 as libc::c_int != 0;
    (*x).sparse_mode = SPARSE_AUTO;
    (*x).symbolic_link = 0 as libc::c_int != 0;
    (*x).backup_type = no_backups;
    (*x).set_mode = 1 as libc::c_int != 0;
    (*x).mode = (0o400 as libc::c_int | 0o200 as libc::c_int) as mode_t;
    (*x).stdin_tty = 0 as libc::c_int != 0;
    (*x).open_dangling_dest_symlink = 0 as libc::c_int != 0;
    (*x).update = 0 as libc::c_int != 0;
    (*x).require_preserve_context = 0 as libc::c_int != 0;
    (*x).preserve_security_context = 0 as libc::c_int != 0;
    (*x).set_security_context = 0 as *mut selabel_handle;
    (*x).preserve_xattr = 0 as libc::c_int != 0;
    (*x).verbose = 0 as libc::c_int != 0;
    (*x).dest_info = 0 as *mut Hash_table;
    (*x).src_info = 0 as *mut Hash_table;
}
unsafe extern "C" fn get_labeling_handle() -> *mut selabel_handle {
    static mut initialized: bool = false;
    static mut hnd: *mut selabel_handle = 0 as *const selabel_handle
        as *mut selabel_handle;
    if !initialized {
        initialized = 1 as libc::c_int != 0;
        hnd = selabel_open(
            0 as libc::c_int as libc::c_uint,
            0 as *const selinux_opt,
            0 as libc::c_int as libc::c_uint,
        );
        if hnd.is_null() {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(
                    b"warning: security labeling handle failed\0" as *const u8
                        as *const libc::c_char,
                ),
            );
        }
    }
    return hnd;
}
unsafe extern "C" fn setdefaultfilecon(mut file: *const libc::c_char) {
    let mut st: stat = stat {
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
    let mut scontext: *mut libc::c_char = 0 as *mut libc::c_char;
    if selinux_enabled != 1 as libc::c_int {
        return;
    }
    if lstat(file, &mut st) != 0 as libc::c_int {
        return;
    }
    let mut hnd: *mut selabel_handle = get_labeling_handle();
    if hnd.is_null() {
        return;
    }
    if selabel_lookup(hnd, &mut scontext, file, st.st_mode as libc::c_int)
        != 0 as libc::c_int
    {
        if *__errno_location() != 2 as libc::c_int
            && !ignorable_ctx_err(*__errno_location())
        {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(
                    b"warning: %s: context lookup failed\0" as *const u8
                        as *const libc::c_char,
                ),
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    file,
                ),
            );
        }
        return;
    }
    if lsetfilecon(file, scontext) < 0 as libc::c_int
        && *__errno_location() != 95 as libc::c_int
    {
        error(
            0 as libc::c_int,
            *__errno_location(),
            gettext(
                b"warning: %s: failed to change context to %s\0" as *const u8
                    as *const libc::c_char,
            ),
            quotearg_n_style_colon(0 as libc::c_int, shell_escape_quoting_style, file),
            quote_n(1 as libc::c_int, scontext),
        );
    }
    freecon(scontext);
}
unsafe extern "C" fn target_directory_operand(mut file: *const libc::c_char) -> bool {
    let mut b: *const libc::c_char = last_component(file);
    let mut blen: size_t = strlen(b);
    let mut looks_like_a_dir: bool = blen == 0 as libc::c_int as libc::c_ulong
        || *b.offset(blen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '/' as i32;
    let mut st: stat = stat {
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
    let mut err: libc::c_int = if stat(file, &mut st) == 0 as libc::c_int {
        0 as libc::c_int
    } else {
        *__errno_location()
    };
    let mut is_a_dir: bool = err == 0
        && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint;
    if err != 0 && err != 2 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                err,
                gettext(b"failed to access %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, file),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                err,
                gettext(b"failed to access %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, file),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if (is_a_dir as libc::c_int) < looks_like_a_dir as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                err,
                gettext(
                    b"target %s is not a directory\0" as *const u8 as *const libc::c_char,
                ),
                quotearg_style(shell_escape_always_quoting_style, file),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                err,
                gettext(
                    b"target %s is not a directory\0" as *const u8 as *const libc::c_char,
                ),
                quotearg_style(shell_escape_always_quoting_style, file),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    return is_a_dir;
}
unsafe extern "C" fn announce_mkdir(
    mut dir: *const libc::c_char,
    mut options: *mut libc::c_void,
) {
    let mut x: *const cp_options = options as *const cp_options;
    if (*x).verbose {
        prog_fprintf(
            stdout,
            gettext(b"creating directory %s\0" as *const u8 as *const libc::c_char),
            quotearg_style(shell_escape_always_quoting_style, dir),
        );
    }
}
unsafe extern "C" fn make_ancestor(
    mut dir: *const libc::c_char,
    mut component: *const libc::c_char,
    mut options: *mut libc::c_void,
) -> libc::c_int {
    let mut x: *const cp_options = options as *const cp_options;
    if !((*x).set_security_context).is_null()
        && defaultcon(
            (*x).set_security_context,
            component,
            0o40000 as libc::c_int as mode_t,
        ) < 0 as libc::c_int && !ignorable_ctx_err(*__errno_location())
    {
        error(
            0 as libc::c_int,
            *__errno_location(),
            gettext(
                b"failed to set default creation context for %s\0" as *const u8
                    as *const libc::c_char,
            ),
            quotearg_style(shell_escape_always_quoting_style, dir),
        );
    }
    let mut r: libc::c_int = mkdir(
        component,
        (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int
            | 0o100 as libc::c_int >> 3 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as __mode_t,
    );
    if r == 0 as libc::c_int {
        announce_mkdir(dir, options);
    }
    return r;
}
unsafe extern "C" fn process_dir(
    mut dir: *mut libc::c_char,
    mut wd: *mut savewd,
    mut options: *mut libc::c_void,
) -> libc::c_int {
    let mut x: *const cp_options = options as *const cp_options;
    let mut ret: libc::c_int = if make_dir_parents(
        dir,
        wd,
        Some(
            make_ancestor
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        options,
        dir_mode,
        Some(
            announce_mkdir
                as unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> (),
        ),
        dir_mode_bits,
        owner_id,
        group_id,
        0 as libc::c_int != 0,
    ) as libc::c_int != 0
    {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    if ret == 0 as libc::c_int && !((*x).set_security_context).is_null() {
        if !restorecon(
            (*x).set_security_context,
            last_component(dir),
            0 as libc::c_int != 0,
        ) && !ignorable_ctx_err(*__errno_location())
        {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(
                    b"failed to restore context for %s\0" as *const u8
                        as *const libc::c_char,
                ),
                quotearg_style(shell_escape_always_quoting_style, dir),
            );
        }
    }
    return ret;
}
unsafe extern "C" fn copy_file(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
    mut x: *const cp_options,
) -> bool {
    let mut copy_into_self: bool = false;
    if copy_only_if_needed as libc::c_int != 0 && !need_copy(from, to, x) {
        return 1 as libc::c_int != 0;
    }
    return copy(from, to, 0 as libc::c_int != 0, x, &mut copy_into_self, 0 as *mut bool);
}
unsafe extern "C" fn change_attributes(mut name: *const libc::c_char) -> bool {
    let mut ok: bool = 0 as libc::c_int != 0;
    if !(owner_id == -(1 as libc::c_int) as uid_t
        && group_id == -(1 as libc::c_int) as gid_t)
        && lchown(name, owner_id, group_id) != 0 as libc::c_int
    {
        error(
            0 as libc::c_int,
            *__errno_location(),
            gettext(
                b"cannot change ownership of %s\0" as *const u8 as *const libc::c_char,
            ),
            quotearg_style(shell_escape_always_quoting_style, name),
        );
    } else if chmod(name, mode) != 0 as libc::c_int {
        error(
            0 as libc::c_int,
            *__errno_location(),
            gettext(
                b"cannot change permissions of %s\0" as *const u8 as *const libc::c_char,
            ),
            quotearg_style(shell_escape_always_quoting_style, name),
        );
    } else {
        ok = 1 as libc::c_int != 0;
    }
    if use_default_selinux_context {
        setdefaultfilecon(name);
    }
    return ok;
}
unsafe extern "C" fn change_timestamps(
    mut src_sb: *const stat,
    mut dest: *const libc::c_char,
) -> bool {
    let mut timespec: [timespec; 2] = [timespec { tv_sec: 0, tv_nsec: 0 }; 2];
    timespec[0 as libc::c_int as usize] = get_stat_atime(src_sb);
    timespec[1 as libc::c_int as usize] = get_stat_mtime(src_sb);
    if utimens(dest, timespec.as_mut_ptr() as *const timespec) != 0 {
        error(
            0 as libc::c_int,
            *__errno_location(),
            gettext(
                b"cannot set timestamps for %s\0" as *const u8 as *const libc::c_char,
            ),
            quotearg_style(shell_escape_always_quoting_style, dest),
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn strip(mut name: *const libc::c_char) -> bool {
    let mut status: libc::c_int = 0;
    let mut ok: bool = 0 as libc::c_int != 0;
    let mut pid: pid_t = fork();
    let mut current_block_7: u64;
    match pid {
        -1 => {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(b"fork system call failed\0" as *const u8 as *const libc::c_char),
            );
            current_block_7 = 3640593987805443782;
        }
        0 => {
            execlp(strip_program, strip_program, name, 0 as *mut libc::c_void);
            if ::core::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(b"cannot run %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, strip_program),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(b"cannot run %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, strip_program),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
            current_block_7 = 17144959373768720448;
        }
        _ => {
            current_block_7 = 17144959373768720448;
        }
    }
    match current_block_7 {
        17144959373768720448 => {
            if waitpid(pid, &mut status, 0 as libc::c_int) < 0 as libc::c_int {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(b"waiting for strip\0" as *const u8 as *const libc::c_char),
                );
            } else if !(status & 0x7f as libc::c_int == 0 as libc::c_int)
                || (status & 0xff00 as libc::c_int) >> 8 as libc::c_int != 0
            {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"strip process terminated abnormally\0" as *const u8
                            as *const libc::c_char,
                    ),
                );
            } else {
                ok = 1 as libc::c_int != 0;
            }
        }
        _ => {}
    }
    return ok;
}
unsafe extern "C" fn get_ids() {
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut gr: *mut group = 0 as *mut group;
    if !owner_name.is_null() {
        pw = getpwnam(owner_name);
        if pw.is_null() {
            let mut tmp: uintmax_t = 0;
            if xstrtoumax(
                owner_name,
                0 as *mut *mut libc::c_char,
                0 as libc::c_int,
                &mut tmp,
                b"\0" as *const u8 as *const libc::c_char,
            ) as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint
                || ((if (0 as libc::c_int as uid_t) < -(1 as libc::c_int) as uid_t {
                    -(1 as libc::c_int) as uid_t
                } else {
                    ((1 as libc::c_int as uid_t)
                        << (::core::mem::size_of::<uid_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        .wrapping_mul(2 as libc::c_int as libc::c_uint)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                }) as libc::c_ulong) < tmp
            {
                if ::core::mem::size_of::<C2RustUnnamed_8>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"invalid user %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quote(owner_name),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"invalid user %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quote(owner_name),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            owner_id = tmp as uid_t;
        } else {
            owner_id = (*pw).pw_uid;
        }
        endpwent();
    } else {
        owner_id = -(1 as libc::c_int) as uid_t;
    }
    if !group_name.is_null() {
        gr = getgrnam(group_name);
        if gr.is_null() {
            let mut tmp_0: uintmax_t = 0;
            if xstrtoumax(
                group_name,
                0 as *mut *mut libc::c_char,
                0 as libc::c_int,
                &mut tmp_0,
                b"\0" as *const u8 as *const libc::c_char,
            ) as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint
                || ((if (0 as libc::c_int as gid_t) < -(1 as libc::c_int) as gid_t {
                    -(1 as libc::c_int) as gid_t
                } else {
                    ((1 as libc::c_int as gid_t)
                        << (::core::mem::size_of::<gid_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        .wrapping_mul(2 as libc::c_int as libc::c_uint)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                }) as libc::c_ulong) < tmp_0
            {
                if ::core::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"invalid group %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quote(group_name),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"invalid group %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quote(group_name),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            group_id = tmp_0 as gid_t;
        } else {
            group_id = (*gr).gr_gid;
        }
        endgrent();
    } else {
        group_id = -(1 as libc::c_int) as gid_t;
    };
}
unsafe extern "C" fn install_file_in_file(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
    mut x: *const cp_options,
) -> bool {
    let mut from_sb: stat = stat {
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
    if (*x).preserve_timestamps as libc::c_int != 0
        && stat(from, &mut from_sb) != 0 as libc::c_int
    {
        error(
            0 as libc::c_int,
            *__errno_location(),
            gettext(b"cannot stat %s\0" as *const u8 as *const libc::c_char),
            quotearg_style(shell_escape_always_quoting_style, from),
        );
        return 0 as libc::c_int != 0;
    }
    if !copy_file(from, to, x) {
        return 0 as libc::c_int != 0;
    }
    if strip_files {
        if !strip(to) {
            if unlink(to) != 0 as libc::c_int {
                if ::core::mem::size_of::<C2RustUnnamed_9>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        gettext(
                            b"cannot unlink %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, to),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        gettext(
                            b"cannot unlink %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, to),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            return 0 as libc::c_int != 0;
        }
    }
    if (*x).preserve_timestamps as libc::c_int != 0
        && (strip_files as libc::c_int != 0
            || !(from_sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint))
        && !change_timestamps(&mut from_sb, to)
    {
        return 0 as libc::c_int != 0;
    }
    return change_attributes(to);
}
unsafe extern "C" fn mkancesdirs_safe_wd(
    mut from: *const libc::c_char,
    mut to: *mut libc::c_char,
    mut x: *mut cp_options,
    mut save_always: bool,
) -> bool {
    let mut save_working_directory: bool = save_always as libc::c_int != 0
        || !(*from.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
            && *to.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32);
    let mut status: libc::c_int = 0 as libc::c_int;
    let mut wd: savewd = savewd {
        state: INITIAL_STATE,
        val: C2RustUnnamed_0 { fd: 0 },
    };
    savewd_init(&mut wd);
    if !save_working_directory {
        savewd_finish(&mut wd);
    }
    if mkancesdirs(
        to,
        &mut wd,
        Some(
            make_ancestor
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        x as *mut libc::c_void,
    ) == -(1 as libc::c_int) as libc::c_long
    {
        error(
            0 as libc::c_int,
            *__errno_location(),
            gettext(b"cannot create directory %s\0" as *const u8 as *const libc::c_char),
            quotearg_style(shell_escape_always_quoting_style, to),
        );
        status = 1 as libc::c_int;
    }
    if save_working_directory {
        let mut restore_result: libc::c_int = savewd_restore(&mut wd, status);
        let mut restore_errno: libc::c_int = *__errno_location();
        savewd_finish(&mut wd);
        if (0 as libc::c_int) < restore_result {
            return 0 as libc::c_int != 0;
        }
        if restore_result < 0 as libc::c_int && status == 0 as libc::c_int {
            error(
                0 as libc::c_int,
                restore_errno,
                gettext(
                    b"cannot create directory %s\0" as *const u8 as *const libc::c_char,
                ),
                quotearg_style(shell_escape_always_quoting_style, to),
            );
            return 0 as libc::c_int != 0;
        }
    }
    return status == 0 as libc::c_int;
}
unsafe extern "C" fn install_file_in_file_parents(
    mut from: *const libc::c_char,
    mut to: *mut libc::c_char,
    mut x: *const cp_options,
) -> bool {
    return mkancesdirs_safe_wd(from, to, x as *mut cp_options, 0 as libc::c_int != 0)
        as libc::c_int != 0 && install_file_in_file(from, to, x) as libc::c_int != 0;
}
unsafe extern "C" fn install_file_in_dir(
    mut from: *const libc::c_char,
    mut to_dir: *const libc::c_char,
    mut x: *const cp_options,
    mut mkdir_and_install: bool,
) -> bool {
    let mut from_base: *const libc::c_char = last_component(from);
    let mut to: *mut libc::c_char = file_name_concat(
        to_dir,
        from_base,
        0 as *mut *mut libc::c_char,
    );
    let mut ret: bool = 1 as libc::c_int != 0;
    if mkdir_and_install {
        ret = mkancesdirs_safe_wd(from, to, x as *mut cp_options, 1 as libc::c_int != 0);
    }
    ret = ret as libc::c_int != 0
        && install_file_in_file(from, to, x) as libc::c_int != 0;
    free(to as *mut libc::c_void);
    return ret;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut optc: libc::c_int = 0;
    let mut exit_status: libc::c_int = 0 as libc::c_int;
    let mut specified_mode: *const libc::c_char = 0 as *const libc::c_char;
    let mut make_backups: bool = 0 as libc::c_int != 0;
    let mut backup_suffix: *const libc::c_char = 0 as *const libc::c_char;
    let mut version_control_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mkdir_and_install: bool = 0 as libc::c_int != 0;
    let mut x: cp_options = cp_options {
        backup_type: no_backups,
        dereference: 0 as Dereference_symlink,
        interactive: 0 as Interactive,
        sparse_mode: SPARSE_UNUSED,
        mode: 0,
        copy_as_regular: false,
        unlink_dest_before_opening: false,
        unlink_dest_after_failed_open: false,
        hard_link: false,
        move_mode: false,
        install_mode: false,
        chown_privileges: false,
        owner_privileges: false,
        one_file_system: false,
        preserve_ownership: false,
        preserve_mode: false,
        preserve_timestamps: false,
        explicit_no_preserve_mode: false,
        set_security_context: 0 as *mut selabel_handle,
        preserve_links: false,
        data_copy_required: false,
        require_preserve: false,
        preserve_security_context: false,
        require_preserve_context: false,
        preserve_xattr: false,
        require_preserve_xattr: false,
        reduce_diagnostics: false,
        recursive: false,
        set_mode: false,
        symbolic_link: false,
        update: false,
        verbose: false,
        stdin_tty: false,
        open_dangling_dest_symlink: false,
        last_file: false,
        rename_errno: 0,
        reflink_mode: REFLINK_NEVER,
        dest_info: 0 as *mut Hash_table,
        src_info: 0 as *mut Hash_table,
    };
    let mut target_directory: *const libc::c_char = 0 as *const libc::c_char;
    let mut no_target_directory: bool = 0 as libc::c_int != 0;
    let mut n_files: libc::c_int = 0;
    let mut file: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut strip_program_specified: bool = 0 as libc::c_int != 0;
    let mut scontext: *const libc::c_char = 0 as *const libc::c_char;
    selinux_enabled = ((0 as libc::c_int) < is_selinux_enabled()) as libc::c_int;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdin as unsafe extern "C" fn() -> ()));
    cp_option_init(&mut x);
    owner_name = 0 as *mut libc::c_char;
    group_name = 0 as *mut libc::c_char;
    strip_files = 0 as libc::c_int != 0;
    dir_arg = 0 as libc::c_int != 0;
    umask(0 as libc::c_int as __mode_t);
    loop {
        optc = getopt_long(
            argc,
            argv,
            b"bcCsDdg:m:o:pt:TvS:Z\0" as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        match optc {
            98 => {
                make_backups = 1 as libc::c_int != 0;
                if !optarg.is_null() {
                    version_control_string = optarg;
                }
            }
            99 => {}
            67 => {
                copy_only_if_needed = 1 as libc::c_int != 0;
            }
            115 => {
                strip_files = 1 as libc::c_int != 0;
                signal(17 as libc::c_int, None);
            }
            129 => {
                strip_program = xstrdup(optarg);
                strip_program_specified = 1 as libc::c_int != 0;
            }
            100 => {
                dir_arg = 1 as libc::c_int != 0;
            }
            68 => {
                mkdir_and_install = 1 as libc::c_int != 0;
            }
            118 => {
                x.verbose = 1 as libc::c_int != 0;
            }
            103 => {
                group_name = optarg;
            }
            109 => {
                specified_mode = optarg;
            }
            111 => {
                owner_name = optarg;
            }
            112 => {
                x.preserve_timestamps = 1 as libc::c_int != 0;
            }
            83 => {
                make_backups = 1 as libc::c_int != 0;
                backup_suffix = optarg;
            }
            116 => {
                if !target_directory.is_null() {
                    if ::core::mem::size_of::<C2RustUnnamed_19>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"multiple target directories specified\0" as *const u8
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
                                b"multiple target directories specified\0" as *const u8
                                    as *const libc::c_char,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                target_directory = optarg;
            }
            84 => {
                no_target_directory = 1 as libc::c_int != 0;
            }
            128 => {
                if selinux_enabled == 0 {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"WARNING: ignoring --preserve-context; this kernel is not SELinux-enabled\0"
                                as *const u8 as *const libc::c_char,
                        ),
                    );
                } else {
                    x.preserve_security_context = 1 as libc::c_int != 0;
                    use_default_selinux_context = 0 as libc::c_int != 0;
                }
            }
            90 => {
                if selinux_enabled != 0 {
                    use_default_selinux_context = 0 as libc::c_int != 0;
                    if !optarg.is_null() {
                        scontext = optarg;
                    } else {
                        x.set_security_context = get_labeling_handle();
                    }
                } else if !optarg.is_null() {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"warning: ignoring --context; it requires an SELinux-enabled kernel\0"
                                as *const u8 as *const libc::c_char,
                        ),
                    );
                }
            }
            -130 => {
                usage(0 as libc::c_int);
            }
            -131 => {
                version_etc(
                    stdout,
                    b"install\0" as *const u8 as *const libc::c_char,
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
    }
    if dir_arg as libc::c_int != 0 && strip_files as libc::c_int != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_18>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"the strip option may not be used when installing a directory\0"
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
                    b"the strip option may not be used when installing a directory\0"
                        as *const u8 as *const libc::c_char,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if dir_arg as libc::c_int != 0 && !target_directory.is_null() {
        if ::core::mem::size_of::<C2RustUnnamed_17>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"target directory not allowed when installing a directory\0"
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
                    b"target directory not allowed when installing a directory\0"
                        as *const u8 as *const libc::c_char,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if !target_directory.is_null() {
        let mut st: stat = stat {
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
        let mut stat_success: bool = if stat(target_directory, &mut st)
            == 0 as libc::c_int
        {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0;
        if !mkdir_and_install && !stat_success {
            if ::core::mem::size_of::<C2RustUnnamed_16>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"failed to access %s\0" as *const u8 as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, target_directory),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"failed to access %s\0" as *const u8 as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, target_directory),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if stat_success as libc::c_int != 0
            && !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint)
        {
            if ::core::mem::size_of::<C2RustUnnamed_15>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"target %s is not a directory\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, target_directory),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"target %s is not a directory\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, target_directory),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    x
        .backup_type = (if make_backups as libc::c_int != 0 {
        xget_version(
            gettext(b"backup type\0" as *const u8 as *const libc::c_char),
            version_control_string,
        ) as libc::c_uint
    } else {
        no_backups as libc::c_int as libc::c_uint
    }) as backup_type;
    set_simple_backup_suffix(backup_suffix);
    if x.preserve_security_context as libc::c_int != 0
        && (!(x.set_security_context).is_null() || !scontext.is_null())
    {
        if ::core::mem::size_of::<C2RustUnnamed_14>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"cannot set target context and preserve it\0" as *const u8
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
                    b"cannot set target context and preserve it\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if !scontext.is_null() && setfscreatecon(scontext) < 0 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                gettext(
                    b"failed to set default file creation context to %s\0" as *const u8
                        as *const libc::c_char,
                ),
                quote(scontext),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                *__errno_location(),
                gettext(
                    b"failed to set default file creation context to %s\0" as *const u8
                        as *const libc::c_char,
                ),
                quote(scontext),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    n_files = argc - optind;
    file = argv.offset(optind as isize);
    if n_files
        <= !(dir_arg as libc::c_int != 0 || !target_directory.is_null()) as libc::c_int
    {
        if n_files <= 0 as libc::c_int {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(b"missing file operand\0" as *const u8 as *const libc::c_char),
            );
        } else {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"missing destination file operand after %s\0" as *const u8
                        as *const libc::c_char,
                ),
                quotearg_style(
                    shell_escape_always_quoting_style,
                    *file.offset(0 as libc::c_int as isize),
                ),
            );
        }
        usage(1 as libc::c_int);
    }
    if no_target_directory {
        if !target_directory.is_null() {
            if ::core::mem::size_of::<C2RustUnnamed_12>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"cannot combine --target-directory (-t) and --no-target-directory (-T)\0"
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
                        b"cannot combine --target-directory (-t) and --no-target-directory (-T)\0"
                            as *const u8 as *const libc::c_char,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if (2 as libc::c_int) < n_files {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(b"extra operand %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(
                    shell_escape_always_quoting_style,
                    *file.offset(2 as libc::c_int as isize),
                ),
            );
            usage(1 as libc::c_int);
        }
    } else if !(dir_arg as libc::c_int != 0 || !target_directory.is_null()) {
        if 2 as libc::c_int <= n_files
            && target_directory_operand(
                *file.offset((n_files - 1 as libc::c_int) as isize),
            ) as libc::c_int != 0
        {
            n_files -= 1;
            target_directory = *file.offset(n_files as isize);
        } else if (2 as libc::c_int) < n_files {
            if ::core::mem::size_of::<C2RustUnnamed_11>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"target %s is not a directory\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(
                        shell_escape_always_quoting_style,
                        *file.offset((n_files - 1 as libc::c_int) as isize),
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
                        b"target %s is not a directory\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(
                        shell_escape_always_quoting_style,
                        *file.offset((n_files - 1 as libc::c_int) as isize),
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    if !specified_mode.is_null() {
        let mut change: *mut mode_change = mode_compile(specified_mode);
        if change.is_null() {
            if ::core::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(b"invalid mode %s\0" as *const u8 as *const libc::c_char),
                    quote(specified_mode),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(b"invalid mode %s\0" as *const u8 as *const libc::c_char),
                    quote(specified_mode),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        mode = mode_adjust(
            0 as libc::c_int as mode_t,
            0 as libc::c_int != 0,
            0 as libc::c_int as mode_t,
            change,
            0 as *mut mode_t,
        );
        dir_mode = mode_adjust(
            0 as libc::c_int as mode_t,
            1 as libc::c_int != 0,
            0 as libc::c_int as mode_t,
            change,
            &mut dir_mode_bits,
        );
        free(change as *mut libc::c_void);
    }
    if strip_program_specified as libc::c_int != 0 && !strip_files {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            gettext(
                b"WARNING: ignoring --strip-program option as -s option was not specified\0"
                    as *const u8 as *const libc::c_char,
            ),
        );
    }
    if copy_only_if_needed as libc::c_int != 0
        && x.preserve_timestamps as libc::c_int != 0
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            gettext(
                b"options --compare (-C) and --preserve-timestamps are mutually exclusive\0"
                    as *const u8 as *const libc::c_char,
            ),
        );
        usage(1 as libc::c_int);
    }
    if copy_only_if_needed as libc::c_int != 0 && strip_files as libc::c_int != 0 {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            gettext(
                b"options --compare (-C) and --strip are mutually exclusive\0"
                    as *const u8 as *const libc::c_char,
            ),
        );
        usage(1 as libc::c_int);
    }
    if copy_only_if_needed as libc::c_int != 0 && extra_mode(mode) as libc::c_int != 0 {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            gettext(
                b"the --compare (-C) option is ignored when you specify a mode with non-permission bits\0"
                    as *const u8 as *const libc::c_char,
            ),
        );
    }
    get_ids();
    if dir_arg {
        exit_status = savewd_process_files(
            n_files,
            file,
            Some(
                process_dir
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        *mut savewd,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            &mut x as *mut cp_options as *mut libc::c_void,
        );
    } else {
        hash_init();
        if target_directory.is_null() {
            if if mkdir_and_install as libc::c_int != 0 {
                install_file_in_file_parents(
                    *file.offset(0 as libc::c_int as isize),
                    *file.offset(1 as libc::c_int as isize),
                    &mut x,
                ) as libc::c_int
            } else {
                install_file_in_file(
                    *file.offset(0 as libc::c_int as isize),
                    *file.offset(1 as libc::c_int as isize),
                    &mut x,
                ) as libc::c_int
            } == 0
            {
                exit_status = 1 as libc::c_int;
            }
        } else {
            let mut i: libc::c_int = 0;
            dest_info_init(&mut x);
            i = 0 as libc::c_int;
            while i < n_files {
                if !install_file_in_dir(
                    *file.offset(i as isize),
                    target_directory,
                    &mut x,
                    i == 0 as libc::c_int && mkdir_and_install as libc::c_int != 0,
                ) {
                    exit_status = 1 as libc::c_int;
                }
                i += 1;
                i;
            }
        }
    }
    return exit_status;
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
