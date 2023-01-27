use std::collections::{HashMap, HashSet};

use crate::validate::{get_args, get_dest, Instr, Op, Prog};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Location {
    fn_name: String,
    block_number: usize,
    line_number: usize,
}

fn vars_used_in_instr(instr: &Instr, scope: &HashMap<String, Location>) -> Vec<Location> {
    let mut locations: Vec<Location> = Vec::new();
    for arg in get_args(instr) {
        if let Some(loc) = scope.get(&arg) {
            locations.push(loc.clone());
        }
    }
    locations
}

fn eliminate_dead_vars(prog: Prog) -> Prog {
    let mut scope: HashMap<String, Location> = HashMap::new();
    let mut references_of_var: HashMap<Location, HashSet<Location>> = HashMap::new();
    let mut vars_used_in_loc: HashMap<Location, HashSet<Location>> = HashMap::new();
    for func in prog.clone().functions {
        for (block_number, block) in func.instrs.iter().enumerate() {
            for (line_number, instr) in block.iter().enumerate() {
                let loc = Location {
                    fn_name: func.name.clone(),
                    block_number,
                    line_number,
                };
                let outbound_refs = vars_used_in_instr(instr, &scope);
                let mut vars_used = HashSet::new();
                for reference in outbound_refs {
                    references_of_var
                        .entry(reference.clone())
                        .or_default()
                        .insert(loc.clone());
                    vars_used.insert(reference);
                }
                vars_used_in_loc.insert(loc.clone(), vars_used);
                if let Some(dest) = get_dest(&instr) {
                    scope.insert(dest, loc.clone());
                }
            }
        }
    }
    print!("scope: {:?}", scope);
    print!("references_of_var: {:?}", references_of_var);
    print!("vars_used_in_loc: {:?}", vars_used_in_loc);

    prog
}

#[cfg(test)]
mod tests {
    use crate::{cfg::ensure_cfg, dead_code_elimination::eliminate_dead_vars, validate::load_prog};

    #[test]
    fn elim_simple() {
        let mut prog = load_prog("./json/tdce/simple.json");
        // prog = ensure_cfg(prog);
        // print!("{:?}", prog);
        let prog2 = eliminate_dead_vars(prog);

        assert!(false)
    }
}
