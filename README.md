# isa-l-rust

Rust FFI bindings for [Intel® Intelligent Storage Acceleration Library (ISA-L)](https://github.com/intel/isa-l).

ISA-L provides highly optimized, processor-specific implementations of storage-related functions including CRC computation, erasure coding, compression, RAID parity, and memory utilities. This crate compiles ISA-L from source via CMake and exposes its C API directly to Rust.

## Modules

| Module | Description |
|--------|-------------|
| **CRC16/32** | T10-DIF, IEEE, gzip-reflected, and iSCSI CRC variants |
| **CRC64** | ECMA, ISO, Jones, and Rocksoft polynomials in reflected/normal forms |
| **Erasure Code** | Reed-Solomon encode/decode with GF(2⁸) arithmetic |
| **GF(2⁸) Vector** | Galois field vector multiply, dot product, and multiply-accumulate |
| **igzip** | Deflate/inflate with gzip and zlib wrapper support |
| **RAID** | XOR (RAID5) and P+Q (RAID6) parity generation and verification |
| **Memory** | Zero-detect utility |

All multi-binary functions auto-dispatch to the best available instruction set at runtime (SSE, AVX, AVX2, etc.). Architecture-specific variants are also exposed behind `#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]`.

## Requirements

- **Rust** 1.56+ (edition 2021)
- **CMake** 3.x
- **NASM** 2.15+ (for x86/x86_64 assembly kernels)
- A C compiler (gcc or clang)

On Ubuntu/Debian:

```sh
sudo apt install cmake nasm build-essential
```

## Building

```sh
cargo build
```

The build script compiles ISA-L as a static library via CMake and links it into the crate. No system-wide ISA-L installation is required — the source is included as a Git submodule under `isa-l/`.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
isa-l-rust = { path = "path/to/isa-l-rust" }
```

### CRC32 Example

```rust
use isa_l_rust::crc32_gzip_refl;

let data = b"Hello, ISA-L!";
let crc = unsafe { crc32_gzip_refl(0, data.as_ptr(), data.len() as u64) };
println!("CRC32: {crc:#010x}");
```

### Erasure Coding Example

```rust
use isa_l_rust::*;

// 3 data sources + 2 parity, each block is 1024 bytes
let k = 3i32;
let p = 2i32;
let len = 1024i32;

unsafe {
    let mut matrix = vec![0u8; ((k + p) * k) as usize];
    gf_gen_cauchy1_matrix(matrix.as_mut_ptr(), k + p, k);

    // Initialize encoding tables from the parity rows of the matrix
    let mut g_tbls = vec![0u8; (k * p * 32) as usize];
    ec_init_tables(k, p, matrix[(k * k) as usize..].as_mut_ptr(), g_tbls.as_mut_ptr());

    // Allocate data and coding buffers, then encode...
    // ec_encode_data(len, k, p, g_tbls.as_mut_ptr(), data_ptrs, coding_ptrs);
}
```

### GF(2⁸) Arithmetic Example

```rust
use isa_l_rust::{gf_mul, gf_inv};

unsafe {
    let a: u8 = 42;
    let inv = gf_inv(a);
    assert_eq!(gf_mul(a, inv), 1); // a * a⁻¹ = 1 in GF(2⁸)
}
```

## Opaque Structs

The igzip compression/decompression structs (`isal_zstream`, `inflate_state`, `isal_hufftables`, etc.) are exposed as opaque `#[repr(C)]` types. They must be allocated and initialized through the provided C functions:

```rust
use std::alloc::{alloc_zeroed, dealloc, Layout};
use isa_l_rust::*;

unsafe {
    // Allocate a zeroed isal_zstream (get the size from C sizeof)
    // Then initialize it:
    // isal_deflate_init(stream_ptr);
}
```

## Testing

```sh
cargo test
```

Tests verify version queries, CRC determinism, GF(2⁸) multiplicative inverses, zero-detection, and Adler-32 computation.

## License

The ISA-L C library is licensed under the [BSD 3-Clause License](isa-l/LICENSE). This Rust binding crate follows the same license.
