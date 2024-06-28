use ::libc;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type ucs4_t = uint32_t;
pub type ptrdiff_t = libc::c_long;
#[no_mangle]
pub unsafe extern "C" fn u8_uctomb(
    mut s: *mut uint8_t,
    mut uc: ucs4_t,
    mut n: ptrdiff_t,
) -> libc::c_int {
    if uc < 0x80 as libc::c_int as libc::c_uint {
        if n > 0 as libc::c_int as libc::c_long {
            *s.offset(0 as libc::c_int as isize) = uc as uint8_t;
            return 1 as libc::c_int;
        }
    } else {
        let mut count: libc::c_int = 0;
        if uc < 0x800 as libc::c_int as libc::c_uint {
            count = 2 as libc::c_int;
        } else if uc < 0x10000 as libc::c_int as libc::c_uint {
            if uc < 0xd800 as libc::c_int as libc::c_uint
                || uc >= 0xe000 as libc::c_int as libc::c_uint
            {
                count = 3 as libc::c_int;
            } else {
                return -(1 as libc::c_int)
            }
        } else if uc < 0x110000 as libc::c_int as libc::c_uint {
            count = 4 as libc::c_int;
        } else {
            return -(1 as libc::c_int)
        }
        if n >= count as libc::c_long {
            let mut current_block_26: u64;
            match count {
                4 => {
                    *s
                        .offset(
                            3 as libc::c_int as isize,
                        ) = (0x80 as libc::c_int as libc::c_uint
                        | uc & 0x3f as libc::c_int as libc::c_uint) as uint8_t;
                    uc = uc >> 6 as libc::c_int;
                    uc |= 0x10000 as libc::c_int as libc::c_uint;
                    current_block_26 = 10236277382995459720;
                }
                3 => {
                    current_block_26 = 10236277382995459720;
                }
                2 => {
                    current_block_26 = 3836233754994685684;
                }
                _ => {
                    current_block_26 = 1109700713171191020;
                }
            }
            match current_block_26 {
                10236277382995459720 => {
                    *s
                        .offset(
                            2 as libc::c_int as isize,
                        ) = (0x80 as libc::c_int as libc::c_uint
                        | uc & 0x3f as libc::c_int as libc::c_uint) as uint8_t;
                    uc = uc >> 6 as libc::c_int;
                    uc |= 0x800 as libc::c_int as libc::c_uint;
                    current_block_26 = 3836233754994685684;
                }
                _ => {}
            }
            match current_block_26 {
                3836233754994685684 => {
                    *s
                        .offset(
                            1 as libc::c_int as isize,
                        ) = (0x80 as libc::c_int as libc::c_uint
                        | uc & 0x3f as libc::c_int as libc::c_uint) as uint8_t;
                    uc = uc >> 6 as libc::c_int;
                    uc |= 0xc0 as libc::c_int as libc::c_uint;
                    *s.offset(0 as libc::c_int as isize) = uc as uint8_t;
                }
                _ => {}
            }
            return count;
        }
    }
    return -(2 as libc::c_int);
}
