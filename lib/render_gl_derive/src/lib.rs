#![recursion_limit = "128"]

use core::panic;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Field, Fields, Lit, Meta, MetaNameValue};

#[proc_macro_derive(VertexAttribPointers, attributes(location))]
pub fn vertex_attrib_pointers_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    generate_impl(&ast)
}

fn generate_impl(ast: &DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let generics = &ast.generics;
    let where_clause = &ast.generics.where_clause;
    let fields_vertex_attrib_pointer = generate_vertex_attrib_pointer_calls(&ast.data);
    let a = quote! {
        impl #ident #generics #where_clause {
            fn vertex_attrib_pointers(gl: &gl::Gl) {
                let stride = std::mem::size_of::<Self>(); // byte offset between consecutive attributes
                let offset = 0; // offset of the first component

                #(#fields_vertex_attrib_pointer)*
            }
        }
    };
    a.into()
}

fn generate_vertex_attrib_pointer_calls(data: &Data) -> Vec<TokenStream2> {
    // panic!("data = {:#?}", data);
    match data {
        Data::Enum(_) => panic!("VertexAttribPointers can not be implemented for enums"),
        Data::Struct(DataStruct { struct_token: _, fields: Fields::Unit, semi_token: _ }) => {
            panic!("VertexAttribPointers can not be implemented for Unit structs")
        }
        Data::Union(_) => {
            panic!("VertexAttribPointers can not be implemented for unions")
        }
        Data::Struct(DataStruct { struct_token: _, fields: Fields::Unnamed(_), semi_token: _ }) => {
            panic!("VertexAttribPointers can not be implemented for Tuple structs")
        }
        Data::Struct(DataStruct {
            struct_token: _,
            fields: Fields::Named(ref s),
            semi_token: _,
        }) => s.named.iter().map(generate_struct_field_vertex_attrib_pointer_call).collect(),
    }
}

fn generate_struct_field_vertex_attrib_pointer_call(field: &Field) -> TokenStream2 {
    // panic!("field = {:#?}", field);
    let field_name = match field.ident {
        Some(ref i) => format!("{}", i),
        None => String::from(""),
    };
    let location_attr = field
        .attrs
        .iter()
        .filter(|a| a.path.get_ident().is_some() && a.path.get_ident().unwrap() == "location")
        .next()
        .unwrap_or_else(|| panic!("Field {:?} is missing #[location = ?] attribute", field_name));

    let location_value_literal = match location_attr.parse_meta() {
        Ok(Meta::NameValue(MetaNameValue { lit: Lit::Int(arg), path: _, eq_token: _ })) => arg,
        _ => panic!("Field {} location attribute value must be an integer literal", field_name),
    };
    let field_ty = &field.ty;
    quote! {
        let location = #location_value_literal; // layout (location = #location_attr)
        unsafe {
            #field_ty::vertex_attrib_pointer(gl, stride, location, offset);
        }
        let offset = offset + std::mem::size_of::<#field_ty>(); // offset of the second component
    }
}
