use std::path::Path;

use anyhow::{bail, Result};
use wasmtime::*;

fn execute<P>(wasm_path: P) -> Result<i64>
where
    P: AsRef<Path>,
{
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let mut store: Store<()> = Store::new(&engine, ());

    let linker = Linker::new(store.engine());

    let module = Module::from_file(store.engine(), wasm_path)?;
    let canister = linker.instantiate(&mut store, &module)?;

    let double_bar = canister.get_typed_func::<(), i64>(&mut store, "double-bar")?;
    let res = double_bar.call(&mut store, ())?;
    Ok(res)
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        // The first arg will be the name of current binary.
        bail!("Expecting one argument: path to the canister WASM file");
    }
    let c = execute(args.last().unwrap())?;
    println!("{c}");
    Ok(())
}
