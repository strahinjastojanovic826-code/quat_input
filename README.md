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

---

## Benchmark Performance Results

| Test Name / Component | Workload Description | Execution Time |
| :--- | :--- | :--- |
| `test_01_single_quat_parse` | Single character parsing | 4.9 µs |
| `test_02_quat_input_from_str` | Basic string parsing into `QuatInput` | 20.8 µs |
| `test_03_quat_byte_packing` | Packing 4 quats into a single byte | 7.7 µs |
| `test_04_quat_stream_get` | Zero-allocation index lookup | 700 ns |
| `test_05_quat_stream_iterator` | Zero-allocation stream iteration | 3.4 µs |
| `test_06_error_invalid_char` | Error handling for invalid characters | 2.1 µs |
| `test_07_error_index_out_of_bounds` | Boundary safety check (Index out of bounds) | 500 ns |
| `test_08_const_functions` | Compile-time (`const`) evaluation | 100 ns |
| `test_09_stress_empty_and_boundaries` | Edge cases & empty stream stress check | 1.9 µs |
| `test_10_stress_large_stream` | Massive stream processing (16,000 quats) | 1.30 ms |
| `test_11_stress_ffi_safety_and_panics` | FFI safety, pointer handling & panic catch | 6.1 µs |
| `test_12_quat_arithmetic_and_logic` | Quaternary arithmetic & multi-valued logic | 500 ns |
| `test_13_pack_fast_equivalence` | Fast vectorized packing verification | 2.9 µs |
| `test_14_simd_packing_optimized` | SIMD-optimized pack (1,000 iterations) | 196.76 ms |
| `test_15_dsp_modulation` | QPSK stream modulation (10,000 elements) | 172 µs |
| `test_16_flash_page_creation` | NAND Flash page creation (1,000 iterations) | 1.20 ms |

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
quat_input = "0.3.0"
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

## FFI Integration (Python Example)

Because the library exposes a stable C ABI (`extern "C"`) protected with panic safety (`catch_unwind`), you can easily integrate it into Python via `ctypes`:

```python
import ctypes
import os

# 1. Učitavanje biblioteke
lib_path = os.path.abspath("./target/release/libquat_input.so")
lib = ctypes.CDLL(lib_path)

# 2. Definicija strukture
class _FfiQuatStream(ctypes.Structure):
    pass

# 3. Povezivanje argumenata i povratnih vrednosti (Obavezno za rad!)
lib.quat_stream_create.argtypes = [
    ctypes.POINTER(ctypes.c_uint8), 
    ctypes.c_size_t, 
    ctypes.c_size_t, 
    ctypes.POINTER(ctypes.POINTER(_FfiQuatStream))
]
lib.quat_stream_create.restype = ctypes.c_int32

lib.quat_stream_destroy.argtypes = [ctypes.POINTER(_FfiQuatStream)]
lib.quat_stream_destroy.restype = None

lib.quat_stream_get.argtypes = [
    ctypes.POINTER(_FfiQuatStream), 
    ctypes.c_size_t, 
    ctypes.POINTER(ctypes.c_uint8), 
    ctypes.POINTER(ctypes.c_int32)
]
lib.quat_stream_get.restype = ctypes.c_int32
```

---

## FFI Integration (C Example)

If you are using `quat_input` from a native **C** or **C++** application, you can leverage the exposed FFI interface with panic safety and raw pointer management:

```c
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

// Opaque structure matching FfiQuatStream
typedef struct FfiQuatStream FfiQuatStream;

// FFI function declarations exported by libquat_input
extern int32_t quat_stream_create(const uint8_t* bytes, size_t len, size_t total_quats, FfiQuatStream** out_stream);
extern void quat_stream_destroy(FfiQuatStream* stream);
extern int32_t quat_stream_get(const FfiQuatStream* stream, size_t index, uint8_t* out_val, int32_t* out_err);

int main() {
    // 1. Raw packed bytes (e.g., 2 bytes containing 8 quaternary values)
    uint8_t raw_bytes[] = {0xE4, 0x1B};
    size_t total_quats = 8;

    FfiQuatStream* stream = NULL;

    // 2. Safely create the stream (with catch_unwind protection on Rust side)
    int32_t create_res = quat_stream_create(raw_bytes, sizeof(raw_bytes), total_quats, &stream);
    if (create_res != 0 || stream == NULL) {
        printf("Failed to create FFI quat stream, error code: %d\n", create_res);
        return 1;
    }

    uint8_t out_val = 0;
    int32_t out_err = 0;

    // 3. Read an element safely by index without allocation
    int32_t status = quat_stream_get(stream, 0, &out_val, &out_err);
    if (status == 0 && out_err == 0) {
        printf("Quat at index 0: %u\n", out_val);
    }

    // 4. Clean up memory and destroy the stream
    quat_stream_destroy(stream);
    printf("Stream successfully destroyed.\n");

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