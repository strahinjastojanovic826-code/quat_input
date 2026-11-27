use crate::quat::QuatByte;
use crate::QuatStream;

/// Standard simulated NAND Flash page size in bytes
pub const FLASH_PAGE_SIZE: usize = 256;
pub const FLASH_PAGES_PER_BLOCK: usize = 64;

#[derive(Debug, Clone)]
pub struct QuatFlashPage {
    pub page_id: u32,
    pub data: Vec<QuatByte>, // Packed quaternary data
    pub valid_quats: usize,
    pub checksum: u32,
}

impl QuatFlashPage {
    /// Creates a new flash page from raw packed bytes, ensuring it fits the page constraint.
    pub fn new(page_id: u32, bytes: &[QuatByte]) -> Result<Self, &'static str> {
        if bytes.len() > FLASH_PAGE_SIZE {
            return Err("Data exceeds NAND flash page size limit");
        }
        let mut page_data = bytes.to_vec();
        page_data.resize(FLASH_PAGE_SIZE, QuatByte(0)); // Padding with zero quats (00)
        let valid_quats = bytes.len() * 4;
        let checksum = Self::compute_checksum(&page_data);

        Ok(Self {
            page_id,
            data: page_data,
            valid_quats,
            checksum,
        })
    }

    /// Creates a flash page tracking the exact number of valid quaternary symbols (avoiding V0 padding ambiguity).
    pub fn new_with_quat_len(
        page_id: u32,
        bytes: &[QuatByte],
        valid_quats: usize,
    ) -> Result<Self, &'static str> {
        if bytes.len() > FLASH_PAGE_SIZE {
            return Err("Data exceeds NAND flash page size limit");
        }
        if valid_quats > bytes.len() * 4 {
            return Err("valid_quats exceeds provided packed byte capacity");
        }

        let mut page_data = bytes.to_vec();
        page_data.resize(FLASH_PAGE_SIZE, QuatByte(0));
        let checksum = Self::compute_checksum(&page_data);

        Ok(Self {
            page_id,
            data: page_data,
            valid_quats,
            checksum,
        })
    }

    /// Computes a deterministic 32-bit FNV-1a checksum over the page payload.
    pub fn compute_checksum(data: &[QuatByte]) -> u32 {
        let mut hash: u32 = 0x811c_9dc5;
        for b in data {
            hash ^= b.0 as u32;
            hash = hash.wrapping_mul(0x0100_0193);
        }
        hash
    }

    /// Verifies page integrity against stored checksum.
    #[inline]
    pub fn verify_integrity(&self) -> bool {
        Self::compute_checksum(&self.data) == self.checksum
    }

    /// Returns a zero-copy `QuatStream` over the valid quaternary symbols in this page.
    pub fn as_stream<'a>(&'a self, raw_buf: &'a [u8]) -> Option<QuatStream<'a>> {
        if raw_buf.len() < self.data.len() {
            return None;
        }
        Some(QuatStream::new(raw_buf, self.valid_quats))
    }

}

/// Represents a NAND Flash erase/management block composed of multiple `QuatFlashPage` entries.
#[derive(Debug, Clone)]
pub struct QuatFlashBlock {
    pub block_id: u32,
    pub pages: Vec<QuatFlashPage>,
}

impl QuatFlashBlock {
    pub fn new(block_id: u32) -> Self {
        Self {
            block_id,
            pages: Vec::with_capacity(FLASH_PAGES_PER_BLOCK),
        }
    }

    pub fn push_page(&mut self, page: QuatFlashPage) -> Result<(), &'static str> {
        if self.pages.len() >= FLASH_PAGES_PER_BLOCK {
            return Err("NAND flash block is full");
        }
        self.pages.push(page);
        Ok(())
    }

    pub fn verify_all(&self) -> bool {
        self.pages.iter().all(|p| p.verify_integrity())
    }
}