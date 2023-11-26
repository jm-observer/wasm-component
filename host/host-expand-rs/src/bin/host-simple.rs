use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store
};
use wasmtime_wasi::preview2::{
    pipe::MemoryOutputPipe, Table, WasiCtx, WasiCtxBuilder, WasiView
};

/**
cargo build --target wasm32-wasi --release --package client-host-simple
wasm-tools component new .\target\wasm32-wasi\release\client-host-simple.wasm -o client-host-simple.wasm --adapt ./wasi_snapshot_preview1.reactor.15.0.0.wasm
**/
fn main() -> wasmtime::Result<()> {
    // Configure an `Engine` and compile the `Component` that is being
    // run for the application.
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let component =
        Component::from_file(&engine, "client-host-simple.wasm")?;

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
    HelloWorld::add_to_linker(&mut linker, |state: &mut MyState| {
        state
    })?;
    wasmtime_wasi::preview2::command::sync::add_to_linker(
        &mut linker
    )?;

    let stdout = MemoryOutputPipe::new(4096);
    let stderr = MemoryOutputPipe::new(4096);

    // Create our wasi context.
    let mut builder = WasiCtxBuilder::new();
    builder.stdout(stdout.clone()).stderr(stderr.clone());

    // As with the core wasm API of Wasmtime instantiation occurs
    // within a `Store`. The bindings structure contains an
    // `instantiate` method which takes the store, component, and
    // linker. This returns the `bindings` structure which is an
    // instance of `HelloWorld` and supports typed access
    // to the exports of the component.
    let mut store = Store::new(
        &engine,
        MyState {
            name:  "me".to_string(),
            table: Table::new(),
            wasi:  builder.build()
        }
    );
    let (bindings, _) =
        HelloWorld::instantiate(&mut store, &component, &linker)?;

    // Here our `greet` function doesn't take any parameters for the
    // component, but in the Wasmtime embedding API the first
    // argument is always a `Store`.
    bindings.call_greet(&mut store)?;

    // Print the output from the invoked WASM module
    // It should display "Hello, world!"
    let bytes: Vec<u8> = stdout.contents().into();

    println!("{}", std::str::from_utf8(&bytes[..])?);
    Ok(())
}

struct MyState {
    name:  String,
    table: Table,
    wasi:  WasiCtx
}

// Imports into the world, like the `name` import for this world, are
// satisfied through traits.
impl HelloWorldImports for MyState {
    // Note the `Result` return value here where `Ok` is returned back
    // to the component and `Err` will raise a trap.
    fn name(&mut self) -> wasmtime::Result<String> {
        Ok(self.name.clone())
    }
}

impl WasiView for MyState {
    fn table(&self) -> &Table {
        &self.table
    }

    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }

    fn ctx(&self) -> &WasiCtx {
        &self.wasi
    }

    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

pub struct HelloWorld {
    greet: wasmtime::component::Func
}
pub trait HelloWorldImports {
    fn name(&mut self) -> wasmtime::Result<String>;
}
const _: () = {
    use wasmtime::component::__internal::anyhow;
    impl HelloWorld {
        pub fn add_to_linker<T, U>(
            linker: &mut wasmtime::component::Linker<T>,
            get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static
        ) -> wasmtime::Result<()>
        where
            U: HelloWorldImports {
            Self::add_root_to_linker(linker, get)?;
            Ok(())
        }

        pub fn add_root_to_linker<T, U>(
            linker: &mut wasmtime::component::Linker<T>,
            get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static
        ) -> wasmtime::Result<()>
        where
            U: HelloWorldImports {
            let mut linker = linker.root();
            linker.func_wrap(
                "name",
                move |mut caller: wasmtime::StoreContextMut<
                    '_,
                    T
                >,
                      (): ()| {
                    let host = get(caller.data_mut());
                    let r = HelloWorldImports::name(host);
                    Ok((r?,))
                }
            )?;
            Ok(())
        }

        /// Instantiates the provided `module` using the specified
        /// parameters, wrapping up the result in a structure that
        /// translates between wasm and the host.
        pub fn instantiate<T>(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<T>
        ) -> wasmtime::Result<(Self, wasmtime::component::Instance)>
        {
            let instance =
                linker.instantiate(&mut store, component)?;
            Ok((Self::new(store, &instance)?, instance))
        }

        /// Instantiates a pre-instantiated module using the specified
        /// parameters, wrapping up the result in a structure that
        /// translates between wasm and the host.
        pub fn instantiate_pre<T>(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            instance_pre: &wasmtime::component::InstancePre<T>
        ) -> wasmtime::Result<(Self, wasmtime::component::Instance)>
        {
            let instance = instance_pre.instantiate(&mut store)?;
            Ok((Self::new(store, &instance)?, instance))
        }

        /// Low-level creation wrapper for wrapping up the exports
        /// of the `instance` provided in this structure of wasm
        /// exports.
        ///
        /// This function will extract exports from the `instance`
        /// defined within `store` and wrap them all up in the
        /// returned structure which can be used to interact with
        /// the wasm module.
        pub fn new(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance
        ) -> wasmtime::Result<Self> {
            let mut store = store.as_context_mut();
            let mut exports = instance.exports(&mut store);
            let mut __exports = exports.root();
            let greet =
                *__exports.typed_func::<(), ()>("greet")?.func();
            Ok(HelloWorld { greet })
        }

        pub fn call_greet<S: wasmtime::AsContextMut>(
            &self,
            mut store: S
        ) -> wasmtime::Result<()> {
            let callee = unsafe {
                wasmtime::component::TypedFunc::<(), ()>::new_unchecked(self.greet)
            };
            let () = callee.call(store.as_context_mut(), ())?;
            callee.post_return(store.as_context_mut())?;
            Ok(())
        }
    }
};
const _: &str = include_str!(
    r#"D:\git\wasm-component\host-expand-rs\../wit/host-simple.wit"#
);
