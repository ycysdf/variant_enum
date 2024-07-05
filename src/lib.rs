use proc_macro::TokenStream;
use quote::quote;
use std::iter;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, parse_quote, parse_quote_spanned, Expr, Field, FieldMutability, Fields, FieldsUnnamed, ItemEnum, Visibility};

#[proc_macro_attribute]
pub fn typed_variant(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemEnum);
    let attr: proc_macro2::TokenStream = attr.into();
    let variant_structs: Vec<_> = ast
       .variants
       .iter_mut()
       .map(|n| {
           let inner_attr = n.attrs.iter().enumerate().find(|n| n.1.path().is_ident("inner")).map(|n| n.0).map(|i| n.attrs.remove(i));
           let ident = &n.ident;
           let inner_attr = inner_attr.and_then(|n| n.parse_args().ok().map(|n: Expr| quote! {#[#n]}));
           match &n.fields {
               Fields::Named(named_fields) => quote! {
               #inner_attr
               #[#attr]
               pub struct #ident #named_fields
            },
               Fields::Unnamed(unnamed_fields) => quote! {
               #inner_attr
               #[#attr]
               pub struct #ident #unnamed_fields;
            },
               Fields::Unit => quote! {
               #inner_attr
               #[#attr]
               pub struct #ident;
            },
           }
       })
       .collect();

    for variant in ast.variants.iter_mut() {
        let ident = &variant.ident;
        variant.fields = Fields::Unnamed(FieldsUnnamed {
            paren_token: Default::default(),
            unnamed: Punctuated::from_iter(iter::once(Field {
                attrs: vec![],
                vis: Visibility::Inherited,
                mutability: FieldMutability::None,
                ident: None,
                colon_token: None,
                ty: parse_quote! {
               #ident
            },
            })),
        })
    }
    let try_from_impls: Vec<_> = ast
       .variants
       .iter()
       .map(|n| {
           let ident = &n.ident;
           let enum_ident = &ast.ident;
           quote! {
            impl TryFrom<#enum_ident> for #ident {
               type Error = #enum_ident;

               fn try_from(value: #enum_ident) -> Result<Self, Self::Error> {
                  if let #enum_ident::#ident(m) = value {
                     Ok(m)
                  } else {
                     Err(value)
                  }
               }
            }
         }
       })
       .collect();

    (quote! {
      #[#attr]
      #ast
      #(#variant_structs)*
      #(#try_from_impls)*

   })
       .into()
}
