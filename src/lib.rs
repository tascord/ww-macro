use proc_macro::TokenStream;
use syn::{
    parse::{Parse, ParseStream}, parse_macro_input, Block, Generics, Ident, Token, Type, Visibility
};

struct Meta(
    pub Visibility,
    pub Ident,
    pub Generics,
    pub Type,
    pub Ident,
    pub Type,
    pub Block,
);

impl Parse for Meta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        /*
        (vis) fn (name)<(generics)>(arg: Type) -> Type {
             (block)
        }
        */

        let vis = input.parse::<Visibility>()?;
        let _ = input.parse::<Token![fn]>()?;
        let name = input.parse::<Ident>()?;
        let generics = input.parse::<Generics>()?;

        let content;
        let _ = syn::parenthesized!(content in input);
        let arg_name = content.parse::<Ident>()?;
        let _ = content.parse::<Token![:]>()?;
        let arg = content.parse::<Type>()?;

        let _ = input.parse::<Token![->]>()?;
        let ret = input.parse::<Type>()?;
        let block = input.parse::<Block>()?;

        Ok(Meta(vis, name, generics, arg, arg_name, ret, block))
    }
}

#[proc_macro_attribute]
pub fn worked(attr: TokenStream, item: TokenStream) -> TokenStream {
    let meta = parse_macro_input!(item as Meta);

    let attr = parse_macro_input!(attr as syn::LitStr);
    let path = attr.value();

    let (vis, name, generics, arg, arg_name, ret, block) =
        (meta.0, meta.1, meta.2, meta.3, meta.4, meta.5, meta.6);
    let quiet_name = quote::format_ident!("__{}", name.to_string());
    let quiet_name_string = quiet_name.to_string();

    quote::quote! {
        #[wasm_bindgen]
        pub fn #quiet_name #generics(i__: Vec<u8>) -> js_sys::Array {
            let #arg_name: #arg = bincode::decode_from_slice(&i__, bincode::config::standard()).unwrap().0;
            let res = #block;
            
            let array = js_sys::Array::new();
            for item in bincode::encode_to_vec(&res, bincode::config::standard()).unwrap() {
                array.push(&JsValue::from(item));
            }

            array
        }

        #vis async fn #name #generics(i: #arg, c: impl Fn(#ret) + 'static) {
            let w = WrappedWorker::<#arg, #ret>::new(#path).await;
            w.run_task(#quiet_name_string, i, c);
        }
    }
    .into()
}
