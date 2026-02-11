//! Rust FFI bindings for Intel ISA-L (Intelligent Storage Acceleration Library).
//!
//! This crate provides raw FFI bindings to the isa-l library, which contains
//! optimized functions for:
//! - CRC (16/32/64-bit) computation
//! - Erasure coding (Reed-Solomon encode/decode)
//! - Compression/decompression (igzip - deflate/gzip/zlib)
//! - RAID (XOR and P+Q parity)
//! - Memory utilities
//! - GF(2^8) vector operations
//!
//! The igzip compression structs are complex and layout-sensitive. This crate
//! exposes them as opaque types meant to be allocated and initialized via the
//! provided C functions (e.g. `isal_deflate_init`, `isal_inflate_init`).

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_void};

// ---------------------------------------------------------------------------
// Constants: isal_api.h
// ---------------------------------------------------------------------------
pub const ISAL_MAJOR_VERSION: u32 = 2;
pub const ISAL_MINOR_VERSION: u32 = 31;
pub const ISAL_PATCH_VERSION: u32 = 1;

// ---------------------------------------------------------------------------
// Constants: igzip_lib.h  — Flush flags
// ---------------------------------------------------------------------------
pub const NO_FLUSH: c_int = 0;
pub const SYNC_FLUSH: c_int = 1;
pub const FULL_FLUSH: c_int = 2;

// ---------------------------------------------------------------------------
// Constants: igzip_lib.h  — Gzip flags
// ---------------------------------------------------------------------------
pub const IGZIP_DEFLATE: c_int = 0;
pub const IGZIP_GZIP: c_int = 1;
pub const IGZIP_GZIP_NO_HDR: c_int = 2;
pub const IGZIP_ZLIB: c_int = 3;
pub const IGZIP_ZLIB_NO_HDR: c_int = 4;

// ---------------------------------------------------------------------------
// Constants: igzip_lib.h  — Compression return values
// ---------------------------------------------------------------------------
pub const COMP_OK: c_int = 0;
pub const INVALID_FLUSH: c_int = -7;
pub const INVALID_PARAM: c_int = -8;
pub const STATELESS_OVERFLOW: c_int = -1;
pub const ISAL_INVALID_OPERATION: c_int = -9;
pub const ISAL_INVALID_STATE: c_int = -3;
pub const ISAL_INVALID_LEVEL: c_int = -4;
pub const ISAL_INVALID_LEVEL_BUF: c_int = -5;

// ---------------------------------------------------------------------------
// Constants: igzip_lib.h  — Inflate return values
// ---------------------------------------------------------------------------
pub const ISAL_DECOMP_OK: c_int = 0;
pub const ISAL_END_INPUT: c_int = 1;
pub const ISAL_OUT_OVERFLOW: c_int = 2;
pub const ISAL_NAME_OVERFLOW: c_int = 3;
pub const ISAL_COMMENT_OVERFLOW: c_int = 4;
pub const ISAL_EXTRA_OVERFLOW: c_int = 5;
pub const ISAL_NEED_DICT: c_int = 6;
pub const ISAL_INVALID_BLOCK: c_int = -1;
pub const ISAL_INVALID_SYMBOL: c_int = -2;
pub const ISAL_INVALID_LOOKBACK: c_int = -3;
pub const ISAL_INVALID_WRAPPER: c_int = -4;
pub const ISAL_UNSUPPORTED_METHOD: c_int = -5;
pub const ISAL_INCORRECT_CHECKSUM: c_int = -6;

// ---------------------------------------------------------------------------
// Constants: igzip_lib.h  — Hufftable types
// ---------------------------------------------------------------------------
pub const IGZIP_HUFFTABLE_CUSTOM: c_int = 0;
pub const IGZIP_HUFFTABLE_DEFAULT: c_int = 1;
pub const IGZIP_HUFFTABLE_STATIC: c_int = 2;

// ---------------------------------------------------------------------------
// Constants: igzip_lib.h  — Compression levels
// ---------------------------------------------------------------------------
pub const ISAL_DEF_MIN_LEVEL: c_int = 0;
pub const ISAL_DEF_MAX_LEVEL: c_int = 3;

