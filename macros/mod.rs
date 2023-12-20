
#[macro_export]
macro_rules! get_peri_mem
{
    ($addr:expr, $typ:ty) => 
    {
        unsafe { &mut *($addr as *mut $typ) }
    }
}