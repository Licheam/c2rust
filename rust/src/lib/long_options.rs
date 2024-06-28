use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn exit(_: libc::c_int) -> !;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optind: libc::c_int;
    static mut opterr: libc::c_int;
    fn version_etc_va(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        authors: ::core::ffi::VaList,
    );
    static mut exit_failure: libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
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
pub type size_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn parse_long_options(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut command_name: *const libc::c_char,
    mut package: *const libc::c_char,
    mut version: *const libc::c_char,
    mut usage_func: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    mut args: ...
) {
    let mut c: libc::c_int = 0;
    let mut saved_opterr: libc::c_int = 0;
    saved_opterr = opterr;
    opterr = 0 as libc::c_int;
    if argc == 2 as libc::c_int
        && {
            c = getopt_long(
                argc,
                argv,
                b"+\0" as *const u8 as *const libc::c_char,
                long_options.as_ptr(),
                0 as *mut libc::c_int,
            );
            c != -(1 as libc::c_int)
        }
    {
        match c {
            104 => {
                (Some(usage_func.expect("non-null function pointer")))
                    .expect("non-null function pointer")(0 as libc::c_int);
            }
            118 => {
                let mut authors: ::core::ffi::VaListImpl;
                authors = args.clone();
                version_etc_va(
                    stdout,
                    command_name,
                    package,
                    version,
                    authors.as_va_list(),
                );
                exit(0 as libc::c_int);
            }
            _ => {}
        }
    }
    opterr = saved_opterr;
    optind = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn parse_gnu_standard_options_only(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut command_name: *const libc::c_char,
    mut package: *const libc::c_char,
    mut version: *const libc::c_char,
    mut scan_all: bool,
    mut usage_func: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    mut args: ...
) {
    let mut c: libc::c_int = 0;
    let mut saved_opterr: libc::c_int = opterr;
    opterr = 1 as libc::c_int;
    let mut optstring: *const libc::c_char = if scan_all as libc::c_int != 0 {
        b"\0" as *const u8 as *const libc::c_char
    } else {
        b"+\0" as *const u8 as *const libc::c_char
    };
    c = getopt_long(argc, argv, optstring, long_options.as_ptr(), 0 as *mut libc::c_int);
    if c != -(1 as libc::c_int) {
        match c {
            104 => {
                (Some(usage_func.expect("non-null function pointer")))
                    .expect("non-null function pointer")(0 as libc::c_int);
            }
            118 => {
                let mut authors: ::core::ffi::VaListImpl;
                authors = args.clone();
                version_etc_va(
                    stdout,
                    command_name,
                    package,
                    version,
                    authors.as_va_list(),
                );
                exit(0 as libc::c_int);
            }
            _ => {
                (Some(usage_func.expect("non-null function pointer")))
                    .expect("non-null function pointer")(exit_failure);
            }
        }
    }
    opterr = saved_opterr;
}
static mut long_options: [option; 3] = [
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
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
