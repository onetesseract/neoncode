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

#[cfg(test)]
mod tests {
    use std::fmt::format;
    use std::path::Path;

    use crate::Value;
    use crate::builder::Block;
    use crate::builder::Builder;
    use crate::instruction::IAdd;
    use crate::instruction::ReturnVal;
    use crate::mapper::Function;
    use crate::mapper::Frame;
    use crate::mapper::Map;
    use std::fs;
    use std::path;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn return_isub() {
        let sub = IAdd { add_to: Value::from(0), add_by: Value::from(1), ret: Value::from(2)};
        let v = ReturnVal {val: Value::from(0)};
        let r = v.as_instruction();
        println!("{:?}", sub.as_instruction().render());
        println!("{:?}", r.render());
    }

    #[test]
    fn map_integrity() {
        let functions = vec![Function {argc: 7, frame_index: 0}, Function {argc: 3, frame_index: 1}];
        let frames = vec![Frame {start_ptr: 19, varcount: 2}];
        let m = Map {functions, frames};
        println!("Original: {:?}", m);
        let coded = m.render();
        println!("Coded len: {}", coded.len());
        println!("Coded: {:?}", coded);
        let decoded = Map::from(coded);
        println!("Decoded: {:?}", decoded);
        assert_eq!(m, decoded);
    }

    #[test]
    fn builder_integrity() {
        let mut builder = Builder {blocks: vec![], protos: vec![]};
        let b = Block::new(format!("hello"));
        builder.blocks.push(b);
        let v = builder.render();
        println!("{:?}", v);

    }

    #[test]
    fn file_test() {
        let mut builder = Builder {blocks: vec![], protos: vec![]};
        let b = Block::new(format!("hello"));
        builder.blocks.push(b);
        let v = builder.render();
        println!("{:?}", v);
        let path = path::Path::new("prog.neonbin");
        fs::write(path, v).unwrap();
    }
}
