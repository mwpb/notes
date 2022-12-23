use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct Arg {
    pub name: String,
    #[serde(alias = "type")]
    pub typ: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Label {
    pub label: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Value {
    Int(f64),
    Bool(bool),
}

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Instr {
    Op(Op),
    Label(Label),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Func {
    pub name: String,
    pub args: Option<Vec<String>>,
    #[serde(alias = "type")]
    pub typ: Option<String>,
    pub instrs: Vec<Instr>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Prog {
    pub functions: Vec<Func>,
}

pub fn is_terminator(instr: &Instr) -> bool {
    match instr {
        Instr::Label(_) => false,
        Instr::Op(i) => ["jmp", "br", "ret"].contains(&i.op.as_str()),
    }
}

pub fn load_prog(path: &str) -> Prog {
    let data = fs::File::open(path).expect("Error loading file.");
    let value = serde_json::from_reader(data).expect("Error parsing file.");
    println!("{}", value);
    serde_json::from_value(value).unwrap()
}
