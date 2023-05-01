use extism::{Context, Plugin};
use plugin_system::greet;

fn main() {
    let plugin_path = "plugin.wasm";
    let wasm = std::fs::read(plugin_path).unwrap();
    let context = Context::new();
    let mut plugin = Plugin::new(&context, &wasm, [], true).unwrap();
    let res = greet(&mut plugin, "WebAssembly").unwrap();
    println!("{}", res);
}
