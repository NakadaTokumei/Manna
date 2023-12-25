use crate::get_peri_mem;
use crate::interal::read_write::RW;

pub struct CRC
{
    pub DR: RW<u32>,
    pub IDR: RW<u32>,
    pub CR: RW<u32>
}

pub fn get_crc() -> &'static mut CRC
{
    get_peri_mem!(0x40023000, CRC)
}