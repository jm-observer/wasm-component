use wit_bindgen_rust_macro::Config;

fn main() {
    let src = r#"
    {
    // the name of the world in the `*.wit` input file
    world: "host",

    // For all exported worlds, interfaces, and resources, this specifies what
    // type they're corresponding to in this module. In this case the `MyHost`
    // struct defined below is going to define the exports of the `world`,
    // namely the `run` function.
    exports: {
        world: MyHost,
    },
}
    "#;
    let conf: Config = syn::parse_str(src).unwrap();
    println!("{:?}", conf);
}
