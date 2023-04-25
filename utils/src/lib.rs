extern crate proc_macro;
use proc_macro::TokenStream;

/// Simplifies the exporting of functions to C.
#[proc_macro_attribute]
pub fn ffi(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_backup = item.clone();
    let res = parse_func(item).to_string();
    let settings = parse_settings(attr);
    let res = res.replace("__FFI_RAW_MODIFIERS__", &settings);
    res.parse().unwrap_or(item_backup)
}

// parses the function it should be attached to
fn parse_func(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let ret = &input.sig.output;
    let inputs = &input.sig.inputs;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;
    let vis = &input.vis;
    let result = quote::quote! {
        #(#attrs)*
        #[no_mangle]
        #vis __FFI_RAW_MODIFIERS__ extern "C" fn #name(#inputs) #ret {
            #body
        }
    };
    result.into()
}

// parses the settings (can be none, const, unsafe or both)
fn parse_settings<'a>(attr: TokenStream) -> String {
    let modifiers_str = attr.to_string();
    let mut res = String::new();
    if modifiers_str.contains("const") {
        res.push_str("const ");
    }
    if modifiers_str.contains("unsafe") {
        res.push_str("unsafe ");
    }
    res
}