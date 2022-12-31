use darling::{FromField, ToTokens};
use proc_macro::TokenStream;
use syn::*;

#[macro_use]
extern crate quote;

#[derive(FromField, Debug)]
#[darling(attributes(weedle))]
struct MacroArgs {
    parse: Option<String>,
}

fn string_to_tokens(s: &str) -> Result<proc_macro2::TokenStream> {
    let lit = syn::parse2::<Lit>(s.to_token_stream())?;
    let s = match lit {
        Lit::Str(s) => s,
        _ => panic!("How did we get non-str literal?"),
    };
    let expr: Expr = s.parse()?;
    Ok(expr.to_token_stream())
}

fn string_to_ident(s: String) -> Result<Ident> {
    let lit = syn::parse2::<Lit>(s.to_token_stream())?;
    let s = match lit {
        Lit::Str(s) => s,
        _ => panic!("How did we get non-str literal?"),
    };
    let ident: Ident = s.parse()?;
    Ok(ident)
}

fn get_parser_from_field(field: &Field) -> Result<proc_macro2::TokenStream> {
    let args = MacroArgs::from_field(field).map_err(syn::Error::from)?;
    let parser = match args.parse {
        Some(p) => string_to_tokens(&p)?,
        _ => {
            let ty = &field.ty;
            quote! { weedle!(#ty) }
        }
    };
    Ok(parser)
}

fn generate_tuple_struct(
    id: &Ident,
    generics: &Generics,
    data_struct: &DataStruct,
) -> Result<TokenStream> {
    let mut count = 0;
    let field_ids = data_struct
        .fields
        .iter()
        .map(|_| {
            let id = string_to_ident(format!("m{count}")).unwrap();
            count += 1;
            quote! { #id }
        })
        .collect::<Vec<_>>();
    let field_parsers = data_struct
        .fields
        .iter()
        .map(get_parser_from_field)
        .collect::<Result<Vec<_>>>()?;

    let result = quote! {
        impl<'a> crate::Parse<'a> for #id #generics {
            fn parse(input: &'a str) -> crate::IResult<&'a str, Self> {
                use nom::lib::std::result::Result::Ok;
                let (input, (#(#field_ids,)*)) = nom::sequence::tuple((
                    #(#field_parsers,)*
                ))(input)?;

                Ok((input, Self(#(#field_ids,)*)))
            }
        }
    };

    // eprintln!("\n***\nglobal_impl: {}\n---\n", result);

    Ok(result.into())
}

fn generate_named_struct(
    id: &Ident,
    generics: &Generics,
    data_struct: &DataStruct,
) -> Result<TokenStream> {
    let field_ids: Vec<_> = data_struct
        .fields
        .iter()
        .map(|field| {
            let id = field.ident.as_ref().expect("How did we get unnamed field?");
            quote! { #id }
        })
        .collect();
    let field_parsers = data_struct
        .fields
        .iter()
        .map(|field| -> Result<proc_macro2::TokenStream> {
            let id = field.ident.as_ref().expect("How did we get unnamed field?");
            let parser = get_parser_from_field(field)?;
            Ok(quote! { let (input, #id) = #parser(input)?; })
        })
        .collect::<Result<Vec<_>>>()?;

    let result = quote! {
        impl<'a> crate::Parse<'a> for #id #generics {
            fn parse(input: &'a str) -> crate::IResult<&'a str, Self> {
                use nom::lib::std::result::Result::Ok;
                #(#field_parsers)*

                Ok((input, Self {
                    #(#field_ids),*
                }))
            }
        }
    };

    // eprintln!("\n***\nglobal_impl: {}\n---\n", result);

    Ok(result.into())
}

fn generate_struct(
    id: &Ident,
    generics: &Generics,
    data_struct: &DataStruct,
) -> Result<TokenStream> {
    let all_named = data_struct.fields.iter().all(|field| field.ident.is_some());
    if all_named {
        generate_named_struct(id, generics, data_struct)
    } else {
        generate_tuple_struct(id, generics, data_struct)
    }
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

#[proc_macro_derive(Weedle, attributes(weedle))]
pub fn weedle(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    // Build and return the generated impl
    match generate(&ast) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error().into(),
    }
}
