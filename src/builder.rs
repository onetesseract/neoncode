use crate::{instruction::{Instruction}, mapper::{Frame, Function, Map}, types::Type};

pub struct Builder {
    pub blocks: Vec<Block>,
    pub protos: Vec<Prototype>,
}

pub struct Prototype {
    pub name: String,
    pub argc: u8,
    pub frame_index: u8,
}

pub struct Block {
    pub name: String,
    pub cont: Vec<Instruction>,
    pub varcount: usize,
}

impl Block {
    pub fn new(name: String) -> Block {
        Block{name, cont: vec![], varcount: 0}
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
    pub fn render(&self) -> Vec<u8> {
        let mut ret = vec![];
        let mut indicies = vec![];
        for i in &self.blocks {
            let mut rendered = i.render();
            indicies.push((rendered.len(), i.varcount));
            ret.append(&mut rendered);
        }
        let mut m = Map {frames: vec![], functions: vec![]};
        let mut running_total: usize = 0;
        for (len, varcount) in indicies {
            let f = Frame {start_ptr: (running_total+len) as u64, varcount: varcount as u8};
            running_total+=len;
            m.frames.push(f);
        }
        for i in &self.protos {
            let fun = Function {argc: i.argc, frame_index: i.frame_index};
            m.functions.push(fun);
        }
        let mut rendered_map = m.render();
        let map_len = rendered_map.len() as u64;
        rendered_map.append(&mut ret);
        ret = rendered_map;
        for n in 8..0 {
            ret.insert(0, map_len.to_be_bytes()[n])
        }
        ret

    }
}




pub struct Variable {
    pub name: String,
    pub ty: Type,
}