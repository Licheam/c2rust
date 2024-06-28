use ::libc;
extern "C" {
    fn wcstombs(__s: *mut libc::c_char, __pwcs: *const wchar_t, __n: size_t) -> size_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn mbstowcs(__pwcs: *mut wchar_t, __s: *const libc::c_char, __n: size_t) -> size_t;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn free(_: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn mempcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn wcwidth(__c: wchar_t) -> libc::c_int;
    fn wcswidth(__s: *const wchar_t, __n: size_t) -> libc::c_int;
    fn iswprint(__wc: wint_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type mbs_align_t = libc::c_uint;
pub const MBS_ALIGN_CENTER: mbs_align_t = 2;
pub const MBS_ALIGN_RIGHT: mbs_align_t = 1;
pub const MBS_ALIGN_LEFT: mbs_align_t = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const MBA_NO_RIGHT_PAD: C2RustUnnamed = 8;
pub const MBA_NO_LEFT_PAD: C2RustUnnamed = 4;
pub const MBA_UNIBYTE_ONLY: C2RustUnnamed = 2;
pub const MBA_UNIBYTE_FALLBACK: C2RustUnnamed = 1;
pub type wint_t = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn mbsalign(
    mut src: *const libc::c_char,
    mut dest: *mut libc::c_char,
    mut dest_size: size_t,
    mut width: *mut size_t,
    mut align: mbs_align_t,
    mut flags: libc::c_int,
) -> size_t {
    let mut current_block: u64;
    let mut ret: size_t = 18446744073709551615 as libc::c_ulong;
    let mut src_size: size_t = (strlen(src))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut newstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str_wc: *mut wchar_t = 0 as *mut wchar_t;
    let mut str_to_print: *const libc::c_char = src;
    let mut n_cols: size_t = src_size.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut n_used_bytes: size_t = n_cols;
    let mut n_spaces: size_t = 0 as libc::c_int as size_t;
    let mut conversion: bool = 0 as libc::c_int != 0;
    let mut wc_enabled: bool = 0 as libc::c_int != 0;
    if flags & MBA_UNIBYTE_ONLY as libc::c_int == 0
        && __ctype_get_mb_cur_max() > 1 as libc::c_int as libc::c_ulong
    {
        let mut src_chars: size_t = mbstowcs(
            0 as *mut wchar_t,
            src,
            0 as libc::c_int as size_t,
        );
        if src_chars == 18446744073709551615 as libc::c_ulong {
            if flags & MBA_UNIBYTE_FALLBACK as libc::c_int != 0 {
                current_block = 10895712959632226215;
            } else {
                current_block = 2434533687135482962;
            }
        } else {
            src_chars = (src_chars as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
            str_wc = malloc(
                src_chars
                    .wrapping_mul(::core::mem::size_of::<wchar_t>() as libc::c_ulong),
            ) as *mut wchar_t;
            if str_wc.is_null() {
                if flags & MBA_UNIBYTE_FALLBACK as libc::c_int != 0 {
                    current_block = 10895712959632226215;
                } else {
                    current_block = 2434533687135482962;
                }
            } else {
                if mbstowcs(str_wc, src, src_chars) != 0 as libc::c_int as libc::c_ulong
                {
                    *str_wc
                        .offset(
                            src_chars.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) = '\0' as i32;
                    wc_enabled = 1 as libc::c_int != 0;
                    conversion = wc_ensure_printable(str_wc);
                    n_cols = wcswidth(str_wc, src_chars) as size_t;
                }
                current_block = 7149356873433890176;
            }
        }
    } else {
        current_block = 7149356873433890176;
    }
    match current_block {
        7149356873433890176 => {
            if wc_enabled as libc::c_int != 0
                && (conversion as libc::c_int != 0 || n_cols > *width)
            {
                if conversion {
                    src_size = (wcstombs(
                        0 as *mut libc::c_char,
                        str_wc,
                        0 as libc::c_int as size_t,
                    ))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong);
                }
                newstr = malloc(src_size) as *mut libc::c_char;
                if newstr.is_null() {
                    if flags & MBA_UNIBYTE_FALLBACK as libc::c_int != 0 {
                        current_block = 10895712959632226215;
                    } else {
                        current_block = 2434533687135482962;
                    }
                } else {
                    str_to_print = newstr;
                    n_cols = wc_truncate(str_wc, *width);
                    n_used_bytes = wcstombs(newstr, str_wc, src_size);
                    current_block = 10895712959632226215;
                }
            } else {
                current_block = 10895712959632226215;
            }
        }
        _ => {}
    }
    match current_block {
        10895712959632226215 => {
            if n_cols > *width {
                n_cols = *width;
                n_used_bytes = n_cols;
            }
            if *width > n_cols {
                n_spaces = (*width).wrapping_sub(n_cols);
            }
            *width = n_cols;
            let mut start_spaces: size_t = 0;
            let mut end_spaces: size_t = 0;
            match align as libc::c_uint {
                0 => {
                    start_spaces = 0 as libc::c_int as size_t;
                    end_spaces = n_spaces;
                }
                1 => {
                    start_spaces = n_spaces;
                    end_spaces = 0 as libc::c_int as size_t;
                }
                2 | _ => {
                    start_spaces = n_spaces
                        .wrapping_div(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            n_spaces.wrapping_rem(2 as libc::c_int as libc::c_ulong),
                        );
                    end_spaces = n_spaces
                        .wrapping_div(2 as libc::c_int as libc::c_ulong);
                }
            }
            if flags & MBA_NO_LEFT_PAD as libc::c_int != 0 {
                start_spaces = 0 as libc::c_int as size_t;
            }
            if flags & MBA_NO_RIGHT_PAD as libc::c_int != 0 {
                end_spaces = 0 as libc::c_int as size_t;
            }
            if dest_size != 0 as libc::c_int as libc::c_ulong {
                let mut space_left: size_t = 0;
                let mut dest_end: *mut libc::c_char = dest
                    .offset(dest_size as isize)
                    .offset(-(1 as libc::c_int as isize));
                dest = mbs_align_pad(dest, dest_end, start_spaces);
                space_left = dest_end.offset_from(dest) as libc::c_long as size_t;
                dest = mempcpy(
                    dest as *mut libc::c_void,
                    str_to_print as *const libc::c_void,
                    if n_used_bytes < space_left { n_used_bytes } else { space_left },
                ) as *mut libc::c_char;
                mbs_align_pad(dest, dest_end, end_spaces);
            }
            ret = n_used_bytes
                .wrapping_add(
                    start_spaces
                        .wrapping_add(end_spaces)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong),
                );
        }
        _ => {}
    }
    free(str_wc as *mut libc::c_void);
    free(newstr as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ambsalign(
    mut src: *const libc::c_char,
    mut width: *mut size_t,
    mut align: mbs_align_t,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    let mut orig_width: size_t = *width;
    let mut size: size_t = *width;
    let mut req: size_t = size;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    while req >= size {
        let mut nbuf: *mut libc::c_char = 0 as *mut libc::c_char;
        size = req.wrapping_add(1 as libc::c_int as libc::c_ulong);
        nbuf = realloc(buf as *mut libc::c_void, size) as *mut libc::c_char;
        if nbuf.is_null() {
            free(buf as *mut libc::c_void);
            buf = 0 as *mut libc::c_char;
            break;
        } else {
            buf = nbuf;
            *width = orig_width;
            req = mbsalign(src, buf, size, width, align, flags);
            if !(req == 18446744073709551615 as libc::c_ulong) {
                continue;
            }
            free(buf as *mut libc::c_void);
            buf = 0 as *mut libc::c_char;
            break;
        }
    }
    return buf;
}
unsafe extern "C" fn wc_ensure_printable(mut wchars: *mut wchar_t) -> bool {
    let mut replaced: bool = 0 as libc::c_int != 0;
    let mut wc: *mut wchar_t = wchars;
    while *wc != 0 {
        if iswprint(*wc as wint_t) == 0 {
            *wc = 0xfffd as libc::c_int;
            replaced = 1 as libc::c_int != 0;
        }
        wc = wc.offset(1);
        wc;
    }
    return replaced;
}
unsafe extern "C" fn wc_truncate(mut wc: *mut wchar_t, mut width: size_t) -> size_t {
    let mut cells: size_t = 0 as libc::c_int as size_t;
    let mut next_cells: libc::c_int = 0 as libc::c_int;
    while *wc != 0 {
        next_cells = wcwidth(*wc);
        if next_cells == -(1 as libc::c_int) {
            *wc = 0xfffd as libc::c_int;
            next_cells = 1 as libc::c_int;
        }
        if cells.wrapping_add(next_cells as libc::c_ulong) > width {
            break;
        }
        cells = (cells as libc::c_ulong).wrapping_add(next_cells as libc::c_ulong)
            as size_t as size_t;
        wc = wc.offset(1);
        wc;
    }
    *wc = '\0' as i32;
    return cells;
}
unsafe extern "C" fn mbs_align_pad(
    mut dest: *mut libc::c_char,
    mut dest_end: *const libc::c_char,
    mut n_spaces: size_t,
) -> *mut libc::c_char {
    loop {
        let fresh0 = n_spaces;
        n_spaces = n_spaces.wrapping_sub(1);
        if !(fresh0 != 0 && dest < dest_end as *mut libc::c_char) {
            break;
        }
        let fresh1 = dest;
        dest = dest.offset(1);
        *fresh1 = ' ' as i32 as libc::c_char;
    }
    *dest = '\0' as i32 as libc::c_char;
    return dest;
}
