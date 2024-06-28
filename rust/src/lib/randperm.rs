use ::libc;
extern "C" {
    pub type randint_source;
    pub type hash_table;
    fn randint_genmax(_: *mut randint_source, genmax: randint) -> randint;
    fn free(_: *mut libc::c_void);
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_free(table: *mut Hash_table);
    fn hash_remove(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_insert(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xalloc_die();
    fn xreallocarray(p: *mut libc::c_void, n: size_t, s: size_t) -> *mut libc::c_void;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
}
pub type __uintmax_t = libc::c_ulong;
pub type uintmax_t = __uintmax_t;
pub type size_t = libc::c_ulong;
pub type randint = uintmax_t;
pub type sparse_map = Hash_table;
pub type Hash_table = hash_table;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sparse_ent_ {
    pub index: size_t,
    pub val: size_t,
}
pub type Hash_tuning = hash_tuning;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_tuning {
    pub shrink_threshold: libc::c_float,
    pub shrink_factor: libc::c_float,
    pub growth_threshold: libc::c_float,
    pub growth_factor: libc::c_float,
    pub is_n_buckets: bool,
}
pub type Hash_data_freer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type Hash_comparator = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_hasher = Option::<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
#[no_mangle]
pub unsafe extern "C" fn randperm_new(
    mut r: *mut randint_source,
    mut h: size_t,
    mut n: size_t,
) -> *mut size_t {
    let mut v: *mut size_t = 0 as *mut size_t;
    match h {
        0 => {
            v = 0 as *mut size_t;
        }
        1 => {
            v = xmalloc(::core::mem::size_of::<size_t>() as libc::c_ulong)
                as *mut size_t;
            *v.offset(0 as libc::c_int as isize) = randint_choose(r, n);
        }
        _ => {
            let mut sparse: bool = n
                >= (128 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
                && n.wrapping_div(h) >= 32 as libc::c_int as libc::c_ulong;
            let mut i: size_t = 0;
            let mut sv: *mut sparse_map = 0 as *mut sparse_map;
            if sparse {
                sv = sparse_new(h.wrapping_mul(2 as libc::c_int as libc::c_ulong));
                if sv.is_null() {
                    xalloc_die();
                }
                v = xnmalloc(h, ::core::mem::size_of::<size_t>() as libc::c_ulong)
                    as *mut size_t;
            } else {
                sv = 0 as *mut sparse_map;
                v = xnmalloc(n, ::core::mem::size_of::<size_t>() as libc::c_ulong)
                    as *mut size_t;
                i = 0 as libc::c_int as size_t;
                while i < n {
                    *v.offset(i as isize) = i;
                    i = i.wrapping_add(1);
                    i;
                }
            }
            i = 0 as libc::c_int as size_t;
            while i < h {
                let mut j: size_t = i.wrapping_add(randint_choose(r, n.wrapping_sub(i)));
                if sparse {
                    sparse_swap(sv, v, i, j);
                } else {
                    swap(v, i, j);
                }
                i = i.wrapping_add(1);
                i;
            }
            if sparse {
                sparse_free(sv);
            } else {
                v = xnrealloc(
                    v as *mut libc::c_void,
                    h,
                    ::core::mem::size_of::<size_t>() as libc::c_ulong,
                ) as *mut size_t;
            }
        }
    }
    return v;
}
#[inline]
unsafe extern "C" fn randint_choose(
    mut s: *mut randint_source,
    mut choices: randint,
) -> randint {
    return randint_genmax(s, choices.wrapping_sub(1 as libc::c_int as libc::c_ulong));
}
#[no_mangle]
pub unsafe extern "C" fn randperm_bound(mut h: size_t, mut n: size_t) -> size_t {
    let mut lg_n: uintmax_t = (floor_lg(n) + 1 as libc::c_int) as uintmax_t;
    let mut ar: uintmax_t = lg_n.wrapping_mul(h);
    let mut bound: size_t = ar
        .wrapping_add(8 as libc::c_int as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(8 as libc::c_int as libc::c_ulong);
    return bound;
}
#[inline]
unsafe extern "C" fn count_leading_zeros(mut x: libc::c_uint) -> libc::c_int {
    return (if x != 0 {
        x.leading_zeros() as i32 as libc::c_ulong
    } else {
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
    }) as libc::c_int;
}
#[inline]
unsafe extern "C" fn count_leading_zeros_l(mut x: libc::c_ulong) -> libc::c_int {
    return (if x != 0 {
        x.leading_zeros() as i32 as libc::c_ulong
    } else {
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
    }) as libc::c_int;
}
#[inline]
unsafe extern "C" fn count_leading_zeros_ll(mut x: libc::c_ulonglong) -> libc::c_int {
    return (if x != 0 {
        x.leading_zeros() as i32 as libc::c_ulong
    } else {
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_ulonglong>() as libc::c_ulong)
    }) as libc::c_int;
}
#[inline]
unsafe extern "C" fn xnrealloc(
    mut p: *mut libc::c_void,
    mut n: size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    return xreallocarray(p, n, s);
}
unsafe extern "C" fn floor_lg(mut n: size_t) -> libc::c_int {
    return if n == 0 as libc::c_int as libc::c_ulong {
        -(1 as libc::c_int)
    } else if 64 as libc::c_int <= 32 as libc::c_int {
        32 as libc::c_int - 1 as libc::c_int - count_leading_zeros(n as libc::c_uint)
    } else if 64 as libc::c_int <= 64 as libc::c_int {
        64 as libc::c_int - 1 as libc::c_int - count_leading_zeros_l(n)
    } else {
        64 as libc::c_int - 1 as libc::c_int
            - count_leading_zeros_ll(n as libc::c_ulonglong)
    };
}
unsafe extern "C" fn swap(mut v: *mut size_t, mut i: size_t, mut j: size_t) {
    let mut t: size_t = *v.offset(i as isize);
    *v.offset(i as isize) = *v.offset(j as isize);
    *v.offset(j as isize) = t;
}
unsafe extern "C" fn sparse_hash_(
    mut x: *const libc::c_void,
    mut table_size: size_t,
) -> size_t {
    let mut ent: *const sparse_ent_ = x as *const sparse_ent_;
    return ((*ent).index).wrapping_rem(table_size);
}
unsafe extern "C" fn sparse_cmp_(
    mut x: *const libc::c_void,
    mut y: *const libc::c_void,
) -> bool {
    let mut ent1: *const sparse_ent_ = x as *const sparse_ent_;
    let mut ent2: *const sparse_ent_ = y as *const sparse_ent_;
    return (*ent1).index == (*ent2).index;
}
unsafe extern "C" fn sparse_new(mut size_hint: size_t) -> *mut sparse_map {
    return hash_initialize(
        size_hint,
        0 as *const Hash_tuning,
        Some(
            sparse_hash_ as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
        ),
        Some(
            sparse_cmp_
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
        ),
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
}
unsafe extern "C" fn sparse_swap(
    mut sv: *mut sparse_map,
    mut v: *mut size_t,
    mut i: size_t,
    mut j: size_t,
) {
    let mut v1: *mut sparse_ent_ = hash_remove(
        sv,
        &mut {
            let mut init = sparse_ent_ {
                index: i,
                val: 0 as libc::c_int as size_t,
            };
            init
        } as *mut sparse_ent_ as *const libc::c_void,
    ) as *mut sparse_ent_;
    let mut v2: *mut sparse_ent_ = hash_remove(
        sv,
        &mut {
            let mut init = sparse_ent_ {
                index: j,
                val: 0 as libc::c_int as size_t,
            };
            init
        } as *mut sparse_ent_ as *const libc::c_void,
    ) as *mut sparse_ent_;
    if v1.is_null() {
        v1 = xmalloc(::core::mem::size_of::<sparse_ent_>() as libc::c_ulong)
            as *mut sparse_ent_;
        (*v1).val = i;
        (*v1).index = (*v1).val;
    }
    if v2.is_null() {
        v2 = xmalloc(::core::mem::size_of::<sparse_ent_>() as libc::c_ulong)
            as *mut sparse_ent_;
        (*v2).val = j;
        (*v2).index = (*v2).val;
    }
    let mut t: size_t = (*v1).val;
    (*v1).val = (*v2).val;
    (*v2).val = t;
    if (hash_insert(sv, v1 as *const libc::c_void)).is_null() {
        xalloc_die();
    }
    if (hash_insert(sv, v2 as *const libc::c_void)).is_null() {
        xalloc_die();
    }
    *v.offset(i as isize) = (*v1).val;
}
unsafe extern "C" fn sparse_free(mut sv: *mut sparse_map) {
    hash_free(sv);
}
