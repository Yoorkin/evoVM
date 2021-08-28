use std::any::Any;

pub enum Primitive{
    I,U,L,UL,A,F,D,B,W,Memory
}

pub enum Value{
    I(i32),U(u32),L(i64),UL(u64),A(i32),F(f32),D(f64),B(i8),W(i16),Memory(i32)
}

pub enum Bytecode{
    Cvt{src:Primitive,dst:Primitive},
    Push{kind:Value},
    Pop{kind:Primitive},
    Store{kind:Primitive},
    Load{kind:Primitive},
    Stm{size:i32,address:i32},
    Ldm{size:i32,address:i32},
    Add{kind:Primitive},
    Sub{kind:Primitive},
    Mul{kind:Primitive},
    Div{kind:Primitive},
    Neg{kind:Primitive},
    Cmp{kind:Primitive},
    And,
    Or,
    Xor,
    Not,
    Jmp,
    Jif,
    Ret
}