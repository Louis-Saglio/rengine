use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn get_dimensions_from_env_var(_input: TokenStream) -> TokenStream {
    let dim_str = option_env!("DIMENSIONS").unwrap_or("2");
    let dim_usize = dim_str.parse::<usize>().expect("Expected DIMENSIONS to be usize");
    let expanded = quote! {
        #dim_usize
    };
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn get_pop_size_from_env_var(_input: TokenStream) -> TokenStream {
    let dim_str = option_env!("POP_SIZE").unwrap_or("100");
    let dim_usize = dim_str.parse::<usize>().expect("Expected POP_SIZE to be usize");
    let expanded = quote! {
        #dim_usize
    };
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn get_g_from_env_var(_input: TokenStream) -> TokenStream {
    let dim_str = option_env!("G").unwrap_or("1");
    let dim_usize = dim_str.parse::<f64>().expect("Expected G to be f64");
    let expanded = quote! {
        #dim_usize
    };
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn get_worker_nbr_from_env_var(_input: TokenStream) -> TokenStream {
    let dim_str = option_env!("WORKER_NBR").unwrap_or("1");
    let dim_usize = dim_str.parse::<usize>().expect("Expected WORKER_NBR to be usize");
    let expanded = quote! {
        #dim_usize
    };
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn get_minimal_distance_from_env_var(_input: TokenStream) -> TokenStream {
    let dim_str = option_env!("MINIMAL_DISTANCE").unwrap_or("0");
    let dim_usize = dim_str.parse::<f64>().expect("Expected MINIMAL_DISTANCE to be f64");
    let expanded = quote! {
        #dim_usize
    };
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn get_iterations_from_env_var(_input: TokenStream) -> TokenStream {
    let dim_str = option_env!("ITERATIONS").unwrap_or("0");
    let dim_usize = dim_str.parse::<u32>().expect("Expected ITERATIONS to be u32");
    let expanded = quote! {
        #dim_usize
    };
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn get_desired_ups_from_env_var(_input: TokenStream) -> TokenStream {
    let dim_str = option_env!("DESIRED_UPS").unwrap_or("0");
    let dim_usize = dim_str.parse::<u16>().expect("Expected DESIRED_UPS to be u16");
    let expanded = quote! {
        #dim_usize
    };
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn get_particle_shape_from_env_var(_input: TokenStream) -> TokenStream {
    let dim_str = option_env!("PARTICLE_SHAPE").unwrap_or("circle");
    let expanded = quote! {
        #dim_str
    };
    TokenStream::from(expanded)
}
