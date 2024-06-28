use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type hash_table;
    pub type selabel_handle;
    static mut stderr: *mut FILE;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn uname(__name: *mut utsname) -> libc::c_int;
    fn freecon(con: *mut libc::c_char);
    fn setfscreatecon(context: *const libc::c_char) -> libc::c_int;
    fn rpl_lgetfilecon(
        path: *const libc::c_char,
        con: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn fstatat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn lchmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn fchmod(__fd: libc::c_int, __mode: __mode_t) -> libc::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn mknod(
        __path: *const libc::c_char,
        __mode: __mode_t,
        __dev: __dev_t,
    ) -> libc::c_int;
    fn mkfifo(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn euidaccess(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
    fn lchown(
        __file: *const libc::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> libc::c_int;
    fn geteuid() -> __uid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn rmdir(__path: *const libc::c_char) -> libc::c_int;
    fn getpagesize() -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    fn copy_file_range(
        __infd: libc::c_int,
        __pinoff: *mut __off64_t,
        __outfd: libc::c_int,
        __poutoff: *mut __off64_t,
        __length: size_t,
        __flags: libc::c_uint,
    ) -> ssize_t;
    fn gnu_dev_major(__dev: __dev_t) -> libc::c_uint;
    fn gnu_dev_minor(__dev: __dev_t) -> libc::c_uint;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn mempcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn canonicalize_file_name(__name: *const libc::c_char) -> *mut libc::c_char;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn dir_name(file: *const libc::c_char) -> *mut libc::c_char;
    static mut program_name: *const libc::c_char;
    fn quotearg_n_style(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn fallocate(
        __fd: libc::c_int,
        __mode: libc::c_int,
        __offset: __off_t,
        __len: __off_t,
    ) -> libc::c_int;
    fn qset_acl(_: *const libc::c_char, _: libc::c_int, _: mode_t) -> libc::c_int;
    fn set_acl(_: *const libc::c_char, _: libc::c_int, _: mode_t) -> libc::c_int;
    fn copy_acl(
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: mode_t,
    ) -> libc::c_int;
    static mut simple_backup_suffix: *const libc::c_char;
    fn backup_file_rename(
        _: libc::c_int,
        _: *const libc::c_char,
        _: backup_type,
    ) -> *mut libc::c_char;
    fn buffer_lcm(_: size_t, _: size_t, _: size_t) -> size_t;
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn forget_created(ino: ino_t, dev: dev_t);
    fn remember_copied(
        node: *const libc::c_char,
        ino: ino_t,
        dev: dev_t,
    ) -> *mut libc::c_char;
    fn src_to_dest_lookup(ino: ino_t, dev: dev_t) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn fdadvise(fd: libc::c_int, offset: off_t, len: off_t, advice: fadvice_t);
    fn open_safer(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
    fn record_file(ht: *mut Hash_table, file: *const libc::c_char, stats: *const stat);
    fn seen_file(
        ht: *const Hash_table,
        file: *const libc::c_char,
        stats: *const stat,
    ) -> bool;
    fn strmode(mode: mode_t, str: *mut libc::c_char);
    fn file_name_concat(
        dir: *const libc::c_char,
        base: *const libc::c_char,
        base_in_result: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn force_linkat(
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: bool,
        _: libc::c_int,
    ) -> libc::c_int;
    fn force_symlinkat(
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
        _: bool,
        _: libc::c_int,
    ) -> libc::c_int;
    fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> size_t;
    fn triple_hash(x: *const libc::c_void, table_size: size_t) -> size_t;
    fn triple_free(x: *mut libc::c_void);
    fn triple_hash_no_name(x: *const libc::c_void, table_size: size_t) -> size_t;
    fn triple_compare(x: *const libc::c_void, y: *const libc::c_void) -> bool;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn renameatu(
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_uint,
    ) -> libc::c_int;
    fn same_name(source: *const libc::c_char, dest: *const libc::c_char) -> bool;
    fn savedir(_: *const libc::c_char, _: savedir_option) -> *mut libc::c_char;
    fn utimecmp(
        _: *const libc::c_char,
        _: *const stat,
        _: *const stat,
        _: libc::c_int,
    ) -> libc::c_int;
    fn fdutimens(
        _: libc::c_int,
        _: *const libc::c_char,
        _: *const timespec,
    ) -> libc::c_int;
    fn utimens(_: *const libc::c_char, _: *const timespec) -> libc::c_int;
    fn lutimens(_: *const libc::c_char, _: *const timespec) -> libc::c_int;
    fn can_write_any_file() -> bool;
    fn areadlink_with_size(
        filename: *const libc::c_char,
        size_hint: size_t,
    ) -> *mut libc::c_char;
    fn yesno() -> bool;
    fn xstrtoumax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut uintmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
    fn restorecon(
        selabel_handle: *mut selabel_handle,
        path: *const libc::c_char,
        recurse: bool,
    ) -> bool;
    fn defaultcon(
        selabel_handle: *mut selabel_handle,
        path: *const libc::c_char,
        mode: mode_t,
    ) -> libc::c_int;
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
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
pub type ptrdiff_t = libc::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [libc::c_char; 65],
    pub nodename: [libc::c_char; 65],
    pub release: [libc::c_char; 65],
    pub version: [libc::c_char; 65],
    pub machine: [libc::c_char; 65],
    pub domainname: [libc::c_char; 65],
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
pub type uintmax_t = __uintmax_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_tuning {
    pub shrink_threshold: libc::c_float,
    pub shrink_factor: libc::c_float,
    pub growth_threshold: libc::c_float,
    pub growth_factor: libc::c_float,
    pub is_n_buckets: bool,
}
pub type Hash_tuning = hash_tuning;
pub type Hash_table = hash_table;
pub type Hash_hasher = Option::<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
pub type Hash_comparator = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_data_freer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
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
pub struct dir_list {
    pub parent: *mut dir_list,
    pub ino: ino_t,
    pub dev: dev_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub _gl_dummy: libc::c_int,
}
pub const PLAIN_SCANTYPE: scantype = 1;
pub type scantype = libc::c_uint;
pub const LSEEK_SCANTYPE: scantype = 3;
pub const ZERO_SCANTYPE: scantype = 2;
pub const ERROR_SCANTYPE: scantype = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union scan_inference {
    pub ext_start: off_t,
}
pub const IO_BUFSIZE: C2RustUnnamed_0 = 131072;
pub const LONGINT_OK: strtol_error = 0;
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub type fadvice_t = libc::c_uint;
pub const FADVISE_RANDOM: fadvice_t = 1;
pub const FADVISE_WILLNEED: fadvice_t = 3;
pub const FADVISE_DONTNEED: fadvice_t = 4;
pub const FADVISE_NOREUSE: fadvice_t = 5;
pub const FADVISE_SEQUENTIAL: fadvice_t = 2;
pub const FADVISE_NORMAL: fadvice_t = 0;
pub type savedir_option = libc::c_uint;
pub const SAVEDIR_SORT_FASTREAD: savedir_option = 2;
pub const SAVEDIR_SORT_INODE: savedir_option = 2;
pub const SAVEDIR_SORT_NAME: savedir_option = 1;
pub const SAVEDIR_SORT_NONE: savedir_option = 0;
pub const UTIMECMP_TRUNCATE_SOURCE: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
pub type C2RustUnnamed_1 = libc::c_uint;
#[inline]
unsafe extern "C" fn dot_or_dotdot(mut file_name: *const libc::c_char) -> bool {
    if *file_name.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
        let mut sep: libc::c_char = *file_name
            .offset(
                ((*file_name.offset(1 as libc::c_int as isize) as libc::c_int
                    == '.' as i32) as libc::c_int + 1 as libc::c_int) as isize,
            );
        return sep == 0 || sep as libc::c_int == '/' as i32;
    } else {
        return 0 as libc::c_int != 0
    };
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
unsafe extern "C" fn is_nul(mut buf: *const libc::c_void, mut length: size_t) -> bool {
    let mut p: *const libc::c_uchar = buf as *const libc::c_uchar;
    let mut word: libc::c_uchar = 0;
    if length == 0 {
        return 1 as libc::c_int != 0;
    }
    while (length
        & (::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_long != 0
    {
        if *p != 0 {
            return 0 as libc::c_int != 0;
        }
        p = p.offset(1);
        p;
        length = length.wrapping_sub(1);
        length;
        if length == 0 {
            return 1 as libc::c_int != 0;
        }
    }
    loop {
        memcpy(
            &mut word as *mut libc::c_uchar as *mut libc::c_void,
            p as *const libc::c_void,
            ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
        );
        if word != 0 {
            return 0 as libc::c_int != 0;
        }
        p = p.offset(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong as isize);
        length = (length as libc::c_ulong)
            .wrapping_sub(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            as size_t as size_t;
        if length == 0 {
            return 1 as libc::c_int != 0;
        }
        if (length & 15 as libc::c_int as libc::c_ulong) as libc::c_long
            == 0 as libc::c_int as libc::c_long
        {
            break;
        }
    }
    return memcmp(buf, p as *const libc::c_void, length) == 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn is_ENOTSUP(mut err: libc::c_int) -> bool {
    return err == 95 as libc::c_int
        || 95 as libc::c_int != 95 as libc::c_int && err == 95 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn set_process_security_ctx(
    mut src_name: *const libc::c_char,
    mut dst_name: *const libc::c_char,
    mut mode: mode_t,
    mut new_dst: bool,
    mut x: *const cp_options,
) -> bool {
    if (*x).preserve_security_context {
        let mut all_errors: bool = !(*x).data_copy_required
            || (*x).require_preserve_context as libc::c_int != 0;
        let mut some_errors: bool = !all_errors && !(*x).reduce_diagnostics;
        let mut con: *mut libc::c_char = 0 as *mut libc::c_char;
        if 0 as libc::c_int <= rpl_lgetfilecon(src_name, &mut con) {
            if setfscreatecon(con) < 0 as libc::c_int {
                if all_errors as libc::c_int != 0
                    || some_errors as libc::c_int != 0
                        && !errno_unsupported(*__errno_location())
                {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        gettext(
                            b"failed to set default file creation context to %s\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quote(con),
                    );
                }
                if (*x).require_preserve_context {
                    freecon(con);
                    return 0 as libc::c_int != 0;
                }
            }
            freecon(con);
        } else {
            if all_errors as libc::c_int != 0
                || some_errors as libc::c_int != 0
                    && !errno_unsupported(*__errno_location())
            {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"failed to get security context of %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, src_name),
                );
            }
            if (*x).require_preserve_context {
                return 0 as libc::c_int != 0;
            }
        }
    } else if !((*x).set_security_context).is_null() {
        if new_dst as libc::c_int != 0
            && defaultcon((*x).set_security_context, dst_name, mode) < 0 as libc::c_int
            && !ignorable_ctx_err(*__errno_location())
        {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(
                    b"failed to set default file creation context for %s\0" as *const u8
                        as *const libc::c_char,
                ),
                quotearg_style(shell_escape_always_quoting_style, dst_name),
            );
        }
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn set_file_security_ctx(
    mut dst_name: *const libc::c_char,
    mut recurse: bool,
    mut x: *const cp_options,
) -> bool {
    let mut all_errors: bool = !(*x).data_copy_required
        || (*x).require_preserve_context as libc::c_int != 0;
    let mut some_errors: bool = !all_errors && !(*x).reduce_diagnostics;
    if !restorecon((*x).set_security_context, dst_name, recurse) {
        if all_errors as libc::c_int != 0
            || some_errors as libc::c_int != 0 && !errno_unsupported(*__errno_location())
        {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(
                    b"failed to set the security context of %s\0" as *const u8
                        as *const libc::c_char,
                ),
                quotearg_n_style(
                    0 as libc::c_int,
                    shell_escape_always_quoting_style,
                    dst_name,
                ),
            );
        }
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn copy(
    mut src_name: *const libc::c_char,
    mut dst_name: *const libc::c_char,
    mut nonexistent_dst: bool,
    mut options: *const cp_options,
    mut copy_into_self: *mut bool,
    mut rename_succeeded: *mut bool,
) -> bool {
    if valid_options(options) {} else {
        __assert_fail(
            b"valid_options (options)\0" as *const u8 as *const libc::c_char,
            b"src/copy.c\0" as *const u8 as *const libc::c_char,
            3092 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 91],
                &[libc::c_char; 91],
            >(
                b"_Bool copy(const char *, const char *, _Bool, const struct cp_options *, _Bool *, _Bool *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_18793: {
        if valid_options(options) {} else {
            __assert_fail(
                b"valid_options (options)\0" as *const u8 as *const libc::c_char,
                b"src/copy.c\0" as *const u8 as *const libc::c_char,
                3092 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 91],
                    &[libc::c_char; 91],
                >(
                    b"_Bool copy(const char *, const char *, _Bool, const struct cp_options *, _Bool *, _Bool *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    top_level_src_name = src_name;
    top_level_dst_name = dst_name;
    let mut first_dir_created_per_command_line_arg: bool = 0 as libc::c_int != 0;
    return copy_internal(
        src_name,
        dst_name,
        nonexistent_dst,
        0 as *const stat,
        0 as *mut dir_list,
        options,
        1 as libc::c_int != 0,
        &mut first_dir_created_per_command_line_arg,
        copy_into_self,
        rename_succeeded,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dest_info_init(mut x: *mut cp_options) {
    (*x)
        .dest_info = hash_initialize(
        61 as libc::c_int as size_t,
        0 as *const Hash_tuning,
        Some(triple_hash as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t),
        Some(
            triple_compare
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
        ),
        Some(triple_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    if ((*x).dest_info).is_null() {
        xalloc_die();
    }
}
#[no_mangle]
pub unsafe extern "C" fn chown_failure_ok(mut x: *const cp_options) -> bool {
    return (*__errno_location() == 1 as libc::c_int
        || *__errno_location() == 22 as libc::c_int) && !(*x).chown_privileges;
}
#[no_mangle]
pub unsafe extern "C" fn cp_options_default(mut x: *mut cp_options) {
    memset(
        x as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<cp_options>() as libc::c_ulong,
    );
    (*x).owner_privileges = geteuid() == 0 as libc::c_int as libc::c_uint;
    (*x).chown_privileges = (*x).owner_privileges;
    (*x).rename_errno = -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn src_info_init(mut x: *mut cp_options) {
    (*x)
        .src_info = hash_initialize(
        61 as libc::c_int as size_t,
        0 as *const Hash_tuning,
        Some(
            triple_hash_no_name
                as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
        ),
        Some(
            triple_compare
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
        ),
        Some(triple_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    if ((*x).src_info).is_null() {
        xalloc_die();
    }
}
#[no_mangle]
pub unsafe extern "C" fn cached_umask() -> mode_t {
    static mut mask: mode_t = -(1 as libc::c_int) as mode_t;
    if mask == -(1 as libc::c_int) as mode_t {
        mask = umask(0 as libc::c_int as __mode_t);
        umask(mask);
    }
    return mask;
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
unsafe extern "C" fn get_stat_atime(mut st: *const stat) -> timespec {
    return (*st).st_atim;
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
#[inline]
unsafe extern "C" fn ignorable_ctx_err(mut err: libc::c_int) -> bool {
    return err == 95 as libc::c_int || err == 61 as libc::c_int;
}
unsafe extern "C" fn copy_internal(
    mut src_name: *const libc::c_char,
    mut dst_name: *const libc::c_char,
    mut new_dst: bool,
    mut parent: *const stat,
    mut ancestors: *mut dir_list,
    mut x: *const cp_options,
    mut command_line_arg: bool,
    mut first_dir_created_per_command_line_arg: *mut bool,
    mut copy_into_self: *mut bool,
    mut rename_succeeded: *mut bool,
) -> bool {
    let mut current_block: u64;
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
    let mut dst_sb: stat = stat {
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
    let mut src_mode: mode_t = 0;
    let mut dst_mode: mode_t = 0;
    let mut dst_mode_bits: mode_t = 0;
    let mut omitted_permissions: mode_t = 0;
    let mut restore_dst_mode: bool = 0 as libc::c_int != 0;
    let mut earlier_file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dst_backup: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut delayed_ok: bool = false;
    let mut copied_as_regular: bool = 0 as libc::c_int != 0;
    let mut dest_is_symlink: bool = 0 as libc::c_int != 0;
    let mut have_dst_lstat: bool = 0 as libc::c_int != 0;
    *copy_into_self = 0 as libc::c_int != 0;
    let mut rename_errno: libc::c_int = (*x).rename_errno;
    if (*x).move_mode {
        if rename_errno < 0 as libc::c_int {
            rename_errno = if renameatu(
                -(100 as libc::c_int),
                src_name,
                -(100 as libc::c_int),
                dst_name,
                ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint,
            ) != 0
            {
                *__errno_location()
            } else {
                0 as libc::c_int
            };
        }
        new_dst = rename_errno == 0 as libc::c_int;
        if !rename_succeeded.is_null() {
            *rename_succeeded = new_dst;
        }
    }
    if if rename_errno == 0 as libc::c_int {
        !(*x).last_file as libc::c_int
    } else {
        (rename_errno != 17 as libc::c_int
            || (*x).interactive as libc::c_uint
                != I_ALWAYS_NO as libc::c_int as libc::c_uint) as libc::c_int
    } != 0
    {
        let mut name: *const libc::c_char = if rename_errno == 0 as libc::c_int {
            dst_name
        } else {
            src_name
        };
        let mut fstatat_flags: libc::c_int = if (*x).dereference as libc::c_uint
            == DEREF_NEVER as libc::c_int as libc::c_uint
        {
            0x100 as libc::c_int
        } else {
            0 as libc::c_int
        };
        if follow_fstatat(-(100 as libc::c_int), name, &mut src_sb, fstatat_flags)
            != 0 as libc::c_int
        {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(b"cannot stat %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, name),
            );
            return 0 as libc::c_int != 0;
        }
        src_mode = src_sb.st_mode;
        if src_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint && !(*x).recursive
        {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                if !(*x).install_mode {
                    gettext(
                        b"-r not specified; omitting directory %s\0" as *const u8
                            as *const libc::c_char,
                    )
                } else {
                    gettext(
                        b"omitting directory %s\0" as *const u8 as *const libc::c_char,
                    )
                },
                quotearg_style(shell_escape_always_quoting_style, src_name),
            );
            return 0 as libc::c_int != 0;
        }
    }
    if command_line_arg as libc::c_int != 0 && !((*x).src_info).is_null() {
        if !(src_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint)
            && (*x).backup_type as libc::c_uint
                == no_backups as libc::c_int as libc::c_uint
            && seen_file((*x).src_info, src_name, &mut src_sb) as libc::c_int != 0
        {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"warning: source file %s specified more than once\0" as *const u8
                        as *const libc::c_char,
                ),
                quotearg_style(shell_escape_always_quoting_style, src_name),
            );
            return 1 as libc::c_int != 0;
        }
        record_file((*x).src_info, src_name, &mut src_sb);
    }
    let mut dereference: bool = should_dereference(x, command_line_arg);
    if !new_dst {
        if !(rename_errno == 17 as libc::c_int
            && (*x).interactive as libc::c_uint
                == I_ALWAYS_NO as libc::c_int as libc::c_uint)
        {
            let mut use_lstat: bool = !(src_mode
                & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint)
                && (!(*x).copy_as_regular
                    || src_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint
                    || src_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o120000 as libc::c_int as libc::c_uint)
                || (*x).move_mode as libc::c_int != 0
                || (*x).symbolic_link as libc::c_int != 0
                || (*x).hard_link as libc::c_int != 0
                || (*x).backup_type as libc::c_uint
                    != no_backups as libc::c_int as libc::c_uint
                || (*x).unlink_dest_before_opening as libc::c_int != 0;
            let mut fstatat_flags_0: libc::c_int = if use_lstat as libc::c_int != 0 {
                0x100 as libc::c_int
            } else {
                0 as libc::c_int
            };
            if follow_fstatat(
                -(100 as libc::c_int),
                dst_name,
                &mut dst_sb,
                fstatat_flags_0,
            ) == 0 as libc::c_int
            {
                have_dst_lstat = use_lstat;
                rename_errno = 17 as libc::c_int;
            } else if !(*__errno_location() == 40 as libc::c_int
                && (*x).unlink_dest_after_failed_open as libc::c_int != 0)
            {
                if *__errno_location() != 2 as libc::c_int {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        gettext(b"cannot stat %s\0" as *const u8 as *const libc::c_char),
                        quotearg_style(shell_escape_always_quoting_style, dst_name),
                    );
                    return 0 as libc::c_int != 0;
                } else {
                    new_dst = 1 as libc::c_int != 0;
                }
            }
        }
        if rename_errno == 17 as libc::c_int {
            let mut return_now: bool = 0 as libc::c_int != 0;
            if (*x).interactive as libc::c_uint
                != I_ALWAYS_NO as libc::c_int as libc::c_uint
                && !same_file_ok(
                    src_name,
                    &mut src_sb,
                    dst_name,
                    &mut dst_sb,
                    x,
                    &mut return_now,
                )
            {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"%s and %s are the same file\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_n_style(
                        0 as libc::c_int,
                        shell_escape_always_quoting_style,
                        src_name,
                    ),
                    quotearg_n_style(
                        1 as libc::c_int,
                        shell_escape_always_quoting_style,
                        dst_name,
                    ),
                );
                return 0 as libc::c_int != 0;
            }
            if (*x).update as libc::c_int != 0
                && !(src_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o40000 as libc::c_int as libc::c_uint)
            {
                let mut options: libc::c_int = if (*x).preserve_timestamps as libc::c_int
                    != 0
                    && !((*x).move_mode as libc::c_int != 0
                        && dst_sb.st_dev == src_sb.st_dev)
                {
                    UTIMECMP_TRUNCATE_SOURCE as libc::c_int
                } else {
                    0 as libc::c_int
                };
                if 0 as libc::c_int
                    <= utimecmp(dst_name, &mut dst_sb, &mut src_sb, options)
                {
                    if !rename_succeeded.is_null() {
                        *rename_succeeded = 1 as libc::c_int != 0;
                    }
                    earlier_file = remember_copied(
                        dst_name,
                        src_sb.st_ino,
                        src_sb.st_dev,
                    );
                    if !earlier_file.is_null() {
                        if !create_hard_link(
                            earlier_file,
                            dst_name,
                            1 as libc::c_int != 0,
                            (*x).verbose,
                            dereference,
                        ) {
                            current_block = 7284661225219459306;
                        } else {
                            current_block = 12497913735442871383;
                        }
                    } else {
                        current_block = 12497913735442871383;
                    }
                    match current_block {
                        7284661225219459306 => {}
                        _ => return 1 as libc::c_int != 0,
                    }
                } else {
                    current_block = 3689906465960840878;
                }
            } else {
                current_block = 3689906465960840878;
            }
            match current_block {
                7284661225219459306 => {}
                _ => {
                    if (*x).move_mode {
                        if abandon_move(x, dst_name, &mut dst_sb) {
                            if !rename_succeeded.is_null() {
                                *rename_succeeded = 1 as libc::c_int != 0;
                            }
                            return 1 as libc::c_int != 0;
                        }
                    } else if !(src_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint)
                        && ((*x).interactive as libc::c_uint
                            == I_ALWAYS_NO as libc::c_int as libc::c_uint
                            || (*x).interactive as libc::c_uint
                                == I_ASK_USER as libc::c_int as libc::c_uint
                                && !overwrite_ok(x, dst_name, &mut dst_sb))
                    {
                        return 1 as libc::c_int != 0
                    }
                    if return_now {
                        return 1 as libc::c_int != 0;
                    }
                    if !(dst_sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint)
                    {
                        if src_mode & 0o170000 as libc::c_int as libc::c_uint
                            == 0o40000 as libc::c_int as libc::c_uint
                        {
                            if (*x).move_mode as libc::c_int != 0
                                && (*x).backup_type as libc::c_uint
                                    != no_backups as libc::c_int as libc::c_uint
                            {} else {
                                error(
                                    0 as libc::c_int,
                                    0 as libc::c_int,
                                    gettext(
                                        b"cannot overwrite non-directory %s with directory %s\0"
                                            as *const u8 as *const libc::c_char,
                                    ),
                                    quotearg_n_style(
                                        0 as libc::c_int,
                                        shell_escape_always_quoting_style,
                                        dst_name,
                                    ),
                                    quotearg_n_style(
                                        1 as libc::c_int,
                                        shell_escape_always_quoting_style,
                                        src_name,
                                    ),
                                );
                                return 0 as libc::c_int != 0;
                            }
                        }
                        if command_line_arg as libc::c_int != 0
                            && (*x).backup_type as libc::c_uint
                                != numbered_backups as libc::c_int as libc::c_uint
                            && seen_file((*x).dest_info, dst_name, &mut dst_sb)
                                as libc::c_int != 0
                        {
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                gettext(
                                    b"will not overwrite just-created %s with %s\0" as *const u8
                                        as *const libc::c_char,
                                ),
                                quotearg_n_style(
                                    0 as libc::c_int,
                                    shell_escape_always_quoting_style,
                                    dst_name,
                                ),
                                quotearg_n_style(
                                    1 as libc::c_int,
                                    shell_escape_always_quoting_style,
                                    src_name,
                                ),
                            );
                            return 0 as libc::c_int != 0;
                        }
                    }
                    if !(src_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint)
                    {
                        if dst_sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
                            == 0o40000 as libc::c_int as libc::c_uint
                        {
                            if (*x).move_mode as libc::c_int != 0
                                && (*x).backup_type as libc::c_uint
                                    != no_backups as libc::c_int as libc::c_uint
                            {} else {
                                error(
                                    0 as libc::c_int,
                                    0 as libc::c_int,
                                    gettext(
                                        b"cannot overwrite directory %s with non-directory\0"
                                            as *const u8 as *const libc::c_char,
                                    ),
                                    quotearg_style(shell_escape_always_quoting_style, dst_name),
                                );
                                return 0 as libc::c_int != 0;
                            }
                        }
                    }
                    if (*x).move_mode {
                        if src_sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
                            == 0o40000 as libc::c_int as libc::c_uint
                            && !(dst_sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
                                == 0o40000 as libc::c_int as libc::c_uint)
                            && (*x).backup_type as libc::c_uint
                                == no_backups as libc::c_int as libc::c_uint
                        {
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                gettext(
                                    b"cannot move directory onto non-directory: %s -> %s\0"
                                        as *const u8 as *const libc::c_char,
                                ),
                                quotearg_n_style_colon(
                                    0 as libc::c_int,
                                    shell_escape_quoting_style,
                                    src_name,
                                ),
                                quotearg_n_style_colon(
                                    0 as libc::c_int,
                                    shell_escape_quoting_style,
                                    dst_name,
                                ),
                            );
                            return 0 as libc::c_int != 0;
                        }
                    }
                    let mut srcbase: *const libc::c_char = 0 as *const libc::c_char;
                    if (*x).backup_type as libc::c_uint
                        != no_backups as libc::c_int as libc::c_uint
                        && {
                            srcbase = last_component(src_name);
                            !dot_or_dotdot(srcbase)
                        }
                        && ((*x).move_mode as libc::c_int != 0
                            || !(dst_sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
                                == 0o40000 as libc::c_int as libc::c_uint))
                    {
                        if (*x).backup_type as libc::c_uint
                            != numbered_backups as libc::c_int as libc::c_uint
                            && source_is_dst_backup(srcbase, &mut src_sb, dst_name)
                                as libc::c_int != 0
                        {
                            let mut fmt: *const libc::c_char = 0 as *const libc::c_char;
                            fmt = if (*x).move_mode as libc::c_int != 0 {
                                gettext(
                                    b"backing up %s might destroy source;  %s not moved\0"
                                        as *const u8 as *const libc::c_char,
                                )
                            } else {
                                gettext(
                                    b"backing up %s might destroy source;  %s not copied\0"
                                        as *const u8 as *const libc::c_char,
                                )
                            };
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                fmt,
                                quotearg_n_style(
                                    0 as libc::c_int,
                                    shell_escape_always_quoting_style,
                                    dst_name,
                                ),
                                quotearg_n_style(
                                    1 as libc::c_int,
                                    shell_escape_always_quoting_style,
                                    src_name,
                                ),
                            );
                            return 0 as libc::c_int != 0;
                        }
                        let mut tmp_backup: *mut libc::c_char = backup_file_rename(
                            -(100 as libc::c_int),
                            dst_name,
                            (*x).backup_type,
                        );
                        if !tmp_backup.is_null() {
                            dst_backup = ({
                                let mut __old: *const libc::c_char = tmp_backup;
                                let mut __len: size_t = (strlen(__old))
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
                                let mut fresh0 = ::std::vec::from_elem(0, __len as usize);
                                let mut __new: *mut libc::c_char = fresh0.as_mut_ptr()
                                    as *mut libc::c_char;
                                memcpy(
                                    __new as *mut libc::c_void,
                                    __old as *const libc::c_void,
                                    __len,
                                ) as *mut libc::c_char
                            });
                            free(tmp_backup as *mut libc::c_void);
                        } else if *__errno_location() != 2 as libc::c_int {
                            error(
                                0 as libc::c_int,
                                *__errno_location(),
                                gettext(
                                    b"cannot backup %s\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(shell_escape_always_quoting_style, dst_name),
                            );
                            return 0 as libc::c_int != 0;
                        }
                        new_dst = 1 as libc::c_int != 0;
                    } else if !(dst_sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint) && !(*x).move_mode
                        && ((*x).unlink_dest_before_opening as libc::c_int != 0
                            || (*x).data_copy_required as libc::c_int != 0
                                && ((*x).preserve_links as libc::c_int != 0
                                    && (1 as libc::c_int as libc::c_ulong) < dst_sb.st_nlink
                                    || (*x).dereference as libc::c_uint
                                        == DEREF_NEVER as libc::c_int as libc::c_uint
                                        && !(src_sb.st_mode
                                            & 0o170000 as libc::c_int as libc::c_uint
                                            == 0o100000 as libc::c_int as libc::c_uint)))
                    {
                        if unlink(dst_name) != 0 as libc::c_int
                            && *__errno_location() != 2 as libc::c_int
                        {
                            error(
                                0 as libc::c_int,
                                *__errno_location(),
                                gettext(
                                    b"cannot remove %s\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(shell_escape_always_quoting_style, dst_name),
                            );
                            return 0 as libc::c_int != 0;
                        }
                        new_dst = 1 as libc::c_int != 0;
                        if (*x).verbose {
                            printf(
                                gettext(
                                    b"removed %s\n\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(shell_escape_always_quoting_style, dst_name),
                            );
                        }
                    }
                    current_block = 14116432890150942211;
                }
            }
        } else {
            current_block = 14116432890150942211;
        }
    } else {
        current_block = 14116432890150942211;
    }
    match current_block {
        14116432890150942211 => {
            if command_line_arg as libc::c_int != 0 && !((*x).dest_info).is_null()
                && !(*x).move_mode
                && (*x).backup_type as libc::c_uint
                    == no_backups as libc::c_int as libc::c_uint
            {
                let mut lstat_ok: bool = 1 as libc::c_int != 0;
                let mut tmp_buf: stat = stat {
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
                let mut dst_lstat_sb: *mut stat = 0 as *mut stat;
                if have_dst_lstat {
                    dst_lstat_sb = &mut dst_sb;
                } else if lstat(dst_name, &mut tmp_buf) == 0 as libc::c_int {
                    dst_lstat_sb = &mut tmp_buf;
                } else {
                    lstat_ok = 0 as libc::c_int != 0;
                }
                if lstat_ok as libc::c_int != 0
                    && (*dst_lstat_sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o120000 as libc::c_int as libc::c_uint
                    && seen_file((*x).dest_info, dst_name, dst_lstat_sb) as libc::c_int
                        != 0
                {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"will not copy %s through just-created symlink %s\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quotearg_n_style(
                            0 as libc::c_int,
                            shell_escape_always_quoting_style,
                            src_name,
                        ),
                        quotearg_n_style(
                            1 as libc::c_int,
                            shell_escape_always_quoting_style,
                            dst_name,
                        ),
                    );
                    return 0 as libc::c_int != 0;
                }
            }
            if (*x).verbose as libc::c_int != 0 && !(*x).move_mode
                && !(src_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o40000 as libc::c_int as libc::c_uint)
            {
                emit_verbose(src_name, dst_name, dst_backup);
            }
            if rename_errno == 0 as libc::c_int {
                earlier_file = 0 as *mut libc::c_char;
            } else if (*x).recursive as libc::c_int != 0
                && src_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o40000 as libc::c_int as libc::c_uint
            {
                if command_line_arg {
                    earlier_file = remember_copied(
                        dst_name,
                        src_sb.st_ino,
                        src_sb.st_dev,
                    );
                } else {
                    earlier_file = src_to_dest_lookup(src_sb.st_ino, src_sb.st_dev);
                }
            } else if (*x).move_mode as libc::c_int != 0
                && src_sb.st_nlink == 1 as libc::c_int as libc::c_ulong
            {
                earlier_file = src_to_dest_lookup(src_sb.st_ino, src_sb.st_dev);
            } else if (*x).preserve_links as libc::c_int != 0 && !(*x).hard_link
                && ((1 as libc::c_int as libc::c_ulong) < src_sb.st_nlink
                    || command_line_arg as libc::c_int != 0
                        && (*x).dereference as libc::c_uint
                            == DEREF_COMMAND_LINE_ARGUMENTS as libc::c_int
                                as libc::c_uint
                    || (*x).dereference as libc::c_uint
                        == DEREF_ALWAYS as libc::c_int as libc::c_uint)
            {
                earlier_file = remember_copied(dst_name, src_sb.st_ino, src_sb.st_dev);
            }
            if !earlier_file.is_null() {
                if src_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o40000 as libc::c_int as libc::c_uint
                {
                    if same_name(src_name, earlier_file) {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"cannot copy a directory, %s, into itself, %s\0"
                                    as *const u8 as *const libc::c_char,
                            ),
                            quotearg_n_style(
                                0 as libc::c_int,
                                shell_escape_always_quoting_style,
                                top_level_src_name,
                            ),
                            quotearg_n_style(
                                1 as libc::c_int,
                                shell_escape_always_quoting_style,
                                top_level_dst_name,
                            ),
                        );
                        *copy_into_self = 1 as libc::c_int != 0;
                        current_block = 7284661225219459306;
                    } else if same_name(dst_name, earlier_file) {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"warning: source directory %s specified more than once\0"
                                    as *const u8 as *const libc::c_char,
                            ),
                            quotearg_style(
                                shell_escape_always_quoting_style,
                                top_level_src_name,
                            ),
                        );
                        if (*x).move_mode as libc::c_int != 0
                            && !rename_succeeded.is_null()
                        {
                            *rename_succeeded = 1 as libc::c_int != 0;
                        }
                        return 1 as libc::c_int != 0;
                    } else if (*x).dereference as libc::c_uint
                        == DEREF_ALWAYS as libc::c_int as libc::c_uint
                        || command_line_arg as libc::c_int != 0
                            && (*x).dereference as libc::c_uint
                                == DEREF_COMMAND_LINE_ARGUMENTS as libc::c_int
                                    as libc::c_uint
                    {
                        current_block = 16077153431071379266;
                    } else {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"will not create hard link %s to directory %s\0"
                                    as *const u8 as *const libc::c_char,
                            ),
                            quotearg_n_style(
                                0 as libc::c_int,
                                shell_escape_always_quoting_style,
                                dst_name,
                            ),
                            quotearg_n_style(
                                1 as libc::c_int,
                                shell_escape_always_quoting_style,
                                earlier_file,
                            ),
                        );
                        current_block = 7284661225219459306;
                    }
                } else if !create_hard_link(
                    earlier_file,
                    dst_name,
                    1 as libc::c_int != 0,
                    (*x).verbose,
                    dereference,
                ) {
                    current_block = 7284661225219459306;
                } else {
                    return 1 as libc::c_int != 0
                }
            } else {
                current_block = 16077153431071379266;
            }
            match current_block {
                7284661225219459306 => {}
                _ => {
                    if (*x).move_mode {
                        if rename_errno == 17 as libc::c_int {
                            rename_errno = if rename(src_name, dst_name)
                                == 0 as libc::c_int
                            {
                                0 as libc::c_int
                            } else {
                                *__errno_location()
                            };
                        }
                        if rename_errno == 0 as libc::c_int {
                            if (*x).verbose {
                                printf(
                                    gettext(b"renamed \0" as *const u8 as *const libc::c_char),
                                );
                                emit_verbose(src_name, dst_name, dst_backup);
                            }
                            if !((*x).set_security_context).is_null() {
                                set_file_security_ctx(dst_name, 1 as libc::c_int != 0, x);
                            }
                            if !rename_succeeded.is_null() {
                                *rename_succeeded = 1 as libc::c_int != 0;
                            }
                            if command_line_arg as libc::c_int != 0 && !(*x).last_file {
                                record_file((*x).dest_info, dst_name, &mut src_sb);
                            }
                            return 1 as libc::c_int != 0;
                        }
                        if rename_errno == 22 as libc::c_int {
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                gettext(
                                    b"cannot move %s to a subdirectory of itself, %s\0"
                                        as *const u8 as *const libc::c_char,
                                ),
                                quotearg_n_style(
                                    0 as libc::c_int,
                                    shell_escape_always_quoting_style,
                                    top_level_src_name,
                                ),
                                quotearg_n_style(
                                    1 as libc::c_int,
                                    shell_escape_always_quoting_style,
                                    top_level_dst_name,
                                ),
                            );
                            *copy_into_self = 1 as libc::c_int != 0;
                            return 1 as libc::c_int != 0;
                        }
                        if rename_errno != 18 as libc::c_int {
                            error(
                                0 as libc::c_int,
                                rename_errno,
                                gettext(
                                    b"cannot move %s to %s\0" as *const u8
                                        as *const libc::c_char,
                                ),
                                quotearg_n_style(
                                    0 as libc::c_int,
                                    shell_escape_always_quoting_style,
                                    src_name,
                                ),
                                quotearg_n_style(
                                    1 as libc::c_int,
                                    shell_escape_always_quoting_style,
                                    dst_name,
                                ),
                            );
                            forget_created(src_sb.st_ino, src_sb.st_dev);
                            return 0 as libc::c_int != 0;
                        }
                        if (if src_mode & 0o170000 as libc::c_int as libc::c_uint
                            == 0o40000 as libc::c_int as libc::c_uint
                        {
                            rmdir(dst_name)
                        } else {
                            unlink(dst_name)
                        }) != 0 as libc::c_int && *__errno_location() != 2 as libc::c_int
                        {
                            error(
                                0 as libc::c_int,
                                *__errno_location(),
                                gettext(
                                    b"inter-device move failed: %s to %s; unable to remove target\0"
                                        as *const u8 as *const libc::c_char,
                                ),
                                quotearg_n_style(
                                    0 as libc::c_int,
                                    shell_escape_always_quoting_style,
                                    src_name,
                                ),
                                quotearg_n_style(
                                    1 as libc::c_int,
                                    shell_escape_always_quoting_style,
                                    dst_name,
                                ),
                            );
                            forget_created(src_sb.st_ino, src_sb.st_dev);
                            return 0 as libc::c_int != 0;
                        }
                        if (*x).verbose as libc::c_int != 0
                            && !(src_mode & 0o170000 as libc::c_int as libc::c_uint
                                == 0o40000 as libc::c_int as libc::c_uint)
                        {
                            printf(
                                gettext(b"copied \0" as *const u8 as *const libc::c_char),
                            );
                            emit_verbose(src_name, dst_name, dst_backup);
                        }
                        new_dst = 1 as libc::c_int != 0;
                    }
                    dst_mode_bits = (if (*x).set_mode as libc::c_int != 0 {
                        (*x).mode
                    } else {
                        src_mode
                    })
                        & (0o4000 as libc::c_int | 0o2000 as libc::c_int
                            | 0o1000 as libc::c_int
                            | (0o400 as libc::c_int | 0o200 as libc::c_int
                                | 0o100 as libc::c_int)
                            | (0o400 as libc::c_int | 0o200 as libc::c_int
                                | 0o100 as libc::c_int) >> 3 as libc::c_int
                            | (0o400 as libc::c_int | 0o200 as libc::c_int
                                | 0o100 as libc::c_int) >> 3 as libc::c_int
                                >> 3 as libc::c_int) as libc::c_uint;
                    omitted_permissions = dst_mode_bits
                        & (if (*x).preserve_ownership as libc::c_int != 0 {
                            (0o400 as libc::c_int | 0o200 as libc::c_int
                                | 0o100 as libc::c_int) >> 3 as libc::c_int
                                | (0o400 as libc::c_int | 0o200 as libc::c_int
                                    | 0o100 as libc::c_int) >> 3 as libc::c_int
                                    >> 3 as libc::c_int
                        } else {
                            (if src_mode & 0o170000 as libc::c_int as libc::c_uint
                                == 0o40000 as libc::c_int as libc::c_uint
                            {
                                0o200 as libc::c_int >> 3 as libc::c_int
                                    | 0o200 as libc::c_int >> 3 as libc::c_int
                                        >> 3 as libc::c_int
                            } else {
                                0 as libc::c_int
                            })
                        }) as libc::c_uint;
                    delayed_ok = 1 as libc::c_int != 0;
                    if !set_process_security_ctx(
                        src_name,
                        dst_name,
                        src_mode,
                        new_dst,
                        x,
                    ) {
                        return 0 as libc::c_int != 0;
                    }
                    if src_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint
                    {
                        let mut dir: *mut dir_list = 0 as *mut dir_list;
                        if is_ancestor(&mut src_sb, ancestors) {
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                gettext(
                                    b"cannot copy cyclic symbolic link %s\0" as *const u8
                                        as *const libc::c_char,
                                ),
                                quotearg_style(shell_escape_always_quoting_style, src_name),
                            );
                            current_block = 7284661225219459306;
                        } else {
                            let mut fresh1 = ::std::vec::from_elem(
                                0,
                                ::core::mem::size_of::<dir_list>() as libc::c_ulong as usize,
                            );
                            dir = fresh1.as_mut_ptr() as *mut dir_list;
                            (*dir).parent = ancestors;
                            (*dir).ino = src_sb.st_ino;
                            (*dir).dev = src_sb.st_dev;
                            if new_dst as libc::c_int != 0
                                || !(dst_sb.st_mode
                                    & 0o170000 as libc::c_int as libc::c_uint
                                    == 0o40000 as libc::c_int as libc::c_uint)
                            {
                                if mkdir(dst_name, dst_mode_bits & !omitted_permissions)
                                    != 0 as libc::c_int
                                {
                                    error(
                                        0 as libc::c_int,
                                        *__errno_location(),
                                        gettext(
                                            b"cannot create directory %s\0" as *const u8
                                                as *const libc::c_char,
                                        ),
                                        quotearg_style(shell_escape_always_quoting_style, dst_name),
                                    );
                                    current_block = 7284661225219459306;
                                } else if lstat(dst_name, &mut dst_sb) != 0 as libc::c_int {
                                    error(
                                        0 as libc::c_int,
                                        *__errno_location(),
                                        gettext(
                                            b"cannot stat %s\0" as *const u8 as *const libc::c_char,
                                        ),
                                        quotearg_style(shell_escape_always_quoting_style, dst_name),
                                    );
                                    current_block = 7284661225219459306;
                                } else {
                                    if dst_sb.st_mode
                                        & (0o400 as libc::c_int | 0o200 as libc::c_int
                                            | 0o100 as libc::c_int) as libc::c_uint
                                        != (0o400 as libc::c_int | 0o200 as libc::c_int
                                            | 0o100 as libc::c_int) as libc::c_uint
                                    {
                                        dst_mode = dst_sb.st_mode;
                                        restore_dst_mode = 1 as libc::c_int != 0;
                                        if lchmod(
                                            dst_name,
                                            dst_mode
                                                | (0o400 as libc::c_int | 0o200 as libc::c_int
                                                    | 0o100 as libc::c_int) as libc::c_uint,
                                        ) != 0 as libc::c_int
                                        {
                                            error(
                                                0 as libc::c_int,
                                                *__errno_location(),
                                                gettext(
                                                    b"setting permissions for %s\0" as *const u8
                                                        as *const libc::c_char,
                                                ),
                                                quotearg_style(shell_escape_always_quoting_style, dst_name),
                                            );
                                            current_block = 7284661225219459306;
                                        } else {
                                            current_block = 13262925809421758307;
                                        }
                                    } else {
                                        current_block = 13262925809421758307;
                                    }
                                    match current_block {
                                        7284661225219459306 => {}
                                        _ => {
                                            if !*first_dir_created_per_command_line_arg {
                                                remember_copied(dst_name, dst_sb.st_ino, dst_sb.st_dev);
                                                *first_dir_created_per_command_line_arg = 1 as libc::c_int
                                                    != 0;
                                            }
                                            if (*x).verbose {
                                                if (*x).move_mode {
                                                    printf(
                                                        gettext(
                                                            b"created directory %s\n\0" as *const u8
                                                                as *const libc::c_char,
                                                        ),
                                                        quotearg_style(shell_escape_always_quoting_style, dst_name),
                                                    );
                                                } else {
                                                    emit_verbose(src_name, dst_name, 0 as *const libc::c_char);
                                                }
                                            }
                                            current_block = 16185292562584120790;
                                        }
                                    }
                                }
                            } else {
                                omitted_permissions = 0 as libc::c_int as mode_t;
                                if !((*x).set_security_context).is_null()
                                    || (*x).preserve_security_context as libc::c_int != 0
                                {
                                    if !set_file_security_ctx(
                                        dst_name,
                                        0 as libc::c_int != 0,
                                        x,
                                    ) {
                                        if (*x).require_preserve_context {
                                            current_block = 7284661225219459306;
                                        } else {
                                            current_block = 16185292562584120790;
                                        }
                                    } else {
                                        current_block = 16185292562584120790;
                                    }
                                } else {
                                    current_block = 16185292562584120790;
                                }
                            }
                            match current_block {
                                7284661225219459306 => {}
                                _ => {
                                    if !((*x).one_file_system as libc::c_int != 0
                                        && !parent.is_null() && (*parent).st_dev != src_sb.st_dev)
                                    {
                                        delayed_ok = copy_dir(
                                            src_name,
                                            dst_name,
                                            new_dst,
                                            &mut src_sb,
                                            dir,
                                            x,
                                            first_dir_created_per_command_line_arg,
                                            copy_into_self,
                                        );
                                    }
                                    current_block = 7986280648684494582;
                                }
                            }
                        }
                    } else if (*x).symbolic_link {
                        dest_is_symlink = 1 as libc::c_int != 0;
                        if *src_name as libc::c_int != '/' as i32 {
                            let mut dot_sb: stat = stat {
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
                            let mut dst_parent_sb: stat = stat {
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
                            let mut dst_parent: *mut libc::c_char = 0
                                as *mut libc::c_char;
                            let mut in_current_dir: bool = false;
                            dst_parent = dir_name(dst_name);
                            in_current_dir = strcmp(
                                b".\0" as *const u8 as *const libc::c_char,
                                dst_parent,
                            ) == 0 as libc::c_int
                                || stat(
                                    b".\0" as *const u8 as *const libc::c_char,
                                    &mut dot_sb,
                                ) != 0 as libc::c_int
                                || stat(dst_parent, &mut dst_parent_sb) != 0 as libc::c_int
                                || dot_sb.st_ino == dst_parent_sb.st_ino
                                    && dot_sb.st_dev == dst_parent_sb.st_dev;
                            free(dst_parent as *mut libc::c_void);
                            if !in_current_dir {
                                error(
                                    0 as libc::c_int,
                                    0 as libc::c_int,
                                    gettext(
                                        b"%s: can make relative symbolic links only in current directory\0"
                                            as *const u8 as *const libc::c_char,
                                    ),
                                    quotearg_n_style_colon(
                                        0 as libc::c_int,
                                        shell_escape_quoting_style,
                                        dst_name,
                                    ),
                                );
                                current_block = 7284661225219459306;
                            } else {
                                current_block = 3173376877683785480;
                            }
                        } else {
                            current_block = 3173376877683785480;
                        }
                        match current_block {
                            7284661225219459306 => {}
                            _ => {
                                let mut err: libc::c_int = force_symlinkat(
                                    src_name,
                                    -(100 as libc::c_int),
                                    dst_name,
                                    (*x).unlink_dest_after_failed_open,
                                    -(1 as libc::c_int),
                                );
                                if (0 as libc::c_int) < err {
                                    error(
                                        0 as libc::c_int,
                                        err,
                                        gettext(
                                            b"cannot create symbolic link %s to %s\0" as *const u8
                                                as *const libc::c_char,
                                        ),
                                        quotearg_n_style(
                                            0 as libc::c_int,
                                            shell_escape_always_quoting_style,
                                            dst_name,
                                        ),
                                        quotearg_n_style(
                                            1 as libc::c_int,
                                            shell_escape_always_quoting_style,
                                            src_name,
                                        ),
                                    );
                                    current_block = 7284661225219459306;
                                } else {
                                    current_block = 7986280648684494582;
                                }
                            }
                        }
                    } else if (*x).hard_link as libc::c_int != 0
                        && !(1 as libc::c_int == 0
                            && src_mode & 0o170000 as libc::c_int as libc::c_uint
                                == 0o120000 as libc::c_int as libc::c_uint
                            && (*x).dereference as libc::c_uint
                                == DEREF_NEVER as libc::c_int as libc::c_uint)
                    {
                        let mut replace: bool = (*x).unlink_dest_after_failed_open
                            as libc::c_int != 0
                            || (*x).interactive as libc::c_uint
                                == I_ASK_USER as libc::c_int as libc::c_uint;
                        if !create_hard_link(
                            src_name,
                            dst_name,
                            replace,
                            0 as libc::c_int != 0,
                            dereference,
                        ) {
                            current_block = 7284661225219459306;
                        } else {
                            current_block = 7986280648684494582;
                        }
                    } else if src_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o100000 as libc::c_int as libc::c_uint
                        || (*x).copy_as_regular as libc::c_int != 0
                            && !(src_mode & 0o170000 as libc::c_int as libc::c_uint
                                == 0o120000 as libc::c_int as libc::c_uint)
                    {
                        copied_as_regular = 1 as libc::c_int != 0;
                        if !copy_reg(
                            src_name,
                            dst_name,
                            x,
                            dst_mode_bits
                                & (0o400 as libc::c_int | 0o200 as libc::c_int
                                    | 0o100 as libc::c_int
                                    | (0o400 as libc::c_int | 0o200 as libc::c_int
                                        | 0o100 as libc::c_int) >> 3 as libc::c_int
                                    | (0o400 as libc::c_int | 0o200 as libc::c_int
                                        | 0o100 as libc::c_int) >> 3 as libc::c_int
                                        >> 3 as libc::c_int) as libc::c_uint,
                            omitted_permissions,
                            &mut new_dst,
                            &mut src_sb,
                        ) {
                            current_block = 7284661225219459306;
                        } else {
                            current_block = 7986280648684494582;
                        }
                    } else if src_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o10000 as libc::c_int as libc::c_uint
                    {
                        if mknod(
                            dst_name,
                            src_mode & !omitted_permissions,
                            0 as libc::c_int as __dev_t,
                        ) != 0 as libc::c_int
                        {
                            if mkfifo(
                                dst_name,
                                src_mode & !(0o10000 as libc::c_int) as libc::c_uint
                                    & !omitted_permissions,
                            ) != 0 as libc::c_int
                            {
                                error(
                                    0 as libc::c_int,
                                    *__errno_location(),
                                    gettext(
                                        b"cannot create fifo %s\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                    quotearg_style(shell_escape_always_quoting_style, dst_name),
                                );
                                current_block = 7284661225219459306;
                            } else {
                                current_block = 7986280648684494582;
                            }
                        } else {
                            current_block = 7986280648684494582;
                        }
                    } else if src_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o60000 as libc::c_int as libc::c_uint
                        || src_mode & 0o170000 as libc::c_int as libc::c_uint
                            == 0o20000 as libc::c_int as libc::c_uint
                        || src_mode & 0o170000 as libc::c_int as libc::c_uint
                            == 0o140000 as libc::c_int as libc::c_uint
                    {
                        if mknod(
                            dst_name,
                            src_mode & !omitted_permissions,
                            src_sb.st_rdev,
                        ) != 0 as libc::c_int
                        {
                            error(
                                0 as libc::c_int,
                                *__errno_location(),
                                gettext(
                                    b"cannot create special file %s\0" as *const u8
                                        as *const libc::c_char,
                                ),
                                quotearg_style(shell_escape_always_quoting_style, dst_name),
                            );
                            current_block = 7284661225219459306;
                        } else {
                            current_block = 7986280648684494582;
                        }
                    } else if src_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o120000 as libc::c_int as libc::c_uint
                    {
                        let mut src_link_val: *mut libc::c_char = areadlink_with_size(
                            src_name,
                            src_sb.st_size as size_t,
                        );
                        dest_is_symlink = 1 as libc::c_int != 0;
                        if src_link_val.is_null() {
                            error(
                                0 as libc::c_int,
                                *__errno_location(),
                                gettext(
                                    b"cannot read symbolic link %s\0" as *const u8
                                        as *const libc::c_char,
                                ),
                                quotearg_style(shell_escape_always_quoting_style, src_name),
                            );
                            current_block = 7284661225219459306;
                        } else {
                            let mut symlink_err: libc::c_int = force_symlinkat(
                                src_link_val,
                                -(100 as libc::c_int),
                                dst_name,
                                (*x).unlink_dest_after_failed_open,
                                -(1 as libc::c_int),
                            );
                            if (0 as libc::c_int) < symlink_err
                                && (*x).update as libc::c_int != 0 && !new_dst
                                && dst_sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
                                    == 0o120000 as libc::c_int as libc::c_uint
                                && dst_sb.st_size as libc::c_ulong == strlen(src_link_val)
                            {
                                let mut dest_link_val: *mut libc::c_char = areadlink_with_size(
                                    dst_name,
                                    dst_sb.st_size as size_t,
                                );
                                if !dest_link_val.is_null() {
                                    if strcmp(dest_link_val, src_link_val) == 0 as libc::c_int {
                                        symlink_err = 0 as libc::c_int;
                                    }
                                    free(dest_link_val as *mut libc::c_void);
                                }
                            }
                            free(src_link_val as *mut libc::c_void);
                            if (0 as libc::c_int) < symlink_err {
                                error(
                                    0 as libc::c_int,
                                    symlink_err,
                                    gettext(
                                        b"cannot create symbolic link %s\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                    quotearg_style(shell_escape_always_quoting_style, dst_name),
                                );
                                current_block = 7284661225219459306;
                            } else {
                                if (*x).preserve_security_context {
                                    restore_default_fscreatecon_or_die();
                                }
                                if (*x).preserve_ownership {
                                    if 1 as libc::c_int != 0
                                        && lchown(dst_name, src_sb.st_uid, src_sb.st_gid)
                                            != 0 as libc::c_int && !chown_failure_ok(x)
                                    {
                                        error(
                                            0 as libc::c_int,
                                            *__errno_location(),
                                            gettext(
                                                b"failed to preserve ownership for %s\0" as *const u8
                                                    as *const libc::c_char,
                                            ),
                                            dst_name,
                                        );
                                        if (*x).require_preserve {
                                            current_block = 7284661225219459306;
                                        } else {
                                            current_block = 7986280648684494582;
                                        }
                                    } else {
                                        current_block = 7986280648684494582;
                                    }
                                } else {
                                    current_block = 7986280648684494582;
                                }
                            }
                        }
                    } else {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"%s has unknown file type\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            quotearg_style(shell_escape_always_quoting_style, src_name),
                        );
                        current_block = 7284661225219459306;
                    }
                    match current_block {
                        7284661225219459306 => {}
                        _ => {
                            if !new_dst && !(*x).copy_as_regular
                                && !(src_mode & 0o170000 as libc::c_int as libc::c_uint
                                    == 0o40000 as libc::c_int as libc::c_uint)
                                && (!((*x).set_security_context).is_null()
                                    || (*x).preserve_security_context as libc::c_int != 0)
                            {
                                if !set_file_security_ctx(
                                    dst_name,
                                    0 as libc::c_int != 0,
                                    x,
                                ) {
                                    if (*x).require_preserve_context {
                                        current_block = 7284661225219459306;
                                    } else {
                                        current_block = 9957735944531162038;
                                    }
                                } else {
                                    current_block = 9957735944531162038;
                                }
                            } else {
                                current_block = 9957735944531162038;
                            }
                            match current_block {
                                7284661225219459306 => {}
                                _ => {
                                    if command_line_arg as libc::c_int != 0
                                        && !((*x).dest_info).is_null()
                                    {
                                        let mut sb: stat = stat {
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
                                        if lstat(dst_name, &mut sb) == 0 as libc::c_int {
                                            record_file((*x).dest_info, dst_name, &mut sb);
                                        }
                                    }
                                    if (*x).hard_link as libc::c_int != 0
                                        && !(src_mode & 0o170000 as libc::c_int as libc::c_uint
                                            == 0o40000 as libc::c_int as libc::c_uint)
                                        && !(1 as libc::c_int == 0
                                            && src_mode & 0o170000 as libc::c_int as libc::c_uint
                                                == 0o120000 as libc::c_int as libc::c_uint
                                            && (*x).dereference as libc::c_uint
                                                == DEREF_NEVER as libc::c_int as libc::c_uint)
                                    {
                                        return delayed_ok;
                                    }
                                    if copied_as_regular {
                                        return delayed_ok;
                                    }
                                    if (*x).preserve_timestamps {
                                        let mut timespec: [timespec; 2] = [timespec {
                                            tv_sec: 0,
                                            tv_nsec: 0,
                                        }; 2];
                                        timespec[0 as libc::c_int
                                            as usize] = get_stat_atime(&mut src_sb);
                                        timespec[1 as libc::c_int
                                            as usize] = get_stat_mtime(&mut src_sb);
                                        if (if dest_is_symlink as libc::c_int != 0 {
                                            utimens_symlink(dst_name, timespec.as_mut_ptr())
                                        } else {
                                            utimens(dst_name, timespec.as_mut_ptr() as *const timespec)
                                        }) != 0 as libc::c_int
                                        {
                                            error(
                                                0 as libc::c_int,
                                                *__errno_location(),
                                                gettext(
                                                    b"preserving times for %s\0" as *const u8
                                                        as *const libc::c_char,
                                                ),
                                                quotearg_style(shell_escape_always_quoting_style, dst_name),
                                            );
                                            if (*x).require_preserve {
                                                return 0 as libc::c_int != 0;
                                            }
                                        }
                                    }
                                    if !dest_is_symlink
                                        && (*x).preserve_ownership as libc::c_int != 0
                                        && (new_dst as libc::c_int != 0
                                            || !(src_sb.st_uid == dst_sb.st_uid
                                                && src_sb.st_gid == dst_sb.st_gid))
                                    {
                                        match set_owner(
                                            x,
                                            dst_name,
                                            -(1 as libc::c_int),
                                            &mut src_sb,
                                            new_dst,
                                            &mut dst_sb,
                                        ) {
                                            -1 => return 0 as libc::c_int != 0,
                                            0 => {
                                                src_mode
                                                    &= !(0o4000 as libc::c_int | 0o2000 as libc::c_int
                                                        | 0o1000 as libc::c_int) as libc::c_uint;
                                            }
                                            _ => {}
                                        }
                                    }
                                    if (*x).preserve_xattr as libc::c_int != 0
                                        && !copy_attr(
                                            src_name,
                                            -(1 as libc::c_int),
                                            dst_name,
                                            -(1 as libc::c_int),
                                            x,
                                        ) && (*x).require_preserve_xattr as libc::c_int != 0
                                    {
                                        return 0 as libc::c_int != 0;
                                    }
                                    if dest_is_symlink {
                                        return delayed_ok;
                                    }
                                    set_author(dst_name, -(1 as libc::c_int), &mut src_sb);
                                    if (*x).preserve_mode as libc::c_int != 0
                                        || (*x).move_mode as libc::c_int != 0
                                    {
                                        if copy_acl(
                                            src_name,
                                            -(1 as libc::c_int),
                                            dst_name,
                                            -(1 as libc::c_int),
                                            src_mode,
                                        ) != 0 as libc::c_int
                                            && (*x).require_preserve as libc::c_int != 0
                                        {
                                            return 0 as libc::c_int != 0;
                                        }
                                    } else if (*x).set_mode {
                                        if set_acl(dst_name, -(1 as libc::c_int), (*x).mode)
                                            != 0 as libc::c_int
                                        {
                                            return 0 as libc::c_int != 0;
                                        }
                                    } else if (*x).explicit_no_preserve_mode as libc::c_int != 0
                                        && new_dst as libc::c_int != 0
                                    {
                                        let mut default_permissions: libc::c_int = if src_mode
                                            & 0o170000 as libc::c_int as libc::c_uint
                                            == 0o40000 as libc::c_int as libc::c_uint
                                            || src_mode & 0o170000 as libc::c_int as libc::c_uint
                                                == 0o140000 as libc::c_int as libc::c_uint
                                        {
                                            0o400 as libc::c_int | 0o200 as libc::c_int
                                                | 0o100 as libc::c_int
                                                | (0o400 as libc::c_int | 0o200 as libc::c_int
                                                    | 0o100 as libc::c_int) >> 3 as libc::c_int
                                                | (0o400 as libc::c_int | 0o200 as libc::c_int
                                                    | 0o100 as libc::c_int) >> 3 as libc::c_int
                                                    >> 3 as libc::c_int
                                        } else {
                                            0o400 as libc::c_int | 0o200 as libc::c_int
                                                | 0o400 as libc::c_int >> 3 as libc::c_int
                                                | 0o200 as libc::c_int >> 3 as libc::c_int
                                                | 0o400 as libc::c_int >> 3 as libc::c_int
                                                    >> 3 as libc::c_int
                                                | 0o200 as libc::c_int >> 3 as libc::c_int
                                                    >> 3 as libc::c_int
                                        };
                                        if set_acl(
                                            dst_name,
                                            -(1 as libc::c_int),
                                            default_permissions as libc::c_uint & !cached_umask(),
                                        ) != 0 as libc::c_int
                                        {
                                            return 0 as libc::c_int != 0;
                                        }
                                    } else {
                                        if omitted_permissions != 0 {
                                            omitted_permissions &= !cached_umask();
                                            if omitted_permissions != 0 && !restore_dst_mode {
                                                if new_dst as libc::c_int != 0
                                                    && lstat(dst_name, &mut dst_sb) != 0 as libc::c_int
                                                {
                                                    error(
                                                        0 as libc::c_int,
                                                        *__errno_location(),
                                                        gettext(
                                                            b"cannot stat %s\0" as *const u8 as *const libc::c_char,
                                                        ),
                                                        quotearg_style(shell_escape_always_quoting_style, dst_name),
                                                    );
                                                    return 0 as libc::c_int != 0;
                                                }
                                                dst_mode = dst_sb.st_mode;
                                                if omitted_permissions & !dst_mode != 0 {
                                                    restore_dst_mode = 1 as libc::c_int != 0;
                                                }
                                            }
                                        }
                                        if restore_dst_mode {
                                            if lchmod(dst_name, dst_mode | omitted_permissions)
                                                != 0 as libc::c_int
                                            {
                                                error(
                                                    0 as libc::c_int,
                                                    *__errno_location(),
                                                    gettext(
                                                        b"preserving permissions for %s\0" as *const u8
                                                            as *const libc::c_char,
                                                    ),
                                                    quotearg_style(shell_escape_always_quoting_style, dst_name),
                                                );
                                                if (*x).require_preserve {
                                                    return 0 as libc::c_int != 0;
                                                }
                                            }
                                        }
                                    }
                                    return delayed_ok;
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if (*x).preserve_security_context {
        restore_default_fscreatecon_or_die();
    }
    if earlier_file.is_null() {
        forget_created(src_sb.st_ino, src_sb.st_dev);
    }
    if !dst_backup.is_null() {
        if rename(dst_backup, dst_name) != 0 as libc::c_int {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(b"cannot un-backup %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, dst_name),
            );
        } else if (*x).verbose {
            printf(
                gettext(b"%s -> %s (unbackup)\n\0" as *const u8 as *const libc::c_char),
                quotearg_n_style(
                    0 as libc::c_int,
                    shell_escape_always_quoting_style,
                    dst_backup,
                ),
                quotearg_n_style(
                    1 as libc::c_int,
                    shell_escape_always_quoting_style,
                    dst_name,
                ),
            );
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn owner_failure_ok(mut x: *const cp_options) -> bool {
    return (*__errno_location() == 1 as libc::c_int
        || *__errno_location() == 22 as libc::c_int) && !(*x).owner_privileges;
}
static mut top_level_src_name: *const libc::c_char = 0 as *const libc::c_char;
static mut top_level_dst_name: *const libc::c_char = 0 as *const libc::c_char;
unsafe extern "C" fn follow_fstatat(
    mut dirfd: libc::c_int,
    mut filename: *const libc::c_char,
    mut st: *mut stat,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = fstatat(dirfd, filename, st, flags);
    if 0 as libc::c_int != 0 && result == 0 as libc::c_int
        && flags & 0x100 as libc::c_int == 0
        && (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o20000 as libc::c_int as libc::c_uint
    {
        static mut stdin_rdev: dev_t = 0;
        static mut stdin_rdev_status: libc::c_schar = 0;
        if stdin_rdev_status as libc::c_int == 0 as libc::c_int {
            let mut stdin_st: stat = stat {
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
            if stat(b"/dev/stdin\0" as *const u8 as *const libc::c_char, &mut stdin_st)
                == 0 as libc::c_int
                && stdin_st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o20000 as libc::c_int as libc::c_uint
                && gnu_dev_minor(stdin_st.st_rdev) == 0 as libc::c_int as libc::c_uint
            {
                stdin_rdev = stdin_st.st_rdev;
                stdin_rdev_status = 1 as libc::c_int as libc::c_schar;
            } else {
                stdin_rdev_status = -(1 as libc::c_int) as libc::c_schar;
            }
        }
        if (0 as libc::c_int) < stdin_rdev_status as libc::c_int
            && gnu_dev_major(stdin_rdev) == gnu_dev_major((*st).st_rdev)
        {
            result = fstat(gnu_dev_minor((*st).st_rdev) as libc::c_int, st);
        }
    }
    return result;
}
#[inline]
unsafe extern "C" fn utimens_symlink(
    mut file: *const libc::c_char,
    mut timespec: *const timespec,
) -> libc::c_int {
    let mut err: libc::c_int = lutimens(file, timespec);
    if err != 0 && *__errno_location() == 38 as libc::c_int {
        err = 0 as libc::c_int;
    }
    return err;
}
unsafe extern "C" fn punch_hole(
    mut fd: libc::c_int,
    mut offset: off_t,
    mut length: off_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    ret = fallocate(fd, 0x2 as libc::c_int | 0x1 as libc::c_int, offset, length);
    if ret < 0 as libc::c_int
        && (is_ENOTSUP(*__errno_location()) as libc::c_int != 0
            || *__errno_location() == 38 as libc::c_int)
    {
        ret = 0 as libc::c_int;
    }
    return ret;
}
unsafe extern "C" fn create_hole(
    mut fd: libc::c_int,
    mut name: *const libc::c_char,
    mut punch_holes: bool,
    mut size: off_t,
) -> bool {
    let mut file_end: off_t = lseek(fd, size, 1 as libc::c_int);
    if file_end < 0 as libc::c_int as libc::c_long {
        error(
            0 as libc::c_int,
            *__errno_location(),
            gettext(b"cannot lseek %s\0" as *const u8 as *const libc::c_char),
            quotearg_style(shell_escape_always_quoting_style, name),
        );
        return 0 as libc::c_int != 0;
    }
    if punch_holes as libc::c_int != 0
        && punch_hole(fd, file_end - size, size) < 0 as libc::c_int
    {
        error(
            0 as libc::c_int,
            *__errno_location(),
            gettext(b"error deallocating %s\0" as *const u8 as *const libc::c_char),
            quotearg_style(shell_escape_always_quoting_style, name),
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn functional_copy_file_range() -> bool {
    static mut version_allowed: libc::c_int = -(1 as libc::c_int);
    if version_allowed == -(1 as libc::c_int) {
        version_allowed = 0 as libc::c_int;
    } else {
        return version_allowed != 0
    }
    let mut name: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };
    if uname(&mut name) == -(1 as libc::c_int) {
        return version_allowed != 0;
    }
    let mut p: *mut libc::c_char = (name.release).as_mut_ptr();
    let mut ver: [uintmax_t; 2] = [
        0 as libc::c_int as uintmax_t,
        0 as libc::c_int as uintmax_t,
    ];
    let mut iver: size_t = 0 as libc::c_int as size_t;
    loop {
        let mut err: strtol_error = xstrtoumax(
            p,
            &mut p,
            10 as libc::c_int,
            &mut *ver.as_mut_ptr().offset(iver as isize),
            0 as *const libc::c_char,
        );
        if err as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint
            || {
                let fresh2 = p;
                p = p.offset(1);
                *fresh2 as libc::c_int != '.' as i32
            }
        {
            break;
        }
        iver = iver.wrapping_add(1);
        if !(iver
            < (::core::mem::size_of::<[uintmax_t; 2]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<uintmax_t>() as libc::c_ulong))
        {
            break;
        }
    }
    version_allowed = (ver[0 as libc::c_int as usize] > 5 as libc::c_int as libc::c_ulong
        || ver[0 as libc::c_int as usize] == 5 as libc::c_int as libc::c_ulong
            && ver[1 as libc::c_int as usize] >= 3 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    return version_allowed != 0;
}
unsafe extern "C" fn sparse_copy(
    mut src_fd: libc::c_int,
    mut dest_fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut buf_size: size_t,
    mut hole_size: size_t,
    mut punch_holes: bool,
    mut allow_reflink: bool,
    mut src_name: *const libc::c_char,
    mut dst_name: *const libc::c_char,
    mut max_n_read: uintmax_t,
    mut total_n_read: *mut off_t,
    mut last_write_made_hole: *mut bool,
) -> bool {
    *last_write_made_hole = 0 as libc::c_int != 0;
    *total_n_read = 0 as libc::c_int as off_t;
    if hole_size == 0 && allow_reflink as libc::c_int != 0
        && functional_copy_file_range() as libc::c_int != 0
    {
        while max_n_read != 0 {
            let mut ssize_max: ssize_t = if (0 as libc::c_int as ssize_t)
                < -(1 as libc::c_int) as ssize_t
            {
                -(1 as libc::c_int) as ssize_t
            } else {
                (((1 as libc::c_int as ssize_t)
                    << (::core::mem::size_of::<ssize_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            };
            let mut copy_max: ptrdiff_t = (((if (ssize_max as libc::c_ulong)
                < 18446744073709551615 as libc::c_ulong
            {
                ssize_max as libc::c_ulong
            } else {
                18446744073709551615 as libc::c_ulong
            }) >> 30 as libc::c_int) << 30 as libc::c_int) as ptrdiff_t;
            let mut n_copied: ssize_t = copy_file_range(
                src_fd,
                0 as *mut __off64_t,
                dest_fd,
                0 as *mut __off64_t,
                if max_n_read < copy_max as libc::c_ulong {
                    max_n_read
                } else {
                    copy_max as libc::c_ulong
                },
                0 as libc::c_int as libc::c_uint,
            );
            if n_copied == 0 as libc::c_int as libc::c_long {
                if *total_n_read == 0 as libc::c_int as libc::c_long {
                    break;
                }
                return 1 as libc::c_int != 0;
            } else {
                if n_copied < 0 as libc::c_int as libc::c_long {
                    if *__errno_location() == 38 as libc::c_int
                        || is_ENOTSUP(*__errno_location()) as libc::c_int != 0
                        || *__errno_location() == 22 as libc::c_int
                        || *__errno_location() == 9 as libc::c_int
                        || *__errno_location() == 18 as libc::c_int
                        || *__errno_location() == 26 as libc::c_int
                    {
                        break;
                    }
                    if *__errno_location() == 1 as libc::c_int
                        && *total_n_read == 0 as libc::c_int as libc::c_long
                    {
                        break;
                    }
                    if *__errno_location() == 4 as libc::c_int {
                        n_copied = 0 as libc::c_int as ssize_t;
                    } else {
                        error(
                            0 as libc::c_int,
                            *__errno_location(),
                            gettext(
                                b"error copying %s to %s\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            quotearg_n_style(
                                0 as libc::c_int,
                                shell_escape_always_quoting_style,
                                src_name,
                            ),
                            quotearg_n_style(
                                1 as libc::c_int,
                                shell_escape_always_quoting_style,
                                dst_name,
                            ),
                        );
                        return 0 as libc::c_int != 0;
                    }
                }
                max_n_read = (max_n_read as libc::c_ulong)
                    .wrapping_sub(n_copied as libc::c_ulong) as uintmax_t as uintmax_t;
                *total_n_read += n_copied;
            }
        }
    }
    let mut make_hole: bool = 0 as libc::c_int != 0;
    let mut psize: off_t = 0 as libc::c_int as off_t;
    while max_n_read != 0 {
        let mut n_read: ssize_t = read(
            src_fd,
            buf as *mut libc::c_void,
            if max_n_read < buf_size { max_n_read } else { buf_size },
        );
        if n_read < 0 as libc::c_int as libc::c_long {
            if *__errno_location() == 4 as libc::c_int {
                continue;
            }
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, src_name),
            );
            return 0 as libc::c_int != 0;
        } else {
            if n_read == 0 as libc::c_int as libc::c_long {
                break;
            }
            max_n_read = (max_n_read as libc::c_ulong)
                .wrapping_sub(n_read as libc::c_ulong) as uintmax_t as uintmax_t;
            *total_n_read += n_read;
            let mut csize: size_t = if hole_size != 0 { hole_size } else { buf_size };
            let mut cbuf: *mut libc::c_char = buf;
            let mut pbuf: *mut libc::c_char = buf;
            while n_read != 0 {
                let mut prev_hole: bool = make_hole;
                csize = if csize < n_read as libc::c_ulong {
                    csize
                } else {
                    n_read as libc::c_ulong
                };
                if hole_size != 0 && csize != 0 {
                    make_hole = is_nul(cbuf as *const libc::c_void, csize);
                }
                let mut transition: bool = make_hole as libc::c_int
                    != prev_hole as libc::c_int && psize != 0;
                let mut last_chunk: bool = n_read as libc::c_ulong == csize && !make_hole
                    || csize == 0;
                if transition as libc::c_int != 0 || last_chunk as libc::c_int != 0 {
                    if !transition {
                        psize = (psize as libc::c_ulong).wrapping_add(csize) as off_t
                            as off_t;
                    }
                    if !prev_hole {
                        if full_write(
                            dest_fd,
                            pbuf as *const libc::c_void,
                            psize as size_t,
                        ) != psize as libc::c_ulong
                        {
                            error(
                                0 as libc::c_int,
                                *__errno_location(),
                                gettext(
                                    b"error writing %s\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(shell_escape_always_quoting_style, dst_name),
                            );
                            return 0 as libc::c_int != 0;
                        }
                    } else if !create_hole(dest_fd, dst_name, punch_holes, psize) {
                        return 0 as libc::c_int != 0
                    }
                    pbuf = cbuf;
                    psize = csize as off_t;
                    if last_chunk {
                        if csize == 0 {
                            n_read = 0 as libc::c_int as ssize_t;
                        }
                        if transition {
                            csize = 0 as libc::c_int as size_t;
                        } else {
                            psize = 0 as libc::c_int as off_t;
                        }
                    }
                } else {
                    let (fresh3, fresh4) = psize.overflowing_add(csize);
                    *(&mut psize as *mut off_t) = fresh3;
                    if fresh4 {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"overflow reading %s\0" as *const u8 as *const libc::c_char,
                            ),
                            quotearg_style(shell_escape_always_quoting_style, src_name),
                        );
                        return 0 as libc::c_int != 0;
                    }
                }
                n_read = (n_read as libc::c_ulong).wrapping_sub(csize) as ssize_t
                    as ssize_t;
                cbuf = cbuf.offset(csize as isize);
            }
            *last_write_made_hole = make_hole;
        }
    }
    if make_hole as libc::c_int != 0
        && !create_hole(dest_fd, dst_name, punch_holes, psize)
    {
        return 0 as libc::c_int != 0
    } else {
        return 1 as libc::c_int != 0
    };
}
#[inline]
unsafe extern "C" fn clone_file(
    mut dest_fd: libc::c_int,
    mut src_fd: libc::c_int,
) -> libc::c_int {
    return ioctl(
        dest_fd,
        ((1 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | ((0x94 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int)
                as libc::c_uint
            | ((9 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        src_fd,
    );
}
unsafe extern "C" fn write_zeros(mut fd: libc::c_int, mut n_bytes: off_t) -> bool {
    static mut zeros: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut nz: size_t = IO_BUFSIZE as libc::c_int as size_t;
    if zeros.is_null() {
        static mut fallback: [libc::c_char; 1024] = [0; 1024];
        zeros = calloc(nz, 1 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
        if zeros.is_null() {
            zeros = fallback.as_mut_ptr();
            nz = ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong;
        }
    }
    while n_bytes != 0 {
        let mut n: size_t = if nz < n_bytes as libc::c_ulong {
            nz
        } else {
            n_bytes as libc::c_ulong
        };
        if full_write(fd, zeros as *const libc::c_void, n) != n {
            return 0 as libc::c_int != 0;
        }
        n_bytes = (n_bytes as libc::c_ulong).wrapping_sub(n) as off_t as off_t;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn lseek_copy(
    mut src_fd: libc::c_int,
    mut dest_fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut buf_size: size_t,
    mut hole_size: size_t,
    mut ext_start: off_t,
    mut src_total_size: off_t,
    mut sparse_mode: Sparse_type,
    mut allow_reflink: bool,
    mut src_name: *const libc::c_char,
    mut dst_name: *const libc::c_char,
) -> bool {
    let mut current_block: u64;
    let mut last_ext_start: off_t = 0 as libc::c_int as off_t;
    let mut last_ext_len: off_t = 0 as libc::c_int as off_t;
    let mut dest_pos: off_t = 0 as libc::c_int as off_t;
    let mut wrote_hole_at_eof: bool = 1 as libc::c_int != 0;
    loop {
        if !(0 as libc::c_int as libc::c_long <= ext_start) {
            current_block = 9520865839495247062;
            break;
        }
        let mut ext_end: off_t = lseek(src_fd, ext_start, 4 as libc::c_int);
        if ext_end < 0 as libc::c_int as libc::c_long {
            if *__errno_location() != 6 as libc::c_int {
                current_block = 16433050379348534696;
                break;
            }
            ext_end = src_total_size;
            if ext_end <= ext_start {
                src_total_size = lseek(
                    src_fd,
                    0 as libc::c_int as __off_t,
                    2 as libc::c_int,
                );
                if src_total_size < 0 as libc::c_int as libc::c_long {
                    current_block = 16433050379348534696;
                    break;
                }
                if src_total_size <= ext_start {
                    current_block = 9520865839495247062;
                    break;
                }
                ext_end = src_total_size;
            }
        }
        if src_total_size < ext_end {
            src_total_size = ext_end;
        }
        if lseek(src_fd, ext_start, 0 as libc::c_int) < 0 as libc::c_int as libc::c_long
        {
            current_block = 16433050379348534696;
            break;
        }
        wrote_hole_at_eof = 0 as libc::c_int != 0;
        let mut ext_hole_size: off_t = ext_start - last_ext_start - last_ext_len;
        if ext_hole_size != 0 {
            if sparse_mode as libc::c_uint != SPARSE_NEVER as libc::c_int as libc::c_uint
            {
                if !create_hole(
                    dest_fd,
                    dst_name,
                    sparse_mode as libc::c_uint
                        == SPARSE_ALWAYS as libc::c_int as libc::c_uint,
                    ext_hole_size,
                ) {
                    return 0 as libc::c_int != 0;
                }
                wrote_hole_at_eof = 1 as libc::c_int != 0;
            } else if !write_zeros(dest_fd, ext_hole_size) {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(b"%s: write failed\0" as *const u8 as *const libc::c_char),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        dst_name,
                    ),
                );
                return 0 as libc::c_int != 0;
            }
        }
        let mut ext_len: off_t = ext_end - ext_start;
        last_ext_start = ext_start;
        last_ext_len = ext_len;
        let mut n_read: off_t = 0;
        let mut read_hole: bool = false;
        if !sparse_copy(
            src_fd,
            dest_fd,
            buf,
            buf_size,
            if sparse_mode as libc::c_uint == SPARSE_NEVER as libc::c_int as libc::c_uint
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                hole_size
            },
            1 as libc::c_int != 0,
            allow_reflink,
            src_name,
            dst_name,
            ext_len as uintmax_t,
            &mut n_read,
            &mut read_hole,
        ) {
            return 0 as libc::c_int != 0;
        }
        dest_pos = ext_start + n_read;
        if n_read != 0 {
            wrote_hole_at_eof = read_hole;
        }
        if n_read < ext_len {
            src_total_size = dest_pos;
            current_block = 9520865839495247062;
            break;
        } else {
            ext_start = lseek(src_fd, dest_pos, 3 as libc::c_int);
            if !(ext_start < 0 as libc::c_int as libc::c_long) {
                continue;
            }
            if *__errno_location() != 6 as libc::c_int {
                current_block = 16433050379348534696;
                break;
            } else {
                current_block = 9520865839495247062;
                break;
            }
        }
    }
    match current_block {
        16433050379348534696 => {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(b"cannot lseek %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, src_name),
            );
            return 0 as libc::c_int != 0;
        }
        _ => {
            if (dest_pos < src_total_size || wrote_hole_at_eof as libc::c_int != 0)
                && (if sparse_mode as libc::c_uint
                    == SPARSE_NEVER as libc::c_int as libc::c_uint
                {
                    write_zeros(dest_fd, src_total_size - dest_pos) as libc::c_int
                } else {
                    (ftruncate(dest_fd, src_total_size) == 0 as libc::c_int)
                        as libc::c_int
                }) == 0
            {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"failed to extend %s\0" as *const u8 as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, dst_name),
                );
                return 0 as libc::c_int != 0;
            }
            if sparse_mode as libc::c_uint
                == SPARSE_ALWAYS as libc::c_int as libc::c_uint
                && dest_pos < src_total_size
                && punch_hole(dest_fd, dest_pos, src_total_size - dest_pos)
                    < 0 as libc::c_int
            {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"error deallocating %s\0" as *const u8 as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, dst_name),
                );
                return 0 as libc::c_int != 0;
            }
            return 1 as libc::c_int != 0;
        }
    };
}
unsafe extern "C" fn is_ancestor(
    mut sb: *const stat,
    mut ancestors: *const dir_list,
) -> bool {
    while !ancestors.is_null() {
        if (*ancestors).ino == (*sb).st_ino && (*ancestors).dev == (*sb).st_dev {
            return 1 as libc::c_int != 0;
        }
        ancestors = (*ancestors).parent;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn errno_unsupported(mut err: libc::c_int) -> bool {
    return err == 95 as libc::c_int || err == 61 as libc::c_int;
}
unsafe extern "C" fn copy_attr(
    mut src_path: *const libc::c_char,
    mut src_fd: libc::c_int,
    mut dst_path: *const libc::c_char,
    mut dst_fd: libc::c_int,
    mut x: *const cp_options,
) -> bool {
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn copy_dir(
    mut src_name_in: *const libc::c_char,
    mut dst_name_in: *const libc::c_char,
    mut new_dst: bool,
    mut src_sb: *const stat,
    mut ancestors: *mut dir_list,
    mut x: *const cp_options,
    mut first_dir_created_per_command_line_arg: *mut bool,
    mut copy_into_self: *mut bool,
) -> bool {
    let mut name_space: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut namep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut non_command_line_options: cp_options = *x;
    let mut ok: bool = 1 as libc::c_int != 0;
    name_space = savedir(src_name_in, SAVEDIR_SORT_FASTREAD);
    if name_space.is_null() {
        error(
            0 as libc::c_int,
            *__errno_location(),
            gettext(b"cannot access %s\0" as *const u8 as *const libc::c_char),
            quotearg_style(shell_escape_always_quoting_style, src_name_in),
        );
        return 0 as libc::c_int != 0;
    }
    if (*x).dereference as libc::c_uint
        == DEREF_COMMAND_LINE_ARGUMENTS as libc::c_int as libc::c_uint
    {
        non_command_line_options.dereference = DEREF_NEVER;
    }
    let mut new_first_dir_created: bool = 0 as libc::c_int != 0;
    namep = name_space;
    while *namep as libc::c_int != '\0' as i32 {
        let mut local_copy_into_self: bool = false;
        let mut src_name: *mut libc::c_char = file_name_concat(
            src_name_in,
            namep,
            0 as *mut *mut libc::c_char,
        );
        let mut dst_name: *mut libc::c_char = file_name_concat(
            dst_name_in,
            namep,
            0 as *mut *mut libc::c_char,
        );
        let mut first_dir_created: bool = *first_dir_created_per_command_line_arg;
        ok = (ok as libc::c_int
            & copy_internal(
                src_name,
                dst_name,
                new_dst,
                src_sb,
                ancestors,
                &mut non_command_line_options,
                0 as libc::c_int != 0,
                &mut first_dir_created,
                &mut local_copy_into_self,
                0 as *mut bool,
            ) as libc::c_int) as bool;
        *copy_into_self = (*copy_into_self as libc::c_int
            | local_copy_into_self as libc::c_int) as bool;
        free(dst_name as *mut libc::c_void);
        free(src_name as *mut libc::c_void);
        if local_copy_into_self {
            break;
        }
        new_first_dir_created = (new_first_dir_created as libc::c_int
            | first_dir_created as libc::c_int) as bool;
        namep = namep
            .offset(
                (strlen(namep)).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
    }
    free(name_space as *mut libc::c_void);
    *first_dir_created_per_command_line_arg = new_first_dir_created;
    return ok;
}
unsafe extern "C" fn set_owner(
    mut x: *const cp_options,
    mut dst_name: *const libc::c_char,
    mut dest_desc: libc::c_int,
    mut src_sb: *const stat,
    mut new_dst: bool,
    mut dst_sb: *const stat,
) -> libc::c_int {
    let mut uid: uid_t = (*src_sb).st_uid;
    let mut gid: gid_t = (*src_sb).st_gid;
    if !new_dst
        && ((*x).preserve_mode as libc::c_int != 0 || (*x).move_mode as libc::c_int != 0
            || (*x).set_mode as libc::c_int != 0)
    {
        let mut old_mode: mode_t = (*dst_sb).st_mode;
        let mut new_mode: mode_t = if (*x).preserve_mode as libc::c_int != 0
            || (*x).move_mode as libc::c_int != 0
        {
            (*src_sb).st_mode
        } else {
            (*x).mode
        };
        let mut restrictive_temp_mode: mode_t = old_mode & new_mode
            & (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                as libc::c_uint;
        if (0 as libc::c_int != 0
            || old_mode
                & (0o4000 as libc::c_int | 0o2000 as libc::c_int | 0o1000 as libc::c_int
                    | (0o400 as libc::c_int | 0o200 as libc::c_int
                        | 0o100 as libc::c_int)
                    | (0o400 as libc::c_int | 0o200 as libc::c_int
                        | 0o100 as libc::c_int) >> 3 as libc::c_int
                    | (0o400 as libc::c_int | 0o200 as libc::c_int
                        | 0o100 as libc::c_int) >> 3 as libc::c_int >> 3 as libc::c_int)
                    as libc::c_uint
                & (!new_mode | 0o4000 as libc::c_int as libc::c_uint
                    | 0o2000 as libc::c_int as libc::c_uint
                    | 0o1000 as libc::c_int as libc::c_uint) != 0)
            && qset_acl(dst_name, dest_desc, restrictive_temp_mode) != 0 as libc::c_int
        {
            if !owner_failure_ok(x) {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"clearing permissions for %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, dst_name),
                );
            }
            return -((*x).require_preserve as libc::c_int);
        }
    }
    if 1 as libc::c_int != 0 && dest_desc != -(1 as libc::c_int) {
        if fchown(dest_desc, uid, gid) == 0 as libc::c_int {
            return 1 as libc::c_int;
        }
        if *__errno_location() == 1 as libc::c_int
            || *__errno_location() == 22 as libc::c_int
        {
            let mut saved_errno: libc::c_int = *__errno_location();
            fchown(dest_desc, -(1 as libc::c_int) as __uid_t, gid);
            *__errno_location() = saved_errno;
        }
    } else {
        if lchown(dst_name, uid, gid) == 0 as libc::c_int {
            return 1 as libc::c_int;
        }
        if *__errno_location() == 1 as libc::c_int
            || *__errno_location() == 22 as libc::c_int
        {
            let mut saved_errno_0: libc::c_int = *__errno_location();
            lchown(dst_name, -(1 as libc::c_int) as __uid_t, gid);
            *__errno_location() = saved_errno_0;
        }
    }
    if !chown_failure_ok(x) {
        error(
            0 as libc::c_int,
            *__errno_location(),
            gettext(
                b"failed to preserve ownership for %s\0" as *const u8
                    as *const libc::c_char,
            ),
            quotearg_style(shell_escape_always_quoting_style, dst_name),
        );
        if (*x).require_preserve {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn set_author(
    mut dst_name: *const libc::c_char,
    mut dest_desc: libc::c_int,
    mut src_sb: *const stat,
) {}
unsafe extern "C" fn fchmod_or_lchmod(
    mut desc: libc::c_int,
    mut name: *const libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    if 0 as libc::c_int <= desc {
        return fchmod(desc, mode);
    }
    return lchmod(name, mode);
}
unsafe extern "C" fn infer_scantype(
    mut fd: libc::c_int,
    mut sb: *const stat,
    mut scan_inference: *mut scan_inference,
) -> scantype {
    if !(1 as libc::c_int != 0
        && (*sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint
        && (*sb).st_blocks < (*sb).st_size / 512 as libc::c_int as libc::c_long)
    {
        return PLAIN_SCANTYPE;
    }
    (*scan_inference)
        .ext_start = lseek(fd, 0 as libc::c_int as __off_t, 3 as libc::c_int);
    if 0 as libc::c_int as libc::c_long <= (*scan_inference).ext_start {
        return LSEEK_SCANTYPE
    } else if *__errno_location() != 22 as libc::c_int
        && !is_ENOTSUP(*__errno_location())
    {
        return (if *__errno_location() == 6 as libc::c_int {
            LSEEK_SCANTYPE as libc::c_int
        } else {
            ERROR_SCANTYPE as libc::c_int
        }) as scantype
    }
    return ZERO_SCANTYPE;
}
unsafe extern "C" fn copy_reg(
    mut src_name: *const libc::c_char,
    mut dst_name: *const libc::c_char,
    mut x: *const cp_options,
    mut dst_mode: mode_t,
    mut omitted_permissions: mode_t,
    mut new_dst: *mut bool,
    mut src_sb: *const stat,
) -> bool {
    let mut current_block: u64;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf_alloc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name_alloc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dest_desc: libc::c_int = 0;
    let mut dest_errno: libc::c_int = 0;
    let mut source_desc: libc::c_int = 0;
    let mut src_mode: mode_t = (*src_sb).st_mode;
    let mut sb: stat = stat {
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
    let mut src_open_sb: stat = stat {
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
    let mut scan_inference: scan_inference = scan_inference { ext_start: 0 };
    let mut return_val: bool = 1 as libc::c_int != 0;
    let mut data_copy_required: bool = (*x).data_copy_required;
    source_desc = open_safer(
        src_name,
        0 as libc::c_int | 0 as libc::c_int
            | (if (*x).dereference as libc::c_uint
                == DEREF_NEVER as libc::c_int as libc::c_uint
            {
                0o400000 as libc::c_int
            } else {
                0 as libc::c_int
            }),
    );
    if source_desc < 0 as libc::c_int {
        error(
            0 as libc::c_int,
            *__errno_location(),
            gettext(b"cannot open %s for reading\0" as *const u8 as *const libc::c_char),
            quotearg_style(shell_escape_always_quoting_style, src_name),
        );
        return 0 as libc::c_int != 0;
    }
    if fstat(source_desc, &mut src_open_sb) != 0 as libc::c_int {
        error(
            0 as libc::c_int,
            *__errno_location(),
            gettext(b"cannot fstat %s\0" as *const u8 as *const libc::c_char),
            quotearg_style(shell_escape_always_quoting_style, src_name),
        );
        return_val = 0 as libc::c_int != 0;
    } else if !((*src_sb).st_ino == src_open_sb.st_ino
        && (*src_sb).st_dev == src_open_sb.st_dev)
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            gettext(
                b"skipping file %s, as it was replaced while being copied\0" as *const u8
                    as *const libc::c_char,
            ),
            quotearg_style(shell_escape_always_quoting_style, src_name),
        );
        return_val = 0 as libc::c_int != 0;
    } else {
        if !*new_dst {
            let mut open_flags: libc::c_int = 0o1 as libc::c_int | 0 as libc::c_int
                | (if (*x).data_copy_required as libc::c_int != 0 {
                    0o1000 as libc::c_int
                } else {
                    0 as libc::c_int
                });
            dest_desc = open_safer(dst_name, open_flags);
            dest_errno = *__errno_location();
            if (!((*x).set_security_context).is_null()
                || (*x).preserve_security_context as libc::c_int != 0)
                && 0 as libc::c_int <= dest_desc
            {
                if !set_file_security_ctx(dst_name, 0 as libc::c_int != 0, x) {
                    if (*x).require_preserve_context {
                        return_val = 0 as libc::c_int != 0;
                        current_block = 14878735516729995367;
                    } else {
                        current_block = 13797916685926291137;
                    }
                } else {
                    current_block = 13797916685926291137;
                }
            } else {
                current_block = 13797916685926291137;
            }
            match current_block {
                14878735516729995367 => {}
                _ => {
                    if dest_desc < 0 as libc::c_int
                        && (*x).unlink_dest_after_failed_open as libc::c_int != 0
                    {
                        if unlink(dst_name) != 0 as libc::c_int {
                            error(
                                0 as libc::c_int,
                                *__errno_location(),
                                gettext(
                                    b"cannot remove %s\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(shell_escape_always_quoting_style, dst_name),
                            );
                            return_val = 0 as libc::c_int != 0;
                            current_block = 18261948585808176943;
                        } else {
                            if (*x).verbose {
                                printf(
                                    gettext(
                                        b"removed %s\n\0" as *const u8 as *const libc::c_char,
                                    ),
                                    quotearg_style(shell_escape_always_quoting_style, dst_name),
                                );
                            }
                            *new_dst = 1 as libc::c_int != 0;
                            if !((*x).set_security_context).is_null() {
                                if !set_process_security_ctx(
                                    src_name,
                                    dst_name,
                                    dst_mode,
                                    *new_dst,
                                    x,
                                ) {
                                    return_val = 0 as libc::c_int != 0;
                                    current_block = 18261948585808176943;
                                } else {
                                    current_block = 17788412896529399552;
                                }
                            } else {
                                current_block = 17788412896529399552;
                            }
                        }
                    } else {
                        current_block = 17788412896529399552;
                    }
                }
            }
        } else {
            current_block = 17788412896529399552;
        }
        match current_block {
            18261948585808176943 => {}
            _ => {
                match current_block {
                    17788412896529399552 => {
                        if *new_dst {
                            current_block = 652864300344834934;
                        } else {
                            omitted_permissions = 0 as libc::c_int as mode_t;
                            current_block = 6717214610478484138;
                        }
                        loop {
                            match current_block {
                                652864300344834934 => {
                                    let mut open_flags_0: libc::c_int = 0o1 as libc::c_int
                                        | 0o100 as libc::c_int | 0 as libc::c_int;
                                    dest_desc = open_safer(
                                        dst_name,
                                        open_flags_0 | 0o200 as libc::c_int,
                                        dst_mode & !omitted_permissions,
                                    );
                                    dest_errno = *__errno_location();
                                    if dest_desc < 0 as libc::c_int
                                        && dest_errno == 17 as libc::c_int && !(*x).move_mode
                                    {
                                        let mut dangling_link_sb: stat = stat {
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
                                        if lstat(dst_name, &mut dangling_link_sb)
                                            == 0 as libc::c_int
                                            && dangling_link_sb.st_mode
                                                & 0o170000 as libc::c_int as libc::c_uint
                                                == 0o120000 as libc::c_int as libc::c_uint
                                        {
                                            if (*x).open_dangling_dest_symlink {
                                                dest_desc = open_safer(
                                                    dst_name,
                                                    open_flags_0,
                                                    dst_mode & !omitted_permissions,
                                                );
                                                dest_errno = *__errno_location();
                                            } else {
                                                error(
                                                    0 as libc::c_int,
                                                    0 as libc::c_int,
                                                    gettext(
                                                        b"not writing through dangling symlink %s\0" as *const u8
                                                            as *const libc::c_char,
                                                    ),
                                                    quotearg_style(shell_escape_always_quoting_style, dst_name),
                                                );
                                                return_val = 0 as libc::c_int != 0;
                                                current_block = 18261948585808176943;
                                                break;
                                            }
                                        }
                                    }
                                    if dest_desc < 0 as libc::c_int
                                        && dest_errno == 21 as libc::c_int
                                        && *dst_name as libc::c_int != 0
                                        && *dst_name
                                            .offset(
                                                (strlen(dst_name))
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                            ) as libc::c_int == '/' as i32
                                    {
                                        dest_errno = 20 as libc::c_int;
                                    }
                                    current_block = 6717214610478484138;
                                }
                                _ => {
                                    if dest_desc < 0 as libc::c_int {
                                        if dest_errno == 2 as libc::c_int && !*new_dst
                                            && !(*x).move_mode
                                        {
                                            *new_dst = 1 as libc::c_int != 0;
                                            current_block = 652864300344834934;
                                        } else {
                                            error(
                                                0 as libc::c_int,
                                                dest_errno,
                                                gettext(
                                                    b"cannot create regular file %s\0" as *const u8
                                                        as *const libc::c_char,
                                                ),
                                                quotearg_style(shell_escape_always_quoting_style, dst_name),
                                            );
                                            return_val = 0 as libc::c_int != 0;
                                            current_block = 18261948585808176943;
                                            break;
                                        }
                                    } else if fstat(dest_desc, &mut sb) != 0 as libc::c_int {
                                        current_block = 11793792312832361944;
                                        break;
                                    } else {
                                        current_block = 13321564401369230990;
                                        break;
                                    }
                                }
                            }
                        }
                        match current_block {
                            18261948585808176943 => {}
                            _ => {
                                match current_block {
                                    11793792312832361944 => {
                                        error(
                                            0 as libc::c_int,
                                            *__errno_location(),
                                            gettext(
                                                b"cannot fstat %s\0" as *const u8 as *const libc::c_char,
                                            ),
                                            quotearg_style(shell_escape_always_quoting_style, dst_name),
                                        );
                                        return_val = 0 as libc::c_int != 0;
                                    }
                                    _ => {
                                        if data_copy_required as libc::c_int != 0
                                            && (*x).reflink_mode as libc::c_uint != 0
                                        {
                                            let mut clone_ok: bool = clone_file(dest_desc, source_desc)
                                                == 0 as libc::c_int;
                                            if clone_ok as libc::c_int != 0
                                                || (*x).reflink_mode as libc::c_uint
                                                    == REFLINK_ALWAYS as libc::c_int as libc::c_uint
                                            {
                                                if !clone_ok {
                                                    error(
                                                        0 as libc::c_int,
                                                        *__errno_location(),
                                                        gettext(
                                                            b"failed to clone %s from %s\0" as *const u8
                                                                as *const libc::c_char,
                                                        ),
                                                        quotearg_n_style(
                                                            0 as libc::c_int,
                                                            shell_escape_always_quoting_style,
                                                            dst_name,
                                                        ),
                                                        quotearg_n_style(
                                                            1 as libc::c_int,
                                                            shell_escape_always_quoting_style,
                                                            src_name,
                                                        ),
                                                    );
                                                    return_val = 0 as libc::c_int != 0;
                                                    current_block = 14878735516729995367;
                                                } else {
                                                    data_copy_required = 0 as libc::c_int != 0;
                                                    current_block = 2116367355679836638;
                                                }
                                            } else {
                                                current_block = 2116367355679836638;
                                            }
                                        } else {
                                            current_block = 2116367355679836638;
                                        }
                                        match current_block {
                                            14878735516729995367 => {}
                                            _ => {
                                                if data_copy_required {
                                                    let mut buf_alignment: size_t = getpagesize() as size_t;
                                                    let mut buf_size: size_t = io_blksize(sb);
                                                    let mut hole_size: size_t = (if (0 as libc::c_int
                                                        as libc::c_long) < sb.st_blksize
                                                        && sb.st_blksize as libc::c_ulong
                                                            <= (-(1 as libc::c_int) as size_t)
                                                                .wrapping_div(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                    {
                                                        sb.st_blksize
                                                    } else {
                                                        512 as libc::c_int as libc::c_long
                                                    }) as size_t;
                                                    let mut scantype: scantype = infer_scantype(
                                                        source_desc,
                                                        &mut src_open_sb,
                                                        &mut scan_inference,
                                                    );
                                                    if scantype as libc::c_uint
                                                        == ERROR_SCANTYPE as libc::c_int as libc::c_uint
                                                    {
                                                        error(
                                                            0 as libc::c_int,
                                                            *__errno_location(),
                                                            gettext(
                                                                b"cannot lseek %s\0" as *const u8 as *const libc::c_char,
                                                            ),
                                                            quotearg_style(shell_escape_always_quoting_style, src_name),
                                                        );
                                                        return_val = 0 as libc::c_int != 0;
                                                        current_block = 14878735516729995367;
                                                    } else {
                                                        let mut make_holes: bool = sb.st_mode
                                                            & 0o170000 as libc::c_int as libc::c_uint
                                                            == 0o100000 as libc::c_int as libc::c_uint
                                                            && ((*x).sparse_mode as libc::c_uint
                                                                == SPARSE_ALWAYS as libc::c_int as libc::c_uint
                                                                || (*x).sparse_mode as libc::c_uint
                                                                    == SPARSE_AUTO as libc::c_int as libc::c_uint
                                                                    && scantype as libc::c_uint
                                                                        != PLAIN_SCANTYPE as libc::c_int as libc::c_uint);
                                                        fdadvise(
                                                            source_desc,
                                                            0 as libc::c_int as off_t,
                                                            0 as libc::c_int as off_t,
                                                            FADVISE_SEQUENTIAL,
                                                        );
                                                        if !make_holes {
                                                            let mut blcm_max: size_t = (if (18446744073709551615
                                                                as libc::c_ulong)
                                                                < 9223372036854775807 as libc::c_long as libc::c_ulong
                                                            {
                                                                18446744073709551615 as libc::c_ulong
                                                            } else {
                                                                9223372036854775807 as libc::c_long as libc::c_ulong
                                                            })
                                                                .wrapping_sub(buf_alignment);
                                                            let mut blcm: size_t = buffer_lcm(
                                                                io_blksize(src_open_sb),
                                                                buf_size,
                                                                blcm_max,
                                                            );
                                                            if src_open_sb.st_mode
                                                                & 0o170000 as libc::c_int as libc::c_uint
                                                                == 0o100000 as libc::c_int as libc::c_uint
                                                                && (src_open_sb.st_size as libc::c_ulong) < buf_size
                                                            {
                                                                buf_size = (src_open_sb.st_size
                                                                    + 1 as libc::c_int as libc::c_long) as size_t;
                                                            }
                                                            buf_size = (buf_size as libc::c_ulong)
                                                                .wrapping_add(
                                                                    blcm.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                                                ) as size_t as size_t;
                                                            buf_size = (buf_size as libc::c_ulong)
                                                                .wrapping_sub(buf_size.wrapping_rem(blcm)) as size_t
                                                                as size_t;
                                                            if buf_size == 0 as libc::c_int as libc::c_ulong
                                                                || blcm_max < buf_size
                                                            {
                                                                buf_size = blcm;
                                                            }
                                                        }
                                                        buf_alloc = xmalloc(buf_size.wrapping_add(buf_alignment))
                                                            as *mut libc::c_char;
                                                        buf = ptr_align(
                                                            buf_alloc as *const libc::c_void,
                                                            buf_alignment,
                                                        ) as *mut libc::c_char;
                                                        let mut n_read: off_t = 0;
                                                        let mut wrote_hole_at_eof: bool = 0 as libc::c_int != 0;
                                                        if if scantype as libc::c_uint
                                                            == LSEEK_SCANTYPE as libc::c_int as libc::c_uint
                                                        {
                                                            lseek_copy(
                                                                source_desc,
                                                                dest_desc,
                                                                buf,
                                                                buf_size,
                                                                hole_size,
                                                                scan_inference.ext_start,
                                                                src_open_sb.st_size,
                                                                (if make_holes as libc::c_int != 0 {
                                                                    (*x).sparse_mode as libc::c_uint
                                                                } else {
                                                                    SPARSE_NEVER as libc::c_int as libc::c_uint
                                                                }) as Sparse_type,
                                                                (*x).reflink_mode as libc::c_uint
                                                                    != REFLINK_NEVER as libc::c_int as libc::c_uint,
                                                                src_name,
                                                                dst_name,
                                                            ) as libc::c_int
                                                        } else {
                                                            sparse_copy(
                                                                source_desc,
                                                                dest_desc,
                                                                buf,
                                                                buf_size,
                                                                if make_holes as libc::c_int != 0 {
                                                                    hole_size
                                                                } else {
                                                                    0 as libc::c_int as libc::c_ulong
                                                                },
                                                                (*x).sparse_mode as libc::c_uint
                                                                    == SPARSE_ALWAYS as libc::c_int as libc::c_uint,
                                                                (*x).reflink_mode as libc::c_uint
                                                                    != REFLINK_NEVER as libc::c_int as libc::c_uint,
                                                                src_name,
                                                                dst_name,
                                                                18446744073709551615 as libc::c_ulong,
                                                                &mut n_read,
                                                                &mut wrote_hole_at_eof,
                                                            ) as libc::c_int
                                                        } == 0
                                                        {
                                                            return_val = 0 as libc::c_int != 0;
                                                            current_block = 14878735516729995367;
                                                        } else if wrote_hole_at_eof as libc::c_int != 0
                                                            && ftruncate(dest_desc, n_read) < 0 as libc::c_int
                                                        {
                                                            error(
                                                                0 as libc::c_int,
                                                                *__errno_location(),
                                                                gettext(
                                                                    b"failed to extend %s\0" as *const u8 as *const libc::c_char,
                                                                ),
                                                                quotearg_style(shell_escape_always_quoting_style, dst_name),
                                                            );
                                                            return_val = 0 as libc::c_int != 0;
                                                            current_block = 14878735516729995367;
                                                        } else {
                                                            current_block = 16108440464692313034;
                                                        }
                                                    }
                                                } else {
                                                    current_block = 16108440464692313034;
                                                }
                                                match current_block {
                                                    14878735516729995367 => {}
                                                    _ => {
                                                        if (*x).preserve_timestamps {
                                                            let mut timespec: [timespec; 2] = [timespec {
                                                                tv_sec: 0,
                                                                tv_nsec: 0,
                                                            }; 2];
                                                            timespec[0 as libc::c_int
                                                                as usize] = get_stat_atime(src_sb);
                                                            timespec[1 as libc::c_int
                                                                as usize] = get_stat_mtime(src_sb);
                                                            if fdutimens(
                                                                dest_desc,
                                                                dst_name,
                                                                timespec.as_mut_ptr() as *const timespec,
                                                            ) != 0 as libc::c_int
                                                            {
                                                                error(
                                                                    0 as libc::c_int,
                                                                    *__errno_location(),
                                                                    gettext(
                                                                        b"preserving times for %s\0" as *const u8
                                                                            as *const libc::c_char,
                                                                    ),
                                                                    quotearg_style(shell_escape_always_quoting_style, dst_name),
                                                                );
                                                                if (*x).require_preserve {
                                                                    return_val = 0 as libc::c_int != 0;
                                                                    current_block = 14878735516729995367;
                                                                } else {
                                                                    current_block = 9199578309995299736;
                                                                }
                                                            } else {
                                                                current_block = 9199578309995299736;
                                                            }
                                                        } else {
                                                            current_block = 9199578309995299736;
                                                        }
                                                        match current_block {
                                                            14878735516729995367 => {}
                                                            _ => {
                                                                if (*x).preserve_ownership as libc::c_int != 0
                                                                    && !((*src_sb).st_uid == sb.st_uid
                                                                        && (*src_sb).st_gid == sb.st_gid)
                                                                {
                                                                    match set_owner(
                                                                        x,
                                                                        dst_name,
                                                                        dest_desc,
                                                                        src_sb,
                                                                        *new_dst,
                                                                        &mut sb,
                                                                    ) {
                                                                        -1 => {
                                                                            current_block = 9687774073468160227;
                                                                            match current_block {
                                                                                13732506948080828330 => {
                                                                                    src_mode
                                                                                        &= !(0o4000 as libc::c_int | 0o2000 as libc::c_int
                                                                                            | 0o1000 as libc::c_int) as libc::c_uint;
                                                                                    current_block = 7072655752890836508;
                                                                                }
                                                                                _ => {
                                                                                    return_val = 0 as libc::c_int != 0;
                                                                                    current_block = 14878735516729995367;
                                                                                }
                                                                            }
                                                                        }
                                                                        0 => {
                                                                            current_block = 13732506948080828330;
                                                                            match current_block {
                                                                                13732506948080828330 => {
                                                                                    src_mode
                                                                                        &= !(0o4000 as libc::c_int | 0o2000 as libc::c_int
                                                                                            | 0o1000 as libc::c_int) as libc::c_uint;
                                                                                    current_block = 7072655752890836508;
                                                                                }
                                                                                _ => {
                                                                                    return_val = 0 as libc::c_int != 0;
                                                                                    current_block = 14878735516729995367;
                                                                                }
                                                                            }
                                                                        }
                                                                        _ => {
                                                                            current_block = 7072655752890836508;
                                                                        }
                                                                    }
                                                                } else {
                                                                    current_block = 7072655752890836508;
                                                                }
                                                                match current_block {
                                                                    14878735516729995367 => {}
                                                                    _ => {
                                                                        if (*x).preserve_xattr {
                                                                            let mut access_changed: bool = 0 as libc::c_int != 0;
                                                                            if sb.st_mode & 0o200 as libc::c_int as libc::c_uint == 0
                                                                                && geteuid() != 0 as libc::c_int as libc::c_uint
                                                                            {
                                                                                access_changed = fchmod_or_lchmod(
                                                                                    dest_desc,
                                                                                    dst_name,
                                                                                    (0o400 as libc::c_int | 0o200 as libc::c_int) as mode_t,
                                                                                ) == 0 as libc::c_int;
                                                                            }
                                                                            if !copy_attr(src_name, source_desc, dst_name, dest_desc, x)
                                                                                && (*x).require_preserve_xattr as libc::c_int != 0
                                                                            {
                                                                                return_val = 0 as libc::c_int != 0;
                                                                            }
                                                                            if access_changed {
                                                                                fchmod_or_lchmod(
                                                                                    dest_desc,
                                                                                    dst_name,
                                                                                    dst_mode & !omitted_permissions,
                                                                                );
                                                                            }
                                                                        }
                                                                        set_author(dst_name, dest_desc, src_sb);
                                                                        if (*x).preserve_mode as libc::c_int != 0
                                                                            || (*x).move_mode as libc::c_int != 0
                                                                        {
                                                                            if copy_acl(
                                                                                src_name,
                                                                                source_desc,
                                                                                dst_name,
                                                                                dest_desc,
                                                                                src_mode,
                                                                            ) != 0 as libc::c_int
                                                                                && (*x).require_preserve as libc::c_int != 0
                                                                            {
                                                                                return_val = 0 as libc::c_int != 0;
                                                                            }
                                                                        } else if (*x).set_mode {
                                                                            if set_acl(dst_name, dest_desc, (*x).mode)
                                                                                != 0 as libc::c_int
                                                                            {
                                                                                return_val = 0 as libc::c_int != 0;
                                                                            }
                                                                        } else if (*x).explicit_no_preserve_mode as libc::c_int != 0
                                                                            && *new_dst as libc::c_int != 0
                                                                        {
                                                                            if set_acl(
                                                                                dst_name,
                                                                                dest_desc,
                                                                                (0o400 as libc::c_int | 0o200 as libc::c_int
                                                                                    | 0o400 as libc::c_int >> 3 as libc::c_int
                                                                                    | 0o200 as libc::c_int >> 3 as libc::c_int
                                                                                    | 0o400 as libc::c_int >> 3 as libc::c_int
                                                                                        >> 3 as libc::c_int
                                                                                    | 0o200 as libc::c_int >> 3 as libc::c_int
                                                                                        >> 3 as libc::c_int) as libc::c_uint & !cached_umask(),
                                                                            ) != 0 as libc::c_int
                                                                            {
                                                                                return_val = 0 as libc::c_int != 0;
                                                                            }
                                                                        } else if omitted_permissions != 0 {
                                                                            omitted_permissions &= !cached_umask();
                                                                            if omitted_permissions != 0
                                                                                && fchmod_or_lchmod(dest_desc, dst_name, dst_mode)
                                                                                    != 0 as libc::c_int
                                                                            {
                                                                                error(
                                                                                    0 as libc::c_int,
                                                                                    *__errno_location(),
                                                                                    gettext(
                                                                                        b"preserving permissions for %s\0" as *const u8
                                                                                            as *const libc::c_char,
                                                                                    ),
                                                                                    quotearg_style(shell_escape_always_quoting_style, dst_name),
                                                                                );
                                                                                if (*x).require_preserve {
                                                                                    return_val = 0 as libc::c_int != 0;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                current_block = 14878735516729995367;
                            }
                        }
                    }
                    _ => {}
                }
                match current_block {
                    18261948585808176943 => {}
                    _ => {
                        if close(dest_desc) < 0 as libc::c_int {
                            error(
                                0 as libc::c_int,
                                *__errno_location(),
                                gettext(
                                    b"failed to close %s\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(shell_escape_always_quoting_style, dst_name),
                            );
                            return_val = 0 as libc::c_int != 0;
                        }
                    }
                }
            }
        }
    }
    if close(source_desc) < 0 as libc::c_int {
        error(
            0 as libc::c_int,
            *__errno_location(),
            gettext(b"failed to close %s\0" as *const u8 as *const libc::c_char),
            quotearg_style(shell_escape_always_quoting_style, src_name),
        );
        return_val = 0 as libc::c_int != 0;
    }
    free(buf_alloc as *mut libc::c_void);
    free(name_alloc as *mut libc::c_void);
    return return_val;
}
unsafe extern "C" fn same_file_ok(
    mut src_name: *const libc::c_char,
    mut src_sb: *const stat,
    mut dst_name: *const libc::c_char,
    mut dst_sb: *const stat,
    mut x: *const cp_options,
    mut return_now: *mut bool,
) -> bool {
    let mut src_sb_link: *const stat = 0 as *const stat;
    let mut dst_sb_link: *const stat = 0 as *const stat;
    let mut tmp_dst_sb: stat = stat {
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
    let mut tmp_src_sb: stat = stat {
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
    let mut same_link: bool = false;
    let mut same: bool = (*src_sb).st_ino == (*dst_sb).st_ino
        && (*src_sb).st_dev == (*dst_sb).st_dev;
    *return_now = 0 as libc::c_int != 0;
    if same as libc::c_int != 0 && (*x).hard_link as libc::c_int != 0 {
        *return_now = 1 as libc::c_int != 0;
        return 1 as libc::c_int != 0;
    }
    if (*x).dereference as libc::c_uint == DEREF_NEVER as libc::c_int as libc::c_uint {
        same_link = same;
        if (*src_sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
            && (*dst_sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o120000 as libc::c_int as libc::c_uint
        {
            let mut sn: bool = same_name(src_name, dst_name);
            if !sn {
                if (*x).backup_type as libc::c_uint
                    != no_backups as libc::c_int as libc::c_uint
                {
                    return 1 as libc::c_int != 0;
                }
                if same_link {
                    *return_now = 1 as libc::c_int != 0;
                    return !(*x).move_mode;
                }
            }
            return !sn;
        }
        src_sb_link = src_sb;
        dst_sb_link = dst_sb;
    } else {
        if !same {
            return 1 as libc::c_int != 0;
        }
        if lstat(dst_name, &mut tmp_dst_sb) != 0 as libc::c_int
            || lstat(src_name, &mut tmp_src_sb) != 0 as libc::c_int
        {
            return 1 as libc::c_int != 0;
        }
        src_sb_link = &mut tmp_src_sb;
        dst_sb_link = &mut tmp_dst_sb;
        same_link = (*src_sb_link).st_ino == (*dst_sb_link).st_ino
            && (*src_sb_link).st_dev == (*dst_sb_link).st_dev;
        if (*src_sb_link).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
            && (*dst_sb_link).st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o120000 as libc::c_int as libc::c_uint
            && (*x).unlink_dest_before_opening as libc::c_int != 0
        {
            return 1 as libc::c_int != 0;
        }
    }
    if (*x).backup_type as libc::c_uint != no_backups as libc::c_int as libc::c_uint {
        if !same_link {
            if !(*x).move_mode
                && (*x).dereference as libc::c_uint
                    != DEREF_NEVER as libc::c_int as libc::c_uint
                && (*src_sb_link).st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o120000 as libc::c_int as libc::c_uint
                && !((*dst_sb_link).st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o120000 as libc::c_int as libc::c_uint)
            {
                return 0 as libc::c_int != 0;
            }
            return 1 as libc::c_int != 0;
        }
        return !same_name(src_name, dst_name);
    }
    if (*x).move_mode as libc::c_int != 0
        || (*x).unlink_dest_before_opening as libc::c_int != 0
    {
        if (*dst_sb_link).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
        {
            return 1 as libc::c_int != 0;
        }
        if same_link as libc::c_int != 0
            && (1 as libc::c_int as libc::c_ulong) < (*dst_sb_link).st_nlink
            && !same_name(src_name, dst_name)
        {
            return !(*x).move_mode;
        }
    }
    if !((*src_sb_link).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o120000 as libc::c_int as libc::c_uint)
        && !((*dst_sb_link).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint)
    {
        if !((*src_sb_link).st_ino == (*dst_sb_link).st_ino
            && (*src_sb_link).st_dev == (*dst_sb_link).st_dev)
        {
            return 1 as libc::c_int != 0;
        }
        if (*x).hard_link {
            *return_now = 1 as libc::c_int != 0;
            return 1 as libc::c_int != 0;
        }
    }
    if (*x).move_mode as libc::c_int != 0
        && (*src_sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
        && (1 as libc::c_int as libc::c_ulong) < (*dst_sb_link).st_nlink
    {
        let mut abs_src: *mut libc::c_char = canonicalize_file_name(src_name);
        if !abs_src.is_null() {
            let mut result: bool = !same_name(abs_src, dst_name);
            free(abs_src as *mut libc::c_void);
            return result;
        }
    }
    if (*x).symbolic_link as libc::c_int != 0
        && (*dst_sb_link).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int != 0;
    }
    if (*x).dereference as libc::c_uint == DEREF_NEVER as libc::c_int as libc::c_uint {
        if !((*src_sb_link).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint)
        {
            tmp_src_sb = *src_sb_link;
        } else if stat(src_name, &mut tmp_src_sb) != 0 as libc::c_int {
            return 1 as libc::c_int != 0
        }
        if !((*dst_sb_link).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint)
        {
            tmp_dst_sb = *dst_sb_link;
        } else if stat(dst_name, &mut tmp_dst_sb) != 0 as libc::c_int {
            return 1 as libc::c_int != 0
        }
        if !(tmp_src_sb.st_ino == tmp_dst_sb.st_ino
            && tmp_src_sb.st_dev == tmp_dst_sb.st_dev)
        {
            return 1 as libc::c_int != 0;
        }
        if (*x).hard_link {
            *return_now = !((*dst_sb_link).st_mode
                & 0o170000 as libc::c_int as libc::c_uint
                == 0o120000 as libc::c_int as libc::c_uint);
            return 1 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn writable_destination(
    mut file: *const libc::c_char,
    mut mode: mode_t,
) -> bool {
    return mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o120000 as libc::c_int as libc::c_uint
        || can_write_any_file() as libc::c_int != 0
        || euidaccess(file, 2 as libc::c_int) == 0 as libc::c_int;
}
unsafe extern "C" fn overwrite_ok(
    mut x: *const cp_options,
    mut dst_name: *const libc::c_char,
    mut dst_sb: *const stat,
) -> bool {
    if !writable_destination(dst_name, (*dst_sb).st_mode) {
        let mut perms: [libc::c_char; 12] = [0; 12];
        strmode((*dst_sb).st_mode, perms.as_mut_ptr());
        perms[10 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        fprintf(
            stderr,
            if (*x).move_mode as libc::c_int != 0
                || (*x).unlink_dest_before_opening as libc::c_int != 0
                || (*x).unlink_dest_after_failed_open as libc::c_int != 0
            {
                gettext(
                    b"%s: replace %s, overriding mode %04lo (%s)? \0" as *const u8
                        as *const libc::c_char,
                )
            } else {
                gettext(
                    b"%s: unwritable %s (mode %04lo, %s); try anyway? \0" as *const u8
                        as *const libc::c_char,
                )
            },
            program_name,
            quotearg_style(shell_escape_always_quoting_style, dst_name),
            ((*dst_sb).st_mode
                & (0o4000 as libc::c_int | 0o2000 as libc::c_int | 0o1000 as libc::c_int
                    | (0o400 as libc::c_int | 0o200 as libc::c_int
                        | 0o100 as libc::c_int)
                    | (0o400 as libc::c_int | 0o200 as libc::c_int
                        | 0o100 as libc::c_int) >> 3 as libc::c_int
                    | (0o400 as libc::c_int | 0o200 as libc::c_int
                        | 0o100 as libc::c_int) >> 3 as libc::c_int >> 3 as libc::c_int)
                    as libc::c_uint) as libc::c_ulong,
            &mut *perms.as_mut_ptr().offset(1 as libc::c_int as isize)
                as *mut libc::c_char,
        );
    } else {
        fprintf(
            stderr,
            gettext(b"%s: overwrite %s? \0" as *const u8 as *const libc::c_char),
            program_name,
            quotearg_style(shell_escape_always_quoting_style, dst_name),
        );
    }
    return yesno();
}
unsafe extern "C" fn abandon_move(
    mut x: *const cp_options,
    mut dst_name: *const libc::c_char,
    mut dst_sb: *const stat,
) -> bool {
    if (*x).move_mode {} else {
        __assert_fail(
            b"x->move_mode\0" as *const u8 as *const libc::c_char,
            b"src/copy.c\0" as *const u8 as *const libc::c_char,
            1862 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 81],
                &[libc::c_char; 81],
            >(
                b"_Bool abandon_move(const struct cp_options *, const char *, const struct stat *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_17133: {
        if (*x).move_mode {} else {
            __assert_fail(
                b"x->move_mode\0" as *const u8 as *const libc::c_char,
                b"src/copy.c\0" as *const u8 as *const libc::c_char,
                1862 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 81],
                    &[libc::c_char; 81],
                >(
                    b"_Bool abandon_move(const struct cp_options *, const char *, const struct stat *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return (*x).interactive as libc::c_uint == I_ALWAYS_NO as libc::c_int as libc::c_uint
        || ((*x).interactive as libc::c_uint == I_ASK_USER as libc::c_int as libc::c_uint
            || (*x).interactive as libc::c_uint
                == I_UNSPECIFIED as libc::c_int as libc::c_uint
                && (*x).stdin_tty as libc::c_int != 0
                && !writable_destination(dst_name, (*dst_sb).st_mode))
            && !overwrite_ok(x, dst_name, dst_sb);
}
unsafe extern "C" fn emit_verbose(
    mut src: *const libc::c_char,
    mut dst: *const libc::c_char,
    mut backup_dst_name: *const libc::c_char,
) {
    printf(
        b"%s -> %s\0" as *const u8 as *const libc::c_char,
        quotearg_n_style(0 as libc::c_int, shell_escape_always_quoting_style, src),
        quotearg_n_style(1 as libc::c_int, shell_escape_always_quoting_style, dst),
    );
    if !backup_dst_name.is_null() {
        printf(
            gettext(b" (backup: %s)\0" as *const u8 as *const libc::c_char),
            quotearg_style(shell_escape_always_quoting_style, backup_dst_name),
        );
    }
    putchar_unlocked('\n' as i32);
}
unsafe extern "C" fn restore_default_fscreatecon_or_die() {
    if setfscreatecon(0 as *const libc::c_char) != 0 as libc::c_int {
        if ::core::mem::size_of::<C2RustUnnamed>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                gettext(
                    b"failed to restore the default file creation context\0" as *const u8
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
                    b"failed to restore the default file creation context\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
}
unsafe extern "C" fn create_hard_link(
    mut src_name: *const libc::c_char,
    mut dst_name: *const libc::c_char,
    mut replace: bool,
    mut verbose: bool,
    mut dereference: bool,
) -> bool {
    let mut err: libc::c_int = force_linkat(
        -(100 as libc::c_int),
        src_name,
        -(100 as libc::c_int),
        dst_name,
        if dereference as libc::c_int != 0 {
            0x400 as libc::c_int
        } else {
            0 as libc::c_int
        },
        replace,
        -(1 as libc::c_int),
    );
    if (0 as libc::c_int) < err {
        error(
            0 as libc::c_int,
            err,
            gettext(
                b"cannot create hard link %s to %s\0" as *const u8 as *const libc::c_char,
            ),
            quotearg_n_style(
                0 as libc::c_int,
                shell_escape_always_quoting_style,
                dst_name,
            ),
            quotearg_n_style(
                1 as libc::c_int,
                shell_escape_always_quoting_style,
                src_name,
            ),
        );
        return 0 as libc::c_int != 0;
    }
    if err < 0 as libc::c_int && verbose as libc::c_int != 0 {
        printf(
            gettext(b"removed %s\n\0" as *const u8 as *const libc::c_char),
            quotearg_style(shell_escape_always_quoting_style, dst_name),
        );
    }
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn should_dereference(
    mut x: *const cp_options,
    mut command_line_arg: bool,
) -> bool {
    return (*x).dereference as libc::c_uint
        == DEREF_ALWAYS as libc::c_int as libc::c_uint
        || (*x).dereference as libc::c_uint
            == DEREF_COMMAND_LINE_ARGUMENTS as libc::c_int as libc::c_uint
            && command_line_arg as libc::c_int != 0;
}
unsafe extern "C" fn source_is_dst_backup(
    mut srcbase: *const libc::c_char,
    mut src_st: *const stat,
    mut dst_name: *const libc::c_char,
) -> bool {
    let mut srcbaselen: size_t = strlen(srcbase);
    let mut dstbase: *const libc::c_char = last_component(dst_name);
    let mut dstbaselen: size_t = strlen(dstbase);
    let mut suffixlen: size_t = strlen(simple_backup_suffix);
    if !(srcbaselen == dstbaselen.wrapping_add(suffixlen)
        && memcmp(
            srcbase as *const libc::c_void,
            dstbase as *const libc::c_void,
            dstbaselen,
        ) == 0 as libc::c_int
        && strcmp(srcbase.offset(dstbaselen as isize), simple_backup_suffix)
            == 0 as libc::c_int)
    {
        return 0 as libc::c_int != 0;
    }
    let mut dstlen: size_t = strlen(dst_name);
    let mut dst_back: *mut libc::c_char = xmalloc(
        dstlen.wrapping_add(suffixlen).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    strcpy(
        mempcpy(dst_back as *mut libc::c_void, dst_name as *const libc::c_void, dstlen)
            as *mut libc::c_char,
        simple_backup_suffix,
    );
    let mut dst_back_sb: stat = stat {
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
    let mut dst_back_status: libc::c_int = stat(dst_back, &mut dst_back_sb);
    free(dst_back as *mut libc::c_void);
    return dst_back_status == 0 as libc::c_int
        && ((*src_st).st_ino == dst_back_sb.st_ino
            && (*src_st).st_dev == dst_back_sb.st_dev);
}
unsafe extern "C" fn valid_options(mut co: *const cp_options) -> bool {
    if !co.is_null() {} else {
        __assert_fail(
            b"co != NULL\0" as *const u8 as *const libc::c_char,
            b"src/copy.c\0" as *const u8 as *const libc::c_char,
            3067 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"_Bool valid_options(const struct cp_options *)\0"))
                .as_ptr(),
        );
    }
    'c_19179: {
        if !co.is_null() {} else {
            __assert_fail(
                b"co != NULL\0" as *const u8 as *const libc::c_char,
                b"src/copy.c\0" as *const u8 as *const libc::c_char,
                3067 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"_Bool valid_options(const struct cp_options *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*co).backup_type as libc::c_uint
        <= numbered_backups as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"VALID_BACKUP_TYPE (co->backup_type)\0" as *const u8 as *const libc::c_char,
            b"src/copy.c\0" as *const u8 as *const libc::c_char,
            3068 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"_Bool valid_options(const struct cp_options *)\0"))
                .as_ptr(),
        );
    }
    'c_19131: {
        if (*co).backup_type as libc::c_uint
            <= numbered_backups as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"VALID_BACKUP_TYPE (co->backup_type)\0" as *const u8
                    as *const libc::c_char,
                b"src/copy.c\0" as *const u8 as *const libc::c_char,
                3068 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"_Bool valid_options(const struct cp_options *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*co).sparse_mode as libc::c_uint == SPARSE_NEVER as libc::c_int as libc::c_uint
        || (*co).sparse_mode as libc::c_uint
            == SPARSE_AUTO as libc::c_int as libc::c_uint
        || (*co).sparse_mode as libc::c_uint
            == SPARSE_ALWAYS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"VALID_SPARSE_MODE (co->sparse_mode)\0" as *const u8 as *const libc::c_char,
            b"src/copy.c\0" as *const u8 as *const libc::c_char,
            3069 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"_Bool valid_options(const struct cp_options *)\0"))
                .as_ptr(),
        );
    }
    'c_19043: {
        if (*co).sparse_mode as libc::c_uint
            == SPARSE_NEVER as libc::c_int as libc::c_uint
            || (*co).sparse_mode as libc::c_uint
                == SPARSE_AUTO as libc::c_int as libc::c_uint
            || (*co).sparse_mode as libc::c_uint
                == SPARSE_ALWAYS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"VALID_SPARSE_MODE (co->sparse_mode)\0" as *const u8
                    as *const libc::c_char,
                b"src/copy.c\0" as *const u8 as *const libc::c_char,
                3069 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"_Bool valid_options(const struct cp_options *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*co).reflink_mode as libc::c_uint == REFLINK_NEVER as libc::c_int as libc::c_uint
        || (*co).reflink_mode as libc::c_uint
            == REFLINK_AUTO as libc::c_int as libc::c_uint
        || (*co).reflink_mode as libc::c_uint
            == REFLINK_ALWAYS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"VALID_REFLINK_MODE (co->reflink_mode)\0" as *const u8
                as *const libc::c_char,
            b"src/copy.c\0" as *const u8 as *const libc::c_char,
            3070 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"_Bool valid_options(const struct cp_options *)\0"))
                .as_ptr(),
        );
    }
    'c_18955: {
        if (*co).reflink_mode as libc::c_uint
            == REFLINK_NEVER as libc::c_int as libc::c_uint
            || (*co).reflink_mode as libc::c_uint
                == REFLINK_AUTO as libc::c_int as libc::c_uint
            || (*co).reflink_mode as libc::c_uint
                == REFLINK_ALWAYS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"VALID_REFLINK_MODE (co->reflink_mode)\0" as *const u8
                    as *const libc::c_char,
                b"src/copy.c\0" as *const u8 as *const libc::c_char,
                3070 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"_Bool valid_options(const struct cp_options *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*co).hard_link as libc::c_int != 0 && (*co).symbolic_link as libc::c_int != 0)
    {} else {
        __assert_fail(
            b"!(co->hard_link && co->symbolic_link)\0" as *const u8
                as *const libc::c_char,
            b"src/copy.c\0" as *const u8 as *const libc::c_char,
            3071 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"_Bool valid_options(const struct cp_options *)\0"))
                .as_ptr(),
        );
    }
    'c_18900: {
        if !((*co).hard_link as libc::c_int != 0
            && (*co).symbolic_link as libc::c_int != 0)
        {} else {
            __assert_fail(
                b"!(co->hard_link && co->symbolic_link)\0" as *const u8
                    as *const libc::c_char,
                b"src/copy.c\0" as *const u8 as *const libc::c_char,
                3071 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"_Bool valid_options(const struct cp_options *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*co).reflink_mode as libc::c_uint
        == REFLINK_ALWAYS as libc::c_int as libc::c_uint
        && (*co).sparse_mode as libc::c_uint
            != SPARSE_AUTO as libc::c_int as libc::c_uint)
    {} else {
        __assert_fail(
            b"! (co->reflink_mode == REFLINK_ALWAYS && co->sparse_mode != SPARSE_AUTO)\0"
                as *const u8 as *const libc::c_char,
            b"src/copy.c\0" as *const u8 as *const libc::c_char,
            3074 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"_Bool valid_options(const struct cp_options *)\0"))
                .as_ptr(),
        );
    }
    'c_18833: {
        if !((*co).reflink_mode as libc::c_uint
            == REFLINK_ALWAYS as libc::c_int as libc::c_uint
            && (*co).sparse_mode as libc::c_uint
                != SPARSE_AUTO as libc::c_int as libc::c_uint)
        {} else {
            __assert_fail(
                b"! (co->reflink_mode == REFLINK_ALWAYS && co->sparse_mode != SPARSE_AUTO)\0"
                    as *const u8 as *const libc::c_char,
                b"src/copy.c\0" as *const u8 as *const libc::c_char,
                3074 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"_Bool valid_options(const struct cp_options *)\0"))
                    .as_ptr(),
            );
        }
    };
    return 1 as libc::c_int != 0;
}
