use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Token};

struct DistanceSquaredInput {
    a: Expr,
    b: Expr,
}

impl Parse for DistanceSquaredInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let a: Expr = input.parse()?;
        input.parse::<Token![,]>()?;
        let b: Expr = input.parse()?;
        Ok(DistanceSquaredInput { a, b })
    }
}

pub fn distance_squared(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DistanceSquaredInput);
    let a = input.a;
    let b = input.b;

    let dim_str = option_env!("DIMENSIONS").unwrap_or("2");
    let dim = dim_str.parse::<usize>().expect("Expected DIMENSIONS to be usize");

    let mut exprs = Vec::new();
    for i in 0..dim {
        exprs.push(quote! {
            (a[#i] - b[#i]) * (a[#i] - b[#i])
        });
    }

    let expanded = if exprs.is_empty() {
        quote! { 0.0 }
    } else {
        quote! {
            {
                let a = &#a;
                let b = &#b;
                #(#exprs)+*
            }
        }
    };
    TokenStream::from(expanded)
}
