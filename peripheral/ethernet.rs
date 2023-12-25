use crate::get_peri_mem;
use crate::internal::read_write::RW;

pub struct ETH_CA
{
    pub HR: RW<u32>,
    pub LR: RW<u32>
}

pub struct Ethernet
{
    pub MACCR: RW<u32>,         // 0x00
    pub MACFFR: RW<u32>,        // 0x04
    pub MACHTHR: RW<u32>,       // 0x08
    pub MACHTLR: RW<u32>,       // 0x0c
    pub MACMIIAR: RW<u32>,      // 0x10
    pub MACMIIDR: RW<u32>,      // 0x14
    pub MACFCR: RW<u32>,        // 0x18
    pub MACVLANTR: RW<u32>,     // 0x1c
    _reserved1: [u8; 8],        // 0x20 ~ 0x27
    pub MACRWUFFR: RW<u32>,     // 0x28
    pub MACPMTCSR: RW<u32>,     // 0x2c
    _reserved2: [u8; 8],        // 0x30 ~ 0x37
    pub MACSR: RW<u32>,         // 0x38
    pub MACIMR: RW<u32>,        // 0x3c
    pub MACA: [ETH_CA; 4],      // 0x40 ~ 0x5f
    _reserved3: [u8; 0xa0],     // 0x60 ~ 0xff
    pub MMCCR: RW<u32>,         // 0x100
    pub MMCRIR: RW<u32>,        // 0x104
    pub MMCTIR: RW<u32>,        // 0x108
    pub MMCRIMR: RW<u32>,       // 0x10c
    pub MMCTIMR: RW<u32>,       // 0x110
    _reserved4: [u8; 0x38],     // 0x114 ~ 0x14b
    pub MMCTGFSCCR: RW<u32>,    // 0x14c
    pub MMCTGFMSCCR: RW<u32>,   // 0x150
    _reserved5: [u8; 0x14],     // 0x154 ~ 0x167
    pub MMCTGFCR: RW<u32>,      // 0x168
    _reserved6: [u8; 0x28],     // 0x16c ~ 0x193
    pub MMCRFCECR: RW<u32>,     // 0x194
    pub MMCRFAECR: RW<u32>,     // 0x198
    _reserved7: [u8; 0x28],     // 0x19c ~ 0x1c3
    pub MMCRGUFCR: RW<u32>,     // 0x1c4
    _reserved8: [u8; 0x538],    // 0x1c8 ~ 0x6ff
    pub PTPTSCR: RW<u32>,       // 0x700
    pub PTPSSIR: RW<u32>,       // 0x704
    pub PTPTSHR: RW<u32>,       // 0x708
    pub PTPTSLR: RW<u32>,       // 0x70c
    pub PTPTSHUR: RW<u32>,      // 0x710
    pub PTPTSLUR: RW<u32>,      // 0x714
    pub PTPTSAR: RW<u32>,       // 0x718
    pub PTPTTHR: RW<u32>,       // 0x71c
    pub PTPTTLR: RW<u32>,       // 0x720
    _reserved9: [u8; 0x8dc],    // 0x724 ~ 0xfff
    pub DMABMR: RW<u32>,        // 0x1000
    pub DMATPDR: RW<u32>,       // 0x1004
    pub DMARPDR: RW<u32>,       // 0x1008
    pub DMARDLAR: RW<u32>,      // 0x100c
    pub DMATDLAR: RW<u32>,      // 0x1010
    pub DMASR: RW<u32>,         // 0x1014
    pub DMAOMR: RW<u32>,        // 0x1018
    pub DMAIER: RW<u32>,        // 0x101c
    pub DMAMFBOCR: RW<u32>,     // 0x1020
    _reserved10: [u32; 0x24],   // 0x1024 ~ 0x1047
    pub DMACHTDR: RW<u32>,      // 0x1048
    pub DMACHRDR: RW<u32>,      // 0x104c
    pub DMACHTBAR: RW<u32>,     // 0x1050
    pub DMACHRBAR: RW<u32>      // 0x1054
}

pub fn get_ethernet() -> &'static mut Ethernet
{
    get_peri_mem!(0x40028000, Ethernet)
}