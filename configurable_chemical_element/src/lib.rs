mod element_types;

pub use element_types::{ChemicalElement, ChemicalElementName, ChemicalElementSymbol};

use pyo3::prelude::*;

#[pymodule]
fn configurable_chemical_element(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ChemicalElementName>()?;
    m.add_class::<ChemicalElementSymbol>()?;
    m.add_class::<ChemicalElement>()
}
