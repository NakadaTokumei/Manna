use crate::get_peri_mem;
use crate::internal::read_write::RW;

pub struct RBAR_RASR
{
    pub RBAR: RW<u32>,
    pub RASR: RW<u32>
}

pub struct MPU
{
    pub TYPER: RW<u32>,         // 0x00
    pub CR: RW<u32>,            // 0x04
    pub RNR: RW<u32>,           // 0x08
    pub RB_RA: [RBAR_RASR; 3]   // 0x0c ~ 0x23
}

pub fn get_mcu() -> &'static mut MPU
{
    get_peri_mem!(0xe000ed90, MPU)
}