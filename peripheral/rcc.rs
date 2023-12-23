use crate::get_peri_mem;
use crate::internal::read_write::RW;

pub struct RCC
{
    pub CR: RW<u32>,        // 0x00
    pub CFGR: RW<u32>,      // 0x04
    pub CIR: RW<u32>,       // 0x08
    pub APB2RSTR: RW<u32>,  // 0x0c
    pub APB1RSTR: RW<u32>,  // 0x10
    pub AHBENR: RW<u32>,    // 0x14
    pub APB2ENR: RW<u32>,   // 0x18
    pub APB1ENR: RW<u32>,   // 0x1c
    pub BDCR: RW<u32>,      // 0x20
    pub CSR: RW<u32>        // 0x24
}

pub fn get_rcc() -> &'static mut RCC
{
    get_peri_mem!(0x40021000, RCC)
}