// ---------------------------------------------------------------------------
// Constants: igzip_lib.h  — Inflate flags
// ---------------------------------------------------------------------------
pub const ISAL_INFLATE_DEFLATE: c_int = 0;
pub const ISAL_INFLATE_GZIP: c_int = 1;
pub const ISAL_INFLATE_GZIP_NO_HDR: c_int = 2;
pub const ISAL_INFLATE_ZLIB: c_int = 3;
pub const ISAL_INFLATE_ZLIB_NO_HDR: c_int = 4;
pub const ISAL_INFLATE_ZLIB_NO_HDR_VER: c_int = 5;
pub const ISAL_INFLATE_GZIP_NO_HDR_VER: c_int = 6;

// ---------------------------------------------------------------------------
// Opaque types for complex igzip structs.
//
// These are layout-sensitive C structs with compile-time constant arrays.
// They should be allocated via C helper functions (isal_deflate_init, etc.)
// or via alloc_zeroed with the correct size obtained from C sizeof.
// ---------------------------------------------------------------------------

/// Opaque type for `struct isal_zstream` (compression stream).
#[repr(C)]
pub struct isal_zstream {
    _opaque: [u8; 0],
}

/// Opaque type for `struct inflate_state` (decompression state).
#[repr(C)]
pub struct inflate_state {
    _opaque: [u8; 0],
}

/// Opaque type for `struct isal_hufftables`.
#[repr(C)]
pub struct isal_hufftables {
    _opaque: [u8; 0],
}

/// Opaque type for `struct isal_huff_histogram`.
#[repr(C)]
pub struct isal_huff_histogram {
    _opaque: [u8; 0],
}

/// Opaque type for `struct isal_gzip_header`.
#[repr(C)]
pub struct isal_gzip_header {
    _opaque: [u8; 0],
}

/// Opaque type for `struct isal_zlib_header`.
#[repr(C)]
pub struct isal_zlib_header {
    _opaque: [u8; 0],
}

/// Opaque type for `struct isal_dict`.
#[repr(C)]
pub struct isal_dict {
    _opaque: [u8; 0],
}

