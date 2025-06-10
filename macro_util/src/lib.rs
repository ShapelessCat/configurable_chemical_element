mod helpers;
mod types;

use crate::helpers::{build_all_elements_const, build_enum, compile_error};
use crate::types::{ChemicalElement, ElementField};
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
    let all_elements_const_name = Ident::new("ALL_ELEMENTS", proc_macro2::Span::call_site());

    let all_elements_const = build_all_elements_const(&all_elements, &all_elements_const_name);
    let enum_chem_elem_sym = build_enum(
        &all_elements,
        ElementField::Symbol,
        &all_elements_const_name,
    );
    let enum_chem_elem_name =
        build_enum(&all_elements, ElementField::Name, &all_elements_const_name);

    let output = quote! {
        use serde as _serde;
        use pyo3 as _pyo3;
        use std as _std;

        #all_elements_const
        #enum_chem_elem_sym
        #enum_chem_elem_name

        #[_pyo3::prelude::pyclass(frozen)]
        #[derive(Debug, Clone, Copy, PartialEq, _serde::Serialize, _serde::Deserialize)]
        #[allow(non_snake_case)]
        pub struct ChemicalElement {
            #[pyo3(get)]
            pub symbol: ChemicalElementSymbol,
            #[pyo3(get)]
            pub name: ChemicalElementName,
            #[pyo3(get)]
            pub atomic_number: u8,
            #[cfg(feature = "atomic_mass")]
            #[pyo3(get)]
            pub atomic_mass: f32,
            #[cfg(feature = "period_and_group")]
            #[pyo3(get)]
            pub period: u8,
            #[cfg(feature = "period_and_group")]
            #[pyo3(get)]
            pub group: u8,
            #[cfg(feature = "van_der_Waals_radius")]
            #[pyo3(get)]
            pub van_der_Waals_radius: Option<f32>,
            #[cfg(feature = "covalent_radius")]
            #[pyo3(get)]
            pub covalent_radius: Option<f32>,
            #[cfg(feature = "metallic_radius")]
            #[pyo3(get)]
            pub metallic_radius: Option<f32>,
        }

        #[_pyo3::prelude::pymethods]
        impl ChemicalElement {
            fn __repr__(&self) -> String {
                format!("ChemicalElement.{}", self.symbol)
            }

            fn __str__(&self) -> String {
                format!("ChemicalElement.{}", self.name)
            }

            #[classmethod]
            fn from_atomic_number(cls: &_pyo3::prelude::Bound<'_, _pyo3::types::PyType>, atomic_number: u8) -> _pyo3::prelude::PyResult<Self> {
                match atomic_number.try_into() {
                    Ok(element) => _pyo3::prelude::PyResult::Ok(element),
                    Err(err) => _pyo3::prelude::PyResult::Err(pyo3::exceptions::PyValueError::new_err(err)),
                }
            }

            #[classmethod]
            fn from_symbol(cls: &_pyo3::prelude::Bound<'_, _pyo3::types::PyType>, symbol: ChemicalElementSymbol) -> _pyo3::prelude::PyResult<Self> {
                _pyo3::prelude::PyResult::Ok(symbol.into())
            }

            #[classmethod]
            fn from_name(cls: &_pyo3::prelude::Bound<'_, _pyo3::types::PyType>, name: ChemicalElementName) -> _pyo3::prelude::PyResult<Self> {
                _pyo3::prelude::PyResult::Ok(name.into())
            }
        }

        impl TryFrom<u8> for ChemicalElement {
            type Error = String;

            fn try_from(atomic_number_candidate: u8) -> Result<Self, Self::Error> {
                if atomic_number_candidate == 0 || atomic_number_candidate > 118 {
                    Err(format!("Invalid atomic number: {}", atomic_number_candidate))
                } else {
                    let atomic_number = atomic_number_candidate;
                    let index = (atomic_number - 1) as usize;
                    Ok(#all_elements_const_name[index])
                }
            }
        }

        impl From<ChemicalElementSymbol> for ChemicalElement {
            fn from(symbol: ChemicalElementSymbol) -> Self {
                let atomic_number: _std::num::NonZeroU8 = unsafe { _std::mem::transmute(symbol) };
                let index = (atomic_number.get() - 1) as usize;
                #all_elements_const_name[index]
            }
        }

        impl From<ChemicalElementName> for ChemicalElement {
            fn from(name: ChemicalElementName) -> Self {
                let atomic_number: _std::num::NonZeroU8 = unsafe { _std::mem::transmute(name) };
                let index = (atomic_number.get() - 1) as usize;
                #all_elements_const_name[index]
            }
        }
    };

    output.into()
}
