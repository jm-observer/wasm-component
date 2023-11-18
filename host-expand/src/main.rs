use wasmtime::{component::*, Config, Engine, Store};

bindgen!();

struct MyState;

// Imports into the world, like the `name` import for this world, are
// satisfied through traits.
impl Host_Imports for MyState {
    // Note the `Result` return value here where `Ok` is returned back
    // to the component and `Err` will raise a trap.
    fn print(
        &mut self,
        msg: String
    ) -> std::result::Result<(), wasmtime::Error> {
        println!("print: {}", msg);
        Ok(())
    }
}

fn main() -> wasmtime::Result<()> {
    // Configure an `Engine` and compile the `Component` that is being
    // run for the application.
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let component =
        Component::from_file(&engine, "./client-expand.wasm")?;

    // Instantiation of bindings always happens through a `Linker`.
    // Configuration of the linker is done through a generated
    // `add_to_linker` method on the bindings structure.
    //
    // Note that the closure provided here is a projection from `T` in
    // `Store<T>` to `&mut U` where `U` implements the
    // `HelloWorldImports` trait. In this case the `T`, `MyState`,
    // is stored directly in the structure so no projection is
    // necessary here.
    let mut linker = Linker::new(&engine);
    Host_::add_to_linker(&mut linker, |state: &mut MyState| state)?;

    // As with the core wasm API of Wasmtime instantiation occurs
    // within a `Store`. The bindings structure contains an
    // `instantiate` method which takes the store, component, and
    // linker. This returns the `bindings` structure which is an
    // instance of `HelloWorld` and supports typed access
    // to the exports of the component.
    let mut store = Store::new(&engine, MyState);
    let (bindings, _) =
        Host_::instantiate(&mut store, &component, &linker)?;

    // Here our `greet` function doesn't take any parameters for the
    // component, but in the Wasmtime embedding API the first
    // argument is always a `Store`.
    // bindings.call_greet(&mut store)?;
    bindings.call_run(&mut store)?;
    Ok(())
}
