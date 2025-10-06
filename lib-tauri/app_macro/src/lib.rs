use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, Fields, parse_macro_input};

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let name = &input.ident;
    let builder_name = syn::Ident::new(&format!("{}Builder", name), name.span());

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Builder can only be derived for structs with named fields"),
        },
        _ => panic!("Builder can only be derived for structs"),
    };

    let builder_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;

        quote! {
            #field_name: #field_type
        }
    });

    let setters = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! {
            pub fn #field_name(mut self, value: #field_type) -> Self {
                self.#field_name = value;
                self
            }
        }
    });

    let build_assignments = fields.iter().map(|field| {
        let field_name = &field.ident;

        quote! {
            #field_name: self.#field_name.clone()
        }
    });

    let expanded = quote! {
        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name::default()
            }
        }

        #[derive(Default)]
        pub struct #builder_name {
            #(#builder_fields,)*
        }

        impl #builder_name {
            #(#setters)*

            pub fn build(&mut self) -> #name {
                #name {
                    #(#build_assignments,)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
