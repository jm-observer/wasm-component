const _: () = {
    #[doc(hidden)]
    #[export_name = "run"]
    #[allow(non_snake_case)]
    unsafe extern "C" fn __export_run(arg0: i32, arg1: i32) -> i32 {
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, string::String, vec::Vec};
        #[cfg(target_arch = "wasm32")]
        wit_bindgen::rt::run_ctors_once();
        let len0 = arg1 as usize;
        let result1 = <_GuestImpl as Guest>::run(
            Vec::from_raw_parts(arg0 as *mut _, len0, len0)
        );
        let ptr2 = _RET_AREA.0.as_mut_ptr() as i32;
        match result1 {
            Ok(e) => {
                *((ptr2 + 0) as *mut u8) = (0i32) as u8;
                match e {
                    Some(e) => {
                        *((ptr2 + 4) as *mut u8) = (1i32) as u8;
                        let vec3 =
                            (e.into_bytes()).into_boxed_slice();
                        let ptr3 = vec3.as_ptr() as i32;
                        let len3 = vec3.len() as i32;
                        ::core::mem::forget(vec3);
                        *((ptr2 + 12) as *mut i32) = len3;
                        *((ptr2 + 8) as *mut i32) = ptr3;
                    },
                    None => {
                        *((ptr2 + 4) as *mut u8) = (0i32) as u8;
                    }
                };
            },
            Err(e) => {
                *((ptr2 + 0) as *mut u8) = (1i32) as u8;
                *((ptr2 + 4) as *mut u8) =
                    (wit_bindgen::rt::as_i32(e)) as u8;
            }
        };
        ptr2
    }
    const _: () = {
        #[doc(hidden)]
        #[export_name = "cabi_post_run"]
        #[allow(non_snake_case)]
        unsafe extern "C" fn __post_return_run(arg0: i32) {
            let l0 = i32::from(*((arg0 + 0) as *const u8));
            match l0 {
                0 => {
                    let l1 = i32::from(*((arg0 + 4) as *const u8));
                    match l1 {
                        0 => {},
                        _ => {
                            let l2 = *((arg0 + 8) as *const i32);
                            let l3 = *((arg0 + 12) as *const i32);
                            wit_bindgen::rt::dealloc(
                                l2,
                                (l3) as usize,
                                1
                            );
                        }
                    }
                },
                _ => {}
            }
        }
    };
};
use MyHost as _GuestImpl;
pub trait Guest {
    fn run(
        bytes: wit_bindgen::rt::vec::Vec<u8>
    ) -> Result<Option<wit_bindgen::rt::string::String>, u8>;
}
#[allow(unused_imports)]
use wit_bindgen::rt::{alloc, string::String, vec::Vec};
#[repr(align(4))]
struct _RetArea([u8; 16]);
static mut _RET_AREA: _RetArea = _RetArea([0; 16]);
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:host"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 179] = [
    3, 0, 4, 104, 111, 115, 116, 0, 97, 115, 109, 13, 0, 1, 0, 7, 60,
    1, 65, 2, 1, 65, 5, 1, 112, 125, 1, 107, 115, 1, 106, 1, 1, 1,
    125, 1, 64, 1, 5, 98, 121, 116, 101, 115, 0, 0, 2, 4, 0, 3, 114,
    117, 110, 1, 3, 4, 1, 17, 101, 120, 97, 109, 112, 108, 101, 58,
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
    r#"D:\git\wasm-component\client-expand-rs\../wit/host.wit"#
);

struct MyHost;

impl Guest for MyHost {
    fn run(
        bytes: wit_bindgen::rt::vec::Vec<u8>
    ) -> Result<Option<wit_bindgen::rt::string::String>, u8> {
        String::from_utf8(bytes)
            .map_err(|x| {
                println!("{}", x);
                8
            })
            .map(|x| Some(x))
    }
}
