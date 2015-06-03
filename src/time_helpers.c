// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#include <stdint.h>
#include <time.h>
#include <string.h>
#include <assert.h>
#include <stdlib.h>

#if !defined(_WIN32)
#include <sys/time.h>
#include <sys/types.h>
#include <dirent.h>
#include <signal.h>
#include <unistd.h>
#include <pthread.h>
#else
#include <windows.h>
#include <wincrypt.h>
#include <stdio.h>
#endif

#ifdef __APPLE__
#include <TargetConditionals.h>
#include <mach/mach_time.h>

#if !(TARGET_OS_IPHONE)
#include <crt_externs.h>
#endif
#endif

// Gonk has this symbol, but Android doesn't
#ifndef TARGET_OS_GONK
#ifdef __ANDROID__

#include <android/api-level.h>
#if __ANDROID_API__ < 21
static time_t timegm(struct tm *tm) {
    time_t ret;
    char *tz;

    tz = getenv("TZ");
    if (tz)
        tz = strdup(tz);
    setenv("TZ", "", 1);
    tzset();
    ret = mktime(tm);
    if (tz) {
        setenv("TZ", tz, 1);
        free(tz);
    } else
        unsetenv("TZ");
    tzset();
    return ret;
}
#endif
#endif
#endif

typedef struct {
    int32_t tm_sec;
    int32_t tm_min;
    int32_t tm_hour;
    int32_t tm_mday;
    int32_t tm_mon;
    int32_t tm_year;
    int32_t tm_wday;
    int32_t tm_yday;
    int32_t tm_isdst;
    int32_t tm_utcoff;
    int32_t tm_nsec;
} rust_time_tm;

static void rust_time_tm_to_tm(rust_time_tm* in_tm, struct tm* out_tm) {
    memset(out_tm, 0, sizeof(struct tm));
    out_tm->tm_sec = in_tm->tm_sec;
    out_tm->tm_min = in_tm->tm_min;
    out_tm->tm_hour = in_tm->tm_hour;
    out_tm->tm_mday = in_tm->tm_mday;
    out_tm->tm_mon = in_tm->tm_mon;
    out_tm->tm_year = in_tm->tm_year;
    out_tm->tm_wday = in_tm->tm_wday;
    out_tm->tm_yday = in_tm->tm_yday;
    out_tm->tm_isdst = in_tm->tm_isdst;
}

static void tm_to_rust_tm(struct tm* in_tm,
                          rust_time_tm* out_tm,
                          int32_t utcoff,
                          int32_t nsec) {
    out_tm->tm_sec = in_tm->tm_sec;
    out_tm->tm_min = in_tm->tm_min;
    out_tm->tm_hour = in_tm->tm_hour;
    out_tm->tm_mday = in_tm->tm_mday;
    out_tm->tm_mon = in_tm->tm_mon;
    out_tm->tm_year = in_tm->tm_year;
    out_tm->tm_wday = in_tm->tm_wday;
    out_tm->tm_yday = in_tm->tm_yday;
    out_tm->tm_isdst = in_tm->tm_isdst;
    out_tm->tm_utcoff = utcoff;
    out_tm->tm_nsec = nsec;
}

#if defined(_WIN32)
#if defined(_MSC_VER) && (_MSC_VER >= 1400)
#define GMTIME(clock, result) gmtime_s((result), (clock))
#define LOCALTIME(clock, result) localtime_s((result), (clock))
#define TIMEGM(result) _mkgmtime64(result)
#else
static struct tm* GMTIME(const time_t *clock, struct tm *result) {
    struct tm* t = gmtime(clock);
    if (t == NULL || result == NULL) { return NULL; }
    *result = *t;
    return result;
}
static struct tm* LOCALTIME(const time_t *clock, struct tm *result) {
    struct tm* t = localtime(clock);
    if (t == NULL || result == NULL) { return NULL; }
    *result = *t;
    return result;
}
#define TIMEGM(result) mktime((result)) - _timezone
#endif
#else

#ifdef __native_client__
#define TIMEGM(result) mktime((result)) - _timezone
#else
#define TIMEGM(result) timegm(result)
#endif

#define GMTIME(clock, result) gmtime_r((clock), (result))
#define LOCALTIME(clock, result) localtime_r((clock), (result))

#endif

void
rust_time_gmtime(int64_t sec, int32_t nsec, rust_time_tm *timeptr) {
    struct tm tm;
    time_t s = sec;
    GMTIME(&s, &tm);

    tm_to_rust_tm(&tm, timeptr, 0, nsec);
}

int32_t
rust_time_localtime(int64_t sec, int32_t nsec, rust_time_tm *timeptr) {
    struct tm tm;
    time_t s = sec;
    if (LOCALTIME(&s, &tm) == NULL) { return 0; }

#if defined(_WIN32)
    int32_t utcoff = -timezone;
#elif defined(__native_client__)
    int32_t utcoff = _timezone;
#else
    int32_t utcoff = tm.tm_gmtoff;
#endif

    tm_to_rust_tm(&tm, timeptr, utcoff, nsec);
    return 1;
}

int64_t
rust_time_timegm(rust_time_tm* timeptr) {
    struct tm t;
    rust_time_tm_to_tm(timeptr, &t);
    return TIMEGM(&t);
}

int64_t
rust_time_mktime(rust_time_tm* timeptr) {
    struct tm t;
    rust_time_tm_to_tm(timeptr, &t);
    return mktime(&t);
}
