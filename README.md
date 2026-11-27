# quat_input

`quat_input` is a high-performance, zero-copy Rust library designed for managing quaternary data streams (base-4). Each quaternary digit (`quat`) packs exactly 2 bits of information (`0b00`, `0b01`, `0b10`, `0b11`). 

The library provides zero-allocation stream views, efficient bit-packing, robust error management, and a production-grade FFI (Foreign Function Interface) layer with panic safety for seamless integration with C, C++, Python, and other languages.

---

## Features

- **Zero-Copy Stream Views**: Read and iterate over quaternary streams directly from raw byte slices without heap allocations.
- **Bit-Packed Storage**: Efficiently packs 4 quaternary digits into a single byte (`QuatByte`).
- **Compile-Time & Run-Time Safety**: Supports `const` contexts for parsing and validation.
- **Production-Ready FFI**: Fully equipped with `std::panic::catch_unwind` protection to ensure safe cross-language calls without risking undefined behavior or process crashes.
- **Quaternary Arithmetic & Logic**: Built-in modulo 4 arithmetic (`add`, `sub`) and multi-valued logic operations (`not`, `max`, `min`) directly on `Quat`.
- **Vectorized Performance**: High-performance batch packing (`pack_fast`) optimized via chunk-based word-level manipulation.
- **SIMD-Optimized Batch Packing**: High-performance batch processing designed for hardware acceleration and auto-vectorization.
- **NAND Flash Memory Simulation**: Built-in support for structured quaternary page and block management mimicking modern memory chips.
- **Digital Signal Processing (DSP)**: QPSK phase symbol modulation mapping quaternary logic directly to telecommunication signals.
- **Zero-Allocation Slice Packing**: Pack slices or streams directly into caller-provided mutable byte buffers (`pack_slice_into` / `pack_into`) with capacity checks.
- **Parsing, Conversions & Formatting**: Parse `'0'`–`'3'` strings via `FromStr`, convert cleanly with `TryFrom`/`From`, and format symbols via `Display`.
- **Flash Integrity & Block Management**: Verify NAND flash pages with deterministic 32-bit FNV-1a checksums, track exact valid symbol counts, and organize pages into blocks.
- **Gray-Coded QPSK & Baseband I/Q**: Convert symbols to/from Gray code and modulate/demodulate both QPSK phase symbols and normalized I/Q samples (`IqSample`).
- **Typed & FFI-Mapped Errors**: Structured error handling (`QuatError`) mapped to explicit status codes (`FfiError`) across the C ABI boundary.

---

## Benchmark Performance Results

| Test Case (`tests::*`) | Benchmark Label / Notes | Measured Time | Status |
| :--- | :--- | :--- | :--- |
| `test_01_single_quat_parse` | `test_01_single_quat_parse` | `7.9µs` | `ok` |
| `test_02_quat_input_from_str` | `test_02_quat_input_from_str` | `30.1µs` | `ok` |
| `test_03_quat_byte_packing` | `test_03_quat_byte_packing` | `17.1µs` | `ok` |
| `test_04_quat_stream_get` | `test_04_quat_stream_get` | `7.3µs` | `ok` |
| `test_05_quat_stream_iterator` | `test_05_quat_stream_iterator` | `3.2µs` | `ok` |
| `test_06_error_invalid_char` | `test_06_error_invalid_char` | `2µs` | `ok` |
| `test_07_error_index_out_of_bounds` | `test_07_error_index_out_of_bounds` | `500ns` | `ok` |
| `test_08_const_functions` | `test_08_const_functions` | `0ns` | `ok` |
| `test_09_stress_empty_and_boundaries` | `test_09_stress_empty_and_boundaries` | `2.1µs` | `ok` |
| `test_10_stress_large_stream` | `test_10_stress_large_stream` | `1.1592ms` | `ok` |
| `test_11_stress_ffi_safety_and_panics` | `test_11_stress_ffi_safety_and_panics` | `1.8µs` | `ok` |
| `test_12_quat_arithmetic_and_logic` | `test_12_quat_arithmetic_and_logic` | `300ns` | `ok` |
| `test_13_pack_fast_equivalence` | `test_13_pack_fast_equivalence` | `4.8µs` | `ok` |
| `test_14_simd_packing_benchmark_and_correctness` | `pack_simd_optimized (1000 iterations)` | `205.6701ms` | `ok` |
| `test_15_dsp_modulation_benchmark_and_correctness` | `modulate_stream (10,000 elements)` | `155.6µs` | `ok` |
| `test_16_flash_page_benchmark_and_correctness` | `QuatFlashPage creation (1000 iterations)` | `3.0326ms` | `ok` |
| `test_17_extended_unary_and_binary_quat_ops` | `test_17_extended_unary_and_binary_quat_ops` | `374.5µs` | `ok` |
| `test_18_zero_copy_streaming_and_pack_into` | `test_18_zero_copy_streaming_and_pack_into` | `101.3µs` | `ok` |
| `test_19_flash_page_integrity_and_block` | `test_19_flash_page_integrity_and_block` | `4.2019ms` | `ok` |
| `test_20_dsp_gray_iq_modulation_and_demodulation` | `test_20_dsp_gray_iq_modulation_and_demodulation` | `34.6549ms` | `ok` |
| `test_21_ffi_pack_unpack_and_len` | `test_21_ffi_pack_unpack_and_len` | `555.5µs` | `ok` |

