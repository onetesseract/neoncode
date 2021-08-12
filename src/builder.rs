use crate::{instruction::{Instruction}, mapper::{Frame, Function, Map, Variables}, types::Type};

pub struct Builder {
    pub blocks: Vec<Block>,
    pub functions: Vec<Function>,
    pub consts_shape: Variables,
    pub consts: Vec<u8>,
}

/*
pub struct Prototype {
    pub name: String,
    pub argc: u8,
    pub frame_index: u8,
}
*/
#[derive(Clone)]
pub struct Block {
    pub name: String,
    pub cont: Vec<Instruction>,
    pub variables: Variables,
    // pub consts: Vec<u8>,
    pub const_indexes: Vec<u64>,
}

pub struct Constant {
    pub index: u64,
}

impl Block {
    pub fn new(name: String) -> Block {
        Block{name, cont: vec![], variables: Variables::new(), const_indexes: vec![]}
    }
    pub fn add_variable(&mut self, size: u64) {
        self.variables.sizes.push(size);
    }
    pub fn render(&self) -> Vec<u8> {
        let mut ret = vec![];
        for i in &self.cont {
            ret.append(&mut i.render());
        }
        ret
    }
}

impl Builder {
    pub fn render(mut self) -> Vec<u8> {
        let mut ret = vec![];
        let mut indicies = vec![];
        let mut const_sizes = vec![];
        for i in &self.blocks {
            let mut rendered = i.render();
            indicies.push((rendered.len(), &i.variables));
            const_sizes.push(i.const_indexes.clone());
            ret.append(&mut rendered);
        }
        println!("indicies: {:?}", indicies);
        let mut m = Map {frames: vec![], functions: vec![]}; // todo: refactor into one big nice loop and fix needing all the clones
        let mut running_total: usize = 0;
        let mut tracker = 0;
        for (len, variables) in indicies {
            let f = Frame {start_ptr: (running_total+len) as u64, variables: variables.clone(), const_indexes: const_sizes[tracker].clone()};
            running_total+=len;
            m.frames.push(f);
            tracker+=1;
        }
        for i in &self.functions {
            // let fun = Function {argc: i.argc, frame_index: i.frame_index};
            m.functions.push(i.clone());
        }
        let mut rendered_map = m.render();
        let map_len = rendered_map.len() as u64;
        let mut r = Vec::from(map_len.to_le_bytes());
        println!("ret: {:?}", ret);
        println!("r: {:?}", r);
        println!("rend: {:?}", rendered_map);
        rendered_map.append(&mut self.consts_shape.render());
        rendered_map.append(&mut Vec::from((self.consts.len() as u64).to_le_bytes()));
        rendered_map.append(&mut self.consts);
        rendered_map.append(&mut ret);
        r.append(&mut rendered_map);
        ret = r;
        for n in 8..0 {
            ret.insert(0, map_len.to_le_bytes()[n])
        }
        ret

    }

    pub fn add_const(&mut self, constant: Vec<u8>) -> Constant {
        let index = self.consts_shape.sizes.len();
        let size = constant.len();
        self.consts_shape.sizes.push(size as u64);
        Constant {index: index as u64}
    }
}




pub struct Variable {
    pub name: String,
    pub ty: Type,
}