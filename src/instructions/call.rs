use crate::instruction::Instruction;
use crate::builder::Function;
use crate::traits;
use crate::types::Type;

pub struct Call<'a> {
    function: &'a Function,
    args: Vec<Instruction>,
}

impl traits::Typed for Call {
    fn get_type(&self) -> Type {
        return self.function.get_type();
    }
}

