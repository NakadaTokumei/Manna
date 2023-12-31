use crate::get_peri_mem;
use crate::internal::read_write::RW;

pub struct SysTick
{
    pub CTRL: RW<u32>,
    pub LOAD: RW<u32>,
    pub VAL:  RW<u32>,
    pub CALIB: RW<u32>
}

pub fn get_systick() -> &'static mut SysTick
{
    get_peri_mem!(0xe000e010, SysTick)
}