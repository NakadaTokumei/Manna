use crate::get_peri_mem;
use crate::internal::read_write::RW;

pub struct NVIC
{
    pub ISER: [RW<u32>; 3],     // ISER0 ~ ISER2        (0xe000e100 ~ 0xe000e10b)
    _reserved1: [u8; 0x74],     // Reserved Section     (0xe000e10c ~ 0xe000e17f)
    pub ICER: [RW<u32>; 3],     // ICER0 ~ ICER2        (0xe000e180 ~ 0xe000e18b)
    _reserved2: [u8; 0x74],     // Reserved Section     (0xe000e18c ~ 0xe000e1ff)
    pub ISPR: [RW<u32>; 3],     // ISPR0 ~ ISPR2        (0xe000e200 ~ 0xe000e20b)
    _reserved3: [u8; 0x74],     // Reserved Section     (0xe000e20c ~ 0xe000e27f)
    pub ICPR: [RW<u32>; 3],     // ICPR0 ~ ICPR2        (0xe000e280 ~ 0xe000e28b) 
    _reserved4: [u8; 0x74],     // Reserved Section     (0xe000e28c ~ 0xe000e2ff)
    pub IABR: [RW<u32>; 3],     // IABR0 ~ IABR2        (0xe000e300 ~ 0xe000e30b)
    _reserved5: [u8; 0xf4],     // Reserved Section     (0xe000e30c ~ 0xe000e3ff)
    pub IPR:  [RW<u32>; 20]     // IPR0 ~ IPR20         (0xe000e400 ~ 0xe000e42c)   
}

pub fn get_nvic() -> &'static mut NVIC
{
    get_peri_mem!(0xe000e100, NVIC)
}

pub fn get_nvic_stir() -> &'static mut u32
{
    get_peri_mem!(0xe000ef00, u32)
}