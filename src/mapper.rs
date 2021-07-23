#[derive(Debug, Clone, PartialEq)]
pub struct Map {
    pub functions: Vec<Function>,
    pub frames: Vec<Frame>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub argc: u8,
    pub frame_index: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub start_ptr: u64,
    pub varcount: u8,
}

impl Map {
    pub fn render(&self) -> Vec<u8> {
        let mut ret: Vec<u8> = vec![];
        ret.push(self.functions.len() as u8);
        ret.push(self.frames.len() as u8);
        for i in &self.functions {
            ret.push(i.argc);
            ret.push(i.frame_index);
        }
        for i in &self.frames {
            // oh man this sucks
            for n in 0..8 {
                ret.push(i.start_ptr.to_be_bytes()[n])
            }
            ret.push(i.varcount);
        }
        return ret;
        
    }

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
            let ptr = u64::from_be_bytes(build_arr);
            let f = Frame { start_ptr: ptr, varcount: v[idx] };
            idx += 1;
            ret.frames.push(f)
        }
        
        return ret;
    }
}