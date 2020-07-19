use crate::src::libc;
/*
   File:          autoplay.h

   Created:       May 21, 1998

   Modified:      August 1, 2002

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:
*/
/*
   File:          autop.c

   Created:       May 23, 1998

   Modified:      May 2, 1999

   Author:        Gunnar Andersson (gunnar@radagast.se)

   Contents:      An empty definition of functions from autoplay
                  used when no event handling is necessary.
*/
pub unsafe fn handle_event(mut only_passive_events: libc::c_int,
                                      mut allow_delay: libc::c_int,
                                      mut passive_mode: libc::c_int) {
}
pub unsafe fn toggle_event_status(mut allow_event_handling:
                                                 libc::c_int) {
}
