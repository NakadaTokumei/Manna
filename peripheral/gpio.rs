use crate::get_peri_mem;
use crate::internal::read_write::RW;

pub enum GPIO_Sect
{
    GPIOA,
    GPIOB,
    GPIOC,
    GPIOD,
    GPIOE,
    GPIOF,
    GPIOG
}

pub struct GPIO
{
    pub CRL: RW<u32>,   // 0x00
    pub CRH: RW<u32>,   // 0x04
    pub IDR: RW<u32>,   // 0x08
    pub ODR: RW<u32>,   // 0x0c
    pub BSRR: RW<u32>,  // 0x10
    pub BRR: RW<u32>,   // 0x14
    pub LCKR: RW<u32>  // 0x18
}

pub struct AFIO
{
    pub EVCR: RW<u32>,          // 0x00
    pub MAPR: RW<u32>,          // 0x04
    pub EXTICR: [RW<u32>; 4],   // 0x08 ~ 0x17
    _reserved1: u32,            // 0x18
    pub MAPR2: RW<u32>,         // 0x1c
}

pub fn get_gpio(gpio_sect: GPIO_Sect) -> &'static mut GPIO
{
    match gpio_sect
    {
        GPIO_Sect::GPIOA => get_peri_mem!(0x40010800, GPIO),
        GPIO_Sect::GPIOB => get_peri_mem!(0x40010c00, GPIO),
        GPIO_Sect::GPIOC => get_peri_mem!(0x40011000, GPIO),
        GPIO_Sect::GPIOD => get_peri_mem!(0x40011400, GPIO),
        GPIO_Sect::GPIOE => get_peri_mem!(0x40011800, GPIO),
        GPIO_Sect::GPIOF => get_peri_mem!(0x40011c00, GPIO),
        GPIO_Sect::GPIOG => get_peri_mem!(0x40012000, GPIO)
    }
}

pub fn get_afio() -> &'static mut AFIO
{
    get_peri_mem!(0x40010000, AFIO)
}