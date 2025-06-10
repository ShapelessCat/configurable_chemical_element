use std::ops::Index;

use serde::Deserialize;
use syn::Ident;

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct ChemicalElement {
    pub symbol: String,
    pub name: String,
    pub atomic_number: u8,
    pub atomic_mass: f32,
    pub period: u8,
    pub group: u8,
    pub van_der_Waals_radius: Option<f32>,
    pub covalent_radius: Option<f32>,
    pub metallic_radius: Option<f32>,
}

impl Index<ElementField> for ChemicalElement {
    type Output = String;

    fn index(&self, index: ElementField) -> &Self::Output {
        match index {
            ElementField::Symbol => &self.symbol,
            ElementField::Name => &self.name,
        }
    }
}

#[derive(Clone, Copy)]
pub enum ElementField {
    Symbol,
    Name,
}

impl ElementField {
    pub fn type_name(&self) -> Ident {
        let name = match self {
            ElementField::Symbol => "ChemicalElementSymbol",
            ElementField::Name => "ChemicalElementName",
        };
        Ident::new(name, proc_macro2::Span::call_site())
    }

    pub fn field_name(&self) -> Ident {
        let name = match self {
            ElementField::Symbol => "symbol",
            ElementField::Name => "name",
        };
        Ident::new(name, proc_macro2::Span::call_site())
    }

    pub fn companion(&self) -> ElementField {
        match self {
            ElementField::Symbol => ElementField::Name,
            ElementField::Name => ElementField::Symbol,
        }
    }
}
