use darling::{FromDeriveInput, FromField, ToTokens};
use proc_macro::TokenStream;
use syn::*;

#[macro_use]
extern crate quote;

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(weedle))]
struct MacroTopArgs {
    impl_bound: Option<String>,
}

#[derive(FromField, Debug)]
#[darling(attributes(weedle))]
struct MacroArgs {
    from: Option<String>,
    cond: Option<String>,
    post_check: Option<String>,

    /// Use if you need to convert from Option<T> to Option<U>
    #[darling(default)]
    opt: bool,
}

fn string_to_tokens<T: syn::parse::Parse + ToTokens>(
    s: String,
) -> Result<proc_macro2::TokenStream> {
    let lit = syn::parse2::<Lit>(s.to_token_stream()).expect("How did we get non-literal?");
    let s = match lit {
        Lit::Str(s) => s,
        _ => panic!("How did we get non-str literal?"),
    };
    let expr: T = s.parse()?;
    Ok(expr.to_token_stream())
}

fn get_parser_from_field(field: &Field) -> Result<proc_macro2::TokenStream> {
    let args = MacroArgs::from_field(field).map_err(syn::Error::from)?;
    let ty = &field.ty;
    let mut parser = match args.from {
        Some(from) => {
            let from = string_to_tokens::<Type>(from)?;
            quote! { nom::combinator::into(weedle!(#from)) }
        }
        _ => quote! { weedle!(#ty) },
    };
    if args.opt {
        parser = quote! { nom::combinator::opt(#parser) };
    }
    if let Some(cond) = args.cond {
        let cond = string_to_tokens::<Expr>(cond)?;
        parser = quote! {
            nom::combinator::map(
                nom::combinator::cond(#cond, #parser),
                |opt| opt.flatten()
            )
        }
    }
    if let Some(post_check) = args.post_check {
        let post_check = string_to_tokens::<Ident>(post_check)?;
        parser = quote! {
            nom::combinator::map(
                nom::sequence::tuple((#parser, #post_check)),
                |(body, _post)| body
            )
        }
    }
    Ok(parser)
}

fn generate_tuple_struct(data_struct: &DataStruct) -> Result<proc_macro2::TokenStream> {
    let mut count = 0;
    let field_ids = data_struct
        .fields
        .iter()
        .map(|_| {
            let id = string_to_tokens::<Ident>(format!("m{count}")).unwrap();
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
        use nom::lib::std::result::Result::Ok;
        let (input, (#(#field_ids,)*)) = nom::sequence::tuple((
            #(#field_parsers,)*
        ))(input)?;

        Ok((input, Self(#(#field_ids,)*)))
    };

    // eprintln!("\n***\nglobal_impl: {}\n---\n", result);

    Ok(result)
}

fn generate_named_struct(data_struct: &DataStruct) -> Result<proc_macro2::TokenStream> {
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
        use nom::lib::std::result::Result::Ok;
        #(#field_parsers)*

        Ok((input, Self {
            #(#field_ids),*
        }))
    };

    // eprintln!("\n***\nglobal_impl: {}\n---\n", result);

    Ok(result)
}

fn generate_struct(data_struct: &DataStruct) -> Result<proc_macro2::TokenStream> {
    let all_named = data_struct.fields.iter().all(|field| field.ident.is_some());
    if all_named {
        generate_named_struct(data_struct)
    } else {
        generate_tuple_struct(data_struct)
    }
}

fn generate_enum(data_enum: &DataEnum) -> Result<proc_macro2::TokenStream> {
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

        quote! { weedle!(#ty).map(Self::#variant_id) }
    });

    let result = quote! {
        use nom::Parser;
        alt!(
            #(#field_parsers,)*
        )(input)
    };

    // eprintln!("\n***\nglobal_impl: {}\n---\n", result);

    Ok(result)
}

fn generate(ast: &syn::DeriveInput) -> Result<TokenStream> {
    let args = MacroTopArgs::from_derive_input(ast).map_err(syn::Error::from)?;

    let id = &ast.ident;
    let generics = &ast.generics;
    let type_param_ids = generics.type_params().map(|p| &p.ident).collect::<Vec<_>>();
    let impl_bound = match args.impl_bound {
        Some(b) => string_to_tokens::<WhereClause>(b)?,
        _ => quote! {},
    };
    let impl_body = match &ast.data {
        syn::Data::Struct(data_struct) => generate_struct(data_struct),
        syn::Data::Enum(data_enum) => generate_enum(data_enum),
        syn::Data::Union(_) => panic!("Unions not supported"),
    }?;

    let impl_head = quote! { impl<'a,#(#type_param_ids),*> };
    let impl_tail = quote! { for #id #generics #impl_bound };

    Ok(quote! {
        #impl_head crate::Parse<'a> #impl_tail {
            fn parse_tokens<'slice>(input: crate::tokens::Tokens<'slice, 'a>) -> crate::VerboseResult<crate::tokens::Tokens<'slice, 'a>, Self> {
                #impl_body
            }
        }
    }
    .into())
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
