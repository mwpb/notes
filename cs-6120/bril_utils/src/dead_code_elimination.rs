use std::collections::{HashMap, HashSet};

use crate::validate::{Instr, Prog};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VarLocation {
    fn_name: String,
    block_number: usize,
    line_number: usize,
}

pub type VarScope = HashMap<String, VarLocation>;

// fn get_outbound_references(instr: &Instr, scope: VarScope) -> Vec<VarLocation> {
//     let op = match instr {
//         Instr::Label(l) => return Vec::new(),
//         Instr::Op(o) => o,
//     };

//     let mut locations: Vec<VarLocation> = Vec::new();
//     for arg in op.args.clone() {
//         if let Some(loc) = scope.get(&arg) {
//             locations.push(loc.clone());
//         }
//     }
//     locations
// }

// fn eliminate_dead_vars(prog: Prog) -> Prog {
//     let mut scope: VarScope = HashMap::new();
//     let mut references: HashMap<VarLocation, HashSet<VarLocation>> = HashMap::new();
//     let mut refers_to: HashMap<VarLocation, HashSet<VarLocation>> = HashMap::new();
//     for func in prog.functions {
//         for (block_number, block) in func.instrs.iter().enumerate() {
//             for (line_number, instr) in block.iter().enumerate() {
//                 let loc = VarLocation {
//                     fn_name: func.name.clone(),
//                     block_number,
//                     line_number,
//                 };
//                 let outbound_refs = get_outbound_references(instr, scope);
//                 for reference in outbound_refs {
//                     references.entry(reference).or_default().insert(loc);
//                 }
//                 refers_to.insert(loc, HashSet::from_iter(outbound_refs));

//                 if let Instr::Op(o @ Some()) = instr {
//                     if o.op == "const" && o.dest.is_some() {
//                         scope.insert(o.dest., loc);
//                     }
//                 }
//             }
//         }
//     }

//     prog
// }

#[cfg(test)]
mod tests {
    use crate::{cfg::ensure_cfg, validate::load_prog};

    #[test]
    fn elim_simple() {
        let mut prog = load_prog("./json/tdce/simple.json");
        // prog = ensure_cfg(prog);
        print!("{:?}", prog);
        // assert!(false)
    }
}
