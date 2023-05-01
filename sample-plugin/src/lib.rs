use extism_pdk::plugin_fn;
use extism_pdk::*;

#[plugin_fn]
pub fn greet(person: String) -> FnResult<String> {
    Ok(format!("Hello, {}!", person))
}
