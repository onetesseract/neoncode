#[derive(Debug, Clone, PartialEq)]
pub struct Map {
    pub functions: Vec<Function>,
    pub frames: Vec<Frame>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variables {
    pub sizes: Vec<u64>,
}

impl Variables {
    pub fn render(&self) -> Vec<u8> {
        let mut ret = vec![];
        for n in 0..8 {
            ret.push(self.sizes.len().to_le_bytes()[n]); // append the size of the sizes
        }
        for i in &self.sizes {
            for n in 0..8 {
                ret.push(i.to_le_bytes()[n]); // append the size
            }
        }

        ret
    }
    pub fn new() -> Variables {
        Variables { sizes: vec![] }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub arguments: Variables,
    pub frame_index: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub start_ptr: u64,
    pub variables: Variables,
    pub const_indexes: Vec<u64>,
    // pub constcount: u8,
    // pub consts: Vec<u8>,
}

impl Map {
    pub fn render(&self) -> Vec<u8> {
        let mut ret: Vec<u8> = vec![];
        ret.push(self.functions.len() as u8);
        ret.push(self.frames.len() as u8);
        for i in &self.functions {
            ret.append(&mut i.arguments.render());
            ret.push(i.frame_index);
        }
        for i in &self.frames {
            // oh man this sucks
            for n in 0..8 {
                ret.push(i.start_ptr.to_le_bytes()[n])
            }
            ret.append(&mut i.variables.render());
            for n in 0..8 {
                ret.push(i.const_indexes.len().to_le_bytes()[n])
            }
            for i in &i.const_indexes {
                for n in 0..8 {
                    ret.push(i.to_le_bytes()[n])
                }
            }
            /* 
            for i in &i.consts {
                
                for n in 0..8 {
                    ret.push(consts.len().to_le_bytes()[n]); // append the index of the constant referenced
                }
                
                consts.push(*i);
            }*/
        }
        // ret.append(&mut consts);
        return ret;
        
    }
    /*
    pub fn from(v: Vec<u8>) -> Map {
        let mut ret = Map { functions: vec![], frames: vec![] };
        let mut idx: usize = 0;
        let fnlen = v[idx];
        let framelen = v[idx+1];
        idx += 2;
        for _ in 0..fnlen {
            let f = Function { argc: v[idx], frame_index: v[idx+1] };
            ret.functions.push(f);
            idx += 2;
        }
        for _ in 0..framelen {
            let mut build_arr: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
            for i in 0..8 {
                build_arr[i] = v[idx+i];
            }
            idx += 8;
            let ptr = u64::from_le_bytes(build_arr);

            let f = Frame { start_ptr: ptr, varcount: v[idx] };
            idx += 1;
            ret.frames.push(f)
        }
        
        return ret;
    }*/
}