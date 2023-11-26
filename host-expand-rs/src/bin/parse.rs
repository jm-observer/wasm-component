use quote::quote;
use wasmtime_component_macro_fork::bindgen::{expand, Config};

fn main() {
    let src = quote! {
            {
                path: "../wit/host.wit",
    }
        };
    match syn::parse2::<Config>(src) {
        Ok(conf) => {
            println!("{:?}", conf);
            let token = expand(&conf).unwrap();
            let file_header = "\u{feff}";
            let file_code =
                format!("{}{}", file_header, token.to_string());

            let syntax_tree =
                syn::parse_file(file_code.as_str()).unwrap();

            println!("{}", prettyplease::unparse(&syntax_tree))
        },
        Err(e) => {
            println!("{}", e);
            println!("{:?}", e);
        }
    }
}
