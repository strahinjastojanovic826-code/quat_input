pub mod error;
pub mod quat;
pub mod view;
pub mod input;
pub mod ffi;
pub mod dsp;
pub mod flash;

pub use error::QuatError;
pub use quat::{Quat, QuatByte};
pub use view::QuatStream;
pub use input::QuatInput;

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use std::time::Instant;

    // ==========================================
    // 1. LAKI TESTOVI (Osnovna funkcionalnost)
    // ==========================================

    #[test]
    fn test_01_single_quat_parse() {
        let start = std::time::Instant::now();
        // Najlakši test: parsiranje jednog karaktera u Quat
        let q: Quat = Quat::from_char_const('2').unwrap();
        assert_eq!(q, Quat::V2);
        assert_eq!(q.to_u8(), 2);
        let duration = start.elapsed();
        println!("[BENCHMARK] test_01_single_quat_parse: {:?}", duration);
    }

    #[test]
    fn test_02_quat_input_from_str() {
        let start = std::time::Instant::now();
        // Osnovno parsiranje stringa u QuatInput kolekciju
        let input: QuatInput = QuatInput::from_str("0123").unwrap();
        assert_eq!(input.quats().len(), 4);
        assert_eq!(input.quats()[0], Quat::V0);
        assert_eq!(input.quats()[3], Quat::V3);
        let duration = start.elapsed();
        println!("[BENCHMARK] test_02_quat_input_from_str: {:?}", duration);
    }

    #[test]
    fn test_03_quat_byte_packing() {
        let start = std::time::Instant::now();
        // Test pakovanja 4 kvarta u jedan QuatByte (2 bita po kvatu)
        // q3='3' (11), q2='2' (10), q1='1' (01), q0='0' (00) -> 0b11100100 = 0xE4
        let input: QuatInput = QuatInput::from_str("0123").unwrap();
        let bytes: Vec<QuatByte> = input.pack();
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes[0].0, 0b11100100);
        let duration = start.elapsed();
        println!("[BENCHMARK] test_03_quat_byte_packing: {:?}", duration);
    }

    #[test]
    fn test_04_quat_stream_get() {
        let start = std::time::Instant::now();
        // Test kreiranja stream-a i čitanja po indeksu bez alokacije
        let bytes: Vec<u8> = vec![QuatByte::new(Quat::V3, Quat::V2, Quat::V1, Quat::V0).0];
        let stream: QuatStream<'_> = QuatStream::new(&bytes, 4);

        assert_eq!(stream.len(), 4);
        assert_eq!(stream.get(0).unwrap(), Quat::V0);
        assert_eq!(stream.get(1).unwrap(), Quat::V1);
        assert_eq!(stream.get(2).unwrap(), Quat::V2);
        assert_eq!(stream.get(3).unwrap(), Quat::V3);
        let duration = start.elapsed();
        println!("[BENCHMARK] test_04_quat_stream_get: {:?}", duration);
    }

    #[test]
    fn test_05_quat_stream_iterator() {
        let start = std::time::Instant::now();
        // Test nulto-alokacionog iteratora kroz stream
        let bytes: Vec<u8> = vec![QuatByte::new(Quat::V0, Quat::V1, Quat::V2, Quat::V3).0];
        let stream: QuatStream<'_> = QuatStream::new(&bytes, 4);
        let collected: Result<Vec<Quat>, QuatError> = stream.iter().collect();

        let quats: Vec<Quat> = collected.unwrap();
        assert_eq!(quats, vec![Quat::V3, Quat::V2, Quat::V1, Quat::V0]);
        let duration = start.elapsed();
        println!("[BENCHMARK] test_05_quat_stream_iterator: {:?}", duration);
    }

    #[test]
    fn test_06_error_invalid_char() {
        let start = std::time::Instant::now();
        // Provera da li ispravno vraća grešku za nedozvoljen karakter
        let result: Result<QuatInput, QuatError> = QuatInput::from_str("01X3");
        assert!(matches!(result, Err(QuatError::InvalidChar('X'))));
        let duration = start.elapsed();
        println!("[BENCHMARK] test_06_error_invalid_char: {:?}", duration);
    }

    #[test]
    fn test_07_error_index_out_of_bounds() {
        let start = std::time::Instant::now();
        // Provera bezbednosti granica (index out of bounds)
        let bytes: Vec<u8> = vec![0xE4];
        let stream: QuatStream<'_> = QuatStream::new(&bytes, 4);
        let res: Result<Quat, QuatError> = stream.get(10);
        assert!(matches!(res, Err(QuatError::IndexOutOfBounds(10))));
        let duration = start.elapsed();
        println!("[BENCHMARK] test_07_error_index_out_of_bounds: {:?}", duration);
    }

    #[test]
    fn test_08_const_functions() {
        let start = std::time::Instant::now();
        // Test izvršavanja u compile-time okruženju (const kontekst)
        const Q: Quat = match Quat::from_u8_const(3) {
            Ok(q) => q,
            Err(_) => Quat::V0,
        };
        assert_eq!(Q, Quat::V3);
        let duration = start.elapsed();
        println!("[BENCHMARK] test_08_const_functions: {:?}", duration);
    }

    #[test]
    fn test_09_stress_empty_and_boundaries() {
        let start = std::time::Instant::now();
        // Stres test za ekstremne i prazne ulaze (Edge cases)
        let input: QuatInput = QuatInput::from_str("").unwrap();
        assert!(input.quats().is_empty());
        let bytes: Vec<QuatByte> = input.pack();
        assert!(bytes.is_empty());

        let raw_bytes: Vec<u8> = bytes.iter().map(|q: &QuatByte| q.0).collect();
        let stream: QuatStream<'_> = QuatStream::new(&raw_bytes, 0);
        assert!(stream.is_empty());
        assert!(stream.get(0).is_err());
        let duration = start.elapsed();
        println!("[BENCHMARK] test_09_stress_empty_and_boundaries: {:?}", duration);
    }

    #[test]
    fn test_10_stress_large_stream() {
        let start = std::time::Instant::now();
        // Stres test sa masivnim ulazom (16.000 kvatova / 4.000 bajtova)
        let mut s: String = String::new();
        let pattern: &str = "0123321001233210";
        for _ in 0..1000 {
            s.push_str(pattern);
        }
        let input: QuatInput = QuatInput::from_str(&s).unwrap();
        assert_eq!(input.quats().len(), 16000);

        let bytes: Vec<QuatByte> = input.pack();
        assert_eq!(bytes.len(), 4000);

        let raw_bytes: Vec<u8> = bytes.iter().map(|q: &QuatByte| q.0).collect();
        let stream: QuatStream<'_> = QuatStream::new(&raw_bytes, 16000);

        // Provera integriteta na nasumičnim tačkama velikog niza
        assert_eq!(stream.get(0).unwrap(), Quat::V0);
        assert_eq!(stream.get(1).unwrap(), Quat::V1);
        assert_eq!(stream.get(7999).unwrap(), Quat::V0);
        assert_eq!(stream.get(15999).unwrap(), Quat::V0);
        let duration = start.elapsed();
        println!("[BENCHMARK] test_10_stress_large_stream: {:?}", duration);
    }

    #[test]
    fn test_11_stress_ffi_safety_and_panics() {
        let start = std::time::Instant::now();
        // Najstresniji test: Simulacija spoljnog FFI okruženja (C/Python interakcija)
        // Testira kreiranje, rad sa pokazivačima, hvatanje grešaka i bezbedno uništenje.
        use crate::ffi::*;

        let raw_bytes: [u8; 2] = [0xE4, 0x1B];
        let mut stream_ptr: *mut FfiQuatStream = std::ptr::null_mut();

        unsafe {
            // 1. Kreiranje FFI streama
            let create_res: i32 = quat_stream_create(
                raw_bytes.as_ptr(),
                raw_bytes.len(),
                8,
                &mut stream_ptr,
            );
            assert_eq!(create_res, 0);
            assert!(!stream_ptr.is_null());

            let mut out_val: u8 = 0;
            let mut out_err: i32 = 0;

            // 2. Uspešno čitanje prvog elementa
            let get_res: i32 = quat_stream_get(stream_ptr, 0, &mut out_val, &mut out_err);
            assert_eq!(get_res, 0);
            assert_eq!(out_err, 0);
            assert_eq!(out_val, 0); // V0

            // 3. Uspešno čitanje poslednjeg validnog elementa
            let get_res_7: i32 = quat_stream_get(stream_ptr, 7, &mut out_val, &mut out_err);
            assert_eq!(get_res_7, 0);
            assert_eq!(out_err, 0);
            assert_eq!(out_val, 0);

            // 4. Pokušaj čitanja van granica (treba da vrati FFI kod greške bez rušenja)
            quat_stream_get(stream_ptr, 100, &mut out_val, &mut out_err);
            assert_eq!(out_err, FfiError::IndexOutOfBounds as i32);

            // 5. Bezbedno oslobađanje memorije
            quat_stream_destroy(stream_ptr);
        }
        let duration = start.elapsed();
        println!("[BENCHMARK] test_11_stress_ffi_safety_and_panics: {:?}", duration);
    }

    #[test]
    fn test_12_quat_arithmetic_and_logic() {
        let start = std::time::Instant::now();
        // Test quaternary arithmetic modulo 4
        let q1: Quat = Quat::V1; // 1
        let q2: Quat = Quat::V2; // 2
        let q3: Quat = Quat::V3; // 3

        // Addition: V1 + V2 = V3 (1 + 2 = 3)
        assert_eq!(q1.add(q2), Quat::V3);
        // Modulo wrap-around: V2 + V3 = V1 (2 + 3 = 5 -> 1 mod 4)
        assert_eq!(q2.add(q3), Quat::V1);

        // Subtraction: V3 - V2 = V1 (3 - 2 = 1)
        assert_eq!(q3.sub(q2), Quat::V1);

        // Post negation: NOT(V1) = V2 (3 - 1 = 2)
        assert_eq!(q1.not(), Quat::V2);
        assert_eq!(Quat::V0.not(), Quat::V3);

        // Max and Min operations (Multi-valued logic equivalent to OR/AND)
        assert_eq!(q1.max(q3), Quat::V3);
        assert_eq!(q1.min(q3), Quat::V1);
        let duration = start.elapsed();
        println!("[BENCHMARK] test_12_quat_arithmetic_and_logic: {:?}", duration);
    }

    #[test]
    fn test_13_pack_fast_equivalence() {
        let start = std::time::Instant::now();
        // Test if fast vectorized packing yields identical results to standard pack
        let input: QuatInput = QuatInput::from_str("012332100123").unwrap();

        let standard_bytes: Vec<QuatByte> = input.pack();
        let fast_bytes: Vec<QuatByte> = input.pack_fast();

        assert_eq!(standard_bytes, fast_bytes);
        assert_eq!(fast_bytes.len(), 3);
        let duration = start.elapsed();
        println!("[BENCHMARK] test_13_pack_fast_equivalence: {:?}", duration);
    }

