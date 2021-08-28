use std::vec;
use crate::bytecode::*;
use std::str;
use crate::byteMemory::*;
use std::borrow::Borrow;
use vm_macros::*;
use std::ops::{Neg, Add, Sub, Div, Mul};
use std::convert::TryFrom;

struct BinaryFile{
    codes:Vec<Bytecode>
}

struct Machine{
    operand:Stack<512>,
    frame:Stack<512>
}

macro_rules! make_match {
    ($val:ident,$obj:ident,$func:ident,$(($e:ident,$t:ty)),*) => {
        match($val){
            $(Primitive::$e => $obj.$func::<$t>()),*
            , _ => {}
        }
    };
    ($val:ident,$obj:ident,$func:ident) => {
        make_match!($val,$obj,$func,(I,i32),(U,u32),(L,i64),(UL,u64),(A,i32),(F,f32),(D,f64),(B,i8),(W,i16))
    }
}

macro_rules! make_cvt {
    ($obj:ident.$func:ident,($dst_enum:ident,$src_type:ty),[$(($dst_enum_list:ident,$dst_type_list:ty)),*]) => {
        match($dst_enum){
            $(Primitive::$dst_enum_list => $obj.$func::<$src_type,$dst_type_list>()),*
            , _ => {}
        }
    };
    ($obj:ident.$func:ident,$src_enum:ident,$dst_enum:ident,[$(($src_enum_list:ident,$src_type_list:ty)),*]) => {
        match($src_enum){
            $(Primitive::$src_enum_list => make_cvt!($obj.$func,($dst_enum,$src_type_list),[(I,i32),(U,u32),(L,i64),(UL,u64),(A,i32),(F,f32),(D,f64),(B,i8),(W,i16)])),*
            , _ => {}
        }
    };
    ($src_enum:ident,$dst_enum:ident,$obj:ident.$func:ident) => {
        make_cvt!($obj.$func,$src_enum,$dst_enum,[(I,i32),(U,u32),(L,i64),(UL,u64),(A,i32),(F,f32),(D,f64),(B,i8),(W,i16)])
    };
}


impl Machine{
    fn add_operand<T:Clone + Add<Output = T>>(&mut self){
        let lhs = self.operand.pop::<T>();
        let rhs = self.operand.pop::<T>();
        let result = lhs + rhs;
        self.operand.push::<T>(&result);
    }

    fn sub_operand<T:Clone + Sub<Output = T>>(&mut self){
        let lhs = self.operand.pop::<T>();
        let rhs = self.operand.pop::<T>();
        let result = lhs - rhs;
        self.operand.push::<T>(&result);
    }

    fn mul_operand<T:Clone + Mul<Output = T>>(&mut self){
        let lhs = self.operand.pop::<T>();
        let rhs = self.operand.pop::<T>();
        let result = lhs * rhs;
        self.operand.push::<T>(&result);
    }

    fn div_operand<T:Clone + Div<Output = T>>(&mut self){
        let lhs = self.operand.pop::<T>();
        let rhs = self.operand.pop::<T>();
        let result = lhs / rhs;
        self.operand.push::<T>(&result);
    }

    fn neg_operand<T:Clone + Neg<Output = T>>(&mut self){
        let rhs = self.operand.pop::<T>();
        self.operand.push(&-rhs);
    }

    fn cmp_operand<T:Clone + PartialOrd>(&mut self){
        let lhs = self.operand.pop::<T>();
        let rhs = self.operand.pop::<T>();
        self.operand.push::<i8>(&(if lhs > rhs { 1 } else if lhs < rhs { -1 } else { 0 }));
    }

    fn store_operand<T:Clone>(&mut self){
        let value = self.operand.pop::<T>();
        let address = self.operand.pop::<*mut T>();
        unsafe {
            *address = value;
        }
    }

    fn load_operand<T:Clone>(&mut self){
        let address = self.operand.pop::<*mut T>();
        unsafe {
            self.operand.push::<T>(&*address);
        }
    }

    fn cvt_operand<Src:Clone,Dst:Clone+From<Src>>(&mut self){
        let rhs = self.operand.pop::<Src>();
        self.operand.push::<Dst>(Dst::from(rhs).borrow());
    }

    fn executeCode(&mut self, code:Bytecode){
        match(code){
            Bytecode::Cmp {kind} => make_match!(kind,self,cmp_operand),
            Bytecode::Add {kind} => make_match!(kind,self,add_operand),
            Bytecode::Sub {kind} => make_match!(kind,self,sub_operand),
            Bytecode::Mul {kind} => make_match!(kind,self,mul_operand),
            Bytecode::Div {kind} => make_match!(kind,self,div_operand),
            Bytecode::Neg {kind} => make_match!(kind,self,neg_operand,(I,i32),(L,i64),(F,f32),(D,f64),(B,i8),(W,i16)),
            Bytecode::Store {kind} => make_match!(kind,self,store_operand),
            Bytecode::Load {kind} => make_match!(kind,self,load_operand),
            Bytecode::Cvt {src,dst} => match(src){
                I => match(dst){
                    U => self.cvt_operand::<i32,u32>(),
                    _ => {}
                }
                _ => {}
            }

             _ => {}
        }
    }
}