// ---------------------------------------------------------------------------
// FFI function declarations
// ---------------------------------------------------------------------------
extern "C" {
    // ======================================================================
    // isal_api.h — Version
    // ======================================================================

    /// Get library version in string format.
    pub fn isal_get_version_str() -> *const c_char;

    /// Get library version in numerical format.
    pub fn isal_get_version() -> c_uint;

    // ======================================================================
    // crc.h — CRC16 / CRC32 functions
    // ======================================================================

    /// CRC16 T10-DIF, multi-binary (auto-dispatched).
    pub fn crc16_t10dif(init_crc: u16, buf: *const c_uchar, len: u64) -> u16;

    /// CRC16 T10-DIF with copy, multi-binary.
    pub fn crc16_t10dif_copy(init_crc: u16, dst: *mut u8, src: *mut u8, len: u64) -> u16;

    /// CRC32 IEEE (normal polynomial), multi-binary.
    pub fn crc32_ieee(init_crc: u32, buf: *const c_uchar, len: u64) -> u32;

    /// CRC32 gzip reflected (RFC 1952), multi-binary.
    pub fn crc32_gzip_refl(init_crc: u32, buf: *const c_uchar, len: u64) -> u32;

    /// CRC32 iSCSI, multi-binary.
    pub fn crc32_iscsi(buffer: *mut c_uchar, len: c_int, init_crc: c_uint) -> c_uint;

    // CRC base (software fallback) versions
    pub fn crc16_t10dif_base(seed: u16, buf: *mut u8, len: u64) -> u16;
    pub fn crc16_t10dif_copy_base(init_crc: u16, dst: *mut u8, src: *mut u8, len: u64) -> u16;
    pub fn crc32_ieee_base(seed: u32, buf: *mut u8, len: u64) -> u32;
    pub fn crc32_gzip_refl_base(seed: u32, buf: *mut u8, len: u64) -> u32;
    pub fn crc32_iscsi_base(buffer: *mut c_uchar, len: c_int, crc_init: c_uint) -> c_uint;

    // ======================================================================
    // crc64.h — CRC64 functions
    // ======================================================================

    // Multi-binary (auto-dispatched)
    pub fn crc64_ecma_refl(init_crc: u64, buf: *const c_uchar, len: u64) -> u64;
    pub fn crc64_ecma_norm(init_crc: u64, buf: *const c_uchar, len: u64) -> u64;
    pub fn crc64_iso_refl(init_crc: u64, buf: *const c_uchar, len: u64) -> u64;
    pub fn crc64_iso_norm(init_crc: u64, buf: *const c_uchar, len: u64) -> u64;
    pub fn crc64_jones_refl(init_crc: u64, buf: *const c_uchar, len: u64) -> u64;
    pub fn crc64_jones_norm(init_crc: u64, buf: *const c_uchar, len: u64) -> u64;
    pub fn crc64_rocksoft_refl(init_crc: u64, buf: *const c_uchar, len: u64) -> u64;
    pub fn crc64_rocksoft_norm(init_crc: u64, buf: *const c_uchar, len: u64) -> u64;

    // Base (software fallback) versions
    pub fn crc64_ecma_refl_base(init_crc: u64, buf: *const c_uchar, len: u64) -> u64;
    pub fn crc64_ecma_norm_base(init_crc: u64, buf: *const c_uchar, len: u64) -> u64;
    pub fn crc64_iso_refl_base(init_crc: u64, buf: *const c_uchar, len: u64) -> u64;
    pub fn crc64_iso_norm_base(init_crc: u64, buf: *const c_uchar, len: u64) -> u64;
    pub fn crc64_jones_refl_base(init_crc: u64, buf: *const c_uchar, len: u64) -> u64;
    pub fn crc64_jones_norm_base(init_crc: u64, buf: *const c_uchar, len: u64) -> u64;
    pub fn crc64_rocksoft_refl_base(init_crc: u64, buf: *const c_uchar, len: u64) -> u64;
    pub fn crc64_rocksoft_norm_base(init_crc: u64, buf: *const c_uchar, len: u64) -> u64;

    // Arch-specific by8 versions (x86 SSE3+CLMUL)
    pub fn crc64_ecma_refl_by8(init_crc: u64, buf: *const c_uchar, len: u64) -> u64;
    pub fn crc64_ecma_norm_by8(init_crc: u64, buf: *const c_uchar, len: u64) -> u64;
    pub fn crc64_iso_refl_by8(init_crc: u64, buf: *const c_uchar, len: u64) -> u64;
    pub fn crc64_iso_norm_by8(init_crc: u64, buf: *const c_uchar, len: u64) -> u64;
    pub fn crc64_jones_refl_by8(init_crc: u64, buf: *const c_uchar, len: u64) -> u64;
    pub fn crc64_jones_norm_by8(init_crc: u64, buf: *const c_uchar, len: u64) -> u64;
    pub fn crc64_rocksoft_refl_by8(init_crc: u64, buf: *const c_uchar, len: u64) -> u64;
    pub fn crc64_rocksoft_norm_by8(init_crc: u64, buf: *const c_uchar, len: u64) -> u64;

    // ======================================================================
    // erasure_code.h — Erasure coding
    // ======================================================================

    /// Initialize tables for fast erasure code encode/decode.
    pub fn ec_init_tables(k: c_int, rows: c_int, a: *mut c_uchar, gftbls: *mut c_uchar);
    pub fn ec_init_tables_base(k: c_int, rows: c_int, a: *mut c_uchar, gftbls: *mut c_uchar);
    pub fn ec_init_tables_gfni(k: c_int, rows: c_int, a: *mut c_uchar, gftbls: *mut c_uchar);

    /// Generate or decode erasure codes on blocks of data (multi-binary).
    pub fn ec_encode_data(
        len: c_int,
        k: c_int,
        rows: c_int,
        gftbls: *mut c_uchar,
        data: *mut *mut c_uchar,
        coding: *mut *mut c_uchar,
    );
    pub fn ec_encode_data_base(
        len: c_int,
        srcs: c_int,
        dests: c_int,
        v: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    /// Single-source update for erasure code encode/decode (multi-binary).
    pub fn ec_encode_data_update(
        len: c_int,
        k: c_int,
        rows: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        data: *mut c_uchar,
        coding: *mut *mut c_uchar,
    );
    pub fn ec_encode_data_update_base(
        len: c_int,
        k: c_int,
        rows: c_int,
        vec_i: c_int,
        v: *mut c_uchar,
        data: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    // GF(2^8) vector dot product (multi-binary + base)
    pub fn gf_vect_dot_prod(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut c_uchar,
    );
    pub fn gf_vect_dot_prod_base(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut c_uchar,
    );

    // GF(2^8) vector multiply-accumulate (multi-binary + base)
    pub fn gf_vect_mad(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut c_uchar,
    );
    pub fn gf_vect_mad_base(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        v: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut c_uchar,
    );

    // GF(2^8) utility functions
    /// Single element GF(2^8) multiply.
    pub fn gf_mul(a: c_uchar, b: c_uchar) -> c_uchar;

    /// Single element GF(2^8) inverse.
    pub fn gf_inv(a: c_uchar) -> c_uchar;

    /// Generate Reed-Solomon (Vandermonde) encoding matrix.
    pub fn gf_gen_rs_matrix(a: *mut c_uchar, m: c_int, k: c_int);

    /// Generate Cauchy encoding matrix.
    pub fn gf_gen_cauchy1_matrix(a: *mut c_uchar, m: c_int, k: c_int);

    /// Invert a matrix in GF(2^8). Returns 0 on success.
    pub fn gf_invert_matrix(input: *mut c_uchar, output: *mut c_uchar, n: c_int) -> c_int;

    // ======================================================================
    // gf_vect_mul.h — GF(2^8) vector multiply
    // ======================================================================

    /// GF(2^8) vector multiply by constant (multi-binary).
    pub fn gf_vect_mul(
        len: c_int,
        gftbl: *mut c_uchar,
        src: *mut c_void,
        dest: *mut c_void,
    ) -> c_int;

    /// Initialize 32-byte table for GF(2^8) vector multiply.
    pub fn gf_vect_mul_init(c: c_uchar, gftbl: *mut c_uchar);

    /// GF(2^8) vector multiply by constant (baseline).
    pub fn gf_vect_mul_base(
        len: c_int,
        a: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut c_uchar,
    ) -> c_int;

    // ======================================================================
    // igzip_lib.h — Compression
    // ======================================================================

    /// Initialize compression stream.
    pub fn isal_deflate_init(stream: *mut isal_zstream);

    /// Reset compression stream (preserves user settings).
    pub fn isal_deflate_reset(stream: *mut isal_zstream);

    /// Initialize compression stream for stateless operation.
    pub fn isal_deflate_stateless_init(stream: *mut isal_zstream);

    /// Compress data. Returns COMP_OK or error code.
    pub fn isal_deflate(stream: *mut isal_zstream) -> c_int;

    /// Stateless (one-shot) compression. Returns COMP_OK or error code.
    pub fn isal_deflate_stateless(stream: *mut isal_zstream) -> c_int;

    /// Set huffman tables on compression stream.
    pub fn isal_deflate_set_hufftables(
        stream: *mut isal_zstream,
        hufftables: *mut isal_hufftables,
        r#type: c_int,
    ) -> c_int;

    /// Set compression dictionary.
    pub fn isal_deflate_set_dict(stream: *mut isal_zstream, dict: *mut u8, dict_len: u32) -> c_int;

    /// Process dictionary for reuse.
    pub fn isal_deflate_process_dict(
        stream: *mut isal_zstream,
        dict_str: *mut isal_dict,
        dict: *mut u8,
        dict_len: u32,
    ) -> c_int;

    /// Reset compression dictionary from pre-processed data.
    pub fn isal_deflate_reset_dict(stream: *mut isal_zstream, dict_str: *mut isal_dict) -> c_int;

    /// Update histogram of deflate symbols.
    pub fn isal_update_histogram(
        in_stream: *mut u8,
        length: c_int,
        histogram: *mut isal_huff_histogram,
    );

    /// Create custom huffman tables from histogram.
    pub fn isal_create_hufftables(
        hufftables: *mut isal_hufftables,
        histogram: *mut isal_huff_histogram,
    ) -> c_int;

    /// Create custom huffman tables (subset, skips zero-frequency literals).
    pub fn isal_create_hufftables_subset(
        hufftables: *mut isal_hufftables,
        histogram: *mut isal_huff_histogram,
    ) -> c_int;

    /// Initialize gzip header with defaults.
    pub fn isal_gzip_header_init(gz_hdr: *mut isal_gzip_header);

    /// Initialize zlib header with defaults.
    pub fn isal_zlib_header_init(z_hdr: *mut isal_zlib_header);

    /// Write gzip header to output stream.
    pub fn isal_write_gzip_header(stream: *mut isal_zstream, gz_hdr: *mut isal_gzip_header) -> u32;

    /// Write zlib header to output stream.
    pub fn isal_write_zlib_header(stream: *mut isal_zstream, z_hdr: *mut isal_zlib_header) -> u32;

    // ======================================================================
    // igzip_lib.h — Decompression
    // ======================================================================

    /// Initialize decompression state.
    pub fn isal_inflate_init(state: *mut inflate_state);

    /// Reset decompression state.
    pub fn isal_inflate_reset(state: *mut inflate_state);

    /// Decompress data. Returns ISAL_DECOMP_OK or error code.
    pub fn isal_inflate(state: *mut inflate_state) -> c_int;

    /// Stateless (one-shot) decompression.
    pub fn isal_inflate_stateless(state: *mut inflate_state) -> c_int;

    /// Set decompression dictionary.
    pub fn isal_inflate_set_dict(state: *mut inflate_state, dict: *mut u8, dict_len: u32) -> c_int;

    /// Read gzip header from input stream.
    pub fn isal_read_gzip_header(state: *mut inflate_state, gz_hdr: *mut isal_gzip_header)
        -> c_int;

    /// Read zlib header from input stream.
    pub fn isal_read_zlib_header(
        state: *mut inflate_state,
        zlib_hdr: *mut isal_zlib_header,
    ) -> c_int;

    // ======================================================================
    // igzip_lib.h — Adler32
    // ======================================================================

    /// Adler-32 checksum (multi-binary).
    pub fn isal_adler32(init: u32, buf: *const c_uchar, len: u64) -> u32;

    // ======================================================================
    // mem_routines.h — Memory utilities
    // ======================================================================

    /// Detect if a memory region is all zeros.
    /// Returns 0 if all zeros, non-zero otherwise.
    pub fn isal_zero_detect(mem: *mut c_void, len: usize) -> c_int;

    // ======================================================================
    // raid.h — RAID parity (multi-binary)
    // ======================================================================

    /// Generate XOR parity vector (RAID5).
    pub fn xor_gen(vects: c_int, len: c_int, array: *mut *mut c_void) -> c_int;

    /// Check XOR parity.
    pub fn xor_check(vects: c_int, len: c_int, array: *mut *mut c_void) -> c_int;

    /// Generate P+Q parity vectors (RAID6).
    pub fn pq_gen(vects: c_int, len: c_int, array: *mut *mut c_void) -> c_int;

    /// Check P+Q parity.
    pub fn pq_check(vects: c_int, len: c_int, array: *mut *mut c_void) -> c_int;

    // RAID base versions
    pub fn xor_gen_base(vects: c_int, len: c_int, array: *mut *mut c_void) -> c_int;
    pub fn xor_check_base(vects: c_int, len: c_int, array: *mut *mut c_void) -> c_int;
    pub fn pq_gen_base(vects: c_int, len: c_int, array: *mut *mut c_void) -> c_int;
    pub fn pq_check_base(vects: c_int, len: c_int, array: *mut *mut c_void) -> c_int;
}

// ===========================================================================
// x86-specific arch variants (SSE/AVX/AVX2)
// ===========================================================================
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
extern "C" {
    // --- gf_vect_mul arch variants ---
    pub fn gf_vect_mul_sse(
        len: c_int,
        gftbl: *mut c_uchar,
        src: *mut c_void,
        dest: *mut c_void,
    ) -> c_int;
    pub fn gf_vect_mul_avx(
        len: c_int,
        gftbl: *mut c_uchar,
        src: *mut c_void,
        dest: *mut c_void,
    ) -> c_int;

    // --- ec_encode_data arch variants ---
    pub fn ec_encode_data_sse(
        len: c_int,
        k: c_int,
        rows: c_int,
        gftbls: *mut c_uchar,
        data: *mut *mut c_uchar,
        coding: *mut *mut c_uchar,
    );
    pub fn ec_encode_data_avx(
        len: c_int,
        k: c_int,
        rows: c_int,
        gftbls: *mut c_uchar,
        data: *mut *mut c_uchar,
        coding: *mut *mut c_uchar,
    );
    pub fn ec_encode_data_avx2(
        len: c_int,
        k: c_int,
        rows: c_int,
        gftbls: *mut c_uchar,
        data: *mut *mut c_uchar,
        coding: *mut *mut c_uchar,
    );

    // --- ec_encode_data_gfni arch variants ---
    pub fn ec_encode_data_avx2_gfni(
        len: c_int,
        k: c_int,
        rows: c_int,
        gftbls: *mut c_uchar,
        data: *mut *mut c_uchar,
        coding: *mut *mut c_uchar,
    );
    pub fn ec_encode_data_avx512_gfni(
        len: c_int,
        k: c_int,
        rows: c_int,
        gftbls: *mut c_uchar,
        data: *mut *mut c_uchar,
        coding: *mut *mut c_uchar,
    );

    // --- ec_encode_data_update arch variants ---
    pub fn ec_encode_data_update_sse(
        len: c_int,
        k: c_int,
        rows: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        data: *mut c_uchar,
        coding: *mut *mut c_uchar,
    );
    pub fn ec_encode_data_update_avx(
        len: c_int,
        k: c_int,
        rows: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        data: *mut c_uchar,
        coding: *mut *mut c_uchar,
    );
    pub fn ec_encode_data_update_avx2(
        len: c_int,
        k: c_int,
        rows: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        data: *mut c_uchar,
        coding: *mut *mut c_uchar,
    );

    // --- ec_encode_data_update_gfni arch variants ---
    pub fn ec_encode_data_update_avx2_gfni(
        len: c_int,
        k: c_int,
        rows: c_int,
        gftbls: *mut c_uchar,
        data: *mut *mut c_uchar,
        coding: *mut *mut c_uchar,
    );
    pub fn ec_encode_data_update_avx512_gfni(
        len: c_int,
        k: c_int,
        rows: c_int,
        gftbls: *mut c_uchar,
        data: *mut *mut c_uchar,
        coding: *mut *mut c_uchar,
    );

    // --- gf_vect_dot_prod arch variants ---
    pub fn gf_vect_dot_prod_sse(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut c_uchar,
    );
    pub fn gf_vect_dot_prod_avx(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut c_uchar,
    );
    pub fn gf_vect_dot_prod_avx2(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut c_uchar,
    );

    // --- gf_2vect_dot_prod arch variants ---
    pub fn gf_2vect_dot_prod_sse(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );
    pub fn gf_2vect_dot_prod_avx(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );
    pub fn gf_2vect_dot_prod_avx2(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    // --- gf_3vect_dot_prod arch variants ---
    pub fn gf_3vect_dot_prod_sse(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );
    pub fn gf_3vect_dot_prod_avx(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );
    pub fn gf_3vect_dot_prod_avx2(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    // --- gf_4vect_dot_prod arch variants ---
    pub fn gf_4vect_dot_prod_sse(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );
    pub fn gf_4vect_dot_prod_avx(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );
    pub fn gf_4vect_dot_prod_avx2(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    // --- gf_5vect_dot_prod arch variants ---
    pub fn gf_5vect_dot_prod_sse(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );
    pub fn gf_5vect_dot_prod_avx(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );
    pub fn gf_5vect_dot_prod_avx2(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    // --- gf_6vect_dot_prod arch variants ---
    pub fn gf_6vect_dot_prod_sse(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );
    pub fn gf_6vect_dot_prod_avx(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );
    pub fn gf_6vect_dot_prod_avx2(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    // --- gf_vect_mad arch variants ---
    pub fn gf_vect_mad_sse(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut c_uchar,
    );
    pub fn gf_vect_mad_avx(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut c_uchar,
    );
    pub fn gf_vect_mad_avx2(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut c_uchar,
    );

    // --- gf_2vect_mad arch variants ---
    pub fn gf_2vect_mad_sse(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );
    pub fn gf_2vect_mad_avx(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );
    pub fn gf_2vect_mad_avx2(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    // --- gf_3vect_mad arch variants ---
    pub fn gf_3vect_mad_sse(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );
    pub fn gf_3vect_mad_avx(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );
    pub fn gf_3vect_mad_avx2(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    // --- gf_4vect_mad arch variants ---
    pub fn gf_4vect_mad_sse(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );
    pub fn gf_4vect_mad_avx(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );
    pub fn gf_4vect_mad_avx2(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    // --- gf_5vect_mad arch variants ---
    pub fn gf_5vect_mad_sse(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );
    pub fn gf_5vect_mad_avx(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );
    pub fn gf_5vect_mad_avx2(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    // --- gf_6vect_mad arch variants ---
    pub fn gf_6vect_mad_sse(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );
    pub fn gf_6vect_mad_avx(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );
    pub fn gf_6vect_mad_avx2(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    // --- AVX512 variants (ec_encode_data, gf_*vect_dot_prod, gf_*vect_mad) ---
    pub fn ec_encode_data_avx512(
        len: c_int,
        k: c_int,
        rows: c_int,
        gftbls: *mut c_uchar,
        data: *mut *mut c_uchar,
        coding: *mut *mut c_uchar,
    );
    pub fn ec_encode_data_update_avx512(
        len: c_int,
        k: c_int,
        rows: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        data: *mut c_uchar,
        coding: *mut *mut c_uchar,
    );

    // gf_vect_dot_prod_avx512
    pub fn gf_vect_dot_prod_avx512(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut c_uchar,
    );

    // gf_2vect_dot_prod_avx512
    pub fn gf_2vect_dot_prod_avx512(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    // gf_3vect_dot_prod_avx512
    pub fn gf_3vect_dot_prod_avx512(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    // gf_4vect_dot_prod_avx512
    pub fn gf_4vect_dot_prod_avx512(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    // gf_5vect_dot_prod_avx512
    pub fn gf_5vect_dot_prod_avx512(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    // gf_6vect_dot_prod_avx512
    pub fn gf_6vect_dot_prod_avx512(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    // gf_vect_mad_avx512
    pub fn gf_vect_mad_avx512(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut c_uchar,
    );

    // gf_2vect_mad_avx512
    pub fn gf_2vect_mad_avx512(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    // gf_3vect_mad_avx512
    pub fn gf_3vect_mad_avx512(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    // gf_4vect_mad_avx512
    pub fn gf_4vect_mad_avx512(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    // gf_5vect_mad_avx512
    pub fn gf_5vect_mad_avx512(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    // gf_6vect_mad_avx512
    pub fn gf_6vect_mad_avx512(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    // --- GFNI variants (AVX2 with GFNI instruction set) ---
    pub fn gf_vect_dot_prod_avx2_gfni(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut c_uchar,
    );

    pub fn gf_2vect_dot_prod_avx2_gfni(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    pub fn gf_3vect_dot_prod_avx2_gfni(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    pub fn gf_4vect_dot_prod_avx2_gfni(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    pub fn gf_5vect_dot_prod_avx2_gfni(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    pub fn gf_6vect_dot_prod_avx2_gfni(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    pub fn gf_vect_mad_avx2_gfni(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut c_uchar,
    );

    pub fn gf_2vect_mad_avx2_gfni(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    pub fn gf_3vect_mad_avx2_gfni(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    pub fn gf_4vect_mad_avx2_gfni(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    pub fn gf_5vect_mad_avx2_gfni(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    pub fn gf_6vect_mad_avx2_gfni(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    // --- AVX512 + GFNI variants ---
    pub fn gf_vect_dot_prod_avx512_gfni(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut c_uchar,
    );

    pub fn gf_2vect_dot_prod_avx512_gfni(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    pub fn gf_3vect_dot_prod_avx512_gfni(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    pub fn gf_4vect_dot_prod_avx512_gfni(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    pub fn gf_5vect_dot_prod_avx512_gfni(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    pub fn gf_6vect_dot_prod_avx512_gfni(
        len: c_int,
        vlen: c_int,
        gftbls: *mut c_uchar,
        src: *mut *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    pub fn gf_vect_mad_avx512_gfni(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut c_uchar,
    );

    pub fn gf_2vect_mad_avx512_gfni(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    pub fn gf_3vect_mad_avx512_gfni(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    pub fn gf_4vect_mad_avx512_gfni(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    pub fn gf_5vect_mad_avx512_gfni(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    pub fn gf_6vect_mad_avx512_gfni(
        len: c_int,
        vec: c_int,
        vec_i: c_int,
        gftbls: *mut c_uchar,
        src: *mut c_uchar,
        dest: *mut *mut c_uchar,
    );

    // --- RAID arch variants ---
    pub fn xor_gen_sse(vects: c_int, len: c_int, array: *mut *mut c_void) -> c_int;
    pub fn xor_gen_avx(vects: c_int, len: c_int, array: *mut *mut c_void) -> c_int;
    pub fn xor_check_sse(vects: c_int, len: c_int, array: *mut *mut c_void) -> c_int;
    pub fn pq_gen_sse(vects: c_int, len: c_int, array: *mut *mut c_void) -> c_int;
    pub fn pq_gen_avx(vects: c_int, len: c_int, array: *mut *mut c_void) -> c_int;
    pub fn pq_gen_avx2(vects: c_int, len: c_int, array: *mut *mut c_void) -> c_int;
    pub fn pq_check_sse(vects: c_int, len: c_int, array: *mut *mut c_void) -> c_int;
}

// ===========================================================================
// Helper: sizeof queries via a small C compilation
// ===========================================================================

// These are provided so users can heap-allocate the opaque structs correctly.
extern "C" {
    // These symbols are defined in our helper C file compiled by build.rs.
    // (We'll add the helper C file next.)
}

// ===========================================================================
// Tests
// ===========================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn test_version() {
        unsafe {
            let ver = isal_get_version();
            assert!(ver > 0, "version should be nonzero");

            let ver_str = isal_get_version_str();
            assert!(!ver_str.is_null());
            let s = CStr::from_ptr(ver_str).to_str().unwrap();
            assert!(s.contains('.'), "version string should contain a dot: {s}");
        }
    }

    #[test]
    fn test_crc32_gzip_refl() {
        let data = b"Hello, ISA-L from Rust!";
        unsafe {
            let crc = crc32_gzip_refl(0, data.as_ptr(), data.len() as u64);
            // Just verify it returns something non-trivial
            assert_ne!(crc, 0);

            // Verify determinism
            let crc2 = crc32_gzip_refl(0, data.as_ptr(), data.len() as u64);
            assert_eq!(crc, crc2);
        }
    }

    #[test]
    fn test_crc64_ecma_refl() {
        let data = b"Test CRC64";
        unsafe {
            let crc = crc64_ecma_refl(0, data.as_ptr(), data.len() as u64);
            assert_ne!(crc, 0);
        }
    }

    #[test]
    fn test_gf_mul_inv() {
        unsafe {
            // gf_mul(a, gf_inv(a)) should equal 1 for any nonzero a
            for a in 1u8..=255 {
                let inv = gf_inv(a);
                let product = gf_mul(a, inv);
                assert_eq!(
                    product, 1,
                    "gf_mul({a}, gf_inv({a})) = {product}, expected 1"
                );
            }
        }
    }

    #[test]
    fn test_isal_zero_detect() {
        let zeros = vec![0u8; 256];
        let nonzeros = vec![1u8; 256];
        unsafe {
            assert_eq!(
                isal_zero_detect(zeros.as_ptr() as *mut _, zeros.len()),
                0,
                "all-zeros should return 0"
            );
            assert_ne!(
                isal_zero_detect(nonzeros.as_ptr() as *mut _, nonzeros.len()),
                0,
                "non-zeros should return non-zero"
            );
        }
    }

    #[test]
    fn test_adler32() {
        let data = b"Hello";
        unsafe {
            let a = isal_adler32(1, data.as_ptr(), data.len() as u64);
            assert_ne!(a, 1, "adler32 should change from init");
        }
    }
}
