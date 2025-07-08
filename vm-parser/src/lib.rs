use proc_macro::TokenStream;

use quote::{format_ident, quote};

#[proc_macro]
pub fn vm_execute(_: TokenStream) -> TokenStream {
    let lines = std::fs::read_to_string("prospero.vm").unwrap();

    let mut statements: Vec<proc_macro2::TokenStream> = vec![];

    let mut last_out = format_ident!("ERROR");

    for line in lines.lines().skip(1) {
        let mut words = line.split_ascii_whitespace();
        let out = words.next().unwrap();

        let op = words.next().unwrap();

        let out = format_ident!("{}", out);
        last_out = out.clone();

        statements.push(match op {
            "var-x" => quote!(
                let #out: f64 = vx;
            ),
            "var-y" => quote!(
                let #out: f64 = vy;
            ),
            "add" => {
                let a = format_ident!("{}", words.next().unwrap());
                let b = format_ident!("{}", words.next().unwrap());
                quote!(
                    let #out: f64 = #a + #b;
                )
            }
            "sub" => {
                let a = format_ident!("{}", words.next().unwrap());
                let b = format_ident!("{}", words.next().unwrap());
                quote!(
                    let #out: f64 = #a - #b;
                )
            }
            "mul" => {
                let a = format_ident!("{}", words.next().unwrap());
                let b = format_ident!("{}", words.next().unwrap());
                quote!(
                    let #out: f64 = #a * #b;
                )
            }
            "max" => {
                let a = format_ident!("{}", words.next().unwrap());
                let b = format_ident!("{}", words.next().unwrap());
                quote!(
                    let #out: f64 = #a.max(#b);
                )
            }
            "min" => {
                let a = format_ident!("{}", words.next().unwrap());
                let b = format_ident!("{}", words.next().unwrap());
                quote!(
                    let #out: f64 = #a.min(#b);
                )
            }
            "neg" => {
                let a = format_ident!("{}", words.next().unwrap());
                quote!(
                    let #out: f64 = -#a;
                )
            }
            "sqrt" => {
                let a = format_ident!("{}", words.next().unwrap());
                quote!(
                    let #out: f64 = #a.sqrt();
                )
            }
            "square" => {
                let a = format_ident!("{}", words.next().unwrap());
                quote!(
                    let #out: f64 = #a * #a;
                )
            }
            "const" => {
                let a = words.next().unwrap().parse::<f64>().unwrap();
                quote!(
                    let #out: f64 = #a;
                )
            }
            _ => panic!("computer says no: {op:?}"),
        });
    }

    statements.push(quote!(return #last_out));

    quote!(
        #(#statements)*
    )
    .into()
}
