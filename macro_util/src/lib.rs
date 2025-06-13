mod helpers;
mod internal_types;

use crate::helpers::{build_all_elements_const, build_chemical_element, build_conditional_consts, build_enum, compile_error};
use crate::internal_types::{ChemicalElement, ElementField};
use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

#[proc_macro]
pub fn generate_element_types(_input: TokenStream) -> TokenStream {
    let json_data = include_str!("../periodic_table_info_sample.json");
    let all_elements: Vec<ChemicalElement> = match serde_json::from_str(json_data) {
        Ok(elements) => elements,
        Err(e) => return compile_error(format!("Failed to parse element info JSON: {e}")),
    };
    if all_elements.is_empty() {
        return compile_error("No element information found in the provided JSON file.");
    }

    let conditional_constants_constants = build_conditional_consts(&all_elements);
    let all_elements_const_name = Ident::new("ALL_ELEMENTS", proc_macro2::Span::call_site());

    let all_elements_const = build_all_elements_const(&all_elements, &all_elements_const_name);
    let chemical_element_symbol = build_enum(
        &all_elements,
        ElementField::Symbol,
        &all_elements_const_name,
    );
    let chemical_element_name =
        build_enum(&all_elements, ElementField::Name, &all_elements_const_name);
    let chemical_element = build_chemical_element(&all_elements_const_name);

    let output = quote! {
        use serde as _serde;
        use pyo3 as _pyo3;
        use std as _std;

        #conditional_constants_constants

        #all_elements_const

        #chemical_element_symbol

        #chemical_element_name

        #chemical_element
    };

    output.into()
}
