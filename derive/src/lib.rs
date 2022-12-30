use proc_macro::TokenStream;
use syn::*;

#[macro_use]
extern crate quote;

pub(crate) fn gen_impl(ast: &syn::DeriveInput) -> Result<TokenStream> {
    let id = &ast.ident;
    let generics = &ast.generics;
    match &ast.data {
        syn::Data::Struct(data_struct) => {
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
                            #(#field_parsers),*
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
        syn::Data::Enum(_) => panic!("Enums not supported yet"),
        syn::Data::Union(_) => panic!("Unions not supported"),
    }
}

#[proc_macro_derive(Weedle)]
pub fn weedle(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    // Build and return the generated impl
    match gen_impl(&ast) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error().into(),
    }
}
