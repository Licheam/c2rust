use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn xreallocarray(p: *mut libc::c_void, n: size_t, s: size_t) -> *mut libc::c_void;
    fn xpalloc(
        pa: *mut libc::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut libc::c_void;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tokenbuffer {
    pub size: size_t,
    pub buffer: *mut libc::c_char,
}
pub type token_buffer = tokenbuffer;
pub type idx_t = ptrdiff_t;
pub type word = size_t;
pub const bits_per_word: C2RustUnnamed = 64;
pub type C2RustUnnamed = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn readtokens(
    mut stream: *mut FILE,
    mut projected_n_tokens: size_t,
    mut delim: *const libc::c_char,
    mut n_delim: size_t,
    mut tokens_out: *mut *mut *mut libc::c_char,
    mut token_lengths: *mut *mut size_t,
) -> size_t {
    let mut tb: token_buffer = token_buffer {
        size: 0,
        buffer: 0 as *mut libc::c_char,
    };
    let mut token: *mut token_buffer = &mut tb;
    let mut tokens: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut lengths: *mut size_t = 0 as *mut size_t;
    let mut sz: idx_t = 0;
    let mut n_tokens: idx_t = 0;
    if projected_n_tokens == 0 as libc::c_int as libc::c_ulong {
        projected_n_tokens = 64 as libc::c_int as size_t;
    } else {
        projected_n_tokens = projected_n_tokens.wrapping_add(1);
        projected_n_tokens;
    }
    sz = projected_n_tokens as idx_t;
    tokens = xnmalloc(
        sz as size_t,
        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    lengths = xnmalloc(sz as size_t, ::core::mem::size_of::<size_t>() as libc::c_ulong)
        as *mut size_t;
    n_tokens = 0 as libc::c_int as idx_t;
    init_tokenbuffer(token);
    loop {
        let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut token_length: size_t = readtoken(stream, delim, n_delim, token);
        if n_tokens >= sz {
            tokens = xpalloc(
                tokens as *mut libc::c_void,
                &mut sz,
                1 as libc::c_int as idx_t,
                -(1 as libc::c_int) as ptrdiff_t,
                ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong as idx_t,
            ) as *mut *mut libc::c_char;
            lengths = xreallocarray(
                lengths as *mut libc::c_void,
                sz as size_t,
                ::core::mem::size_of::<size_t>() as libc::c_ulong,
            ) as *mut size_t;
        }
        if token_length == -(1 as libc::c_int) as size_t {
            let ref mut fresh0 = *tokens.offset(n_tokens as isize);
            *fresh0 = 0 as *mut libc::c_char;
            *lengths.offset(n_tokens as isize) = 0 as libc::c_int as size_t;
            break;
        } else {
            tmp = xnmalloc(
                token_length.wrapping_add(1 as libc::c_int as libc::c_ulong),
                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
            ) as *mut libc::c_char;
            *lengths.offset(n_tokens as isize) = token_length;
            let ref mut fresh1 = *tokens.offset(n_tokens as isize);
            *fresh1 = memcpy(
                tmp as *mut libc::c_void,
                (*token).buffer as *const libc::c_void,
                token_length.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            n_tokens += 1;
            n_tokens;
        }
    }
    free((*token).buffer as *mut libc::c_void);
    *tokens_out = tokens;
    if !token_lengths.is_null() {
        *token_lengths = lengths;
    } else {
        free(lengths as *mut libc::c_void);
    }
    return n_tokens as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn readtoken(
    mut stream: *mut FILE,
    mut delim: *const libc::c_char,
    mut n_delim: size_t,
    mut tokenbuffer: *mut token_buffer,
) -> size_t {
    let mut c: libc::c_int = 0;
    let mut i: idx_t = 0;
    let mut isdelim: [word; 4] = [0; 4];
    memset(
        isdelim.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[word; 4]>() as libc::c_ulong,
    );
    i = 0 as libc::c_int as idx_t;
    while (i as libc::c_ulong) < n_delim {
        let mut ch: libc::c_uchar = *delim.offset(i as isize) as libc::c_uchar;
        set_nth_bit(ch as size_t, isdelim.as_mut_ptr());
        i += 1;
        i;
    }
    c = getc_unlocked(stream);
    while c >= 0 as libc::c_int
        && get_nth_bit(c as size_t, isdelim.as_mut_ptr()) as libc::c_int != 0
    {
        c = getc_unlocked(stream);
    }
    let mut p: *mut libc::c_char = (*tokenbuffer).buffer;
    let mut n: idx_t = (*tokenbuffer).size as idx_t;
    i = 0 as libc::c_int as idx_t;
    loop {
        if c < 0 as libc::c_int && i == 0 as libc::c_int as libc::c_long {
            return -(1 as libc::c_int) as size_t;
        }
        if i == n {
            p = xpalloc(
                p as *mut libc::c_void,
                &mut n,
                1 as libc::c_int as idx_t,
                -(1 as libc::c_int) as ptrdiff_t,
                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong as idx_t,
            ) as *mut libc::c_char;
        }
        if c < 0 as libc::c_int {
            *p.offset(i as isize) = 0 as libc::c_int as libc::c_char;
            break;
        } else if get_nth_bit(c as size_t, isdelim.as_mut_ptr()) {
            *p.offset(i as isize) = 0 as libc::c_int as libc::c_char;
            break;
        } else {
            let fresh2 = i;
            i = i + 1;
            *p.offset(fresh2 as isize) = c as libc::c_char;
            c = getc_unlocked(stream);
        }
    }
    (*tokenbuffer).buffer = p;
    (*tokenbuffer).size = n as size_t;
    return i as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn init_tokenbuffer(mut tokenbuffer: *mut token_buffer) {
    (*tokenbuffer).size = 0 as libc::c_int as size_t;
    (*tokenbuffer).buffer = 0 as *mut libc::c_char;
}
unsafe extern "C" fn get_nth_bit(mut n: size_t, mut bitset: *const word) -> bool {
    return *bitset
        .offset(n.wrapping_div(bits_per_word as libc::c_int as libc::c_ulong) as isize)
        >> n.wrapping_rem(bits_per_word as libc::c_int as libc::c_ulong)
        & 1 as libc::c_int as libc::c_ulong != 0;
}
unsafe extern "C" fn set_nth_bit(mut n: size_t, mut bitset: *mut word) {
    let mut one: size_t = 1 as libc::c_int as size_t;
    let ref mut fresh3 = *bitset
        .offset(n.wrapping_div(bits_per_word as libc::c_int as libc::c_ulong) as isize);
    *fresh3 |= one << n.wrapping_rem(bits_per_word as libc::c_int as libc::c_ulong);
}
