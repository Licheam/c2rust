static __attribute__((always_inline)) void always_inline_static(void) {}
static __attribute__((__noinline__)) void noinline_static(void) {}
static void inline inline_static(void) {}
static void inline __attribute__((__gnu_inline__)) gnu_inline_static(void) {}
static void __attribute__((used, __cold__)) cold_used_attrs(void) {}
void __attribute__((__always_inline__)) always_inline_nonstatic(void) {}
void inline __attribute__((gnu_inline)) gnu_inline_nonstatic(void) {}
void __attribute__((noinline)) noinline_nonstatic(void) {}
void inline inline_nonstatic(void) {}
extern void gnu_inline_non_canonical_definition_extern(void);
extern void inline inline_extern(void) {}
extern void inline __attribute__((always_inline)) always_inline_extern(void) {}
extern void inline __attribute__((__gnu_inline__)) gnu_inline_extern(void) {}
extern void inline __attribute__((gnu_inline, always_inline)) always_inline_gnu_inline_extern(void) {}
extern void inline __attribute__((gnu_inline)) gnu_inline_non_canonical_definition_extern(void) {}
#ifndef __APPLE__
// aliases are not allowed on darwin
void __attribute__((alias("inline_extern"))) aliased_fn(void);
#endif // __APPLE__

void ensure_use(void) {
    always_inline_static();
    always_inline_nonstatic();
    gnu_inline_nonstatic();
    inline_static();
    noinline_static();
    inline_nonstatic();
    gnu_inline_static();
    inline_extern();
    always_inline_extern();
    gnu_inline_extern();
    always_inline_gnu_inline_extern();
    gnu_inline_non_canonical_definition_extern();
#ifndef __APPLE__
    aliased_fn();
#endif // __APPLE__
}