#[test]
    fn test_14_simd_packing_benchmark_and_correctness() {
        // 1. Correctness check for SIMD-optimized packing
        let input_str: String = "0123321001233210".repeat(1000); // 16,000 quats
        let input = QuatInput::from_str(&input_str).unwrap();
        
        let standard_bytes = input.pack();
        let simd_bytes = input.pack_simd_optimized();
        
        assert_eq!(standard_bytes, simd_bytes);

        // 2. Inline benchmark (no Criterion, measures single operations)
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _ = input.pack_simd_optimized();
        }
        let duration = start.elapsed();
        println!("[BENCHMARK] pack_simd_optimized (1000 iterations): {:?}", duration);
    }

    #[test]
    fn test_15_dsp_modulation_benchmark_and_correctness() {
        // 1. Correctness check for DSP QPSK mapping
        let quats = vec![Quat::V0, Quat::V1, Quat::V2, Quat::V3];
        let symbols = crate::dsp::modulate_stream(&quats);
        
        assert_eq!(symbols.len(), 4);
        assert_eq!(symbols[0].phase_degrees, 0.0);
        assert_eq!(symbols[1].phase_degrees, 90.0);
        assert_eq!(symbols[2].phase_degrees, 180.0);
        assert_eq!(symbols[3].phase_degrees, 270.0);

        // 2. Inline benchmark
        let large_quats = vec![Quat::V2; 10000];
        let start = std::time::Instant::now();
        let _ = crate::dsp::modulate_stream(&large_quats);
        let duration = start.elapsed();
        println!("[BENCHMARK] modulate_stream (10,000 elements): {:?}", duration);
    }

    #[test]
    fn test_16_flash_page_benchmark_and_correctness() {
        // 1. Correctness check for NAND Flash page creation and padding
        let bytes = vec![QuatByte(0xE4); 10]; // 10 packed bytes
        let page_res = crate::flash::QuatFlashPage::new(1, &bytes);
        
        assert!(page_res.is_ok());
        let page = page_res.unwrap();
        assert_eq!(page.data.len(), crate::flash::FLASH_PAGE_SIZE); // Checks padding to 256 bytes

        // 2. Inline benchmark
        let start = std::time::Instant::now();
        for i in 0..1000 {
            let _ = crate::flash::QuatFlashPage::new(i, &bytes);
        }
        let duration = start.elapsed();
        println!("[BENCHMARK] QuatFlashPage creation (1000 iterations): {:?}", duration);
    }

    #[test]
    fn test_17_extended_unary_and_binary_quat_ops() {
        let start = std::time::Instant::now();

        assert_eq!(Quat::V0.inc_mod4(), Quat::V1);
        assert_eq!(Quat::V3.inc_mod4(), Quat::V0);
        assert_eq!(Quat::V0.dec_mod4(), Quat::V3);
        assert_eq!(Quat::V1.rot2(), Quat::V3);

        assert_eq!(
            Quat::V2.threshold(Quat::V2, Quat::V0, Quat::V3),
            Quat::V3
        );
        assert_eq!(
            Quat::V1.window_literal(Quat::V1, Quat::V2, Quat::V3, Quat::V0),
            Quat::V3
        );

        assert_eq!(Quat::V2.mul_mod4(Quat::V3), Quat::V2);
        assert_eq!(Quat::V3.bit_and(Quat::V1), Quat::V1);
        assert_eq!(Quat::V1.bit_or(Quat::V2), Quat::V3);
        assert_eq!(Quat::V3.bit_xor(Quat::V1), Quat::V2);
        assert_eq!(Quat::V0.implies(Quat::V1), Quat::V3);
        assert_eq!(Quat::V3.xnor_bits(Quat::V3), Quat::V3);

        for _ in 0..10_000 {
            let _ = std::hint::black_box(Quat::V2.mul_mod4(Quat::V3).inc_mod4());
        }

        let duration = start.elapsed();
        println!(
            "[BENCHMARK] test_17_extended_unary_and_binary_quat_ops: {:?}",
            duration
        );
    }

    #[test]
    fn test_18_zero_copy_streaming_and_pack_into() {
        let start = std::time::Instant::now();

        let quats = vec![Quat::V0, Quat::V1, Quat::V2, Quat::V3, Quat::V1];
        let mut packed_buf = vec![0u8; 2];
        let written = crate::input::QuatInput::pack_slice_into(&quats, &mut packed_buf).unwrap();
        assert_eq!(written, 2);

        let stream = crate::view::QuatStream::new(&packed_buf, quats.len());
        let mut out_quats = vec![Quat::V0; quats.len()];
        let unpacked = stream.unpack_into(&mut out_quats).unwrap();
        assert_eq!(unpacked, quats.len());
        assert_eq!(out_quats, quats);

        let packed_chunks: Vec<QuatByte> = stream.packed_bytes().collect();
        assert_eq!(packed_chunks.len(), 2);

        for _ in 0..1000 {
            let _ = std::hint::black_box(stream.unpack_into(&mut out_quats).unwrap());
        }

        let duration = start.elapsed();
        println!(
            "[BENCHMARK] test_18_zero_copy_streaming_and_pack_into: {:?}",
            duration
        );
    }

    #[test]
    fn test_19_flash_page_integrity_and_block() {
        let start = std::time::Instant::now();

        let bytes = vec![QuatByte(0xE4); 4];
        let mut page = crate::flash::QuatFlashPage::new_with_quat_len(1, &bytes, 14).unwrap();
        assert_eq!(page.valid_quats, 14);
        assert!(page.verify_integrity());

        let mut block = crate::flash::QuatFlashBlock::new(0);
        block.push_page(page.clone()).unwrap();
        assert!(block.verify_all());

        page.data[0] = QuatByte(0x00);
        assert!(!page.verify_integrity());

        for i in 0..1000 {
            let p = crate::flash::QuatFlashPage::new_with_quat_len(i, &bytes, 14).unwrap();
            let _ = std::hint::black_box(p.verify_integrity());
        }

        let duration = start.elapsed();
        println!(
            "[BENCHMARK] test_19_flash_page_integrity_and_block: {:?}",
            duration
        );
    }

    #[test]
    fn test_20_dsp_gray_iq_modulation_and_demodulation() {
        let start = std::time::Instant::now();

        let input = vec![Quat::V0, Quat::V1, Quat::V2, Quat::V3];
        for &q in &input {
            assert_eq!(Quat::from_gray(q.to_gray()), q);
        }

        let iq_samples = crate::dsp::modulate_iq_stream(&input);
        let demodulated = crate::dsp::demodulate_iq_stream(&iq_samples);
        assert_eq!(demodulated, input);

        let large = vec![Quat::V2; 10_000];
        for _ in 0..100 {
            let samples = crate::dsp::modulate_iq_stream(&large);
            let _ = std::hint::black_box(crate::dsp::demodulate_iq_stream(&samples));
        }

        let duration = start.elapsed();
        println!(
            "[BENCHMARK] test_20_dsp_gray_iq_modulation_and_demodulation: {:?}",
            duration
        );
    }

    #[test]
    fn test_21_ffi_pack_unpack_and_len() {
        use crate::ffi::*;

        let start = std::time::Instant::now();

        let raw_quats: [u8; 6] = [0, 1, 2, 3, 1, 2];
        let mut packed = [0u8; 4];
        let mut written: usize = 0;
        let mut unpacked: [u8; 6] = [0; 6];

        unsafe {
            let pack_status = quat_pack_buffer(
                raw_quats.as_ptr(),
                raw_quats.len(),
                packed.as_mut_ptr(),
                packed.len(),
                &mut written,
            );
            assert_eq!(pack_status, FfiError::Ok as i32);
            assert_eq!(written, 2);

            let unpack_status = quat_unpack_buffer(
                packed.as_ptr(),
                written,
                raw_quats.len(),
                unpacked.as_mut_ptr(),
                unpacked.len(),
            );
            assert_eq!(unpack_status, FfiError::Ok as i32);
            assert_eq!(unpacked, raw_quats);

            let mut stream_ptr: *mut FfiQuatStream = std::ptr::null_mut();
            let create_res = quat_stream_create(packed.as_ptr(), written, 6, &mut stream_ptr);
            assert_eq!(create_res, 0);
            assert_eq!(quat_stream_len(stream_ptr), 6);
            quat_stream_destroy(stream_ptr);
        }

        for _ in 0..1000 {
            unsafe {
                let _ = std::hint::black_box(quat_pack_buffer(
                    raw_quats.as_ptr(),
                    raw_quats.len(),
                    packed.as_mut_ptr(),
                    packed.len(),
                    &mut written,
                ));
            }
        }

        let duration = start.elapsed();
        println!(
            "[BENCHMARK] test_21_ffi_pack_unpack_and_len: {:?}",
            duration
        );
    }

}