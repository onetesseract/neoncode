use crate::types::Type;

pub trait Typed {
    fn get_type(&self) -> Type;
}