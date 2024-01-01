use crate::get_peri_mem;
use crate::internal::read_write::RW;

pub enum USART_Sect
{
    USART1,
    USART2,
    USART3,
    UART4,
    UART5
}

pub struct USART
{
    pub SR: RW<u32>,    // 0x00
    pub DR: RW<u32>,    // 0x04
    pub BRR: RW<u32>,   // 0x08
    pub CR1: RW<u32>,   // 0x0c
    pub CR2: RW<u32>,   // 0x10
    pub CR3: RW<u32>,   // 0x14
    pub GTPR: RW<u32>,  // 0x18
}

pub fn get_usart(num: USART_Sect) -> &'static mut USART
{
    match num
    {
        USART_Sect::USART1 => get_peri_mem!(0x40013800, USART),
        USART_Sect::USART2 => get_peri_mem!(0x40004400, USART),
        USART_Sect::USART3 => get_peri_mem!(0x40004800, USART),
        USART_Sect::UART4 => get_peri_mem!(0x40004c00, USART),
        USART_Sect::UART5 => get_peri_mem!(0x40005000, USART)
    }
}
