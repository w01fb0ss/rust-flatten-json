extern crate flatten_json;
#[macro_use]
extern crate serde_json;
fn main() {
    let obj = json!({});
    let mut flat = json!({});
    flatten_json::flatten(&obj, &mut flat, None, true, None).unwrap();
    println!("{}", json!(flat));
    println!("aaa");
}
