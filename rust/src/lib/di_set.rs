use ::libc;
extern "C" {
    pub type hash_table;
    pub type ino_map;
    fn hash_lookup(
        table: *const Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_insert_if_absent(
        table: *mut Hash_table,
        entry: *const libc::c_void,
        matched_ent: *mut *const libc::c_void,
    ) -> libc::c_int;
    fn hash_insert(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_free(table: *mut Hash_table);
    fn ino_map_free(_: *mut ino_map);
    fn ino_map_alloc(_: size_t) -> *mut ino_map;
    fn ino_map_insert(_: *mut ino_map, _: ino_t) -> size_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type __dev_t = libc::c_ulong;
pub type __ino_t = libc::c_ulong;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct di_set {
    pub dev_map: *mut hash_table,
    pub ino_map: *mut ino_map,
    pub probe: *mut di_ent,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct di_ent {
    pub dev: dev_t,
    pub ino_set: *mut hash_table,
}
pub type Hash_table = hash_table;
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
pub const INITIAL_DEV_MAP_SIZE: C2RustUnnamed = 11;
pub type Hash_data_freer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type Hash_comparator = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_hasher = Option::<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
pub type C2RustUnnamed = libc::c_uint;
pub type hashint = size_t;
pub const INITIAL_INO_SET_SIZE: C2RustUnnamed_0 = 1021;
pub type C2RustUnnamed_0 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn di_set_alloc() -> *mut di_set {
    let mut dis: *mut di_set = malloc(::core::mem::size_of::<di_set>() as libc::c_ulong)
        as *mut di_set;
    if !dis.is_null() {
        (*dis)
            .dev_map = hash_initialize(
            INITIAL_DEV_MAP_SIZE as libc::c_int as size_t,
            0 as *const Hash_tuning,
            Some(
                di_ent_hash
                    as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
            ),
            Some(
                di_ent_compare
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> bool,
            ),
            Some(di_ent_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
        if ((*dis).dev_map).is_null() {
            free(dis as *mut libc::c_void);
            return 0 as *mut di_set;
        }
        (*dis).ino_map = 0 as *mut ino_map;
        (*dis).probe = 0 as *mut di_ent;
    }
    return dis;
}
#[no_mangle]
pub unsafe extern "C" fn di_set_free(mut dis: *mut di_set) {
    hash_free((*dis).dev_map);
    if !((*dis).ino_map).is_null() {
        ino_map_free((*dis).ino_map);
    }
    free((*dis).probe as *mut libc::c_void);
    free(dis as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn di_set_insert(
    mut dis: *mut di_set,
    mut dev: dev_t,
    mut ino: ino_t,
) -> libc::c_int {
    let mut i: hashint = 0;
    let mut ino_set: *mut hash_table = map_device(dis, dev);
    if ino_set.is_null() {
        return -(1 as libc::c_int);
    }
    i = map_inode_number(dis, ino);
    if i == -(1 as libc::c_int) as size_t {
        return -(1 as libc::c_int);
    }
    return hash_insert_if_absent(
        ino_set,
        i as *const libc::c_void,
        0 as *mut *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn di_set_lookup(
    mut dis: *mut di_set,
    mut dev: dev_t,
    mut ino: ino_t,
) -> libc::c_int {
    let mut i: hashint = 0;
    let mut ino_set: *mut hash_table = map_device(dis, dev);
    if ino_set.is_null() {
        return -(1 as libc::c_int);
    }
    i = map_inode_number(dis, ino);
    if i == -(1 as libc::c_int) as size_t {
        return -(1 as libc::c_int);
    }
    return !(hash_lookup(ino_set, i as *const libc::c_void)).is_null() as libc::c_int;
}
unsafe extern "C" fn di_ent_hash(
    mut x: *const libc::c_void,
    mut table_size: size_t,
) -> size_t {
    let mut p: *const di_ent = x as *const di_ent;
    let mut dev: dev_t = (*p).dev;
    let mut h: size_t = dev;
    let mut i: libc::c_uint = 0;
    let mut n_words: libc::c_uint = (::core::mem::size_of::<dev_t>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_add(
            ((::core::mem::size_of::<dev_t>() as libc::c_ulong)
                .wrapping_rem(::core::mem::size_of::<size_t>() as libc::c_ulong)
                != 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_ulong,
        ) as libc::c_uint;
    i = 1 as libc::c_int as libc::c_uint;
    while i < n_words {
        h
            ^= dev
                >> (8 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<size_t>() as libc::c_ulong)
                    .wrapping_mul(i as libc::c_ulong);
        i = i.wrapping_add(1);
        i;
    }
    return h.wrapping_rem(table_size);
}
unsafe extern "C" fn di_ent_compare(
    mut x: *const libc::c_void,
    mut y: *const libc::c_void,
) -> bool {
    let mut a: *const di_ent = x as *const di_ent;
    let mut b: *const di_ent = y as *const di_ent;
    return (*a).dev == (*b).dev;
}
unsafe extern "C" fn di_ent_free(mut v: *mut libc::c_void) {
    let mut a: *mut di_ent = v as *mut di_ent;
    hash_free((*a).ino_set);
    free(a as *mut libc::c_void);
}
unsafe extern "C" fn di_ino_hash(
    mut i: *const libc::c_void,
    mut table_size: size_t,
) -> size_t {
    return (i as hashint).wrapping_rem(table_size);
}
unsafe extern "C" fn map_device(
    mut dis: *mut di_set,
    mut dev: dev_t,
) -> *mut hash_table {
    let mut ent: *mut di_ent = 0 as *mut di_ent;
    let mut probe: *mut di_ent = (*dis).probe;
    if !probe.is_null() {
        if (*probe).dev == dev {
            return (*probe).ino_set;
        }
    } else {
        probe = malloc(::core::mem::size_of::<di_ent>() as libc::c_ulong) as *mut di_ent;
        (*dis).probe = probe;
        if probe.is_null() {
            return 0 as *mut hash_table;
        }
    }
    (*probe).dev = dev;
    ent = hash_insert((*dis).dev_map, probe as *const libc::c_void) as *mut di_ent;
    if ent.is_null() {
        return 0 as *mut hash_table;
    }
    if ent != probe {
        (*probe).ino_set = (*ent).ino_set;
    } else {
        (*dis).probe = 0 as *mut di_ent;
        (*probe)
            .ino_set = hash_initialize(
            INITIAL_INO_SET_SIZE as libc::c_int as size_t,
            0 as *const Hash_tuning,
            Some(
                di_ino_hash
                    as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
            ),
            None,
            None,
        );
    }
    return (*probe).ino_set;
}
unsafe extern "C" fn map_inode_number(mut dis: *mut di_set, mut ino: ino_t) -> hashint {
    if (0 as libc::c_int as libc::c_ulong) < ino
        && ino
            < (-(1 as libc::c_int) as hashint)
                .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        return ino;
    }
    if ((*dis).ino_map).is_null() {
        (*dis)
            .ino_map = ino_map_alloc(
            (-(1 as libc::c_int) as hashint)
                .wrapping_div(2 as libc::c_int as libc::c_ulong),
        );
        if ((*dis).ino_map).is_null() {
            return -(1 as libc::c_int) as size_t;
        }
    }
    return ino_map_insert((*dis).ino_map, ino);
}
