use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn _obstack_begin(
        _: *mut obstack,
        _: size_t,
        _: size_t,
        _: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> libc::c_int;
    fn _obstack_free(_: *mut obstack, _: *mut libc::c_void);
    fn _obstack_newchunk(_: *mut obstack, _: size_t);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type ptrdiff_t = libc::c_long;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: size_t,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut libc::c_char,
    pub next_free: *mut libc::c_char,
    pub chunk_limit: *mut libc::c_char,
    pub temp: C2RustUnnamed_1,
    pub alignment_mask: size_t,
    pub chunkfun: C2RustUnnamed_0,
    pub freefun: C2RustUnnamed,
    pub extra_arg: *mut libc::c_void,
    #[bitfield(name = "use_extra_arg", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "maybe_empty_object", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "alloc_failed", ty = "libc::c_uint", bits = "2..=2")]
    pub use_extra_arg_maybe_empty_object_alloc_failed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub plain: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub plain: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub i: size_t,
    pub p: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obstack_chunk {
    pub limit: *mut libc::c_char,
    pub prev: *mut _obstack_chunk,
    pub contents: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tokens {
    pub n_tok: size_t,
    pub tok: *mut *mut libc::c_char,
    pub tok_len: *mut size_t,
    pub o_data: obstack,
    pub o_tok: obstack,
    pub o_tok_len: obstack,
}
#[no_mangle]
pub unsafe extern "C" fn readtokens0(mut in_0: *mut FILE, mut t: *mut Tokens) -> bool {
    loop {
        let mut c: libc::c_int = fgetc(in_0);
        if c == -(1 as libc::c_int) {
            let mut len: size_t = ({
                let mut __o: *const obstack = &mut (*t).o_data;
                ((*__o).next_free).offset_from((*__o).object_base) as libc::c_long
                    as size_t
            });
            if len != 0 {
                let mut __o: *mut obstack = &mut (*t).o_data;
                if ({
                    let mut __o1: *const obstack = __o;
                    ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                        as size_t
                }) < 1 as libc::c_int as libc::c_ulong
                {
                    _obstack_newchunk(__o, 1 as libc::c_int as size_t);
                }
                let fresh0 = (*__o).next_free;
                (*__o).next_free = ((*__o).next_free).offset(1);
                *fresh0 = '\0' as i32 as libc::c_char;
                save_token(t);
            }
            break;
        } else {
            let mut __o_0: *mut obstack = &mut (*t).o_data;
            if ({
                let mut __o1: *const obstack = __o_0;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < 1 as libc::c_int as libc::c_ulong
            {
                _obstack_newchunk(__o_0, 1 as libc::c_int as size_t);
            }
            let fresh1 = (*__o_0).next_free;
            (*__o_0).next_free = ((*__o_0).next_free).offset(1);
            *fresh1 = c as libc::c_char;
            if c == '\0' as i32 {
                save_token(t);
            }
        }
    }
    let mut __o_1: *mut obstack = &mut (*t).o_tok;
    if ({
        let mut __o1: *const obstack = __o_1;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
    {
        _obstack_newchunk(
            __o_1,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
        );
    }
    let mut __o1: *mut obstack = __o_1;
    let mut __p1: *mut libc::c_void = (*__o1).next_free as *mut libc::c_void;
    let ref mut fresh2 = *(__p1 as *mut *const libc::c_void);
    *fresh2 = 0 as *const libc::c_void;
    (*__o1)
        .next_free = ((*__o1).next_free)
        .offset(::core::mem::size_of::<*const libc::c_void>() as libc::c_ulong as isize);
    (*t)
        .tok = ({
        let mut __o1_0: *mut obstack = &mut (*t).o_tok as *mut obstack;
        let mut __value: *mut libc::c_void = (*__o1_0).object_base as *mut libc::c_void;
        if (*__o1_0).next_free == __value as *mut libc::c_char {
            (*__o1_0).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
        }
        (*__o1_0)
            .next_free = (if (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
            < ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        {
            (*__o1_0).object_base
        } else {
            0 as *mut libc::c_char
        })
            .offset(
                ((((*__o1_0).next_free)
                    .offset_from(
                        (if (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                            < ::core::mem::size_of::<*mut libc::c_void>()
                                as libc::c_ulong
                        {
                            (*__o1_0).object_base
                        } else {
                            0 as *mut libc::c_char
                        }),
                    ) as libc::c_long as libc::c_ulong)
                    .wrapping_add((*__o1_0).alignment_mask) & !(*__o1_0).alignment_mask)
                    as isize,
            );
        if ((*__o1_0).next_free).offset_from((*__o1_0).chunk as *mut libc::c_char)
            as libc::c_long as size_t
            > ((*__o1_0).chunk_limit).offset_from((*__o1_0).chunk as *mut libc::c_char)
                as libc::c_long as size_t
        {
            (*__o1_0).next_free = (*__o1_0).chunk_limit;
        }
        (*__o1_0).object_base = (*__o1_0).next_free;
        __value
    }) as *mut *mut libc::c_char;
    (*t)
        .tok_len = ({
        let mut __o1_0: *mut obstack = &mut (*t).o_tok_len as *mut obstack;
        let mut __value: *mut libc::c_void = (*__o1_0).object_base as *mut libc::c_void;
        if (*__o1_0).next_free == __value as *mut libc::c_char {
            (*__o1_0).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
        }
        (*__o1_0)
            .next_free = (if (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
            < ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        {
            (*__o1_0).object_base
        } else {
            0 as *mut libc::c_char
        })
            .offset(
                ((((*__o1_0).next_free)
                    .offset_from(
                        (if (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                            < ::core::mem::size_of::<*mut libc::c_void>()
                                as libc::c_ulong
                        {
                            (*__o1_0).object_base
                        } else {
                            0 as *mut libc::c_char
                        }),
                    ) as libc::c_long as libc::c_ulong)
                    .wrapping_add((*__o1_0).alignment_mask) & !(*__o1_0).alignment_mask)
                    as isize,
            );
        if ((*__o1_0).next_free).offset_from((*__o1_0).chunk as *mut libc::c_char)
            as libc::c_long as size_t
            > ((*__o1_0).chunk_limit).offset_from((*__o1_0).chunk as *mut libc::c_char)
                as libc::c_long as size_t
        {
            (*__o1_0).next_free = (*__o1_0).chunk_limit;
        }
        (*__o1_0).object_base = (*__o1_0).next_free;
        __value
    }) as *mut size_t;
    return ferror(in_0) == 0;
}
#[no_mangle]
pub unsafe extern "C" fn readtokens0_free(mut t: *mut Tokens) {
    let mut __o: *mut obstack = &mut (*t).o_data;
    let mut __obj: *mut libc::c_void = 0 as *mut libc::c_void;
    if __obj > (*__o).chunk as *mut libc::c_void
        && __obj < (*__o).chunk_limit as *mut libc::c_void
    {
        (*__o).object_base = __obj as *mut libc::c_char;
        (*__o).next_free = (*__o).object_base;
    } else {
        _obstack_free(__o, __obj);
    }
    let mut __o_0: *mut obstack = &mut (*t).o_tok;
    let mut __obj_0: *mut libc::c_void = 0 as *mut libc::c_void;
    if __obj_0 > (*__o_0).chunk as *mut libc::c_void
        && __obj_0 < (*__o_0).chunk_limit as *mut libc::c_void
    {
        (*__o_0).object_base = __obj_0 as *mut libc::c_char;
        (*__o_0).next_free = (*__o_0).object_base;
    } else {
        _obstack_free(__o_0, __obj_0);
    }
    let mut __o_1: *mut obstack = &mut (*t).o_tok_len;
    let mut __obj_1: *mut libc::c_void = 0 as *mut libc::c_void;
    if __obj_1 > (*__o_1).chunk as *mut libc::c_void
        && __obj_1 < (*__o_1).chunk_limit as *mut libc::c_void
    {
        (*__o_1).object_base = __obj_1 as *mut libc::c_char;
        (*__o_1).next_free = (*__o_1).object_base;
    } else {
        _obstack_free(__o_1, __obj_1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn readtokens0_init(mut t: *mut Tokens) {
    (*t).n_tok = 0 as libc::c_int as size_t;
    (*t).tok = 0 as *mut *mut libc::c_char;
    (*t).tok_len = 0 as *mut size_t;
    _obstack_begin(
        &mut (*t).o_data,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        Some(malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void),
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    _obstack_begin(
        &mut (*t).o_tok,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        Some(malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void),
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    _obstack_begin(
        &mut (*t).o_tok_len,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        Some(malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void),
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
}
unsafe extern "C" fn save_token(mut t: *mut Tokens) {
    let mut len: size_t = ({
        let mut __o: *const obstack = &mut (*t).o_data;
        ((*__o).next_free).offset_from((*__o).object_base) as libc::c_long as size_t
    })
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut s: *const libc::c_char = ({
        let mut __o1: *mut obstack = &mut (*t).o_data as *mut obstack;
        let mut __value: *mut libc::c_void = (*__o1).object_base as *mut libc::c_void;
        if (*__o1).next_free == __value as *mut libc::c_char {
            (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
        }
        (*__o1)
            .next_free = (if (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
            < ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        {
            (*__o1).object_base
        } else {
            0 as *mut libc::c_char
        })
            .offset(
                ((((*__o1).next_free)
                    .offset_from(
                        (if (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                            < ::core::mem::size_of::<*mut libc::c_void>()
                                as libc::c_ulong
                        {
                            (*__o1).object_base
                        } else {
                            0 as *mut libc::c_char
                        }),
                    ) as libc::c_long as libc::c_ulong)
                    .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                    as isize,
            );
        if ((*__o1).next_free).offset_from((*__o1).chunk as *mut libc::c_char)
            as libc::c_long as size_t
            > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut libc::c_char)
                as libc::c_long as size_t
        {
            (*__o1).next_free = (*__o1).chunk_limit;
        }
        (*__o1).object_base = (*__o1).next_free;
        __value
    }) as *const libc::c_char;
    let mut __o: *mut obstack = &mut (*t).o_tok;
    if ({
        let mut __o1: *const obstack = __o;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
    {
        _obstack_newchunk(
            __o,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
        );
    }
    let mut __o1: *mut obstack = __o;
    let mut __p1: *mut libc::c_void = (*__o1).next_free as *mut libc::c_void;
    let ref mut fresh3 = *(__p1 as *mut *const libc::c_void);
    *fresh3 = s as *const libc::c_void;
    (*__o1)
        .next_free = ((*__o1).next_free)
        .offset(::core::mem::size_of::<*const libc::c_void>() as libc::c_ulong as isize);
    let mut __o_0: *mut obstack = &mut (*t).o_tok_len;
    let mut __len: size_t = ::core::mem::size_of::<size_t>() as libc::c_ulong;
    if ({
        let mut __o1_0: *const obstack = __o_0;
        ((*__o1_0).chunk_limit).offset_from((*__o1_0).next_free) as libc::c_long
            as size_t
    }) < __len
    {
        _obstack_newchunk(__o_0, __len);
    }
    memcpy(
        (*__o_0).next_free as *mut libc::c_void,
        &mut len as *mut size_t as *const libc::c_void,
        __len,
    );
    (*__o_0).next_free = ((*__o_0).next_free).offset(__len as isize);
    (*t).n_tok = ((*t).n_tok).wrapping_add(1);
    (*t).n_tok;
}
