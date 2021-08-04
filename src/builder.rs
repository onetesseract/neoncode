use crate::{instruction::{Instruction}, mapper::{Frame, Function, Map}, types::Type};

pub struct Builder {
    pub blocks: Vec<Block>,
    pub functions: Vec<Function>,
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
    pub varcount: usize,
    pub consts: Vec<u8>,
}

impl Block {
    pub fn new(name: String) -> Block {
        Block{name, cont: vec![], varcount: 0, consts: vec![]}
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
        println!("indicies: {:?}", indicies);
        let mut m = Map {frames: vec![], functions: vec![]};
        let mut running_total: usize = 0;
        let mut tracker = 0;
        for (len, varcount) in indicies {
            let f = Frame {start_ptr: (running_total+len) as u64, varcount: varcount as u8, consts: self.blocks[tracker].consts.clone()};
            running_total+=len;
            m.frames.push(f);
            tracker+=1;
        }
        for i in &self.functions {
            // let fun = Function {argc: i.argc, frame_index: i.frame_index};
            m.functions.push(i.clone());
        }
        let (mut rendered_map, mut data_sect) = m.render();
        let map_len = rendered_map.len() as u64;
        let mut r = Vec::from(map_len.to_le_bytes());
        println!("ret: {:?}", ret);
        println!("r: {:?}", r);
        println!("rend: {:?}", rendered_map);
        rendered_map.append(&mut Vec::from((data_sect.len() as u64).to_le_bytes()));
        rendered_map.append(&mut data_sect);
        rendered_map.append(&mut ret);
        r.append(&mut rendered_map);
        ret = r;
        for n in 8..0 {
            ret.insert(0, map_len.to_le_bytes()[n])
        }
        ret

    }
}




pub struct Variable {
    pub name: String,
    pub ty: Type,
}