pub const FAIRBANKS_VENDOR_ID: u16 = 0x0B67;
pub const FAIRBANKS_SCB_900_PRODUCT_ID: u16 = 0x555E;

pub const FAIRBANKS_SCB_900_SCALE: VendorInfo = VendorInfo {
    vendor_id: FAIRBANKS_VENDOR_ID,
    product_id: FAIRBANKS_SCB_900_PRODUCT_ID,
};

#[derive(Debug, Copy, Clone)]
pub struct VendorInfo {
    pub(crate) vendor_id: u16,
    pub(crate) product_id: u16,
}

impl VendorInfo {
    pub fn new(vendor_id: u16, product_id: u16) -> Self {
        Self { vendor_id, product_id }
    }
}