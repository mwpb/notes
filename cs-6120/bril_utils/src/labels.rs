use crate::validate::{Instr, Prog};
use std::collections::HashMap;

pub fn label_usage_from_prog(prog: Prog) -> HashMap<String, u64> {
    let mut label_usage: HashMap<String, u64> = HashMap::new();
    for func in prog.functions {
        for instr in func.instrs[0].clone() {
            match instr.clone() {
                Instr::Op(_) => (),
                Instr::Label(l) => *label_usage.entry(l.label).or_insert(0) += 1,
            }
        }
    }
    label_usage
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::label_usage_from_prog;
    use crate::validate::load_prog;
    #[test]
    fn jmp_labels() {
        let prog = load_prog("./json/jmp.json");
        let label_usage = label_usage_from_prog(prog);
        let mut expected_usage: HashMap<String, u64> = HashMap::new();
        expected_usage.insert(String::from("somewhere"), 1);
        print!("{:?}", label_usage);
        assert_eq!(label_usage, expected_usage)
    }
}
