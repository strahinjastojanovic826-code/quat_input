pub mod error;
pub mod quat;
pub mod view;
pub mod input;
pub mod ffi;

pub use error::QuatError;
pub use quat::{Quat, QuatByte};
pub use view::QuatStream;
pub use input::QuatInput;

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    // ==========================================
    // 1. LAKI TESTOVI (Osnovna funkcionalnost)
    // ==========================================

    #[test]
    fn test_01_single_quat_parse() {
        // Najlakši test: parsiranje jednog karaktera u Quat
        let q = Quat::from_char_const('2').unwrap();
        assert_eq!(q, Quat::V2);
        assert_eq!(q.to_u8(), 2);
    }

    #[test]
    fn test_02_quat_input_from_str() {
        // Osnovno parsiranje stringa u QuatInput kolekciju
        let input = QuatInput::from_str("0123").unwrap();
        assert_eq!(input.quats().len(), 4);
        assert_eq!(input.quats()[0], Quat::V0);
        assert_eq!(input.quats()[3], Quat::V3);
    }

    #[test]
    fn test_03_quat_byte_packing() {
        // Test pakovanja 4 kvata u jedan QuatByte (2 bita po kvatu)
        // q3='3' (11), q2='2' (10), q1='1' (01), q0='0' (00) -> 0b11100100 = 0xE4
        let input = QuatInput::from_str("0123").unwrap();
        let bytes = input.pack();
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes[0].0, 0b11100100);
    }

    #[test]
    fn test_04_quat_stream_get() {
        // Test kreiranja stream-a i čitanja po indeksu bez alokacije
        let bytes = vec![QuatByte::new(Quat::V3, Quat::V2, Quat::V1, Quat::V0).0];
        let stream = QuatStream::new(&bytes, 4);
        
        assert_eq!(stream.len(), 4);
        assert_eq!(stream.get(0).unwrap(), Quat::V0);
        assert_eq!(stream.get(1).unwrap(), Quat::V1);
        assert_eq!(stream.get(2).unwrap(), Quat::V2);
        assert_eq!(stream.get(3).unwrap(), Quat::V3);
    }

    #[test]
    fn test_05_quat_stream_iterator() {
        // Test nulto-alokacionog iteratora kroz stream
        let bytes = vec![QuatByte::new(Quat::V0, Quat::V1, Quat::V2, Quat::V3).0];
        let stream = QuatStream::new(&bytes, 4);
        let collected: Result<Vec<Quat>, QuatError> = stream.iter().collect();
        
        let quats = collected.unwrap();
        assert_eq!(quats, vec![Quat::V3, Quat::V2, Quat::V1, Quat::V0]);
    }

    // ==========================================
    // 2. SREDNJI TESTOVI (Rukovanje greškama i const)
    // ==========================================

    #[test]
    fn test_06_error_invalid_char() {
        // Provera da li ispravno vraća grešku za nedozvoljen karakter
        let result = QuatInput::from_str("01X3");
        assert!(matches!(result, Err(QuatError::InvalidChar('X'))));
    }

    #[test]
    fn test_07_error_index_out_of_bounds() {
        // Provera bezbednosti granica (index out of bounds)
        let bytes = vec![0xE4];
        let stream = QuatStream::new(&bytes, 4);
        let res = stream.get(10);
        assert!(matches!(res, Err(QuatError::IndexOutOfBounds(10))));
    }

    #[test]
    fn test_08_const_functions() {
        // Test izvršavanja u compile-time okruženju (const kontekst)
        const Q: Quat = match Quat::from_u8_const(3) {
            Ok(q) => q,
            Err(_) => Quat::V0,
        };
        assert_eq!(Q, Quat::V3);
    }

    // ==========================================
    // 3. STRES I NAPREDNI TESTOVI (Edge cases & FFI)
    // ==========================================

    #[test]
    fn test_09_stress_empty_and_boundaries() {
        // Stres test za ekstremne i prazne ulaze (Edge cases)
        let input = QuatInput::from_str("").unwrap();
        assert!(input.quats().is_empty());
        let bytes = input.pack();
        assert!(bytes.is_empty());

        let raw_bytes: Vec<u8> = bytes.iter().map(|q| q.0).collect();
        let stream = QuatStream::new(&raw_bytes, 0);

        assert!(stream.is_empty());
        assert!(stream.get(0).is_err());
    }

    #[test]
    fn test_10_stress_large_stream() {
        // Stres test sa masivnim ulazom (16.000 kvatova / 4.000 bajtova)
        let mut s = String::new();
        let pattern = "0123321001233210";
        for _ in 0..1000 {
            s.push_str(pattern);
        }
        let input = QuatInput::from_str(&s).unwrap();
        assert_eq!(input.quats().len(), 16000);

        let bytes = input.pack();
        assert_eq!(bytes.len(), 4000);

        let raw_bytes: Vec<u8> = bytes.iter().map(|q| q.0).collect();
        let stream = QuatStream::new(&raw_bytes, 16000);

        // Provera integriteta na nasumičnim tačkama velikog niza
        assert_eq!(stream.get(0).unwrap(), Quat::V0);
        assert_eq!(stream.get(1).unwrap(), Quat::V1);
        assert_eq!(stream.get(7999).unwrap(), Quat::V0);
        assert_eq!(stream.get(15999).unwrap(), Quat::V0);
    }

    #[test]
    fn test_11_stress_ffi_safety_and_panics() {
        // Najstresniji test: Simulacija spoljnog FFI okruženja (C/Python interakcija)
        // Testira kreiranje, rad sa pokazivačima, hvatanje grešaka i bezbedno uništenje.
        use crate::ffi::*;

        let raw_bytes: [u8; 2] = [0xE4, 0x1B];
        let mut stream_ptr: *mut FfiQuatStream = std::ptr::null_mut();

        unsafe {
            // 1. Kreiranje FFI streama
            let create_res = quat_stream_create(
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
            let get_res = quat_stream_get(stream_ptr, 0, &mut out_val, &mut out_err);
            assert_eq!(get_res, 0);
            assert_eq!(out_err, 0);
            assert_eq!(out_val, 0); // V0

            // 3. Uspešno čitanje poslednjeg validnog elementa
            let get_res_7 = quat_stream_get(stream_ptr, 7, &mut out_val, &mut out_err);
            assert_eq!(get_res_7, 0);
            assert_eq!(out_err, 0);
            assert_eq!(out_val, 0); 

            // 4. Pokušaj čitanja van granica (treba da vrati FFI kod greške bez rušenja)
            quat_stream_get(stream_ptr, 100, &mut out_val, &mut out_err);
            assert_eq!(out_err, FfiError::IndexOutOfBounds as i32);

            // 5. Bezbedno oslobađanje memorije
            quat_stream_destroy(stream_ptr);
        }
    }
}