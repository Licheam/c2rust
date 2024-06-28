use ::libc;
extern "C" {
    fn free(_: *mut libc::c_void);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn x2nrealloc(p: *mut libc::c_void, pn: *mut size_t, s: size_t) -> *mut libc::c_void;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct heap {
    pub array: *mut *mut libc::c_void,
    pub capacity: size_t,
    pub count: size_t,
    pub compare: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
}
#[no_mangle]
pub unsafe extern "C" fn heap_alloc(
    mut compare: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
    mut n_reserve: size_t,
) -> *mut heap {
    let mut heap: *mut heap = xmalloc(::core::mem::size_of::<heap>() as libc::c_ulong)
        as *mut heap;
    if n_reserve == 0 as libc::c_int as libc::c_ulong {
        n_reserve = 1 as libc::c_int as size_t;
    }
    (*heap)
        .array = xnmalloc(
        n_reserve,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
    ) as *mut *mut libc::c_void;
    let ref mut fresh0 = *((*heap).array).offset(0 as libc::c_int as isize);
    *fresh0 = 0 as *mut libc::c_void;
    (*heap).capacity = n_reserve;
    (*heap).count = 0 as libc::c_int as size_t;
    (*heap)
        .compare = if compare.is_some() {
        compare
    } else {
        Some(
            heap_default_compare
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        )
    };
    return heap;
}
#[no_mangle]
pub unsafe extern "C" fn heap_free(mut heap: *mut heap) {
    free((*heap).array as *mut libc::c_void);
    free(heap as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn heap_insert(
    mut heap: *mut heap,
    mut item: *mut libc::c_void,
) -> libc::c_int {
    if ((*heap).capacity).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        <= (*heap).count
    {
        (*heap)
            .array = x2nrealloc(
            (*heap).array as *mut libc::c_void,
            &mut (*heap).capacity,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
        ) as *mut *mut libc::c_void;
    }
    (*heap).count = ((*heap).count).wrapping_add(1);
    let ref mut fresh1 = *((*heap).array).offset((*heap).count as isize);
    *fresh1 = item;
    heapify_up((*heap).array, (*heap).count, (*heap).compare);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn heap_remove_top(mut heap: *mut heap) -> *mut libc::c_void {
    let mut top: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*heap).count == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    top = *((*heap).array).offset(1 as libc::c_int as isize);
    let fresh2 = (*heap).count;
    (*heap).count = ((*heap).count).wrapping_sub(1);
    let ref mut fresh3 = *((*heap).array).offset(1 as libc::c_int as isize);
    *fresh3 = *((*heap).array).offset(fresh2 as isize);
    heapify_down(
        (*heap).array,
        (*heap).count,
        1 as libc::c_int as size_t,
        (*heap).compare,
    );
    return top;
}
unsafe extern "C" fn heap_default_compare(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn heapify_down(
    mut array: *mut *mut libc::c_void,
    mut count: size_t,
    mut initial: size_t,
    mut compare: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
) -> size_t {
    let mut element: *mut libc::c_void = *array.offset(initial as isize);
    let mut parent: size_t = initial;
    while parent <= count.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut child: size_t = (2 as libc::c_int as libc::c_ulong).wrapping_mul(parent);
        if child < count
            && compare
                .expect(
                    "non-null function pointer",
                )(
                *array.offset(child as isize),
                *array
                    .offset(
                        child.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ),
            ) < 0 as libc::c_int
        {
            child = child.wrapping_add(1);
            child;
        }
        if compare
            .expect("non-null function pointer")(*array.offset(child as isize), element)
            <= 0 as libc::c_int
        {
            break;
        }
        let ref mut fresh4 = *array.offset(parent as isize);
        *fresh4 = *array.offset(child as isize);
        parent = child;
    }
    let ref mut fresh5 = *array.offset(parent as isize);
    *fresh5 = element;
    return parent;
}
unsafe extern "C" fn heapify_up(
    mut array: *mut *mut libc::c_void,
    mut count: size_t,
    mut compare: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
) {
    let mut k: size_t = count;
    let mut new_element: *mut libc::c_void = *array.offset(k as isize);
    while k != 1 as libc::c_int as libc::c_ulong
        && compare
            .expect(
                "non-null function pointer",
            )(
            *array.offset(k.wrapping_div(2 as libc::c_int as libc::c_ulong) as isize),
            new_element,
        ) <= 0 as libc::c_int
    {
        let ref mut fresh6 = *array.offset(k as isize);
        *fresh6 = *array
            .offset(k.wrapping_div(2 as libc::c_int as libc::c_ulong) as isize);
        k = (k as libc::c_ulong).wrapping_div(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    let ref mut fresh7 = *array.offset(k as isize);
    *fresh7 = new_element;
}
