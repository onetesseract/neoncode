use crate::Value;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    IAdd(IAdd),
    ISub(ISub),
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
pub struct IAdd {
    pub add_to: Value,
    pub add_by: Value,
    pub ret: Value,
}
impl IAdd {
    pub fn as_instruction(&self) -> Instruction {
        Instruction::IAdd(self.clone())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ISub {
    pub sub_from: Value,
    pub sub_by: Value,
    pub ret: Value,
}
impl ISub {
    pub fn as_instruction(&self) -> Instruction {
        Instruction::ISub(self.clone())
    }
}

impl Instruction {
    pub fn render(self) -> Vec<u8> {
        match self {
            Instruction::IAdd(v) => vec![0,  v.ret.index, v.add_to.index, v.add_by.index ],
            Instruction::ISub(v) => vec![1,  v.ret.index, v.sub_from.index, v.sub_by.index ],
            Instruction::ReturnVal(v) => vec![255, v.val.index],
        }
    }
}