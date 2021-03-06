use crate::oop::Oop;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Slot {
    ConstM1,
    Const0,
    Const1,
    Const2,
    Const3,
    Const4,
    Const5,
    Primitive(Vec<u8>),
    Ref(Oop),
    Nop, //for long, double
}
