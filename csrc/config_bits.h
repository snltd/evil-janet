#include "janet.h"

/* bindgen's macro parser (cexpr) can't handle the ternary inside
   JANET_CURRENT_CONFIG_BITS, so expose the value as a const variable. */
static const unsigned int JANET_CURRENT_CONFIG_BITS_VALUE = JANET_CURRENT_CONFIG_BITS;
