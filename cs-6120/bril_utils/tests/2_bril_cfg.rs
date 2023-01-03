use bril_utils::cfg::cfg_from_prog;
use bril_utils::validate::load_prog;
use serde_json::to_string_pretty;

#[test]
fn cfg_from_jmp_prog() {
    let prog = load_prog("./json/jmp.json");
    let cfg = cfg_from_prog(prog);
    let s = to_string_pretty(&cfg).expect("Error serialising struct.");
    println!("{}", s);
}