---

## Licensing

This project is dual-licensed:

* **Open Source (AGPLv3):** Free to use for open-source projects. If you use or modify this library in your software or network service, your source code must also be open-source under AGPLv3.
* **Commercial License:** Required if you want to use `quat_input` in closed-source or proprietary commercial software without AGPLv3 restrictions.

For commercial licensing, contact: **strahinjastojanovic826@gmail.com**

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
quat_input = "0.4.0"
```

If you plan to compile it as a shared library for FFI use, configure your crate type:

```toml
[lib]
name = "quat_input"
crate-type = ["cdylib", "rlib"]
```

---

## Quick Start (Rust)

```rust
use quat_input::{QuatInput, QuatStream, Quat};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Parse string input containing quaternary characters ('0' to '3')
    let input = QuatInput::from_str("01233210")?;

    // 2. Pack the quaternary digits into raw bytes (standard pack)
    let packed_bytes = input.pack();
    println!("Packed into {} byte(s)", packed_bytes.len());

    // 2b. High-performance vectorized batch packing
    let fast_bytes = input.pack_fast();
    println!("Fast packed into {} byte(s)", fast_bytes.len());

    // 3. Create a zero-copy stream view over the packed bytes
    let stream = QuatStream::new(&packed_bytes, input.quats().len());

    // 4. Access elements safely by index without allocation
    let q = stream.get(2)?;
    println!("Quat at index 2: {}", q); // Outputs: 2

    // 5. Use zero-allocation iteration
    for item in stream.iter() {
        let quat = item?;
        print!("{} ", quat);
    }
    println!();

    // 6. Quaternary arithmetic & multi-valued logic operations
    let q1 = Quat::V1;
    let q2 = Quat::V2;
    let sum = q1.add(q2);
    let negated = q1.not();

    println!("Arithmetic V1 + V2 = {}", sum);
    println!("Post NOT(V1) = {}", negated);

    // 7. Industrial Features: SIMD, DSP & Flash simulation
    let _simd_bytes = input.pack_simd_optimized();
    
    let dsp_symbols = quat_input::dsp::modulate_stream(input.quats());
    println!("DSP QPSK Phase for first quat: {} degrees", dsp_symbols[0].phase_degrees);

    let flash_page = quat_input::flash::QuatFlashPage::new(1, &_simd_bytes).unwrap();
    println!("NAND Flash Page ID: {}, Data size: {} bytes", flash_page.page_id, flash_page.data.len());

    Ok(())
} 
```

---

## Advanced Features (Logic, Zero-Copy, Flash Integrity & IQ DSP)

```rust
use quat_input::dsp::{demodulate_iq_stream, modulate_iq_stream};
use quat_input::flash::QuatFlashPage;
use quat_input::input::QuatInput;
use quat_input::quat::Quat;
use quat_input::view::QuatStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Extended unary & binary quaternary logic
    let a = Quat::V2;
    let b = Quat::V3;
    let shifted = a.inc_mod4();
    let prod = a.mul_mod4(b);
    let gated = a.threshold(Quat::V2, Quat::V0, Quat::V3);
    println!("inc_mod4: {shifted}, mul_mod4: {prod}, threshold: {gated}");

    // 2. Zero-allocation packing & unpacking into caller buffers
    let quats = [Quat::V0, Quat::V1, Quat::V2, Quat::V3, Quat::V1];
    let mut raw_buf = [0u8; 2];
    let written = QuatInput::pack_slice_into(&quats, &mut raw_buf).unwrap();

    let stream = QuatStream::new(&raw_buf[..written], quats.len());
    let mut unpacked = [Quat::V0; 5];
    let count = stream.unpack_into(&mut unpacked)?;
    println!("Unpacked {count} symbols, first: {}", unpacked[0]);

    // 3. Flash page with explicit symbol count and integrity check
    let page = QuatFlashPage::new_with_quat_len(1, &stream.packed_bytes().collect::<Vec<_>>(), quats.len())?;
    println!("Page valid: {}", page.verify_integrity());

    // 4. Gray-mapped I/Q modulation & hard-decision demodulation
    let iq = modulate_iq_stream(&quats);
    let recovered = demodulate_iq_stream(&iq);
    println!("Recovered IQ length: {}", recovered.len());

    Ok(())
}
```

---

## FFI Integration (Python Example)

Because the library exposes a stable C ABI (`extern "C"`) protected with panic safety (`catch_unwind`), you can easily integrate it into Python via `ctypes`:

```python
import ctypes
import os
import platform

