use quote::quote;
use wit_bindgen_rust_macro_fork::Config;

fn main() {
    let src = quote! {
            {
        // the name of the world in the `*.wit` input file
        world: "sensor",
                path: "../../wit/sensor.wit",

        // For all exported worlds, interfaces, and resources, this specifies what
        // type they're corresponding to in this module. In this case the `MyHost`
        // struct defined below is going to define the exports of the `world`,
        // namely the `run` function.
        exports: {
            world: Sensor,
        },
    }
        };
    match syn::parse2::<Config>(src) {
        Ok(conf) => {
            println!("{:?}", conf);
            let token = conf.expand().unwrap();
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
