use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store
};

struct MyState;

fn main() -> wasmtime::Result<()> {
    // Configure an `Engine` and compile the `Component` that is being
    // run for the application.
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let component =
        Component::from_file(&engine, "./client_expand_rs.wasm")?;

    // Instantiation of bindings always happens through a `Linker`.
    // Configuration of the linker is done through a generated
    // `add_to_linker` method on the bindings structure.
    //
    // Note that the closure provided here is a projection from `T` in
    // `Store<T>` to `&mut U` where `U` implements the
    // `HelloWorldImports` trait. In this case the `T`, `MyState`,
    // is stored directly in the structure so no projection is
    // necessary here.
    let mut linker = Linker::<MyState>::new(&engine);
    // Host_::add_to_linker(&mut linker, |state: &mut MyState|
    // state)?;

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
    let _rs = bindings.call_run(&mut store, "abc".as_bytes())?;
    Ok(())
}

pub struct Host_ {
    run: wasmtime::component::Func
}
const _: () = {
    use wasmtime::component::__internal::anyhow;
    impl Host_ {
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
            let run = *__exports
                .typed_func::<(&[u8],), (Result<Option<String>, u8>,)>("run")?
                .func();
            Ok(Host_ { run })
        }

        pub fn call_run<S: wasmtime::AsContextMut>(
            &self,
            mut store: S,
            arg0: &[u8]
        ) -> wasmtime::Result<Result<Option<String>, u8>> {
            let callee = unsafe {
                wasmtime::component::TypedFunc::<
                    (&[u8],),
                    (Result<Option<String>, u8>,)
                >::new_unchecked(self.run)
            };
            let (ret0,) =
                callee.call(store.as_context_mut(), (arg0,))?;
            callee.post_return(store.as_context_mut())?;
            Ok(ret0)
        }
    }
};
const _: &str = include_str!(
    r#"D:\git\wasm-component\host-expand-rs\../wit/host.wit"#
);
