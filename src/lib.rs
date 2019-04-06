use syn::parse::Parse;
use proc_macro2::TokenTree::{
    Ident,
    Group,
    Literal,
    Punct,
};
use quote::quote;

extern crate proc_macro;
extern crate proc_macro2;
use proc_macro2::{
    TokenStream,
    token_stream::IntoIter,
};

use syn::parse::ParseStream;

#[proc_macro]
pub fn frame(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    // println!("{:#?}", input);

    let tokens = input.into_iter();

    let (decodeable_name, tokens) = get_struct_identifier(tokens);

    // let mut field_names = vec![];
    // let mut fields = vec![];

    let fields_meta: Struct = syn::parse2(tokens.clone()).unwrap();
    println!("{:#?}", fields_meta);

    let fields = fields_meta.fields.iter().map(|f| {
        let name = &f.name;
        let typ = &f.typ;
        quote!{
            pub #name: #typ
        }
    });

    let result = quote! {
        pub struct #decodeable_name {
            #(#fields),*
        }

        // impl #decodeable_name {
        //     fn encode(&self, buf: &mut [u8]) -> Option<usize> {
        //         #(#fields)|*
        //     }
        // }
    };
    println!("{:#?}", result);
    result.into()
}

#[derive(Debug)]
enum Bits {
    Full,
    Partial(syn::ExprRange)
}

#[derive(Debug)]
struct Struct {
    pub fields: Vec<Field>,
}

impl Parse for Struct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            fields: input.parse_terminated::<Field, syn::Token![,]>(Field::parse)?.into_iter().collect()
        })
    }
}

#[derive(Debug)]
struct Dependency {
    byte_range: syn::ExprRange,
    bit_range: Bits,
    typ: Box<syn::Type>,
}

impl Parse for Dependency {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            byte_range: parse_range(input)?,
            bit_range: {
                if input.peek(syn::token::Bracket) {
                    Bits::Partial(parse_range(input)?)
                } else {
                    Bits::Full
                }
            },
            typ: {
                input.parse::<syn::Token![:]>()?;
                input.parse()?
            },
        })
    }
}

#[derive(Debug)]
struct Field {
    name: syn::Ident,
    byte_range: syn::ExprRange,
    bit_range: Bits,
    typ: Box<syn::Type>,
    depends_on: Option<Dependency>
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Field {
            name: input.parse()?,
            byte_range: parse_range(input)?,
            bit_range: {
                if input.peek(syn::token::Bracket) {
                    Bits::Partial(parse_range(input)?)
                } else {
                    Bits::Full
                }
            },
            typ: {
                input.parse::<syn::Token![:]>()?;
                input.parse()?
            },
            depends_on: {
                if input.peek(syn::Token![->]) {
                    input.parse::<syn::Token![->]>()?;
                    Some(input.parse()?)
                } else {
                    None
                }
            }
        })
    }
}

fn parse_range(input: ParseStream) -> syn::Result<syn::ExprRange> {
    let content;
    syn::bracketed!(content in input);
    match content.parse()? {
        syn::Expr::Range(range_expr) => Ok(range_expr),
        syn::Expr::Lit(lit_expr) => {
            let exp = Box::new(syn::Expr::Lit(lit_expr));
            Ok(syn::ExprRange {
                attrs: vec![],
                from: Some(exp.clone()),
                limits: syn::RangeLimits::HalfOpen(syn::Token![..](proc_macro2::Span::call_site())),
                to: Some(Box::new(syn::Expr::Binary(syn::ExprBinary {
                    attrs: vec![],
                    left: exp,
                    op: syn::BinOp::Add(syn::Token![+](proc_macro2::Span::call_site())),
                    right: Box::new(syn::Expr::Lit(syn::ExprLit {
                        attrs: vec![],
                        lit: syn::Lit::Int(syn::LitInt::new(1, syn::IntSuffix::I64, proc_macro2::Span::call_site()))
                    }))
                })))
            })
        },
        _ => panic!("Expected range.")
    }
}

fn get_struct_identifier(mut input: IntoIter) -> (proc_macro2::Ident, TokenStream) {
    match input.next() {
        Some(Ident(ident)) => {
            let ident = syn::Ident::new(&ident.to_string(), ident.span());
            match input.next() {
                // Match brace group
                Some(Group(group)) => {
                    // Match curly braces of group
                    match group.delimiter() {
                        proc_macro2::Delimiter::Brace => (ident, group.stream()),
                        _ => panic!("Expected brace.")
                    }
                },
                _ => panic!("Bad token.")
            }
        },
        _ => panic!("Expected a struct identifier.")
    }
}

// println!("{:?}", start);
// let bits: u32 = start.to_string().parse::<u32>().unwrap() - stop.to_string().parse::<u32>().unwrap() + 1;

// let mut mask: u32 = 0 << 1;
// mask |= 1;
// for bit in 0..bits - 1 {
//     mask << 1;
//     mask |= 1;
// }

// let self_ = proc_macro2::Ident::new("self", proc_macro2::Span::call_site());
// encode_ts.extend(vec![
//     Ident(self_),
//     Punct(proc_macro2::Punct::new('.', proc_macro2::Spacing::Alone)),
//     Ident(field_name),
//     Punct(proc_macro2::Punct::new('<', proc_macro2::Spacing::Joint)),
//     Punct(proc_macro2::Punct::new('<', proc_macro2::Spacing::Alone)),
//     Literal(proc_macro2::Literal::u32_suffixed(mask)),
// ].into_iter());

// fields.push(encode_ts);

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        
    }
}
