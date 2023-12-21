use core::cell::UnsafeCell;
use core::ptr;

pub struct RW<T>
{
    _val: UnsafeCell<T>
}

impl<T> RW<T>
{
    pub fn new(new_val: T) -> Self
    {
        RW { _val: UnsafeCell::new(new_val) }
    }

    pub fn write(&self, new_val : T)
        where T: Copy
    {
        unsafe { ptr::write_volatile(self._val.get(), new_val) }
    }

    pub fn read(&self) -> T
    {
        unsafe { ptr::read_volatile(self._val.get()) }
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *mut T
    {
        self._val.get()
    }
}