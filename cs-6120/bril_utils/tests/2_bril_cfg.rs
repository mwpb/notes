use bril_utils::validate::load_prog;
use serde_json::to_string;
#[test]
fn cfg_from_jmp_prog() {
    let prog = load_prog("./tests/json/jmp.json");
    let s = to_string(&prog).expect("Error serialising struct.");
    print!("{s}");
    assert!(false);
}
