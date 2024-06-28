use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type selabel_handle;
    pub type hash_table;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn is_selinux_enabled() -> libc::c_int;
    fn selabel_open(
        backend: libc::c_uint,
        opts: *const selinux_opt,
        nopts: libc::c_uint,
    ) -> *mut selabel_handle;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
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
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn strip_trailing_slashes(file: *mut libc::c_char) -> bool;
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
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn set_simple_backup_suffix(_: *const libc::c_char);
    fn xget_version(
        context: *const libc::c_char,
        arg: *const libc::c_char,
    ) -> backup_type;
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
    fn hash_init();
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn file_name_concat(
        dir: *const libc::c_char,
        base: *const libc::c_char,
        base_in_result: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn rm(file: *const *mut libc::c_char, x: *const rm_options) -> RM_status;
    fn renameatu(
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_uint,
    ) -> libc::c_int;
    fn get_root_dev_ino(root_d_i: *mut dev_ino) -> *mut dev_ino;
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
pub struct dev_ino {
    pub st_ino: ino_t,
    pub st_dev: dev_t,
}
pub type rm_interactive = libc::c_uint;
pub const RMI_NEVER: rm_interactive = 5;
pub const RMI_SOMETIMES: rm_interactive = 4;
pub const RMI_ALWAYS: rm_interactive = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rm_options {
    pub ignore_missing_files: bool,
    pub interactive: rm_interactive,
    pub one_file_system: bool,
    pub recursive: bool,
    pub remove_empty_directories: bool,
    pub root_dev_ino: *mut dev_ino,
    pub preserve_all_root: bool,
    pub stdin_tty: bool,
    pub verbose: bool,
    pub require_restore_cwd: bool,
}
pub type RM_status = libc::c_uint;
pub const RM_NONEMPTY_DIR: RM_status = 5;
pub const RM_ERROR: RM_status = 4;
pub const RM_USER_DECLINED: RM_status = 3;
pub const RM_OK: RM_status = 2;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const STRIP_TRAILING_SLASHES_OPTION: C2RustUnnamed_0 = 128;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
                b"Usage: %s [OPTION]... [-T] SOURCE DEST\n  or:  %s [OPTION]... SOURCE... DIRECTORY\n  or:  %s [OPTION]... -t DIRECTORY SOURCE...\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            program_name,
            program_name,
            program_name,
        );
        fputs_unlocked(
            gettext(
                b"Rename SOURCE to DEST, or move SOURCE(s) to DIRECTORY.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            gettext(
                b"      --backup[=CONTROL]       make a backup of each existing destination file\n  -b                           like --backup but does not accept an argument\n  -f, --force                  do not prompt before overwriting\n  -i, --interactive            prompt before overwrite\n  -n, --no-clobber             do not overwrite an existing file\nIf you specify more than one of -i, -f, -n, only the final one takes effect.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"      --strip-trailing-slashes  remove any trailing slashes from each SOURCE\n                                 argument\n  -S, --suffix=SUFFIX          override the usual backup suffix\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -t, --target-directory=DIRECTORY  move all SOURCE arguments into DIRECTORY\n  -T, --no-target-directory    treat DEST as a normal file\n  -u, --update                 move only when the SOURCE file is newer\n                                 than the destination file or when the\n                                 destination file is missing\n  -v, --verbose                explain what is being done\n  -Z, --context                set SELinux security context of destination\n                                 file to default type\n\0"
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
        emit_ancillary_info(b"mv\0" as *const u8 as *const libc::c_char);
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
unsafe extern "C" fn priv_set_remove_linkdir() -> libc::c_int {
    return -(1 as libc::c_int);
}
static mut remove_trailing_slashes: bool = false;
static mut long_options: [option; 14] = [
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
            name: b"context\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'Z' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"force\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"interactive\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-clobber\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
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
            name: b"strip-trailing-slashes\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: STRIP_TRAILING_SLASHES_OPTION as libc::c_int,
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
            name: b"update\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'u' as i32,
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
unsafe extern "C" fn rm_option_init(mut x: *mut rm_options) {
    (*x).ignore_missing_files = 0 as libc::c_int != 0;
    (*x).remove_empty_directories = 1 as libc::c_int != 0;
    (*x).recursive = 1 as libc::c_int != 0;
    (*x).one_file_system = 0 as libc::c_int != 0;
    (*x).interactive = RMI_NEVER;
    (*x).stdin_tty = 0 as libc::c_int != 0;
    (*x).verbose = 0 as libc::c_int != 0;
    (*x).require_restore_cwd = 1 as libc::c_int != 0;
    static mut dev_ino_buf: dev_ino = dev_ino { st_ino: 0, st_dev: 0 };
    (*x).root_dev_ino = get_root_dev_ino(&mut dev_ino_buf);
    if ((*x).root_dev_ino).is_null() {
        if ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
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
    (*x).preserve_all_root = 0 as libc::c_int != 0;
}
unsafe extern "C" fn cp_option_init(mut x: *mut cp_options) {
    let mut selinux_enabled: bool = (0 as libc::c_int) < is_selinux_enabled();
    cp_options_default(x);
    (*x).copy_as_regular = 0 as libc::c_int != 0;
    (*x).reflink_mode = REFLINK_AUTO;
    (*x).dereference = DEREF_NEVER;
    (*x).unlink_dest_before_opening = 0 as libc::c_int != 0;
    (*x).unlink_dest_after_failed_open = 0 as libc::c_int != 0;
    (*x).hard_link = 0 as libc::c_int != 0;
    (*x).interactive = I_UNSPECIFIED;
    (*x).move_mode = 1 as libc::c_int != 0;
    (*x).install_mode = 0 as libc::c_int != 0;
    (*x).one_file_system = 0 as libc::c_int != 0;
    (*x).preserve_ownership = 1 as libc::c_int != 0;
    (*x).preserve_links = 1 as libc::c_int != 0;
    (*x).preserve_mode = 1 as libc::c_int != 0;
    (*x).preserve_timestamps = 1 as libc::c_int != 0;
    (*x).explicit_no_preserve_mode = 0 as libc::c_int != 0;
    (*x).preserve_security_context = selinux_enabled;
    (*x).set_security_context = 0 as *mut selabel_handle;
    (*x).reduce_diagnostics = 0 as libc::c_int != 0;
    (*x).data_copy_required = 1 as libc::c_int != 0;
    (*x).require_preserve = 0 as libc::c_int != 0;
    (*x).require_preserve_context = 0 as libc::c_int != 0;
    (*x).preserve_xattr = 1 as libc::c_int != 0;
    (*x).require_preserve_xattr = 0 as libc::c_int != 0;
    (*x).recursive = 1 as libc::c_int != 0;
    (*x).sparse_mode = SPARSE_AUTO;
    (*x).symbolic_link = 0 as libc::c_int != 0;
    (*x).set_mode = 0 as libc::c_int != 0;
    (*x).mode = 0 as libc::c_int as mode_t;
    (*x).stdin_tty = isatty(0 as libc::c_int) != 0;
    (*x).open_dangling_dest_symlink = 0 as libc::c_int != 0;
    (*x).update = 0 as libc::c_int != 0;
    (*x).verbose = 0 as libc::c_int != 0;
    (*x).dest_info = 0 as *mut Hash_table;
    (*x).src_info = 0 as *mut Hash_table;
}
unsafe extern "C" fn target_directory_operand(mut file: *const libc::c_char) -> bool {
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
        if ::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
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
    return is_a_dir;
}
unsafe extern "C" fn do_move(
    mut source: *const libc::c_char,
    mut dest: *const libc::c_char,
    mut x: *const cp_options,
) -> bool {
    let mut copy_into_self: bool = false;
    let mut rename_succeeded: bool = false;
    let mut ok: bool = copy(
        source,
        dest,
        0 as libc::c_int != 0,
        x,
        &mut copy_into_self,
        &mut rename_succeeded,
    );
    if ok {
        let mut dir_to_remove: *const libc::c_char = 0 as *const libc::c_char;
        if copy_into_self {
            dir_to_remove = 0 as *const libc::c_char;
            ok = 0 as libc::c_int != 0;
        } else if rename_succeeded {
            dir_to_remove = 0 as *const libc::c_char;
        } else {
            dir_to_remove = source;
        }
        if !dir_to_remove.is_null() {
            let mut rm_options: rm_options = rm_options {
                ignore_missing_files: false,
                interactive: 0 as rm_interactive,
                one_file_system: false,
                recursive: false,
                remove_empty_directories: false,
                root_dev_ino: 0 as *mut dev_ino,
                preserve_all_root: false,
                stdin_tty: false,
                verbose: false,
                require_restore_cwd: false,
            };
            let mut status: RM_status = 0 as RM_status;
            let mut dir: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
            rm_option_init(&mut rm_options);
            rm_options.verbose = (*x).verbose;
            dir[0 as libc::c_int as usize] = dir_to_remove;
            dir[1 as libc::c_int as usize] = 0 as *const libc::c_char;
            status = rm(
                dir.as_mut_ptr() as *mut libc::c_void as *const *mut libc::c_char,
                &mut rm_options,
            );
            if status as libc::c_uint == RM_OK as libc::c_int as libc::c_uint
                || status as libc::c_uint
                    == RM_USER_DECLINED as libc::c_int as libc::c_uint
                || status as libc::c_uint == RM_ERROR as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"VALID_STATUS (status)\0" as *const u8 as *const libc::c_char,
                    b"src/mv.c\0" as *const u8 as *const libc::c_char,
                    239 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 69],
                        &[libc::c_char; 69],
                    >(
                        b"_Bool do_move(const char *, const char *, const struct cp_options *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_8459: {
                if status as libc::c_uint == RM_OK as libc::c_int as libc::c_uint
                    || status as libc::c_uint
                        == RM_USER_DECLINED as libc::c_int as libc::c_uint
                    || status as libc::c_uint == RM_ERROR as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"VALID_STATUS (status)\0" as *const u8 as *const libc::c_char,
                        b"src/mv.c\0" as *const u8 as *const libc::c_char,
                        239 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 69],
                            &[libc::c_char; 69],
                        >(
                            b"_Bool do_move(const char *, const char *, const struct cp_options *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if status as libc::c_uint == RM_ERROR as libc::c_int as libc::c_uint {
                ok = 0 as libc::c_int != 0;
            }
        }
    }
    return ok;
}
unsafe extern "C" fn movefile(
    mut source: *mut libc::c_char,
    mut dest: *mut libc::c_char,
    mut dest_is_dir: bool,
    mut x: *const cp_options,
) -> bool {
    let mut ok: bool = false;
    if remove_trailing_slashes {
        strip_trailing_slashes(source);
    }
    if dest_is_dir {
        let mut src_basename: *const libc::c_char = last_component(source);
        let mut new_dest: *mut libc::c_char = file_name_concat(
            dest,
            src_basename,
            0 as *mut *mut libc::c_char,
        );
        strip_trailing_slashes(new_dest);
        ok = do_move(source, new_dest, x);
        free(new_dest as *mut libc::c_void);
    } else {
        ok = do_move(source, dest, x);
    }
    return ok;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut ok: bool = false;
    let mut make_backups: bool = 0 as libc::c_int != 0;
    let mut backup_suffix: *const libc::c_char = 0 as *const libc::c_char;
    let mut version_control_string: *mut libc::c_char = 0 as *mut libc::c_char;
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
    let mut target_directory: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut no_target_directory: bool = 0 as libc::c_int != 0;
    let mut n_files: libc::c_int = 0;
    let mut file: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut selinux_enabled: bool = (0 as libc::c_int) < is_selinux_enabled();
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdin as unsafe extern "C" fn() -> ()));
    cp_option_init(&mut x);
    priv_set_remove_linkdir();
    loop {
        c = getopt_long(
            argc,
            argv,
            b"bfint:uvS:TZ\0" as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            98 => {
                make_backups = 1 as libc::c_int != 0;
                if !optarg.is_null() {
                    version_control_string = optarg;
                }
            }
            102 => {
                x.interactive = I_ALWAYS_YES;
            }
            105 => {
                x.interactive = I_ASK_USER;
            }
            110 => {
                x.interactive = I_ALWAYS_NO;
            }
            128 => {
                remove_trailing_slashes = 1 as libc::c_int != 0;
            }
            116 => {
                if !target_directory.is_null() {
                    if ::core::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong != 0 {
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
                } else {
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
                    if stat(optarg, &mut st) != 0 as libc::c_int {
                        if ::core::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong
                            != 0
                        {
                            error(
                                1 as libc::c_int,
                                *__errno_location(),
                                gettext(
                                    b"failed to access %s\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(shell_escape_always_quoting_style, optarg),
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
                                quotearg_style(shell_escape_always_quoting_style, optarg),
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        };
                    }
                    if !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint)
                    {
                        if ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong
                            != 0
                        {
                            error(
                                1 as libc::c_int,
                                0 as libc::c_int,
                                gettext(
                                    b"target %s is not a directory\0" as *const u8
                                        as *const libc::c_char,
                                ),
                                quotearg_style(shell_escape_always_quoting_style, optarg),
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
                                quotearg_style(shell_escape_always_quoting_style, optarg),
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        };
                    }
                }
                target_directory = optarg;
            }
            84 => {
                no_target_directory = 1 as libc::c_int != 0;
            }
            117 => {
                x.update = 1 as libc::c_int != 0;
            }
            118 => {
                x.verbose = 1 as libc::c_int != 0;
            }
            83 => {
                make_backups = 1 as libc::c_int != 0;
                backup_suffix = optarg;
            }
            90 => {
                if selinux_enabled {
                    x.preserve_security_context = 0 as libc::c_int != 0;
                    x
                        .set_security_context = selabel_open(
                        0 as libc::c_int as libc::c_uint,
                        0 as *const selinux_opt,
                        0 as libc::c_int as libc::c_uint,
                    );
                    if (x.set_security_context).is_null() {
                        error(
                            0 as libc::c_int,
                            *__errno_location(),
                            gettext(
                                b"warning: ignoring --context\0" as *const u8
                                    as *const libc::c_char,
                            ),
                        );
                    }
                }
            }
            -130 => {
                usage(0 as libc::c_int);
            }
            -131 => {
                version_etc(
                    stdout,
                    b"mv\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    b"Mike Parker\0" as *const u8 as *const libc::c_char,
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
    n_files = argc - optind;
    file = argv.offset(optind as isize);
    if n_files <= target_directory.is_null() as libc::c_int {
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
            if ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
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
    } else if target_directory.is_null() {
        if 2 as libc::c_int <= n_files {} else {
            __assert_fail(
                b"2 <= n_files\0" as *const u8 as *const libc::c_char,
                b"src/mv.c\0" as *const u8 as *const libc::c_char,
                459 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
        'c_9127: {
            if 2 as libc::c_int <= n_files {} else {
                __assert_fail(
                    b"2 <= n_files\0" as *const u8 as *const libc::c_char,
                    b"src/mv.c\0" as *const u8 as *const libc::c_char,
                    459 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 23],
                        &[libc::c_char; 23],
                    >(b"int main(int, char **)\0"))
                        .as_ptr(),
                );
            }
        };
        if n_files == 2 as libc::c_int {
            x
                .rename_errno = if renameatu(
                -(100 as libc::c_int),
                *file.offset(0 as libc::c_int as isize),
                -(100 as libc::c_int),
                *file.offset(1 as libc::c_int as isize),
                ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint,
            ) != 0
            {
                *__errno_location()
            } else {
                0 as libc::c_int
            };
        }
        if x.rename_errno != 0 as libc::c_int
            && target_directory_operand(
                *file.offset((n_files - 1 as libc::c_int) as isize),
            ) as libc::c_int != 0
        {
            x.rename_errno = -(1 as libc::c_int);
            n_files -= 1;
            target_directory = *file.offset(n_files as isize);
        } else if (2 as libc::c_int) < n_files {
            if ::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
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
    if x.interactive as libc::c_uint == I_ALWAYS_NO as libc::c_int as libc::c_uint {
        x.update = 0 as libc::c_int != 0;
    }
    if make_backups as libc::c_int != 0
        && x.interactive as libc::c_uint == I_ALWAYS_NO as libc::c_int as libc::c_uint
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            gettext(
                b"options --backup and --no-clobber are mutually exclusive\0"
                    as *const u8 as *const libc::c_char,
            ),
        );
        usage(1 as libc::c_int);
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
    hash_init();
    if !target_directory.is_null() {
        if 2 as libc::c_int <= n_files {
            dest_info_init(&mut x);
        }
        ok = 1 as libc::c_int != 0;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < n_files {
            x.last_file = i + 1 as libc::c_int == n_files;
            ok = (ok as libc::c_int
                & movefile(
                    *file.offset(i as isize),
                    target_directory,
                    1 as libc::c_int != 0,
                    &mut x,
                ) as libc::c_int) as bool;
            i += 1;
            i;
        }
    } else {
        x.last_file = 1 as libc::c_int != 0;
        ok = movefile(
            *file.offset(0 as libc::c_int as isize),
            *file.offset(1 as libc::c_int as isize),
            0 as libc::c_int != 0,
            &mut x,
        );
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
