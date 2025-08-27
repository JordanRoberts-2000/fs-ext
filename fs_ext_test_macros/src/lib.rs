use {
    proc_macro::TokenStream,
    quote::quote,
    std::collections::HashSet,
    syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, Ident, ItemFn, Token},
};

struct Args {
    idents: Punctuated<Ident, Token![,]>,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            Ok(Self { idents: Punctuated::new() })
        } else {
            Ok(Self { idents: Punctuated::parse_separated_nonempty(input)? })
        }
    }
}

#[proc_macro_attribute]
pub fn fs_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let fn_name = func.sig.ident.clone();

    // Parse test names from the attribute arguments
    let Args { idents } = parse_macro_input!(attr as Args);

    // Collect unique test identifiers
    let mut test_names = HashSet::new();
    for ident in idents {
        test_names.insert(ident.to_string());
    }

    // Define the default test names and their corresponding test functions
    let default_tests = [
        (
            "rejects_missing_path",
            quote! {
                crate::test_utils::assert_fn_rejects_missing_path(|path| {
                    super::#fn_name(path)
                })
            },
        ),
        (
            "rejects_file",
            quote! {
                crate::test_utils::assert_fn_rejects_file_path(|path| {
                    super::#fn_name(path)
                })
            },
        ),
        (
            "rejects_existing_file",
            quote! {
                crate::test_utils::assert_fn_rejects_existing_file(|path| {
                    super::#fn_name(path)
                })
            },
        ),
        (
            "rejects_dir",
            quote! {
                crate::test_utils::assert_fn_rejects_dir_path(|path| {
                    super::#fn_name(path)
                })
            },
        ),
        (
            "rejects_existing_dir",
            quote! {
                crate::test_utils::assert_fn_rejects_existing_dir(|path| {
                    super::#fn_name(path)
                })
            },
        ),
        (
            "existing_dir_ok",
            quote! {
                crate::test_utils::existing_dir_ok(|path| {
                    super::#fn_name(path)
                })
            },
        ),
        (
            "existing_file_ok",
            quote! {
                crate::test_utils::existing_file_ok(|path| {
                    super::#fn_name(path)
                })
            },
        ),
        (
            "new_dir_ok",
            quote! {
                crate::test_utils::new_dir_ok(|path| {
                    super::#fn_name(path)
                })
            },
        ),
        (
            "new_file_ok",
            quote! {
                crate::test_utils::new_file_ok(|path| {
                    super::#fn_name(path)
                })
            },
        ),
    ];

    // Generate tests based on what was requested
    let tests_to_generate = if test_names.is_empty() {
        // If no tests specified, generate nothing
        Vec::new()
    } else {
        // Only generate tests that were explicitly requested
        default_tests.iter().filter(|(name, _)| test_names.contains(*name)).collect()
    };

    // Generate the test functions
    let test_functions = tests_to_generate.iter().map(|(name, test_body)| {
        let test_fn_name = Ident::new(name, fn_name.span());
        quote! {
            #[test]
            fn #test_fn_name() -> io::Result<()> {
                #test_body
            }
        }
    });

    let out = if tests_to_generate.is_empty() {
        // If no tests to generate, just return the original function
        quote! { #func }
    } else {
        // Generate the test module with the requested tests
        let mod_name = Ident::new("__macro_tests", proc_macro2::Span::call_site());

        quote! {
            #func

            #[cfg(test)]
            mod #mod_name {
                use super::*;
                use std::io;

                #(#test_functions)*
            }
        }
    };

    out.into()
}
