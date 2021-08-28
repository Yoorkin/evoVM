use proc_macro::TokenStream;
use std::str::*;
use std::vec::*;
use std::fmt::Debug;
use std::format;
use std::borrow::Borrow;

extern crate proc_macro;

#[proc_macro]
pub fn makeBinOpMatch(op:TokenStream) -> TokenStream {
    let mut str = String::from("match(kind){");
    let pair = vec![("I","i32"),("U","u32"),("UL","u64"),("A","i32"),("F","f32"),("D","f64"),("B","i8"),("W","i16")];
    let op_str = op.to_string();
    for (e,t) in pair{
        str.push_str(format!("Primitive::{Kind} =>",Kind = e).borrow());
        str.push_str("{");
        str.push_str(format!("let a = self.operand.pop::<{Type}>() {Op} self.operand.pop::<{Type}>(); self.operand.push::<{Type}>(a)",
                             Op = op_str,Type = t).borrow());
        str.push_str("},")
    }
    str.push_str("_=>{} }");
    return str.parse().unwrap();
}


#[proc_macro]
pub fn makeAllMatch(item:TokenStream) -> TokenStream {
    let mut str = String::from("match(kind){");
    let tmp = item.to_string();
    println!("{}",tmp);
    let pair = vec![("I","i32"),("U","u32"),("L","i64"),("UL","u64"),("A","i32"),("F","f32"),("D","f64"),("B","i8"),("W","i16")];
    for (e,t) in pair{
        let stmt = tmp.replace("Enum",e).replace("Type",t);
        str.push_str(&stmt);
        str.push_str(",");
    }
    str.push_str("_=>{} }");
    return str.parse().unwrap();
}
