use crate::quat::QuatByte;

/// Standard simulated NAND Flash page size in bytes
pub const FLASH_PAGE_SIZE: usize = 256;

#[derive(Debug, Clone)]
pub struct QuatFlashPage {
    pub page_id: u32,
    pub data: Vec<QuatByte>, // Packed quaternary data
}

impl QuatFlashPage {
    /// Creates a new flash page from raw packed bytes, ensuring it fits the page constraint.
    pub fn new(page_id: u32, bytes: &[QuatByte]) -> Result<Self, &'static str> {
        if bytes.len() > FLASH_PAGE_SIZE {
            return Err("Data exceeds NAND flash page size limit");
        }
        let mut page_data = bytes.to_vec();
        page_data.resize(FLASH_PAGE_SIZE, QuatByte(0)); // Padding with zero quats (00)

        Ok(Self {
            page_id,
            data: page_data,
        })
    }
}