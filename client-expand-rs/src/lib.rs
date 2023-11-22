#[allow(unused_unsafe, clippy::all)]
pub fn print(msg: &str) {
    #[allow(unused_imports)]
    use wit_bindgen::rt::{alloc, string::String, vec::Vec};
    unsafe {
        let vec0 = msg;
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "print"]
            fn wit_import(_: i32, _: i32);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: i32, _: i32) {
            unreachable!()
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
        #[cfg(target_arch = "wasm32")]
        wit_bindgen::rt::run_ctors_once();
        <_GuestImpl as Guest>::run();
    }
};
use MyHost as _GuestImpl;
pub trait Guest {
    fn run();
}
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:host"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 180] = [
    3, 0, 4, 104, 111, 115, 116, 0, 97, 115, 109, 13, 0, 1, 0, 7, 61,
    1, 65, 2, 1, 65, 4, 1, 64, 1, 3, 109, 115, 103, 115, 1, 0, 3, 0,
    5, 112, 114, 105, 110, 116, 1, 0, 1, 64, 0, 1, 0, 4, 0, 3, 114,
    117, 110, 1, 1, 4, 1, 17, 101, 120, 97, 109, 112, 108, 101, 58,
    104, 111, 115, 116, 47, 104, 111, 115, 116, 4, 0, 11, 10, 1, 0,
    4, 104, 111, 115, 116, 3, 0, 0, 0, 16, 12, 112, 97, 99, 107, 97,
    103, 101, 45, 100, 111, 99, 115, 0, 123, 125, 0, 70, 9, 112, 114,
    111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101,
    115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99,
    111, 109, 112, 111, 110, 101, 110, 116, 6, 48, 46, 49, 56, 46,
    49, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45,
    114, 117, 115, 116, 6, 48, 46, 49, 52, 46, 48
];
#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
const _: &str = include_str!(
    r#"D:\git\wasm-component\client-expand-rs\wit\host.wit"#
);

struct MyHost;

impl Guest for MyHost {
    fn run() {
        print("Hello, world!");
    }
}
