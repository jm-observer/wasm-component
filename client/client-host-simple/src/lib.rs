#[allow(unused_unsafe, clippy::all)]
pub fn name() -> wit_bindgen::rt::string::String {
    #[allow(unused_imports)]
    use wit_bindgen::rt::{alloc, string::String, vec::Vec};
    unsafe {
        #[repr(align(4))]
        struct RetArea([u8; 8]);
        let mut ret_area =
            ::core::mem::MaybeUninit::<RetArea>::uninit();
        let ptr0 = ret_area.as_mut_ptr() as i32;
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "name"]
            fn wit_import(_: i32);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: i32) {
            unreachable!()
        }
        wit_import(ptr0);
        let l1 = *((ptr0 + 0) as *const i32);
        let l2 = *((ptr0 + 4) as *const i32);
        let len3 = l2 as usize;
        let bytes3 = Vec::from_raw_parts(l1 as *mut _, len3, len3);
        wit_bindgen::rt::string_lift(bytes3)
    }
}
const _: () = {
    #[doc(hidden)]
    #[export_name = "greet"]
    #[allow(non_snake_case)]
    unsafe extern "C" fn __export_greet() {
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, string::String, vec::Vec};
        #[cfg(target_arch = "wasm32")]
        wit_bindgen::rt::run_ctors_once();
        <_GuestImpl as Guest>::greet();
    }
};
use MyHost as _GuestImpl;
pub trait Guest {
    fn greet();
}
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:hello-world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 195] = [
    3, 0, 11, 104, 101, 108, 108, 111, 45, 119, 111, 114, 108, 100,
    0, 97, 115, 109, 13, 0, 1, 0, 7, 62, 1, 65, 2, 1, 65, 4, 1, 64,
    0, 0, 115, 3, 0, 4, 110, 97, 109, 101, 1, 0, 1, 64, 0, 1, 0, 4,
    0, 5, 103, 114, 101, 101, 116, 1, 1, 4, 1, 22, 109, 121, 58, 112,
    114, 111, 106, 101, 99, 116, 47, 104, 101, 108, 108, 111, 45,
    119, 111, 114, 108, 100, 4, 0, 11, 17, 1, 0, 11, 104, 101, 108,
    108, 111, 45, 119, 111, 114, 108, 100, 3, 0, 0, 0, 16, 12, 112,
    97, 99, 107, 97, 103, 101, 45, 100, 111, 99, 115, 0, 123, 125, 0,
    70, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112,
    114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 2, 13, 119,
    105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 6, 48,
    46, 49, 56, 46, 49, 16, 119, 105, 116, 45, 98, 105, 110, 100,
    103, 101, 110, 45, 114, 117, 115, 116, 6, 48, 46, 49, 52, 46, 48
];
#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
const _: &str = include_str!(
    r#"D:\git\wasm-component\client-expand-rs\../wit/host-simple.wit"#
);

struct MyHost;

impl Guest for MyHost {
    fn greet() {
        println!("client greet");
    }
}