# 1. Load shared library (.dll on Windows, .so on Linux, .dylib on macOS)
if platform.system() == "Windows":
    lib_name = "quat_input.dll"
elif platform.system() == "Darwin":
    lib_name = "libquat_input.dylib"
else:
    lib_name = "libquat_input.so"

lib_path = os.path.abspath(os.path.join("target", "release", lib_name))
lib = ctypes.CDLL(lib_path)


# 2. Opaque handle definition matching FfiQuatStream
class _FfiQuatStream(ctypes.Structure):
    pass


# 3. Function signatures
lib.quat_stream_create.argtypes = [
    ctypes.POINTER(ctypes.c_uint8),
    ctypes.c_size_t,
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.POINTER(_FfiQuatStream)),
]
lib.quat_stream_create.restype = ctypes.c_int32

lib.quat_stream_destroy.argtypes = [ctypes.POINTER(_FfiQuatStream)]
lib.quat_stream_destroy.restype = None

lib.quat_stream_len.argtypes = [ctypes.POINTER(_FfiQuatStream)]
lib.quat_stream_len.restype = ctypes.c_size_t

lib.quat_stream_get.argtypes = [
    ctypes.POINTER(_FfiQuatStream),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_uint8),
    ctypes.POINTER(ctypes.c_int32),
]
lib.quat_stream_get.restype = ctypes.c_int32

lib.quat_pack_buffer.argtypes = [
    ctypes.POINTER(ctypes.c_uint8),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_uint8),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_size_t),
]
lib.quat_pack_buffer.restype = ctypes.c_int32

lib.quat_unpack_buffer.argtypes = [
    ctypes.POINTER(ctypes.c_uint8),
    ctypes.c_size_t,
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_uint8),
    ctypes.c_size_t,
]
lib.quat_unpack_buffer.restype = ctypes.c_int32

# 4. Pack raw quaternary values (0..=3) into 2-bit packed bytes
raw_quats = (ctypes.c_uint8 * 6)(0, 1, 2, 3, 1, 2)
packed_buf = (ctypes.c_uint8 * 4)()
written = ctypes.c_size_t(0)

rc_pack = lib.quat_pack_buffer(raw_quats, 6, packed_buf, 4, ctypes.byref(written))
assert rc_pack == 0, f"quat_pack_buffer failed with code {rc_pack}"

# 5. Create stream, read length and individual elements, then unpack
stream_ptr = ctypes.POINTER(_FfiQuatStream)()
rc_create = lib.quat_stream_create(packed_buf, written.value, 6, ctypes.byref(stream_ptr))
assert rc_create == 0, f"quat_stream_create failed with code {rc_create}"

