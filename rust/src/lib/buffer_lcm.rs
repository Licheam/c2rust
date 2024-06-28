use ::libc;
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn buffer_lcm(
    mut a: size_t,
    mut b: size_t,
    mut lcm_max: size_t,
) -> size_t {
    let mut size: size_t = 0;
    if a == 0 {
        size = if b != 0 {
            b
        } else {
            (8 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
        };
    } else {
        if b != 0 {
            let mut lcm: size_t = 0;
            let mut m: size_t = 0;
            let mut n: size_t = 0;
            let mut q: size_t = 0;
            let mut r: size_t = 0;
            m = a;
            n = b;
            loop {
                r = m.wrapping_rem(n);
                if !(r != 0 as libc::c_int as libc::c_ulong) {
                    break;
                }
                m = n;
                n = r;
            }
            q = a.wrapping_div(n);
            lcm = q.wrapping_mul(b);
            if lcm <= lcm_max && lcm.wrapping_div(b) == q {
                return lcm;
            }
        }
        size = a;
    }
    return if size <= lcm_max { size } else { lcm_max };
}
