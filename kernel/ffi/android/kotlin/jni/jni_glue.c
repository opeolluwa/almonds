#include <jni.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>
#include <stdatomic.h>
#include <almond_kernel.h>

static inline bool boltffi_exception_pending(JNIEnv* env) {
    return (*env)->ExceptionCheck(env);
}

static inline bool boltffi_consume_pending_exception(JNIEnv* env) {
    if (!boltffi_exception_pending(env)) return false;
    (*env)->ExceptionClear(env);
    return true;
}

static inline void boltffi_throw_out_of_memory(JNIEnv* env, const char* message) {
    jclass oom_class = (*env)->FindClass(env, "java/lang/OutOfMemoryError");
    if (oom_class == NULL) return;
    (*env)->ThrowNew(env, oom_class, message);
    (*env)->DeleteLocalRef(env, oom_class);
}

static inline void boltffi_throw_illegal_argument(JNIEnv* env, const char* message) {
    jclass exception_class = (*env)->FindClass(env, "java/lang/IllegalArgumentException");
    if (exception_class == NULL) return;
    (*env)->ThrowNew(env, exception_class, message);
    (*env)->DeleteLocalRef(env, exception_class);
}

static inline void boltffi_throw_runtime(JNIEnv* env, const char* message) {
    jclass exception_class = (*env)->FindClass(env, "java/lang/RuntimeException");
    if (exception_class == NULL) return;
    (*env)->ThrowNew(env, exception_class, message);
    (*env)->DeleteLocalRef(env, exception_class);
}

static inline bool boltffi_try_jlong_to_usize(jlong value, uintptr_t* out_value) {
    if (value < 0) return false;
    uint64_t unsigned_value = (uint64_t)value;
    if (unsigned_value > (uint64_t)UINTPTR_MAX) return false;
    *out_value = (uintptr_t)unsigned_value;
    return true;
}

typedef struct {
    void (*free)(uint64_t handle);
    uint64_t (*clone)(uint64_t handle);
} BoltFFICallbackVTablePrefix;

static inline const BoltFFICallbackVTablePrefix* boltffi_callback_vtable_prefix(
    const BoltFFICallbackHandle* callback
) {
    return callback == NULL ? NULL : (const BoltFFICallbackVTablePrefix*)callback->vtable;
}

static inline void boltffi_release_callback_value(BoltFFICallbackHandle callback) {
    const BoltFFICallbackVTablePrefix* vtable = boltffi_callback_vtable_prefix(&callback);
    if (callback.handle != 0 && vtable != NULL && vtable->free != NULL) {
        vtable->free(callback.handle);
    }
}

static inline BoltFFICallbackHandle* boltffi_jvm_callback_handle_ref(jlong handle) {
    if (handle == 0) return NULL;
    return (BoltFFICallbackHandle*)(uintptr_t)handle;
}

static inline jlong boltffi_jvm_callback_handle_new_owned(
    JNIEnv* env,
    BoltFFICallbackHandle callback
) {
    if (callback.handle == 0 || callback.vtable == NULL) return 0;
    BoltFFICallbackHandle* stored_callback =
        (BoltFFICallbackHandle*)malloc(sizeof(BoltFFICallbackHandle));
    if (stored_callback == NULL) {
        boltffi_release_callback_value(callback);
        boltffi_throw_out_of_memory(env, "Failed to allocate callback handle");
        return 0;
    }
    *stored_callback = callback;
    return (jlong)(uintptr_t)stored_callback;
}

static inline void boltffi_jvm_callback_handle_release(BoltFFICallbackHandle* callback) {
    if (callback == NULL) return;
    boltffi_release_callback_value(*callback);
    free(callback);
}

static inline jlong boltffi_jvm_callback_handle_clone(
    JNIEnv* env,
    const BoltFFICallbackHandle* callback
) {
    const BoltFFICallbackVTablePrefix* vtable = boltffi_callback_vtable_prefix(callback);
    if (callback == NULL || callback->handle == 0 || vtable == NULL || vtable->clone == NULL) {
        return 0;
    }
    BoltFFICallbackHandle cloned_callback = {
        .handle = vtable->clone(callback->handle),
        .vtable = callback->vtable,
    };
    if (cloned_callback.handle == 0) {
        return 0;
    }
    return boltffi_jvm_callback_handle_new_owned(env, cloned_callback);
}

static inline jbyteArray boltffi_buf_to_jbytearray(JNIEnv* env, FfiBuf_u8 buf) {
    if (buf.ptr == NULL) {
        if (buf.len != 0) {
            boltffi_throw_out_of_memory(env, "BoltFFI buffer pointer was null");
        }
        return NULL;
    }
    if (buf.len > (size_t)INT32_MAX) {
        boltffi_free_buf(buf);
        boltffi_throw_out_of_memory(env, "BoltFFI buffer too large for Java byte array");
        return NULL;
    }
    jsize len = (jsize)buf.len;
    jbyteArray arr = (*env)->NewByteArray(env, len);
    if (arr == NULL) {
        boltffi_free_buf(buf);
        return NULL;
    }
    (*env)->SetByteArrayRegion(env, arr, 0, len, (const jbyte*)buf.ptr);
    boltffi_free_buf(buf);
    if (boltffi_exception_pending(env)) {
        (*env)->DeleteLocalRef(env, arr);
        return NULL;
    }
    return arr;
}