try:
    total_len = lib.quat_stream_len(stream_ptr)
    print("Stream length:", total_len)

    out_val = ctypes.c_uint8(0)
    out_err = ctypes.c_int32(0)
    rc_get = lib.quat_stream_get(stream_ptr, 2, ctypes.byref(out_val), ctypes.byref(out_err))
    assert rc_get == 0 and out_err.value == 0
    print("Quat at index 2:", out_val.value)

    unpacked_buf = (ctypes.c_uint8 * 6)()
    rc_unpack = lib.quat_unpack_buffer(packed_buf, written.value, 6, unpacked_buf, 6)
    assert rc_unpack == 0
    print("Unpacked values:", list(unpacked_buf))
finally:
    lib.quat_stream_destroy(stream_ptr)
```

---

## FFI Integration (C Example)

If you are using `quat_input` from a native **C** or **C++** application, you can leverage the exposed FFI interface with panic safety and raw pointer management:

```c
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct FfiQuatStream FfiQuatStream;

extern int32_t quat_stream_create(
    const uint8_t* bytes,
    size_t len,
    size_t total_quats,
    FfiQuatStream** out_stream
);
extern void quat_stream_destroy(FfiQuatStream* stream);
extern size_t quat_stream_len(const FfiQuatStream* stream);
extern int32_t quat_stream_get(
    const FfiQuatStream* stream,
    size_t index,
    uint8_t* out_val,
    int32_t* out_err
);
extern int32_t quat_pack_buffer(
    const uint8_t* raw_quats,
    size_t quat_len,
    uint8_t* out_bytes,
    size_t out_capacity,
    size_t* out_written
);
extern int32_t quat_unpack_buffer(
    const uint8_t* packed_bytes,
    size_t packed_len,
    size_t total_quats,
    uint8_t* out_quats,
    size_t out_capacity
);

int main(void) {
    // 1. Pack raw quaternary values (0..=3) into packed bytes
    uint8_t raw_quats[6] = {0, 1, 2, 3, 1, 2};
    uint8_t packed_bytes[4] = {0};
    size_t written = 0;

    int32_t pack_status = quat_pack_buffer(raw_quats, 6, packed_bytes, 4, &written);
    if (pack_status != 0) {
        fprintf(stderr, "quat_pack_buffer failed: %d\n", pack_status);
        return 1;
    }

    // 2. Create stream handle over packed bytes
    FfiQuatStream* stream = NULL;
    int32_t create_res = quat_stream_create(packed_bytes, written, 6, &stream);
    if (create_res != 0 || stream == NULL) {
        fprintf(stderr, "Failed to create FFI quat stream, code: %d\n", create_res);
        return 1;
    }

    // 3. Query length and read element by index
    size_t len = quat_stream_len(stream);
    uint8_t out_val = 0;
    int32_t out_err = 0;
    int32_t get_status = quat_stream_get(stream, 2, &out_val, &out_err);
    if (get_status == 0 && out_err == 0) {
        printf("Stream len: %zu, Quat at index 2: %u\n", len, out_val);
    }

    // 4. Batch unpack back to u8 array
    uint8_t unpacked[6] = {0};
    int32_t unpack_status = quat_unpack_buffer(packed_bytes, written, 6, unpacked, 6);
    if (unpack_status == 0) {
        printf("Unpacked: %u %u %u %u %u %u\n",
               unpacked[0], unpacked[1], unpacked[2],
               unpacked[3], unpacked[4], unpacked[5]);
    }

    // 5. Destroy stream handle
    quat_stream_destroy(stream);
    return 0;
}
```

---

## Error Handling

The library utilizes a custom `QuatError` enum covering common validation and boundary issues:
- `InvalidChar(char)`: Triggered when an unexpected character is parsed.
- `InvalidRawValue(u8)`: Triggered when an out-of-range raw value is provided.
- `IndexOutOfBounds(usize)`: Triggered when accessing a stream position past its length.

---

## Testing

Run the test suite (covering basic parsing, error boundaries, stress tests with 16,000+ elements, and FFI safety) using Cargo:

```bash
cargo test
```