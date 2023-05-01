use extism::{Error, Plugin};

pub fn greet(plugin: &mut Plugin, person: &str) -> Result<String, Error> {
    let res = plugin.call("greet", person)?;
    Ok(String::from_utf8(res.into())?)
}
