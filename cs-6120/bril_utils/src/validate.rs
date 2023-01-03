use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct Arg {
    pub name: String,
    #[serde(alias = "type")]
    pub typ: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Label {
    pub label: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum Value {
    Int(f64),
    Bool(bool),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Op {
    pub op: String,
    pub dest: Option<String>,
    #[serde(alias = "type")]
    pub typ: Option<String>,
    pub args: Option<Vec<String>>,
    pub funcs: Option<Vec<String>>,
    pub labels: Option<Vec<String>>,
    pub value: Option<Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum Instr {
    Op(Op),
    Label(Label),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Func {
    pub name: String,
    pub args: Option<Vec<String>>,
    #[serde(alias = "type")]
    pub typ: Option<String>,
    pub instrs: Vec<Instr>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Prog {
    pub functions: Vec<Func>,
}

pub fn is_terminator(i: &Op) -> bool {
    ["jmp", "br", "ret"].contains(&i.op.as_str())
}

pub fn load_prog(path: &str) -> Prog {
    let data = fs::File::open(path).expect("Error loading file.");
    let value = serde_json::from_reader(data).expect("Error parsing file.");
    serde_json::from_value(value).unwrap()
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
            Instr::Op(Op {
                op: String::from("const"),
                dest: Some(String::from("v")),
                typ: Some(String::from("int")),
                args: None,
                funcs: None,
                labels: None,
                value: Some(Value::Int(4.0)),
            }),
            Instr::Op(Op {
                op: String::from("jmp"),
                dest: None,
                typ: None,
                args: None,
                funcs: None,
                labels: Some(vec![String::from("somewhere")]),
                value: None,
            }),
            Instr::Op(Op {
                op: String::from("const"),
                dest: Some(String::from("v")),
                typ: Some(String::from("int")),
                args: None,
                funcs: None,
                labels: None,
                value: Some(Value::Int(2.0)),
            }),
            Instr::Label(Label {
                label: String::from("somewhere"),
            }),
            Instr::Op(Op {
                op: String::from("print"),
                dest: None,
                typ: None,
                args: Some(vec![String::from("v")]),
                funcs: None,
                labels: None,
                value: None,
            }),
        ];
        assert!(expected_instrs.iter().eq(instrs.iter()));
    }
}
