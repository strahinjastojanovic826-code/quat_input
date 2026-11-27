//! Foreign Function Interface (`extern "C"`) boundary for `quat_input`.
//!
//! # Safety & FFI Invariants
//! All functions in this module are `unsafe extern "C"` entry points called across
//! the FFI boundary and rely on the following invariants upheld by callers:
//!
//! - **Pointer validity & alignment:** Any input pointer (`bytes`, `raw_quats`,
//!   `packed_bytes`, `stream`) must be properly aligned and valid for reads of the
//!   specified length (`len`, `quat_len`, `packed_len`, `total_quats`) for the
//!   duration of the call (or non-null where required). Any output pointer
//!   (`out_stream`, `out_val`, `out_err`, `out_bytes`, `out_written`, `out_quats`)
//!   must be non-null, properly aligned, and valid for writes of the expected size.
//! - **No aliasing / exclusive access:** Mutable output pointers and raw buffers
//!   must not alias other active references or concurrently mutated memory while
//!   a call is in progress.
//! - **Lifetime & ownership (`Box` raw pointers):** `quat_stream_create` allocates
//!   a heap `FfiQuatStream` via `Box::into_raw`; the caller is responsible for
//!   passing that pointer back to `quat_stream_destroy` (at most once) so it can
//!   be reclaimed with `Box::from_raw`, and must not use `stream` after destruction
//!   or while `quat_stream_destroy` runs.
//! - **Slice construction (`from_raw_parts` / `from_raw_parts_mut`):** Raw pointers
//!   and lengths passed to slice constructors must form a single contiguous allocation
//!   of initialized bytes (`u8`) within `isize::MAX` bytes, and must outlive any
//!   temporary Rust slices created inside the call.
//! - **Panic safety:** FFI entry points wrap fallible work in `catch_unwind` so
//!   panics do not unwind across the `"C"` ABI boundary.

use std::slice;
use std::panic::catch_unwind;
use crate::view::QuatStream;
use crate::error::QuatError;
use crate::Quat;
use crate::QuatInput;

#[repr(C)]
pub struct FfiQuatStream {
    bytes: *const u8,
    len: usize,
    total_quats: usize,
}

#[repr(C)]
pub enum FfiError {
    Ok = 0,
    InvalidChar = 1,
    InvalidRawValue = 2,
    IndexOutOfBounds = 3,
    PanicCaught = -99,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn quat_stream_create(
    bytes: *const u8,
    len: usize,
    total_quats: usize,
    out_stream: *mut *mut FfiQuatStream,
) -> i32 { unsafe {
    let result = catch_unwind(|| {
        if bytes.is_null() || out_stream.is_null() {
            return -1;
        }
        let boxed = Box::new(FfiQuatStream {
            bytes,
            len,
            total_quats,
        });
        *out_stream = Box::into_raw(boxed);
        0
    });

    match result {
        Ok(code) => code,
        Err(_) => -99,
    }
}}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn quat_stream_destroy(stream: *mut FfiQuatStream) { unsafe {
    let _ = catch_unwind(|| {
        if !stream.is_null() {
            let _ = Box::from_raw(stream);
        }
    });
}}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn quat_stream_get(
    stream: *const FfiQuatStream,
    index: usize,
    out_val: *mut u8,
    out_err: *mut i32,
) -> i32 { unsafe {
    let result = catch_unwind(|| {
        if stream.is_null() || out_val.is_null() || out_err.is_null() {
            return -1;
        }
        let s = &*stream;
        let slice = slice::from_raw_parts(s.bytes, s.len);
        let stream_view = QuatStream::new(slice, s.total_quats);

        match stream_view.get(index) {
            Ok(q) => {
                *out_val = q.to_u8();
                *out_err = FfiError::Ok as i32;
                0
            }
            Err(e) => {
                let err_code = match e {
                    QuatError::InvalidChar(_) => FfiError::InvalidChar,
                    QuatError::InvalidRawValue(_) => FfiError::InvalidRawValue,
                    QuatError::IndexOutOfBounds(_) => FfiError::IndexOutOfBounds,
                };
                *out_err = err_code as i32;
                0
            }
        }
    });

    match result {
        Ok(code) => code,
        Err(_) => {
            if !out_err.is_null() {
                *out_err = FfiError::PanicCaught as i32;
            }
            -99
        }
    }
}}

/// Returns the total number of quaternary symbols in the stream, or `0` if `stream` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn quat_stream_len(stream: *const FfiQuatStream) -> usize {
    if stream.is_null() {
        return 0;
    }
    unsafe { (*stream).total_quats }
}
#[inline]
fn map_quat_error(err: QuatError) -> FfiError {
    match err {
        QuatError::InvalidChar(_) => FfiError::InvalidChar,
        QuatError::InvalidRawValue(_) => FfiError::InvalidRawValue,
        QuatError::IndexOutOfBounds(_) => FfiError::IndexOutOfBounds,
    }
}


/// Packs a raw array of `u8` values (each in `0..=3`) into a 2-bit packed byte buffer.
/// Writes the number of packed bytes to `out_written` and returns `FfiError` code as `i32`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn quat_pack_buffer(
    raw_quats: *const u8,
    quat_len: usize,
    out_bytes: *mut u8,
    out_capacity: usize,
    out_written: *mut usize,
) -> i32 {
    let result = catch_unwind(|| unsafe {
        if raw_quats.is_null() || out_bytes.is_null() || out_written.is_null() {
            return -1;
        }

        let input_slice = slice::from_raw_parts(raw_quats, quat_len);
        let mut quats = Vec::with_capacity(quat_len);
        for &v in input_slice {
            match Quat::from_u8_const(v) {
                Ok(q) => quats.push(q),
                Err(e) => return map_quat_error(e) as i32,
            }
        }

        let out_slice = slice::from_raw_parts_mut(out_bytes, out_capacity);
        match QuatInput::pack_slice_into(&quats, out_slice) {
            Some(written) => {
                *out_written = written;
                FfiError::Ok as i32
            }
            None => FfiError::IndexOutOfBounds as i32,
        }
    });

    match result {
        Ok(code) => code,
        Err(_) => FfiError::PanicCaught as i32,
    }
}

/// Unpacks a 2-bit packed byte buffer into an array of `u8` values (`0..=3`).
/// Returns `FfiError` code as `i32`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn quat_unpack_buffer(
    packed_bytes: *const u8,
    packed_len: usize,
    total_quats: usize,
    out_quats: *mut u8,
    out_capacity: usize,
) -> i32 {
    let result = catch_unwind(|| unsafe {
        if packed_bytes.is_null() || out_quats.is_null() {
            return -1;
        }
        if out_capacity < total_quats {
            return FfiError::IndexOutOfBounds as i32;
        }

        let slice = slice::from_raw_parts(packed_bytes, packed_len);
        let stream = QuatStream::new(slice, total_quats);
        let mut temp = vec![Quat::V0; total_quats];

        if let Err(e) = stream.unpack_into(&mut temp) {
            return map_quat_error(e) as i32;
        }

        let out_slice = slice::from_raw_parts_mut(out_quats, total_quats);
        for (dst, src) in out_slice.iter_mut().zip(temp.iter()) {
            *dst = src.to_u8();
        }

        FfiError::Ok as i32
    });

    match result {
        Ok(code) => code,
        Err(_) => FfiError::PanicCaught as i32,
    }
}