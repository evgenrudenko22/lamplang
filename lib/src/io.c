#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <string.h>

#include "io.h"

int count_placeholders(const char* fmt) {
    int count = 0;
    const char* p = fmt;

    while (*p) {
        if (*p == '{' && *(p + 1) && *(p + 2) == '}') {
            count++;
            p += 3;
        } else {
            p++;
        }
    }

    return count;
}

char* format_internal(const char* fmt, va_list args) {
    if (!fmt) return NULL;

    int placeholder_count = count_placeholders(fmt);
    size_t estimated_size = strlen(fmt) + placeholder_count * 64;

    char* result = (char*)malloc(estimated_size);
    if (!result) return NULL;

    const char* p = fmt;
    char* out = result;
    size_t remaining = estimated_size;

    while (*p) {
        if (*p == '{' && *(p + 1) && *(p + 2) == '}') {
            char type = *(p + 1);

            if (type == 's') {
                const char* str = va_arg(args, const char*);
                if (str) {
                    size_t len = strlen(str);
                    if (len < remaining) {
                        strcpy(out, str);
                        out += len;
                        remaining -= len;
                        printf("%s", out);
                    }
                }
            } else if (type == 'f') {
                double val = va_arg(args, double);
                int written = snprintf(out, remaining, "%g", val);
                if (written > 0 && written < remaining) {
                    out += written;
                    remaining -= written;
                }
            }

            p += 3;
        } else {
            if (remaining > 1) {
                *out++ = *p;
                remaining--;
            }
            p++;
        }
    }
    *out = '\0';

    return result;
}

char* format(const char* fmt, ...) {
    va_list args;
    va_start(args, fmt);

    char* result = format_internal(fmt, args);

    va_end(args);

    return result;
}

void print(const char* fmt, ...) {
    va_list args;
    va_start(args, fmt);

    char* result = format_internal(fmt, args);

    va_end(args);

    if (result) {
        printf("%s", result);
        free(result);
    }
}

void println(const char* fmt, ...) {
    va_list args;
    va_start(args, fmt);

    char* result = format_internal(fmt, args);

    va_end(args);

    if (result) {
        printf("%s\n", result);
        free(result);
    }
}

void input(const char* fmt, ...) {
    if (!fmt) return;

    va_list args;
    va_start(args, fmt);

    const char* p = fmt;

    while (*p) {
        if (*p == '{' && *(p + 1) && *(p + 2) == '}') {
            char type = *(p + 1);

            if (type == 's') {
                char** str_ptr = va_arg(args, char**);
                if (str_ptr) {
                    char buffer[256];
                    if (scanf("%255s", buffer) == 1) {
                        *str_ptr = (char*)malloc(strlen(buffer) + 1);
                        if (*str_ptr) {
                            strcpy(*str_ptr, buffer);
                        }
                    }
                }
            } else if (type == 'f') {
                double* val_ptr = va_arg(args, double*);
                if (val_ptr) {
                    scanf("%lf", val_ptr);
                }
            }

            p += 3;
        } else {
            p++;
        }
    }

    va_end(args);
}