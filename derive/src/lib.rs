use proc_macro::TokenStream;
use syn::*;

#[macro_use]
extern crate quote;

fn generate_struct(
    id: &Ident,
    generics: &Generics,
    data_struct: &DataStruct,
) -> Result<TokenStream> {
    let field_ids: Vec<_> = data_struct
        .fields
        .iter()
        .map(|field| {
            let id = field
                .ident
                .as_ref()
                .expect("Tuple struct not supported yet");
            quote! { #id }
        })
        .collect();
    let field_parsers = data_struct.fields.iter().map(|field| {
        let ty = &field.ty;
        quote! { weedle!(#ty) }
    });

    let result = quote! {
        impl<'a> crate::Parse<'a> for #id #generics {
            fn parse(input: &'a str) -> crate::IResult<&'a str, Self> {
                use nom::lib::std::result::Result::Ok;
                let (input, (#(#field_ids),*)) = nom::sequence::tuple((
                    #(#field_parsers,)*
                ))(input)?;

                Ok((input, Self {
                    #(#field_ids),*
                }))
            }
        }
    };

    // eprintln!("\n***\nglobal_impl: {}\n---\n", result);

    Ok(result.into())
}

fn generate_enum(id: &Ident, generics: &Generics, data_enum: &DataEnum) -> Result<TokenStream> {
    let field_parsers = data_enum.variants.iter().map(|variant| {
        let variant_id = &variant.ident;
        let fields = match &variant.fields {
            Fields::Unnamed(unnamed) => unnamed,
            _ => panic!("Only tuple variant enums are supported"),
        };
        if fields.unnamed.len() != 1 {
            panic!("Only one tuple field is supported");
        }
        let field = fields.unnamed.first().unwrap();
        let ty = &field.ty;

        quote! { weedle!(#ty).map(#id::#variant_id) }
    });

    let result = quote! {
        impl<'a> crate::Parse<'a> for #id #generics {
            fn parse(input: &'a str) -> crate::IResult<&'a str, Self> {
                use nom::Parser;
                nom::branch::alt((
                    #(#field_parsers),*
                ))(input)
            }
        }
    };

    // eprintln!("\n***\nglobal_impl: {}\n---\n", result);

    Ok(result.into())
}

fn generate(ast: &syn::DeriveInput) -> Result<TokenStream> {
    let id = &ast.ident;
    let generics = &ast.generics;
    match &ast.data {
        syn::Data::Struct(data_struct) => generate_struct(id, generics, data_struct),
        syn::Data::Enum(data_enum) => generate_enum(id, generics, data_enum),
        syn::Data::Union(_) => panic!("Unions not supported"),
    }
}

#[proc_macro_derive(Weedle)]
pub fn weedle(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    // Build and return the generated impl
    match generate(&ast) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error().into(),
    }
}
