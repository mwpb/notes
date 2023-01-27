use serde::{Deserialize, Deserializer, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct Arg {
    pub name: String,
    #[serde(alias = "type")]
    pub typ: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Label {
    pub label: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Value {
    Int(usize),
    Bool(bool),
}

fn default_empty_vec() -> Vec<String> {
    Vec::new()
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(tag = "op")]
#[serde(rename_all = "lowercase")]
pub enum Op {
    Const {
        dest: String,
        #[serde(alias = "type")]
        typ: String,
        value: Value,
    },
    // Arithmetic
    Add {
        dest: String,
        #[serde(alias = "type")]
        typ: String,
        args: [String; 2],
    },
    Mul {
        dest: String,
        #[serde(alias = "type")]
        typ: String,
        args: [String; 2],
    },
    Sub {
        dest: String,
        #[serde(alias = "type")]
        typ: String,
        args: [String; 2],
    },
    Div {
        dest: String,
        #[serde(alias = "type")]
        typ: String,
        args: [String; 2],
    },
    // Comparison
    Eq {
        dest: String,
        #[serde(alias = "type")]
        typ: String,
        args: [String; 2],
    },
    Lt {
        dest: String,
        #[serde(alias = "type")]
        typ: String,
        args: [String; 2],
    },
    Gt {
        dest: String,
        #[serde(alias = "type")]
        typ: String,
        args: [String; 2],
    },
    Le {
        dest: String,
        #[serde(alias = "type")]
        typ: String,
        args: [String; 2],
    },
    Ge {
        dest: String,
        #[serde(alias = "type")]
        typ: String,
        args: [String; 2],
    },
    // Logic
    Not {
        dest: String,
        #[serde(alias = "type")]
        typ: String,
        args: [String; 1],
    },
    And {
        dest: String,
        #[serde(alias = "type")]
        typ: String,
        args: [String; 2],
    },
    Or {
        dest: String,
        #[serde(alias = "type")]
        typ: String,
        args: [String; 2],
    },
    // Control
    Jmp {
        labels: [String; 1],
    },
    Br {
        args: [String; 1],
        labels: [String; 2],
    },
    Call {
        dest: Option<String>,
        args: Vec<String>,
        funcs: [String; 1],
        #[serde(alias = "type")]
        typ: String,
    },
    Ret {
        args: Vec<String>,
    },
    // Miscellaneous
    Print {
        #[serde(default = "default_empty_vec")]
        args: Vec<String>,
    },
    Nop {},
}

pub fn get_args(instr: &Instr) -> Vec<String> {
    let op = match instr {
        Instr::Label(l) => return vec![],
        Instr::Op(o) => o,
    };
    match op {
        Op::Add { args, .. }
        | Op::Mul { args, .. }
        | Op::Sub { args, .. }
        | Op::Div { args, .. }
        | Op::Eq { args, .. }
        | Op::Lt { args, .. }
        | Op::Gt { args, .. }
        | Op::Ge { args, .. }
        | Op::Le { args, .. }
        | Op::And { args, .. }
        | Op::Or { args, .. } => args.clone().to_vec(),
        Op::Br { args, .. } | Op::Not { args, .. } => args.clone().to_vec(),
        Op::Call { args, .. } | Op::Ret { args, .. } | Op::Print { args, .. } => {
            args.clone().to_vec()
        }
        Op::Const { .. } | Op::Nop { .. } | Op::Jmp { .. } => vec![],
    }
}

pub fn get_dest(instr: &Instr) -> Option<String> {
    let op = match instr {
        Instr::Label(l) => return None,
        Instr::Op(o) => o,
    };
    match op {
        Op::Add { dest, .. }
        | Op::Mul { dest, .. }
        | Op::Sub { dest, .. }
        | Op::Div { dest, .. }
        | Op::Eq { dest, .. }
        | Op::Lt { dest, .. }
        | Op::Gt { dest, .. }
        | Op::Ge { dest, .. }
        | Op::Le { dest, .. }
        | Op::And { dest, .. }
        | Op::Const { dest, .. }
        | Op::Or { dest, .. }
        | Op::Not { dest, .. } => Some(dest.clone()),
        Op::Call { dest, .. } => dest.clone(),
        Op::Ret { .. }
        | Op::Print { .. }
        | Op::Br { .. }
        | Op::Jmp { .. }
        | Op::Jmp { .. }
        | Op::Nop {} => None,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Instr {
    Op(Op),
    Label(Label),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Func {
    pub name: String,
    #[serde(default = "default_empty_vec")]
    pub args: Vec<String>,
    #[serde(alias = "type")]
    pub typ: Option<String>,
    #[serde(deserialize_with = "to_bbs")]
    pub instrs: Vec<Vec<Instr>>,
}

fn to_bbs<'de, D>(deserializer: D) -> Result<Vec<Vec<Instr>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Vec<Instr> = Deserialize::deserialize(deserializer)?;
    Ok(vec![s])
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Prog {
    pub functions: Vec<Func>,
}

pub fn load_prog<'a>(path: &str) -> Prog {
    let data = fs::read_to_string(&path).expect("Error loading file.");
    let prog: Prog = serde_json::from_str(&data).expect("Error parsing file.");
    prog
}

#[cfg(test)]
mod tests {
    use super::load_prog;
    use super::Instr;
    use super::Label;
    use super::Op;
    use super::Value;

    #[test]
    fn load_jmp() {
        let prog = load_prog("./json/jmp.json");

        assert_eq!(prog.functions.len(), 1);
        let main = &prog.functions[0];
        assert_eq!(main.name, "main");
        let instrs = &main.instrs;
        let expected_instrs: [Instr; 5] = [
            Instr::Op(Op::Const {
                dest: "v".into(),
                typ: "int".into(),
                value: Value::Int(4),
            }),
            Instr::Op(Op::Jmp {
                labels: [String::from("somewhere")],
            }),
            Instr::Op(Op::Const {
                dest: "v".into(),
                typ: "int".into(),
                value: Value::Int(2),
            }),
            Instr::Label(Label {
                label: String::from("somewhere"),
            }),
            Instr::Op(Op::Print {
                args: vec!["v".into()],
            }),
        ];
        assert!(expected_instrs.iter().eq(instrs[0].iter()));
    }

    #[test]
    fn load_nop() {
        use super::load_prog;
        use super::Instr;
        use super::Op;
        use super::Value;
        let prog = load_prog("./json/nop.json");

        assert_eq!(prog.functions.len(), 1);
        let main = &prog.functions[0];
        assert_eq!(main.name, "main");
        let instrs = &main.instrs;
        let expected_instrs: [Instr; 5] = [
            Instr::Op(Op::Nop {}),
            Instr::Op(Op::Const {
                dest: "v".into(),
                typ: "int".into(),
                value: Value::Int(5),
            }),
            Instr::Op(Op::Nop {}),
            Instr::Op(Op::Print {
                args: vec!["v".into()],
            }),
            Instr::Op(Op::Nop {}),
        ];
        assert!(expected_instrs.iter().eq(instrs[0].iter()));
    }
}
