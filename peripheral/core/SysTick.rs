use crate::internal::RW;

pub struct SysTick
{
    CTRL: RW<u32>,
    LOAD: RW<u32>,
    VAL:  RW<u32>,
    CALIB: RW<u32>
}

pub fn get_systick()
{
    get_peri_mem!(0xe000e010, SysTick)
}