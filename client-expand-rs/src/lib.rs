#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[allow(unused_unsafe, clippy::all)]
pub fn print(msg: &str) {
    #[allow(unused_imports)]
    use wit_bindgen::rt::{alloc, string::String, vec::Vec};
    unsafe {
        let vec0 = msg;
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: i32, _: i32) {
            ::core::panicking::panic(
                "internal error: entered unreachable code"
            )
        }
        wit_import(ptr0, len0);
    }
}
const _: () = {
    #[doc(hidden)]
    #[export_name = "run"]
    #[allow(non_snake_case)]
    unsafe extern "C" fn __export_run() {
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, string::String, vec::Vec};
        <_GuestImpl as Guest>::run();
    }
};
use MyHost as _GuestImpl;
pub trait Guest {
    fn run();
}
const _: &str = "// wit/host.wit\r\npackage \
                 example:host\r\n\r\nworld host {\r\n  import \
                 print: func(msg: string)\r\n\r\n  export run: \
                 func()\r\n}";
struct MyHost;
impl Guest for MyHost {
    fn run() {
        print("Hello, world!");
    }
}
