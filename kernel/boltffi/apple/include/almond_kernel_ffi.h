#pragma once

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdatomic.h>

typedef struct FfiStatus { int32_t code; } FfiStatus;
typedef struct FfiString { uint8_t* ptr; size_t len; size_t cap; } FfiString;
typedef struct FfiBuf_u8 { uint8_t* ptr; size_t len; size_t cap; size_t align; } FfiBuf_u8;
typedef struct FfiError { FfiString message; } FfiError;
typedef struct BoltFFICallbackHandle { uint64_t handle; const void* vtable; } BoltFFICallbackHandle;

static inline bool boltffi_atomic_u8_cas(volatile uint8_t* target, uint8_t expected, uint8_t desired) {
    return atomic_compare_exchange_strong((_Atomic uint8_t*)target, &expected, desired);
}

static inline uint64_t boltffi_atomic_u64_exchange(volatile uint64_t* target, uint64_t value) {
    return atomic_exchange((_Atomic uint64_t*)target, value);
}

static inline bool boltffi_atomic_u64_cas(volatile uint64_t* target, uint64_t expected, uint64_t desired) {
    return atomic_compare_exchange_strong((_Atomic uint64_t*)target, &expected, desired);
}

static inline uint64_t boltffi_atomic_u64_load(const volatile uint64_t* target) {
    return atomic_load_explicit((const _Atomic uint64_t*)target, memory_order_acquire);
}


typedef int32_t ___ItemType;
#define ___ItemType_Todo 0
#define ___ItemType_Note 1
#define ___ItemType_Reminder 2
#define ___ItemType_Snippet 3
#define ___ItemType_Bookmark 4
typedef int32_t ___Priority;
#define ___Priority_High 0
#define ___Priority_Medium 1
#define ___Priority_Low 2
typedef int32_t ___Tag;
#define ___Tag_Development 0
#define ___Tag_Inspiration 1
#define ___Tag_Design 2
#define ___Tag_Research 3

void boltffi_free_string(FfiString s);
void boltffi_free_buf(FfiBuf_u8 buf);
FfiStatus boltffi_last_error_message(FfiString *out);
void boltffi_clear_last_error(void);
