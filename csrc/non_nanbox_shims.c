#include "janet.h"

/* janet.h declares janet_wrap_integer() as a function in every mode, but
   janet.c only defines it in nanbox builds. Provide it otherwise. The
   parentheses around the name stop the macro in janet.h expanding. */
#if !defined(JANET_NANBOX_32) && !defined(JANET_NANBOX_64)
Janet (janet_wrap_integer)(int32_t x) {
    return janet_wrap_number((double) x);
}
#endif
