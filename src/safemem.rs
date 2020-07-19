use ::libc;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    /*
   File:       error.h

   Created:    June 13, 1998

   Modified:   August 1, 2002

   Author:     Gunnar Andersson (gunnar@radagast.se)

   Contents:   The interface to the error handler.
*/
    #[no_mangle]
    fn fatal_error(format: *const libc::c_char, _: ...);
}
pub type size_t = libc::c_ulong;
/*
   File:       safemem.h

   Created:    August 30, 1998

   Modified:   January 25, 2000

   Author:     Gunnar Andersson (gunnar@radagast.se)

   Contents:   The interface to the safer version of malloc.
*/
/*
   File:            safemem.c

   Created:         August 30, 1998

   Modified:        November 1, 2000

   Author:          Gunnar Andersson (gunnar@radagast.se)

   Contents:        Provides safer memory allocation than malloc().
*/
#[no_mangle]
pub unsafe extern "C" fn safe_malloc(mut size: size_t) -> *mut libc::c_void {
    let mut block = 0 as *mut libc::c_void;
    block = malloc(size);
    if block.is_null() {
        fatal_error(b"%s %d\n\x00" as *const u8 as *const libc::c_char,
                    b"Memory allocation failure when allocating\x00" as
                        *const u8 as *const libc::c_char, size);
    }
    return block;
}
#[no_mangle]
pub unsafe extern "C" fn safe_realloc(mut ptr: *mut libc::c_void,
                                      mut size: size_t) -> *mut libc::c_void {
    let mut block = 0 as *mut libc::c_void;
    block = realloc(ptr, size);
    if block.is_null() {
        fatal_error(b"%s %d\n\x00" as *const u8 as *const libc::c_char,
                    b"Memory allocation failure when allocating\x00" as
                        *const u8 as *const libc::c_char, size);
    }
    return block;
}
