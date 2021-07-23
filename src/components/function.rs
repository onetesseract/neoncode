use crate::traits;

pub struct Function {
    pub proto: Prototype,
    pub body: Vec<Block>,
}

pub struct Prototype {
    pub name: String,
    pub ty: Type,
    pub args: Vec<Variable>,

}

impl traits::Typed for Function {
    fn get_type(&self) -> Type {
        return self.proto.ty;
    }
}