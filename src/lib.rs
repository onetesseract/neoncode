pub mod builder;
pub mod types;
pub mod instruction;
pub mod traits;
pub mod mapper;

#[derive(Debug, Clone, Copy)]
pub struct Value {
    index: u8,
}
impl Value {
    pub fn from(index: u8) -> Value {
        return Value {index}
    }
}
trait IntoValue {
    fn into_value(self) -> Value;
}

impl IntoValue for u8 {
    fn into_value(self) -> Value {
        Value::from(self)
    }
}



#[cfg(test)]
mod tests {
    use crate::Value;
    use crate::builder::Block;
    use crate::builder::Builder;
    use crate::instruction::U8Add;
    use crate::instruction::U8Sub;
    use crate::instruction::ReturnVal;
    use crate::mapper::Function;
    use crate::mapper::Frame;
    use crate::mapper::Map;
    use crate::IntoValue;
    use std::fs;
    use std::path;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn return_isub() {
        let sub = U8Add { add_to: Value::from(0), add_by: Value::from(1), ret: Value::from(2)};
        let v = ReturnVal {val: Value::from(0)};
        let r = v.as_instruction();
        println!("{:?}", sub.as_instruction().render());
        println!("{:?}", r.render());
    }

    #[test]
    fn map_integrity() {
        let functions = vec![Function {argc: 7, frame_index: 0}, Function {argc: 3, frame_index: 1}];
        let frames = vec![Frame {start_ptr: 19, varcount: 2, consts: vec![]}];
        let m = Map {functions, frames};
        println!("Original: {:?}", m);
        let coded = m.render();
        // println!("Coded len: {}", coded.len());
        println!("Coded: {:?}", coded);
        // let decoded = Map::from(coded);
        // println!("Decoded: {:?}", decoded);
        // assert_eq!(m, decoded);
    }

    #[test]
    fn builder_integrity() {
        let mut builder = Builder {blocks: vec![], functions: vec![]};
        let b = Block::new(format!("hello"));
        builder.blocks.push(b.clone());
        builder.blocks.push(b);
        let v = builder.render();
        println!("{:?}", v);

    }

    #[test]
    fn file_test() {
        let mut builder = Builder {blocks: vec![], functions: vec![]};
        let mut b = Block::new(format!("hello"));
        // builder.blocks.push(b.clone());
        b.cont.push(U8Add {add_to: 0.into_value(), add_by: 1.into_value(), ret: 3.into_value()}.as_instruction());
        b.cont.push(U8Add {add_to: 3.into_value(), add_by: 1.into_value(), ret: 3.into_value()}.as_instruction());
        b.cont.push(U8Sub {sub_from: 3.into_value(), sub_by: 2.into_value(), ret: 4.into_value()}.as_instruction());
        b.consts.push(18);
        b.consts.push(10);
        b.consts.push(20);
        b.varcount = 2;
        builder.blocks.push(b);

        let f  = Function {argc: 0, frame_index: 0};
        builder.functions.push(f);
        let v = builder.render();
        println!("{:?}", v);
        let path = path::Path::new("prog.neonbin");
        fs::write(path, v).unwrap();
    }
}
