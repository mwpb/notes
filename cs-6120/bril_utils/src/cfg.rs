use crate::validate::{is_terminator, Func, Instr, Prog};

pub fn cfg_from_prog(prog: Prog) -> Vec<Vec<Instr>> {
    let mut basic_blocks: Vec<Vec<Instr>> = Vec::new();
    for func in prog.functions {
        println!("{}", func.name);
        for basic_block in basic_blocks_from_func(func) {
            basic_blocks.push(basic_block);
        }
    }
    basic_blocks
}

fn basic_blocks_from_func(func: Func) -> Vec<Vec<Instr>> {
    let mut basic_blocks: Vec<Vec<Instr>> = Vec::new();
    let mut block: Vec<Instr> = Vec::new();
    for instr in func.instrs {
        if is_terminator(&instr) {
            block.push(instr);
            basic_blocks.push(block);
            block = Vec::new();
        }
    }
    basic_blocks
}
