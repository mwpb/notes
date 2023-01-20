use crate::validate::{Func, Instr, Op, Prog};

pub fn ensure_cfg(prog: Prog) -> Prog {
    let mut functions: Vec<Func> = Vec::new();
    for func in prog.functions {
        let basic_blocks = ensure_basic_blocks(func.instrs);
        functions.push(Func {
            name: func.name,
            args: func.args,
            typ: func.typ,
            instrs: basic_blocks,
        });
    }
    Prog { functions }
}

fn ensure_basic_blocks(blocks: Vec<Vec<Instr>>) -> Vec<Vec<Instr>> {
    let mut basic_blocks: Vec<Vec<Instr>> = Vec::new();
    let mut block: Vec<Instr> = Vec::new();
    for instr in blocks.into_iter().flatten() {
        match instr {
            Instr::Op(i @ (Op::Jmp { .. } | Op::Br { .. } | Op::Ret { .. })) => {
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

#[cfg(test)]
mod tests {
    use serde_json::to_string_pretty;

    use crate::{
        cfg::ensure_cfg,
        validate::{load_prog, Func, Instr, Label, Op, Prog, Value},
    };

    #[test]
    fn test_jmp() {
        let prog = load_prog("./json/jmp.json");
        let s = to_string_pretty(&prog).expect("Error serialising struct.");
        println!("{}", s);
        let cfg = ensure_cfg(prog);
        let s = to_string_pretty(&cfg).expect("Error serialising struct.");
        println!("{}", s);

        let block1 = vec![
            Instr::Op(Op::Const {
                dest: "v".into(),
                typ: "int".into(),
                value: Value::Int(4),
            }),
            Instr::Op(Op::Jmp {
                labels: ["somewhere".into()],
            }),
        ];
        let block2 = vec![Instr::Op(Op::Const {
            dest: "v".into(),
            typ: "int".into(),
            value: Value::Int(2),
        })];
        let block3 = vec![
            Instr::Label(Label {
                label: "somewhere".into(),
            }),
            Instr::Op(Op::Print {
                args: vec!["v".into()],
            }),
        ];

        let expected_instrs: Vec<Vec<Instr>> = vec![block1, block2, block3];
        let expected_prog = Prog {
            functions: vec![Func {
                name: "main".into(),
                args: vec![],
                instrs: expected_instrs,
                typ: None,
            }],
        };
        assert!(expected_prog.eq(&cfg));
    }
}
