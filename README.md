# quat_input

`quat_input` is a high-performance, zero-copy Rust library designed for managing quaternary data streams (base-4). Each quaternary digit (`quat`) packs exactly 2 bits of information (`0b00`, `0b01`, `0b10`, `0b11`). 

The library provides zero-allocation stream views, efficient bit-packing, robust error management, and a production-grade FFI (Foreign Function Interface) layer with panic safety for seamless integration with C, C++, Python, and other languages.

---

## Features

- **Zero-Copy Stream Views**: Read and iterate over quaternary streams directly from raw byte slices without heap allocations.
- **Bit-Packed Storage**: Efficiently packs 4 quaternary digits into a single byte (`QuatByte`).
- **Compile-Time & Run-Time Safety**: Supports `const` contexts for parsing and validation.
- **Production-Ready FFI**: Fully equipped with `std::panic::catch_unwind` protection to ensure safe cross-language calls without risking undefined behavior or process crashes.

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
quat_input = "0.1.0"
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
use quat_input::{QuatInput, QuatStream};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Parse string input containing quaternary characters ('0' to '3')
    let input = QuatInput::from_str("01233210")?;

    // 2. Pack the quaternary digits into raw bytes (4 quats per byte)
    let packed_bytes = input.pack();
    println!("Packed into {} byte(s)", packed_bytes.len());

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