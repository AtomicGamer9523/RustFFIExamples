extern crate proc_macro;
use proc_macro::TokenStream;
#[path="core"]mod core;

/// Simplifies the exporting of functions to C.
#[proc_macro_attribute]
pub fn ffi(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_backup = item.clone();
    let res = core::parse_func(item).to_string();
    let settings = core::parse_settings(attr);
    let res = res.replace("__FFI_RAW_MODIFIERS__", &settings.0);
    let res = res.replace("__FFI_EXTERN_MODIFIER__", &settings.1);
    res.parse().unwrap_or(item_backup)
}
