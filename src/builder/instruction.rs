use std::convert::TryInto;

use super::builder::{Builder, Frame, Variable, VariableIndex};

#[derive(Clone, Copy)]
pub enum Instruction<'a> {
    Jump(&'a Frame<'a>),
    U8Add(U8Op<'a>),
}

#[derive(Clone, Copy)]
pub struct U8Op<'a> {
    pub ret:    &'a VariableIndex,
    pub first:  &'a VariableIndex,
    pub second: &'a VariableIndex,
}

impl Instruction<'_> {
    pub fn render(self, parent: &Frame, consts: &Vec<Variable>) -> Vec<u8> {
        match self {
            Instruction::U8Add(v) => vec![0x0, v.ret.get(parent, consts).index.unwrap().try_into().unwrap(), v.first.get(parent, consts).index.unwrap().try_into().unwrap(), v.second.get(parent, consts).index.unwrap().try_into().unwrap() ],
            Instruction::Jump(v) => vec![0xF1, v.index.unwrap().try_into().unwrap()],
        }
    }
}