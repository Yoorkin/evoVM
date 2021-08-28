use std::ops::Index;
use std::mem::size_of;
pub struct Memory<const Size:usize>{
    data:[u8;Size]
}

impl<const Size:usize> Memory<Size>{
    pub fn new()->Memory<Size>{
        Memory{data:[0;Size]}
    }

    pub unsafe fn at<T>(&mut self, offset:usize) ->*mut T{
        let ptr = self.data.as_mut_ptr();
        return ptr.offset(offset as isize) as *mut T;
    }
}

pub struct Stack<const Size:usize>{
    memory:Memory<Size>,
    top:usize
}

impl<const Size:usize> Stack<Size>{
    pub fn new()->Stack<Size>{
        Stack{ memory: Memory::new(), top:0}
    }

    pub fn push<T:Clone>(&mut self,value:&T) {
        unsafe{
            if self.top + size_of::<T>() > Size {
                panic!("out of bounds")
            };
            let ptr = self.memory.at::<T>(self.top);
            *ptr = value.clone();
            self.top += size_of::<T>();
        }
    }

    pub fn pop<T:Clone>(&mut self)->T {
        return unsafe {
            self.top -= size_of::<T>();
            if self.top < 0 {
                panic!("out of bounds")
            };
            let ptr = self.memory.at::<T>(self.top);
            return (*ptr).clone();
        }
    }
}