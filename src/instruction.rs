use crate::Value;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    U8Add(U8Add),
    U8Sub(U8Sub),
    U8Mul(U8Mul),
    U8Div(U8Div),
    ReturnVal(ReturnVal),
}

#[derive(Debug, Clone, Copy)]
pub struct ReturnVal {
    pub val: Value,
}
impl ReturnVal {
    pub fn as_instruction(&self) -> Instruction {
        Instruction::ReturnVal(*self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct U8Add {
    pub add_to: Value,
    pub add_by: Value,
    pub ret: Value,
}
impl U8Add {
    pub fn as_instruction(&self) -> Instruction {
        Instruction::U8Add(self.clone())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct U8Sub {
    pub sub_from: Value,
    pub sub_by: Value,
    pub ret: Value,
}
impl U8Sub {
    pub fn as_instruction(&self) -> Instruction {
        Instruction::U8Sub(self.clone())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct U8Mul {
    pub mul_from: Value,
    pub mul_by: Value,
    pub ret: Value,
}
impl U8Mul {
    pub fn as_instruction(&self) -> Instruction {
        Instruction::U8Mul(self.clone())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct U8Div {
    pub div_from: Value,
    pub div_by: Value,
    pub ret: Value,
}
impl U8Div {
    pub fn as_instruction(&self) -> Instruction {
        Instruction::U8Div(self.clone())
    }
}

impl Instruction {
    pub fn render(self) -> Vec<u8> {
        match self {
            Instruction::U8Add(v) => vec![0, v.ret.index, v.add_to.index, v.add_by.index ],
            Instruction::U8Sub(v) => vec![1, v.ret.index, v.sub_from.index, v.sub_by.index ],
            Instruction::U8Mul(v) => vec![2, v.ret.index, v.mul_from.index, v.mul_by.index],
            Instruction::U8Div(v) => vec![3, v.ret.index, v.div_from.index, v.div_by.index],
            Instruction::ReturnVal(v) => vec![255, v.val.index],
        }
    }
}