use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use std::ops::Index;
use syn::Ident;

#[proc_macro]
pub fn generate_element_types(_input: TokenStream) -> TokenStream {
    let json_data = include_str!("../periodic_table_info_sample.json");
    let elements: Vec<Element> = match serde_json::from_str(json_data) {
        Ok(elements) => elements,
        Err(e) => return compile_error(format!("Failed to parse element info JSON: {e}")),
    };
    if elements.is_empty() {
        return compile_error("No element information found in the provided JSON file.");
    }

    let enum_chem_elem_sym = build(&elements, ElementField::Symbol);
    let enum_chem_elem_name = build(&elements, ElementField::Name);

    let output = quote! {
        use serde as _serde;
        use pyo3 as _pyo3;

        use std::num::NonZeroU8;
        use std::sync::OnceLock;
        use std::{fs, mem};

        #enum_chem_elem_sym
        #enum_chem_elem_name

        impl From<ChemicalElementName> for ChemicalElementSymbol {
            fn from(name: ChemicalElementName) -> Self {
                unsafe { std::mem::transmute(name) }
            }
        }

        impl From<ChemicalElementSymbol> for ChemicalElementName {
            fn from(symbol: ChemicalElementSymbol) -> Self {
                unsafe { std::mem::transmute(symbol) }
            }
        }

        #[_pyo3::prelude::pyclass]
        #[derive(Debug, Clone, Copy, PartialEq, _serde::Serialize, _serde::Deserialize)]
        #[allow(non_snake_case)]
        pub struct ChemicalElement {
            pub symbol: ChemicalElementSymbol,
            pub name: ChemicalElementName,
            pub atomic_number: u8,
            #[cfg(feature = "atomic_mass")]
            pub atomic_mass: f32,
            #[cfg(feature = "period_and_group")]
            pub period: u8,
            #[cfg(feature = "period_and_group")]
            pub group: u8,
            #[cfg(feature = "van_der_Waals_radius")]
            pub van_der_Waals_radius: Option<f32>,
            #[cfg(feature = "covalent_radius")]
            pub covalent_radius: Option<f32>,
            #[cfg(feature = "metallic_radius")]
            pub metallic_radius: Option<f32>,
        }

        impl ChemicalElement {
            pub fn all_elements() -> &'static Vec<ChemicalElement> {
                static MEM: OnceLock<Vec<ChemicalElement>> = OnceLock::new();
                MEM.get_or_init(||
                    serde_json::from_str(&#json_data).expect("Failed to parse JSON")
                )
            }
        }

        #[_pyo3::prelude::pymethods]
        impl ChemicalElement {
            fn __repr__(&self) -> String {
                format!("ChemicalElement.{}", self.symbol)
            }

            fn __str__(&self) -> String {
                format!("ChemicalElement.{}", self.name)
            }

            #[staticmethod]
            fn from_atomic_number(atomic_number: u8) -> _pyo3::prelude::PyResult<Self> {
                match atomic_number.try_into() {
                    Ok(element) => _pyo3::prelude::PyResult::Ok(element),
                    Err(err) => _pyo3::prelude::PyResult::Err(pyo3::exceptions::PyValueError::new_err(err)),
                }
            }

            #[staticmethod]
            fn from_symbol(symbol: ChemicalElementSymbol) -> _pyo3::prelude::PyResult<Self> {
                _pyo3::prelude::PyResult::Ok(symbol.into())
            }

            #[staticmethod]
            fn from_name(name: ChemicalElementName) -> _pyo3::prelude::PyResult<Self> {
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
                    let elements = ChemicalElement::all_elements();
                    let index = (atomic_number - 1) as usize;
                    Ok(elements[index])
                }
            }
        }

        impl From<ChemicalElementSymbol> for ChemicalElement {
            fn from(symbol: ChemicalElementSymbol) -> Self {
                let elements = ChemicalElement::all_elements();
                let atomic_number: NonZeroU8 = unsafe { mem::transmute(symbol) };
                let index = (atomic_number.get() - 1) as usize;
                elements[index as usize]
            }
        }

        impl From<ChemicalElementName> for ChemicalElement {
            fn from(name: ChemicalElementName) -> Self {
                let elements = ChemicalElement::all_elements();
                let atomic_number: NonZeroU8 = unsafe { mem::transmute(name) };
                let index = (atomic_number.get() - 1) as usize;
                elements[index as usize]
            }
        }
    };

    output.into()
}

fn build(elements: &[Element], element_field: ElementField) -> proc_macro2::TokenStream {
    let variants = elements.iter().enumerate().map(|(idx, e)| {
        let variant = Ident::new(&e[element_field], proc_macro2::Span::call_site());
        if idx == 0 {
            quote! { #variant = 1 }
        } else {
            quote! { #variant }
        }
    });
    let enum_name = element_field.type_name();
    let field_name = element_field.field_name();
    quote! {
        #[_pyo3::prelude::pyclass]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, _serde::Serialize, _serde::Deserialize)]
        pub enum #enum_name {
            #(#variants),*
        }

        impl std::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }

        impl TryFrom<u8> for #enum_name {
            type Error = String;

            fn try_from(atomic_number_candidate: u8) -> Result<Self, Self::Error> {
                if atomic_number_candidate == 0 || atomic_number_candidate > 118 {
                    Err(format!("Invalid atomic number: {}", atomic_number_candidate))
                } else {
                    let atomic_number = atomic_number_candidate;
                    let elements = ChemicalElement::all_elements();
                    let index = (atomic_number - 1) as usize;
                    Ok(elements[index].#field_name)
                }
            }
        }
        #[_pyo3::prelude::pymethods]
        impl #enum_name {
            #[staticmethod]
            fn from_atomic_number(atomic_number: u8) -> _pyo3::prelude::PyResult<Self> {
                match atomic_number.try_into() {
                    Ok(element) => _pyo3::prelude::PyResult::Ok(element),
                    Err(err) => _pyo3::prelude::PyResult::Err(pyo3::exceptions::PyValueError::new_err(err)),
                }
            }

            #[staticmethod]
            fn from_symbol(symbol: ChemicalElementSymbol) -> _pyo3::prelude::PyResult<Self> {
                _pyo3::prelude::PyResult::Ok(symbol.into())
            }

            #[staticmethod]
            fn from_name(name: ChemicalElementName) -> _pyo3::prelude::PyResult<Self> {
                _pyo3::prelude::PyResult::Ok(name.into())
            }
        }
    }
}

fn compile_error<T: ToTokens>(message: T) -> TokenStream {
    quote! {
        compile_error!(#message);
    }
    .into()
}

#[derive(serde::Deserialize)]
struct Element {
    symbol: String,
    name: String,
}

impl Index<ElementField> for Element {
    type Output = String;

    fn index(&self, index: ElementField) -> &Self::Output {
        match index {
            ElementField::Symbol => &self.symbol,
            ElementField::Name => &self.name,
        }
    }
}

#[derive(Clone, Copy)]
enum ElementField {
    Symbol,
    Name,
}

impl ElementField {
    fn type_name(&self) -> Ident {
        let name = match self {
            ElementField::Symbol => "ChemicalElementSymbol",
            ElementField::Name => "ChemicalElementName",
        };
        Ident::new(name, proc_macro2::Span::call_site())
    }

    fn field_name(&self) -> Ident {
        let name = match self {
            ElementField::Symbol => "symbol",
            ElementField::Name => "name",
        };
        Ident::new(name, proc_macro2::Span::call_site())
    }
}
