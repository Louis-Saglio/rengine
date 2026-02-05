mod distance_squared;
mod env_vars;

use proc_macro::TokenStream;

#[proc_macro]
pub fn distance_squared(input: TokenStream) -> TokenStream {
    distance_squared::distance_squared(input)
}

#[proc_macro]
pub fn get_dimensions_from_env_var(input: TokenStream) -> TokenStream {
    env_vars::get_dimensions_from_env_var(input)
}

#[proc_macro]
pub fn get_pop_size_from_env_var(input: TokenStream) -> TokenStream {
    env_vars::get_pop_size_from_env_var(input)
}

#[proc_macro]
pub fn get_g_from_env_var(input: TokenStream) -> TokenStream {
    env_vars::get_g_from_env_var(input)
}

#[proc_macro]
pub fn get_worker_nbr_from_env_var(input: TokenStream) -> TokenStream {
    env_vars::get_worker_nbr_from_env_var(input)
}

#[proc_macro]
pub fn get_minimal_distance_from_env_var(input: TokenStream) -> TokenStream {
    env_vars::get_minimal_distance_from_env_var(input)
}

#[proc_macro]
pub fn get_iterations_from_env_var(input: TokenStream) -> TokenStream {
    env_vars::get_iterations_from_env_var(input)
}

#[proc_macro]
pub fn get_desired_ups_from_env_var(input: TokenStream) -> TokenStream {
    env_vars::get_desired_ups_from_env_var(input)
}

#[proc_macro]
pub fn get_particle_shape_from_env_var(input: TokenStream) -> TokenStream {
    env_vars::get_particle_shape_from_env_var(input)
}

#[proc_macro]
pub fn get_default_particle_mass_from_env_var(input: TokenStream) -> TokenStream {
    env_vars::get_default_particle_mass_from_env_var(input)
}
