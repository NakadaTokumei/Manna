pub struct RW<T>
{
    _val: T
}

impl<T> RW<T>
{
    pub fn write(&mut self, new_val : T) -> ()
    {
        self._val = new_val;
        ()
    }

    pub fn read(&mut self) -> T
    {
        self._val
    }
}