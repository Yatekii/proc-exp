use syn::LitInt;
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

#[proc_macro]
pub fn frame(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    // println!("{:#?}", input);

    let mut tokens = input.into_iter();

    let (decodeable_name, mut tokens) = get_struct_identifier(tokens);

    let mut field_names = vec![];
    let mut fields = vec![];

    loop {
        let (field, _tokens) = get_field(tokens);
        tokens = _tokens;

        fields.push(field);
        field_names = fields.iter().map(|f| f.field_name.clone()).collect::<Vec<_>>();
    }

    println!("{:?}", fields);

    // Our input function is always equivalent to returning 42, right?
    let result = quote! {
        pub struct #decodeable_name {
            #(pub #field_names: u32),*
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
struct Field {
    field_name: proc_macro2::Ident,
    byte_range: std::ops::Range<u64>,
    typ: proc_macro2::Ident,
}

fn get_struct_identifier(mut input: IntoIter) -> (proc_macro2::Ident, IntoIter) {
    match input.next() {
        Some(Ident(ident)) => {
            let ident = syn::Ident::new(&ident.to_string(), ident.span());
            match input.next() {
                // Match brace group
                Some(Group(group)) => {
                    // Match curly braces of group
                    match group.delimiter() {
                        proc_macro2::Delimiter::Brace => (ident, group.stream().into_iter()),
                        _ => panic!("Expected brace.")
                    }
                },
                _ => panic!("Bad token.")
            }
        },
        _ => panic!("Expected a struct identifier.")
    }
}

fn get_field(input: IntoIter) -> (Field, IntoIter) {
    let (field_name, input) = get_field_identifier(input);

    let (byte_range, mut input, _) = get_byte_range(input);

    input = expect_colon(input);

    let (typ, input) = get_type(input);

    (Field {
        field_name,
        byte_range,
        typ
    }, input)
}

fn get_field_identifier(mut input: IntoIter) -> (proc_macro2::Ident, IntoIter) {
    match input.next() {
        Some(Ident(ident)) => {
            (syn::Ident::new(&ident.to_string(), ident.span()), input)
        },
        _ => panic!("Expected a field identifier.")
    }
}

fn get_byte_range(mut input: IntoIter) -> (std::ops::Range<u64>, IntoIter, IntoIter) {
    // Match byterange group
    match input.next() {
        Some(Group(group)) => {
            // Match parenthesis of group
            match group.delimiter() {
                proc_macro2::Delimiter::Bracket => {
                    let tokens = group.stream().into_iter();
                    let tuple = get_range(tokens);
                    (tuple.0, input, tuple.1)
                },
                _ => panic!("Expected brace.")
            }
        },
        _ => panic!("Bad token.")
    }
}

fn get_number(mut input: IntoIter) -> (Option<u64>, IntoIter) {
    match input.next() {
        Some(Literal(literal)) => {
            (Some(literal.to_string().parse().unwrap()), input)
        },
        _ => (None, input)
    }
}

fn get_range_start(input: IntoIter) -> (u64, IntoIter) {
    if let (Some(number), input) = get_number(input) {
        (number, input)
    } else {
        panic!("Expected the end of a byterange.")
    }
}

fn expect_range_delimiter_or_end(mut input: IntoIter) -> (bool, IntoIter) {
    // Match first value of bitrange.
    let mut point = false;
    match input.next() {
        Some(Punct(literal)) => {
            if literal.as_char() == '.' {
                point = true;
            }
        },
        _ => {
            if input.next().is_some() {
                panic!("Expected no more characters.");
            }
        }
    }
    if point {
        match input.next() {
            Some(Punct(literal)) => {
                if literal.as_char() != '.' {
                    point = false;
                }
            },
            _ => {
                point = false;
            }
        }
    }
    if point {
        (true, input)
    } else {
        (false, input)
    }
}

fn expect_colon(mut input: IntoIter) -> IntoIter {
    match input.next() {
        Some(Punct(literal)) => {
            if literal.as_char() == ':' {
                return input;
            }
        },
        _ => ()
    };
    panic!(format!("Expected ':'."))
}

fn get_range_end(input: IntoIter) -> (u64, IntoIter) {
    if let (Some(number), input) = get_number(input) {
        (number, input)
    } else {
        panic!("Expected the end of a byterange.")
    }
}

fn get_range(input: IntoIter) -> (std::ops::Range<u64>, IntoIter) {
    let (byte_range_start, input) = get_range_start(input);

    let (has_end, input) = expect_range_delimiter_or_end(input);
    if has_end {
        let (byte_range_end, input) = get_range_end(input);
        (byte_range_start..byte_range_end, input)
    } else {
        (byte_range_start..byte_range_start, input)
    }
}

fn get_type(mut input: IntoIter) -> (proc_macro2::Ident, IntoIter) {
    match input.next() {
        Some(Ident(ident)) => {
            (syn::Ident::new(&ident.to_string(), ident.span()), input)
        },
        _ => panic!("Expected a type identifier.")
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
