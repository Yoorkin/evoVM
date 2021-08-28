mod bytecode;
mod executionEngine;
mod byteMemory;
mod convertRules;
macro_rules! make{
    ($($i:expr)*) =>{
        {println!("{}",0$(+$i)*);}
    }
}


fn main() {
    println!("Hello, world!");
    let mut num = 5000;
    make!(1 2 3 4 5);


}