static inline jbyteArray boltffi_status_buf_to_jbytearray(JNIEnv* env, FfiStatus status, FfiBuf_u8 buf) {
    if (status.code != 0) {
        if (buf.ptr != NULL) {
            boltffi_free_buf(buf);
        }
        return NULL;
    }
    return boltffi_buf_to_jbytearray(env, buf);
}

static inline bool boltffi_lookup_static_method(
    JNIEnv* env,
    jclass cls,
    const char* name,
    const char* signature,
    jmethodID* out_method
) {
    *out_method = (*env)->GetStaticMethodID(env, cls, name, signature);
    if (*out_method != NULL) return true;
    boltffi_consume_pending_exception(env);
    return false;
}

typedef enum {
    BOLTFFI_GLOBAL_CLASS_OK = 0,
    BOLTFFI_GLOBAL_CLASS_MISSING = 1,
    BOLTFFI_GLOBAL_CLASS_FATAL = 2
} BoltFFIGlobalClassResult;

static inline BoltFFIGlobalClassResult boltffi_lookup_global_class(
    JNIEnv* env,
    const char* class_name,
    jclass* out_class
) {
    *out_class = NULL;
    jclass local_class = (*env)->FindClass(env, class_name);
    if (local_class == NULL) {
        boltffi_consume_pending_exception(env);
        return BOLTFFI_GLOBAL_CLASS_MISSING;
    }
    jclass global_class = (*env)->NewGlobalRef(env, local_class);
    (*env)->DeleteLocalRef(env, local_class);
    if (global_class == NULL) {
        boltffi_consume_pending_exception(env);
        return BOLTFFI_GLOBAL_CLASS_FATAL;
    }
    *out_class = global_class;
    return BOLTFFI_GLOBAL_CLASS_OK;
}

typedef enum {
    BOLTFFI_STATIC_CALL_CACHE_UNINIT = 0,
    BOLTFFI_STATIC_CALL_CACHE_INITING = 1,
    BOLTFFI_STATIC_CALL_CACHE_READY = 2,
    BOLTFFI_STATIC_CALL_CACHE_FAILED = 3
} BoltFFIStaticCallCacheState;

typedef struct {
    atomic_int state;
    jclass class_ref;
    jmethodID method;
} BoltFFIStaticCallCache;

#define BOLTFFI_STATIC_CALL_CACHE_INIT { 0, NULL, NULL }

static inline bool boltffi_static_call_cache_ensure(
    JNIEnv* env,
    BoltFFIStaticCallCache* cache,
    const char* class_name,
    const char* method_name,
    const char* method_signature
) {
    int state = atomic_load_explicit(&cache->state, memory_order_acquire);
    if (state == BOLTFFI_STATIC_CALL_CACHE_READY) return true;
    if (state == BOLTFFI_STATIC_CALL_CACHE_FAILED) return false;

    int expected = BOLTFFI_STATIC_CALL_CACHE_UNINIT;
    if (atomic_compare_exchange_strong_explicit(
            &cache->state,
            &expected,
            BOLTFFI_STATIC_CALL_CACHE_INITING,
            memory_order_acq_rel,
            memory_order_acquire)) {
        jclass class_ref = NULL;
        jmethodID method = NULL;
        BoltFFIGlobalClassResult class_result =
            boltffi_lookup_global_class(env, class_name, &class_ref);
        if (class_result != BOLTFFI_GLOBAL_CLASS_OK) {
            cache->class_ref = NULL;
            cache->method = NULL;
            atomic_store_explicit(
                &cache->state,
                BOLTFFI_STATIC_CALL_CACHE_FAILED,
                memory_order_release
            );
            return false;
        }
        if (!boltffi_lookup_static_method(env, class_ref, method_name, method_signature, &method)) {
            (*env)->DeleteGlobalRef(env, class_ref);
            cache->class_ref = NULL;
            cache->method = NULL;
            atomic_store_explicit(
                &cache->state,
                BOLTFFI_STATIC_CALL_CACHE_FAILED,
                memory_order_release
            );
            return false;
        }
        cache->class_ref = class_ref;
        cache->method = method;
        atomic_store_explicit(
            &cache->state,
            BOLTFFI_STATIC_CALL_CACHE_READY,
            memory_order_release
        );
        return true;
    }

    do {
        state = atomic_load_explicit(&cache->state, memory_order_acquire);
    } while (state == BOLTFFI_STATIC_CALL_CACHE_INITING);

    return state == BOLTFFI_STATIC_CALL_CACHE_READY;
}

static inline void boltffi_static_call_cache_reset(JNIEnv* env, BoltFFIStaticCallCache* cache) {
    if (cache->class_ref != NULL) {
        (*env)->DeleteGlobalRef(env, cache->class_ref);
        cache->class_ref = NULL;
    }
    cache->method = NULL;
    atomic_store_explicit(&cache->state, BOLTFFI_STATIC_CALL_CACHE_UNINIT, memory_order_release);
}

JNIEXPORT jbyteArray JNICALL Java_com_example_almond_1kernel_Native_boltffi_1last_1error_1message(JNIEnv *env, jclass cls) {
    FfiString out = { 0 };
    FfiStatus status = boltffi_last_error_message(&out);
    if (status.code != 0 || out.ptr == NULL || out.len == 0) {
        boltffi_free_string(out);
        return (*env)->NewByteArray(env, 0);
    }
    jbyteArray result = (*env)->NewByteArray(env, (jsize)out.len);
    if (result != NULL) {
        (*env)->SetByteArrayRegion(env, result, 0, (jsize)out.len, (const jbyte*)out.ptr);
    }
    boltffi_free_string(out);
    return result;
}