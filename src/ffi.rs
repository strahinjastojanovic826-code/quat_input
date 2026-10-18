use std::slice;
use std::panic::catch_unwind;
use crate::view::QuatStream;
use crate::error::QuatError;

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