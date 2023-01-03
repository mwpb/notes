use crate::validate::{is_terminator, Func, Instr, Prog};

pub fn cfg_from_prog(prog: Prog) -> Vec<Vec<Instr>> {
    let mut basic_blocks: Vec<Vec<Instr>> = Vec::new();
    for func in prog.functions {
        basic_blocks.append(&mut basic_blocks_from_func(func));
    }
    basic_blocks
}

fn basic_blocks_from_func(func: Func) -> Vec<Vec<Instr>> {
    let mut basic_blocks: Vec<Vec<Instr>> = Vec::new();
    let mut block: Vec<Instr> = Vec::new();
    for instr in func.instrs {
        match instr {
            Instr::Op(i) if is_terminator(&i) => {
                block.push(Instr::Op(i));
                basic_blocks.push(block);
                block = Vec::new();
            }
            Instr::Label(l) => {
                basic_blocks.push(block);
                block = Vec::new();
                block.push(Instr::Label(l));
            }
            _ => {
                block.push(instr);
            }
        }
    }
    basic_blocks.push(block);
    basic_blocks
}
