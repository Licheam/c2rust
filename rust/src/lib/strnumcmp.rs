use ::libc;
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn strnumcmp(
    mut a: *const libc::c_char,
    mut b: *const libc::c_char,
    mut decimal_point: libc::c_int,
    mut thousands_sep: libc::c_int,
) -> libc::c_int {
    return numcompare(a, b, decimal_point, thousands_sep);
}
#[inline]
unsafe extern "C" fn numcompare(
    mut a: *const libc::c_char,
    mut b: *const libc::c_char,
    mut decimal_point: libc::c_int,
    mut thousands_sep: libc::c_int,
) -> libc::c_int {
    let mut tmpa: libc::c_uchar = *a as libc::c_uchar;
    let mut tmpb: libc::c_uchar = *b as libc::c_uchar;
    let mut tmp: libc::c_int = 0;
    let mut log_a: size_t = 0;
    let mut log_b: size_t = 0;
    if tmpa as libc::c_int == '-' as i32 {
        loop {
            a = a.offset(1);
            tmpa = *a as libc::c_uchar;
            if !(tmpa as libc::c_int == '0' as i32
                || tmpa as libc::c_int == thousands_sep)
            {
                break;
            }
        }
        if tmpb as libc::c_int != '-' as i32 {
            if tmpa as libc::c_int == decimal_point {
                loop {
                    a = a.offset(1);
                    tmpa = *a as libc::c_uchar;
                    if !(tmpa as libc::c_int == '0' as i32) {
                        break;
                    }
                }
            }
            if (tmpa as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint
            {
                return -(1 as libc::c_int);
            }
            while tmpb as libc::c_int == '0' as i32
                || tmpb as libc::c_int == thousands_sep
            {
                b = b.offset(1);
                tmpb = *b as libc::c_uchar;
            }
            if tmpb as libc::c_int == decimal_point {
                loop {
                    b = b.offset(1);
                    tmpb = *b as libc::c_uchar;
                    if !(tmpb as libc::c_int == '0' as i32) {
                        break;
                    }
                }
            }
            return -(((tmpb as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint) as libc::c_int);
        }
        loop {
            b = b.offset(1);
            tmpb = *b as libc::c_uchar;
            if !(tmpb as libc::c_int == '0' as i32
                || tmpb as libc::c_int == thousands_sep)
            {
                break;
            }
        }
        while tmpa as libc::c_int == tmpb as libc::c_int
            && (tmpa as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint
        {
            loop {
                a = a.offset(1);
                tmpa = *a as libc::c_uchar;
                if !(tmpa as libc::c_int == thousands_sep) {
                    break;
                }
            }
            loop {
                b = b.offset(1);
                tmpb = *b as libc::c_uchar;
                if !(tmpb as libc::c_int == thousands_sep) {
                    break;
                }
            }
        }
        if tmpa as libc::c_int == decimal_point
            && !((tmpb as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint)
            || tmpb as libc::c_int == decimal_point
                && !((tmpa as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                    <= 9 as libc::c_int as libc::c_uint)
        {
            return fraccompare(b, a, decimal_point as libc::c_char);
        }
        tmp = tmpb as libc::c_int - tmpa as libc::c_int;
        log_a = 0 as libc::c_int as size_t;
        while (tmpa as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
            <= 9 as libc::c_int as libc::c_uint
        {
            loop {
                a = a.offset(1);
                tmpa = *a as libc::c_uchar;
                if !(tmpa as libc::c_int == thousands_sep) {
                    break;
                }
            }
            log_a = log_a.wrapping_add(1);
            log_a;
        }
        log_b = 0 as libc::c_int as size_t;
        while (tmpb as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
            <= 9 as libc::c_int as libc::c_uint
        {
            loop {
                b = b.offset(1);
                tmpb = *b as libc::c_uchar;
                if !(tmpb as libc::c_int == thousands_sep) {
                    break;
                }
            }
            log_b = log_b.wrapping_add(1);
            log_b;
        }
        if log_a != log_b {
            return if log_a < log_b { 1 as libc::c_int } else { -(1 as libc::c_int) };
        }
        if log_a == 0 {
            return 0 as libc::c_int;
        }
        return tmp;
    } else if tmpb as libc::c_int == '-' as i32 {
        loop {
            b = b.offset(1);
            tmpb = *b as libc::c_uchar;
            if !(tmpb as libc::c_int == '0' as i32
                || tmpb as libc::c_int == thousands_sep)
            {
                break;
            }
        }
        if tmpb as libc::c_int == decimal_point {
            loop {
                b = b.offset(1);
                tmpb = *b as libc::c_uchar;
                if !(tmpb as libc::c_int == '0' as i32) {
                    break;
                }
            }
        }
        if (tmpb as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
            <= 9 as libc::c_int as libc::c_uint
        {
            return 1 as libc::c_int;
        }
        while tmpa as libc::c_int == '0' as i32 || tmpa as libc::c_int == thousands_sep {
            a = a.offset(1);
            tmpa = *a as libc::c_uchar;
        }
        if tmpa as libc::c_int == decimal_point {
            loop {
                a = a.offset(1);
                tmpa = *a as libc::c_uchar;
                if !(tmpa as libc::c_int == '0' as i32) {
                    break;
                }
            }
        }
        return ((tmpa as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
            <= 9 as libc::c_int as libc::c_uint) as libc::c_int;
    } else {
        while tmpa as libc::c_int == '0' as i32 || tmpa as libc::c_int == thousands_sep {
            a = a.offset(1);
            tmpa = *a as libc::c_uchar;
        }
        while tmpb as libc::c_int == '0' as i32 || tmpb as libc::c_int == thousands_sep {
            b = b.offset(1);
            tmpb = *b as libc::c_uchar;
        }
        while tmpa as libc::c_int == tmpb as libc::c_int
            && (tmpa as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint
        {
            loop {
                a = a.offset(1);
                tmpa = *a as libc::c_uchar;
                if !(tmpa as libc::c_int == thousands_sep) {
                    break;
                }
            }
            loop {
                b = b.offset(1);
                tmpb = *b as libc::c_uchar;
                if !(tmpb as libc::c_int == thousands_sep) {
                    break;
                }
            }
        }
        if tmpa as libc::c_int == decimal_point
            && !((tmpb as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint)
            || tmpb as libc::c_int == decimal_point
                && !((tmpa as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                    <= 9 as libc::c_int as libc::c_uint)
        {
            return fraccompare(a, b, decimal_point as libc::c_char);
        }
        tmp = tmpa as libc::c_int - tmpb as libc::c_int;
        log_a = 0 as libc::c_int as size_t;
        while (tmpa as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
            <= 9 as libc::c_int as libc::c_uint
        {
            loop {
                a = a.offset(1);
                tmpa = *a as libc::c_uchar;
                if !(tmpa as libc::c_int == thousands_sep) {
                    break;
                }
            }
            log_a = log_a.wrapping_add(1);
            log_a;
        }
        log_b = 0 as libc::c_int as size_t;
        while (tmpb as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
            <= 9 as libc::c_int as libc::c_uint
        {
            loop {
                b = b.offset(1);
                tmpb = *b as libc::c_uchar;
                if !(tmpb as libc::c_int == thousands_sep) {
                    break;
                }
            }
            log_b = log_b.wrapping_add(1);
            log_b;
        }
        if log_a != log_b {
            return if log_a < log_b { -(1 as libc::c_int) } else { 1 as libc::c_int };
        }
        if log_a == 0 {
            return 0 as libc::c_int;
        }
        return tmp;
    };
}
#[inline]
unsafe extern "C" fn fraccompare(
    mut a: *const libc::c_char,
    mut b: *const libc::c_char,
    mut decimal_point: libc::c_char,
) -> libc::c_int {
    's_78: {
        let mut current_block_11: u64;
        if *a as libc::c_int == decimal_point as libc::c_int
            && *b as libc::c_int == decimal_point as libc::c_int
        {
            loop {
                a = a.offset(1);
                b = b.offset(1);
                if !(*a as libc::c_int == *b as libc::c_int) {
                    break;
                }
                if !((*a as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                    <= 9 as libc::c_int as libc::c_uint)
                {
                    return 0 as libc::c_int;
                }
            }
            if (*a as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint
                && (*b as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                    <= 9 as libc::c_int as libc::c_uint
            {
                return *a as libc::c_int - *b as libc::c_int;
            }
            if (*a as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint
            {
                current_block_11 = 15573152025693471952;
            } else if (*b as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint
            {
                current_block_11 = 3893944784981150523;
            } else {
                return 0 as libc::c_int
            }
        } else {
            let fresh0 = a;
            a = a.offset(1);
            if *fresh0 as libc::c_int == decimal_point as libc::c_int {
                current_block_11 = 15573152025693471952;
            } else {
                let fresh1 = b;
                b = b.offset(1);
                if *fresh1 as libc::c_int == decimal_point as libc::c_int {
                    current_block_11 = 3893944784981150523;
                } else {
                    break 's_78;
                }
            }
        }
        match current_block_11 {
            15573152025693471952 => {
                while *a as libc::c_int == '0' as i32 {
                    a = a.offset(1);
                    a;
                }
                return ((*a as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                    <= 9 as libc::c_int as libc::c_uint) as libc::c_int;
            }
            _ => {
                while *b as libc::c_int == '0' as i32 {
                    b = b.offset(1);
                    b;
                }
                return -(((*b as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                    <= 9 as libc::c_int as libc::c_uint) as libc::c_int);
            }
        }
    }
    return 0 as libc::c_int;
}
