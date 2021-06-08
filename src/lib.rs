use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{Attribute, DeriveInput, Fields, Meta, NestedMeta, Visibility};

#[proc_macro_derive(StructFieldNames, attributes(struct_field_names))]
pub fn derive_field_names(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let (vis, ty, generics) = (&ast.vis, &ast.ident, &ast.generics);
    let names_struct_ident = Ident::new(&(ty.to_string() + "FieldStaticStr"), Span::call_site());

    let fields = filter_fields(match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => panic!("FieldNames can only be derived for structs"),
    });

    let names_struct_fields = fields.iter().map(|(vis, ident)| {
        quote! {
            #vis #ident: &'static str
        }
    });

    let names_const_fields = fields.iter().map(|(_vis, ident)| {
        let ident_name = ident.to_string();
        quote! {
            #ident: #ident_name
        }
    });

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let tokens = quote! {
        #vis struct #names_struct_ident {
            #(#names_struct_fields),*
        }

        impl #impl_generics #ty #ty_generics
            #where_clause
        {
            #vis const FIELD_NAMES: #names_struct_ident = #names_struct_ident {
                #(#names_const_fields),*
            };
        }
    };
    tokens.into()
}

fn filter_fields(fields: &Fields) -> Vec<(Visibility, Ident)> {
    fields
        .iter()
        .filter_map(|field| {
            if field
                .attrs
                .iter()
                .find(|attr| has_skip_attr(attr))
                .is_none()
                && field.ident.is_some()
            {
                let field_vis = field.vis.clone();
                let field_ident = field.ident.as_ref().unwrap().clone();
                Some((field_vis, field_ident))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

const ATTR_PATH_IDENT: &'static str = "struct_field_names";
const ATTR_META_SKIP: &'static str = "skip";

fn has_skip_attr(attr: &Attribute) -> bool {
    if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
        if meta_list.path.is_ident(ATTR_PATH_IDENT) {
            for nested_item in meta_list.nested.iter() {
                if let NestedMeta::Meta(Meta::Path(path)) = nested_item {
                    if path.is_ident(ATTR_META_SKIP) {
                        return true;
                    }
                }
            }
        }
    }
    false
}
