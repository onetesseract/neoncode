use std::convert::TryInto;

use crate::neon::crate_path::binary_encoding::{frame_map, frame_mapArgs, function_map, function_mapArgs, map, mapArgs, variables_type, variables_typeArgs};

use super::instruction::{Instruction, U8Op};

pub struct VariableIndex {
    index: usize,
    is_const: bool,
}

impl VariableIndex {
    pub fn get(&self, parent: &Frame, consts: &Vec<Variable>) -> Variable {
        if self.is_const {
            return consts[self.index];
        } else {
            return parent.variables[self.index].clone();
        }
    }
}

/// A `Variable` has a size in bytes thats it lol
#[derive(Clone, Copy)]
pub struct Variable {
    pub size: usize,
    pub index: Option<usize>,
}

pub struct Frame<'a> {
    pub instructions: Vec<Instruction<'a>>,
    pub variables: Vec<Variable>,
    pub parent: Option<usize>, // shall use this later
    /// only used at compile time to determine the ordering of frames
    pub index: Option<u64>,
}

pub struct Function<'a> {
    pub frames: Vec<Frame<'a>>,
    pub entry_frame_index: usize,
    pub arguments: Vec<Variable>,
    pub parent: Option<&'a Builder<'a>>,
}

pub struct Builder<'a> {
    pub functions: Vec<Function<'a>>,
    pub consts: Vec<Variable>,
    pub consts_data: Vec<u8>,
}

impl<'a> Frame<'a> {
    pub fn new(function: Option<usize>) -> Self {
        Frame {instructions: vec![], variables: vec![], parent: function, index: None}
    }

    pub fn add_instruction(&mut self, instruction: Instruction<'a>) {
        self.instructions.push(instruction);
    }

    pub fn add_variable(&mut self, size: usize) -> VariableIndex {
        let v = Variable {
            size,
            index: None,
        };
        self.variables.push(v);
        let v = VariableIndex {
            index: self.variables.len() - 1,
            is_const: false,
        };
        v
    }

    // oh boy

    pub fn build_jump(&mut self, target: &'a Frame<'a>) {
        self.instructions.push(Instruction::Jump(target))
    }

    pub fn build_u8_add(&mut self, ret: &'a VariableIndex, first: &'a VariableIndex, second: &'a VariableIndex) {
        self.instructions.push(Instruction::U8Add(U8Op {ret, first, second}))
    }

    pub fn render(&self, consts: &Vec<Variable>) -> Vec<u8> {
        let mut ret = vec![];
        for i in &self.instructions {
            ret.append(&mut i.render(self, consts));
        }
        ret
    }
}

impl<'a> Function<'a> {
    pub fn new(builder: Option<&'a Builder<'a>>) -> Self {
        let mut f = Function {parent: builder, frames: vec![], entry_frame_index: 0, arguments: vec![]};
        let frame = Frame::new(Some(f.frames.len()));
        f.frames.push(frame);
        f
    }
    pub fn add_frame(&mut self, self_index: usize) -> usize {
        self.frames.push(Frame::new(Some(self_index)));
        self.frames.len() - 1
    }
}

trait ToSizes {
    fn sizes(&mut self) -> Vec<u64>;
}

impl ToSizes for Vec<Variable> {
    fn sizes(&mut self) -> Vec<u64> {
        let mut ret: Vec<u64> = vec![];
        for i in self {
            ret.push(i.size.try_into().unwrap());
        }
        ret
    }
}

impl<'a, 'ctx> Builder<'a> {
    pub fn new() -> Self {
        Builder {functions: vec![], consts: vec![], consts_data: vec![]}
    }

    pub fn add_function(&mut self) -> usize {
        let func = Function::new(None);
        self.functions.push(func);
        self.functions.len() - 1
    }

    pub fn add_const(&mut self, size: usize, data: Vec<u8>) -> VariableIndex {
        let v = Variable {
            size,
            index: Some(self.consts.len()),
        };
        self.consts.push(v);
        if size != data.len() {
            panic!("Bad const data - size != data length");
        }
        self.consts_data.append(&mut (data.clone()));
        VariableIndex {
            index: self.consts.len() - 1,
            is_const: true,
        }
    }

    pub fn render(&mut self) -> Vec<u8>{
        let mut frame_index = 0;
        let mut frames = vec![];
        let mut instructions = vec![];
        let mut functions = vec![];
        let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
        let mut const_index = 0;
        for i in &mut self.consts {
            i.index = Some(const_index);
            const_index += 1;
        }
        for fun in &mut self.functions {
            for frame in &mut fun.frames {
                let mut var_index = 0;
                for var in &mut frame.variables {
                    var.index = Some(var_index + const_index);
                    var_index += 1;
                }
                frame.index = Some(frame_index);
                frame_index += 1;
                let var_indexes = builder.create_vector(&frame.variables.sizes());
                let variables = variables_type::create(&mut builder, &variables_typeArgs {
                    indexes: Some(var_indexes),
                });
                let rend_frame = frame_map::create(&mut builder, &frame_mapArgs {
                    start_index: instructions.len().try_into().unwrap(),
                    constant_indexes: None,
                    variables: Some(variables),
                });
                frames.push(rend_frame);
                instructions.append(&mut frame.render(&self.consts));
            }
            let var_indexes = builder.create_vector(&fun.arguments.sizes());
            let variables = variables_type::create(&mut builder, &variables_typeArgs {
                indexes: Some(var_indexes),
            });
            let function = function_map::create(&mut builder, &function_mapArgs{
                variables: Some(variables),
                entry_frame_index: fun.entry_frame_index.try_into().unwrap(),
            });
            functions.push(function);
        }
        let const_indexes = builder.create_vector(&self.consts.sizes());
        let consts_shape = variables_type::create(&mut builder, &variables_typeArgs {
            indexes: Some(const_indexes),
        });
        let consts_data = builder.create_vector(&self.consts_data);
        let functions = builder.create_vector(&functions);
        let frames = builder.create_vector(&frames);
        let map = map::create(&mut builder, &mapArgs {
            frames: Some(frames),
            data: Some(consts_data),
            data_shape: Some(consts_shape),
            functions: Some(functions),
        });
        builder.finish(map, None);
        let mut finished = builder.finished_data().to_vec();
        let mut ret = Vec::from((finished.len() as u64).to_le_bytes());
        ret.append(&mut finished);
        ret.append(&mut instructions);
        return ret;
    }
}