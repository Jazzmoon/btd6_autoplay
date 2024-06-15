 
        #ifndef DBUS_ARCH_DEPS_H
        #define DBUS_ARCH_DEPS_H

        #include <stdint.h>
        #include <stdarg.h>
        #include <dbus/dbus-macros.h>

        DBUS_BEGIN_DECLS

        _DBUS_GNUC_EXTENSION typedef int64_t dbus_int64_t;
        _DBUS_GNUC_EXTENSION typedef uint64_t dbus_uint64_t;

        #define DBUS_INT64_CONSTANT(val)  (_DBUS_GNUC_EXTENSION (val##L))
        #define DBUS_UINT64_CONSTANT(val) (_DBUS_GNUC_EXTENSION (val##UL))

        typedef int32_t dbus_int32_t;
        typedef uint32_t dbus_uint32_t;

        typedef int16_t dbus_int16_t;
        typedef uint16_t dbus_uint16_t;

        // Required for 1.14 but not 1.15
        #define DBUS_VA_COPY va_copy

        DBUS_END_DECLS

        #endif /* DBUS_ARCH_DEPS_H */
    