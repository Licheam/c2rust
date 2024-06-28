use ::libc;
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn filevercmp(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
) -> libc::c_int {
    let mut s1_pos: *const libc::c_char = 0 as *const libc::c_char;
    let mut s2_pos: *const libc::c_char = 0 as *const libc::c_char;
    let mut s1_suffix: *const libc::c_char = 0 as *const libc::c_char;
    let mut s2_suffix: *const libc::c_char = 0 as *const libc::c_char;
    let mut s1_len: size_t = 0;
    let mut s2_len: size_t = 0;
    let mut result: libc::c_int = 0;
    let mut simple_cmp: libc::c_int = strcmp(s1, s2);
    if simple_cmp == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if *s1 == 0 {
        return -(1 as libc::c_int);
    }
    if *s2 == 0 {
        return 1 as libc::c_int;
    }
    if 0 as libc::c_int == strcmp(b".\0" as *const u8 as *const libc::c_char, s1) {
        return -(1 as libc::c_int);
    }
    if 0 as libc::c_int == strcmp(b".\0" as *const u8 as *const libc::c_char, s2) {
        return 1 as libc::c_int;
    }
    if 0 as libc::c_int == strcmp(b"..\0" as *const u8 as *const libc::c_char, s1) {
        return -(1 as libc::c_int);
    }
    if 0 as libc::c_int == strcmp(b"..\0" as *const u8 as *const libc::c_char, s2) {
        return 1 as libc::c_int;
    }
    if *s1 as libc::c_int == '.' as i32 && *s2 as libc::c_int != '.' as i32 {
        return -(1 as libc::c_int);
    }
    if *s1 as libc::c_int != '.' as i32 && *s2 as libc::c_int == '.' as i32 {
        return 1 as libc::c_int;
    }
    if *s1 as libc::c_int == '.' as i32 && *s2 as libc::c_int == '.' as i32 {
        s1 = s1.offset(1);
        s1;
        s2 = s2.offset(1);
        s2;
    }
    s1_pos = s1;
    s2_pos = s2;
    s1_suffix = match_suffix(&mut s1_pos);
    s2_suffix = match_suffix(&mut s2_pos);
    s1_len = (if !s1_suffix.is_null() { s1_suffix } else { s1_pos }).offset_from(s1)
        as libc::c_long as size_t;
    s2_len = (if !s2_suffix.is_null() { s2_suffix } else { s2_pos }).offset_from(s2)
        as libc::c_long as size_t;
    if (!s1_suffix.is_null() || !s2_suffix.is_null()) && s1_len == s2_len
        && 0 as libc::c_int == strncmp(s1, s2, s1_len)
    {
        s1_len = s1_pos.offset_from(s1) as libc::c_long as size_t;
        s2_len = s2_pos.offset_from(s2) as libc::c_long as size_t;
    }
    result = verrevcmp(s1, s1_len, s2, s2_len);
    return if result == 0 as libc::c_int { simple_cmp } else { result };
}
#[inline]
unsafe extern "C" fn c_isalnum(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101 | 102
        | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115
        | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72
        | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88
        | 89 | 90 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isalpha(mut c: libc::c_int) -> bool {
    match c {
        97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110
        | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 65 | 66
        | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82
        | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
unsafe extern "C" fn match_suffix(
    mut str: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut match_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut read_alpha: bool = 0 as libc::c_int != 0;
    while **str != 0 {
        if read_alpha {
            read_alpha = 0 as libc::c_int != 0;
            if !c_isalpha(**str as libc::c_int) && '~' as i32 != **str as libc::c_int {
                match_0 = 0 as *const libc::c_char;
            }
        } else if '.' as i32 == **str as libc::c_int {
            read_alpha = 1 as libc::c_int != 0;
            if match_0.is_null() {
                match_0 = *str;
            }
        } else if !c_isalnum(**str as libc::c_int) && '~' as i32 != **str as libc::c_int
        {
            match_0 = 0 as *const libc::c_char;
        }
        *str = (*str).offset(1);
        *str;
    }
    return match_0;
}
unsafe extern "C" fn order(mut c: libc::c_uchar) -> libc::c_int {
    if c_isdigit(c as libc::c_int) {
        return 0 as libc::c_int
    } else if c_isalpha(c as libc::c_int) {
        return c as libc::c_int
    } else if c as libc::c_int == '~' as i32 {
        return -(1 as libc::c_int)
    } else {
        return c as libc::c_int
            + (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
            + 1 as libc::c_int
    };
}
unsafe extern "C" fn verrevcmp(
    mut s1: *const libc::c_char,
    mut s1_len: size_t,
    mut s2: *const libc::c_char,
    mut s2_len: size_t,
) -> libc::c_int {
    let mut s1_pos: size_t = 0 as libc::c_int as size_t;
    let mut s2_pos: size_t = 0 as libc::c_int as size_t;
    while s1_pos < s1_len || s2_pos < s2_len {
        let mut first_diff: libc::c_int = 0 as libc::c_int;
        while s1_pos < s1_len && !c_isdigit(*s1.offset(s1_pos as isize) as libc::c_int)
            || s2_pos < s2_len && !c_isdigit(*s2.offset(s2_pos as isize) as libc::c_int)
        {
            let mut s1_c: libc::c_int = if s1_pos == s1_len {
                0 as libc::c_int
            } else {
                order(*s1.offset(s1_pos as isize) as libc::c_uchar)
            };
            let mut s2_c: libc::c_int = if s2_pos == s2_len {
                0 as libc::c_int
            } else {
                order(*s2.offset(s2_pos as isize) as libc::c_uchar)
            };
            if s1_c != s2_c {
                return s1_c - s2_c;
            }
            s1_pos = s1_pos.wrapping_add(1);
            s1_pos;
            s2_pos = s2_pos.wrapping_add(1);
            s2_pos;
        }
        while *s1.offset(s1_pos as isize) as libc::c_int == '0' as i32 {
            s1_pos = s1_pos.wrapping_add(1);
            s1_pos;
        }
        while *s2.offset(s2_pos as isize) as libc::c_int == '0' as i32 {
            s2_pos = s2_pos.wrapping_add(1);
            s2_pos;
        }
        while c_isdigit(*s1.offset(s1_pos as isize) as libc::c_int) as libc::c_int != 0
            && c_isdigit(*s2.offset(s2_pos as isize) as libc::c_int) as libc::c_int != 0
        {
            if first_diff == 0 {
                first_diff = *s1.offset(s1_pos as isize) as libc::c_int
                    - *s2.offset(s2_pos as isize) as libc::c_int;
            }
            s1_pos = s1_pos.wrapping_add(1);
            s1_pos;
            s2_pos = s2_pos.wrapping_add(1);
            s2_pos;
        }
        if c_isdigit(*s1.offset(s1_pos as isize) as libc::c_int) {
            return 1 as libc::c_int;
        }
        if c_isdigit(*s2.offset(s2_pos as isize) as libc::c_int) {
            return -(1 as libc::c_int);
        }
        if first_diff != 0 {
            return first_diff;
        }
    }
    return 0 as libc::c_int;
}
