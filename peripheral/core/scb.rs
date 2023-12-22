use crate::get_peri_mem;
use crate::internal::read_write::RW;

pub struct SCB
{
    pub CPUID: RW<u32>,     // 0x00
    pub ICSR: RW<u32>,      // 0x04
    pub VTOR: RW<u32>,      // 0x08
    pub AIRCR: RW<u32>,     // 0x0c
    pub SCR: RW<u32>,       // 0x10
    pub CCR: RW<u32>,       // 0x14
    pub SHPR: [RW<u32>; 3], // 0x18 ~ 0x23
    pub SHCRS: RW<u32>,     // 0x24
    pub CFSR: RW<u32>,      // 0x28
    pub HFSR: RW<u32>,      // 0x2c
    _reserved1: u32,    // 0x30
    pub MMAR: RW<u32>,      // 0x34
    pub BFAR: RW<u32>       // 0x38
}

pub fn get_scb() -> &'static mut SCB
{
    get_peri_mem!(0xe000ed00, SCB)
}