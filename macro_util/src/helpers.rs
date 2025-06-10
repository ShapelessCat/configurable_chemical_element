use crate::internal_types::{ChemicalElement, ElementField};
use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

pub fn build_all_elements_const(
    all_elements: &[ChemicalElement],
    all_elements_const_name: &Ident,
) -> proc_macro2::TokenStream {
    let number_of_elements = all_elements.len();
    let elements = all_elements.iter().map(|e| {
        let symbol_variant = Ident::new(&e.symbol, proc_macro2::Span::call_site());
        let name_variant = Ident::new(&e.name, proc_macro2::Span::call_site());
        let atomic_number = e.atomic_number;
        let atomic_mass = e.atomic_mass;
        let period = e.period;
        let group = e.group;
        let van_der_waals_radius = option2token_stream2(e.van_der_Waals_radius);
        let covalent_radius = option2token_stream2(e.covalent_radius);
        let metallic_radius = option2token_stream2(e.metallic_radius);
        quote! {
            ChemicalElement {
                symbol: ChemicalElementSymbol::#symbol_variant,
                name: ChemicalElementName::#name_variant,
                atomic_number: #atomic_number,
                #[cfg(feature = "atomic_mass")]
                atomic_mass: #atomic_mass,
                #[cfg(feature = "period_and_group")]
                period: #period,
                #[cfg(feature = "period_and_group")]
                group: #group,
                #[cfg(feature = "van_der_Waals_radius")]
                van_der_Waals_radius: #van_der_waals_radius,
                #[cfg(feature = "covalent_radius")]
                covalent_radius: #covalent_radius,
                #[cfg(feature = "metallic_radius")]
                metallic_radius: #metallic_radius,
            }
        }
    });
    quote! {
        const #all_elements_const_name: [ChemicalElement; #number_of_elements] = [
            #(#elements,)*
        ];
    }
}

pub fn build_enum(
    elements: &[ChemicalElement],
    element_field_type: ElementField,
    all_elements_const_name: &Ident,
) -> proc_macro2::TokenStream {
    let variants = elements.iter().enumerate().map(|(idx, e)| {
        let v = Ident::new(&e[element_field_type], proc_macro2::Span::call_site());
        if idx == 0 {
            quote! { #v = 1 }
        } else {
            quote! { #v }
        }
    });
    let enum_name = element_field_type.type_name();
    let companion_enum_name = element_field_type.companion().type_name();
    let field_name = element_field_type.field_name();
    let companion_field_name = element_field_type.companion().field_name();

    let try_from_branches = elements.iter().map(|e| {
        let v = &e[element_field_type];
        let v_variant = Ident::new(v, proc_macro2::Span::call_site());
        quote! { #v => Ok(Self :: #v_variant) }
    });

    quote! {
        #[_pyo3::prelude::pyclass]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, _serde::Serialize, _serde::Deserialize)]
        pub enum #enum_name {
            #(#variants,)*
        }

        impl std::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }

        impl From<#companion_enum_name> for #enum_name {
            fn from(#companion_field_name: #companion_enum_name) -> Self {
                unsafe { std::mem::transmute(#companion_field_name) }
            }
        }

        impl TryFrom<u8> for #enum_name {
            type Error = String;

            fn try_from(atomic_number_candidate: u8) -> Result<Self, Self::Error> {
                if atomic_number_candidate == 0 || atomic_number_candidate > 118 {
                    Err(format!("Invalid atomic number: {}", atomic_number_candidate))
                } else {
                    let atomic_number = atomic_number_candidate;
                    let index = (atomic_number - 1) as usize;
                    Ok(#all_elements_const_name[index].#field_name)
                }
            }
        }

        impl<'a> TryFrom<&'a str> for #enum_name {
            type Error = String;

            fn try_from(value: &'a str) -> Result<Self, String> {
                match value {
                    #(#try_from_branches,)*
                    _ => Err(format!("Can't build a {} value from {}", stringify!(#enum_name), value)),
                }
            }
        }

        impl TryFrom<String> for #enum_name {
            type Error = String;

            fn try_from(value: String) -> Result<Self, String> {
                value.as_str().try_into()
            }
        }

        #[_pyo3::prelude::pymethods]
        impl #enum_name {
            #[new]
            fn new(value: String) -> _pyo3::prelude::PyResult<Self> {
                match value.as_str().try_into() {
                    Ok(v) => _pyo3::prelude::PyResult::Ok(v),
                    Err(e) => _pyo3::prelude::PyResult::Err(
                        pyo3::exceptions::PyValueError::new_err(
                            format!("Failed to build a {} value from \"{}\"", stringify!(#enum_name), value)
                        )
                    ),
                }
            }

            #[classmethod]
            fn from_atomic_number(
                cls: &_pyo3::prelude::Bound<'_, _pyo3::types::PyType>,
                atomic_number: u8
            ) -> _pyo3::prelude::PyResult<Self> {
                match atomic_number.try_into() {
                    Ok(ele) => _pyo3::prelude::PyResult::Ok(ele),
                    Err(e) => _pyo3::prelude::PyResult::Err(pyo3::exceptions::PyValueError::new_err(e)),
                }
            }

            #[classmethod]
            fn from_symbol(
                cls: &_pyo3::prelude::Bound<'_, _pyo3::types::PyType>,
                symbol: ChemicalElementSymbol
            ) -> _pyo3::prelude::PyResult<Self> {
                _pyo3::prelude::PyResult::Ok(symbol.into())
            }

            #[classmethod]
            fn from_name(
                cls: &_pyo3::prelude::Bound<'_, _pyo3::types::PyType>,
                name: ChemicalElementName
            ) -> _pyo3::prelude::PyResult<Self> {
                _pyo3::prelude::PyResult::Ok(name.into())
            }
        }
    }
}

pub fn build_chemical_element(all_elements_const_name: &Ident) -> proc_macro2::TokenStream {
    quote! {
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
                format!("{}", self.name)
            }

            #[classmethod]
            fn from_atomic_number(
                cls: &_pyo3::prelude::Bound<'_, _pyo3::types::PyType>,
                atomic_number: u8
            ) -> _pyo3::prelude::PyResult<Self> {
                match atomic_number.try_into() {
                    Ok(element) => _pyo3::prelude::PyResult::Ok(element),
                    Err(err) => _pyo3::prelude::PyResult::Err(pyo3::exceptions::PyValueError::new_err(err)),
                }
            }

            #[classmethod]
            fn from_symbol(
                cls: &_pyo3::prelude::Bound<'_, _pyo3::types::PyType>,
                symbol: ChemicalElementSymbol
            ) -> _pyo3::prelude::PyResult<Self> {
                _pyo3::prelude::PyResult::Ok(symbol.into())
            }

            #[classmethod]
            fn from_name(
                cls: &_pyo3::prelude::Bound<'_, _pyo3::types::PyType>,
                name: ChemicalElementName
            ) -> _pyo3::prelude::PyResult<Self> {
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
    }
}

pub fn compile_error<T: ToTokens>(message: T) -> TokenStream {
    quote! {
        compile_error!(#message);
    }
    .into()
}

fn option2token_stream2<T: ToTokens>(maybe_v: Option<T>) -> proc_macro2::TokenStream {
    match maybe_v {
        Some(v) => quote! { Some(#v) },
        None => quote! { None },
    }
}
