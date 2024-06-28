use ::libc;
extern "C" {
    pub type hash_table;
    fn hash_free(table: *mut Hash_table);
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_insert(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type __ino_t = libc::c_ulong;
pub type ino_t = __ino_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ino_map {
    pub map: *mut hash_table,
    pub next_mapped_ino: size_t,
    pub probe: *mut ino_map_ent,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ino_map_ent {
    pub ino: ino_t,
    pub mapped_ino: size_t,
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
pub const INITIAL_INO_MAP_TABLE_SIZE: C2RustUnnamed = 1021;
pub type Hash_data_freer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type Hash_comparator = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_hasher = Option::<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
pub type C2RustUnnamed = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn ino_map_insert(mut im: *mut ino_map, mut ino: ino_t) -> size_t {
    let mut ent: *mut ino_map_ent = 0 as *mut ino_map_ent;
    let mut probe: *mut ino_map_ent = (*im).probe;
    if !probe.is_null() {
        if (*probe).ino == ino {
            return (*probe).mapped_ino;
        }
    } else {
        probe = malloc(::core::mem::size_of::<ino_map_ent>() as libc::c_ulong)
            as *mut ino_map_ent;
        (*im).probe = probe;
        if probe.is_null() {
            return -(1 as libc::c_int) as size_t;
        }
    }
    (*probe).ino = ino;
    ent = hash_insert((*im).map, probe as *const libc::c_void) as *mut ino_map_ent;
    if ent.is_null() {
        return -(1 as libc::c_int) as size_t;
    }
    if ent != probe {
        (*probe).mapped_ino = (*ent).mapped_ino;
    } else {
        (*im).probe = 0 as *mut ino_map_ent;
        let fresh0 = (*im).next_mapped_ino;
        (*im).next_mapped_ino = ((*im).next_mapped_ino).wrapping_add(1);
        (*probe).mapped_ino = fresh0;
    }
    return (*probe).mapped_ino;
}
#[no_mangle]
pub unsafe extern "C" fn ino_map_alloc(mut next_mapped_ino: size_t) -> *mut ino_map {
    let mut im: *mut ino_map = malloc(::core::mem::size_of::<ino_map>() as libc::c_ulong)
        as *mut ino_map;
    if !im.is_null() {
        (*im)
            .map = hash_initialize(
            INITIAL_INO_MAP_TABLE_SIZE as libc::c_int as size_t,
            0 as *const Hash_tuning,
            Some(
                ino_hash as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
            ),
            Some(
                ino_compare
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> bool,
            ),
            Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
        if ((*im).map).is_null() {
            free(im as *mut libc::c_void);
            return 0 as *mut ino_map;
        }
        (*im).next_mapped_ino = next_mapped_ino;
        (*im).probe = 0 as *mut ino_map_ent;
    }
    return im;
}
#[no_mangle]
pub unsafe extern "C" fn ino_map_free(mut map: *mut ino_map) {
    hash_free((*map).map);
    free((*map).probe as *mut libc::c_void);
    free(map as *mut libc::c_void);
}
unsafe extern "C" fn ino_hash(
    mut x: *const libc::c_void,
    mut table_size: size_t,
) -> size_t {
    let mut p: *const ino_map_ent = x as *const ino_map_ent;
    let mut ino: ino_t = (*p).ino;
    let mut h: size_t = ino;
    let mut i: libc::c_uint = 0;
    let mut n_words: libc::c_uint = (::core::mem::size_of::<ino_t>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_add(
            ((::core::mem::size_of::<ino_t>() as libc::c_ulong)
                .wrapping_rem(::core::mem::size_of::<size_t>() as libc::c_ulong)
                != 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_ulong,
        ) as libc::c_uint;
    i = 1 as libc::c_int as libc::c_uint;
    while i < n_words {
        h
            ^= ino
                >> (8 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<size_t>() as libc::c_ulong)
                    .wrapping_mul(i as libc::c_ulong);
        i = i.wrapping_add(1);
        i;
    }
    return h.wrapping_rem(table_size);
}
unsafe extern "C" fn ino_compare(
    mut x: *const libc::c_void,
    mut y: *const libc::c_void,
) -> bool {
    let mut a: *const ino_map_ent = x as *const ino_map_ent;
    let mut b: *const ino_map_ent = y as *const ino_map_ent;
    return (*a).ino == (*b).ino;
}
