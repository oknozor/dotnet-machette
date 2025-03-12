use std::{fs, path::PathBuf};

use xsd_parser::{
    config::{GenerateFlags, InterpreterFlags, OptimizerFlags, Schema},
    generate, Config, Error,
};

fn main() -> Result<(), Error> {
    let dir = env!("CARGO_MANIFEST_DIR");
    let common = format!("{dir}/assets/Microsoft.Build.CommonTypes.xsd");
    let dir = PathBuf::from(dir);
    let mut config = Config::default();
    config.parser.schemas = vec![Schema::File(common.into())];
    config.interpreter.flags = InterpreterFlags::all();
    config.optimizer.flags = OptimizerFlags::all();
    config.generator.flags = GenerateFlags::all();
    config.parser.debug_output = Some(dir.join("parser.log"));
    config.interpreter.debug_output = Some(dir.join("interpreter.log"));
    config.optimizer.debug_output = Some(dir.join("optimizer.log"));

    let code = generate(config)?;
    fs::write(path, contents)
    Ok(())
}
