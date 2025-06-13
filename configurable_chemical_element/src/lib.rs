mod element_types;

#[cfg(feature = "atomic_mass")]
pub use element_types::atomic_mass_of;
#[cfg(feature = "covalent_radius")]
pub use element_types::covalent_radius_of;
#[cfg(feature = "period_and_group")]
pub use element_types::group_of;
#[cfg(feature = "metallic_radius")]
pub use element_types::metallic_radius_of;
#[cfg(feature = "period_and_group")]
pub use element_types::period_of;
#[cfg(feature = "van_der_Waals_radius")]
pub use element_types::van_der_waals_radius_of;
pub use element_types::{ChemicalElement, ChemicalElementName, ChemicalElementSymbol};

use pyo3::prelude::*;

#[pymodule]
fn configurable_chemical_element(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ChemicalElementName>()?;
    m.add_class::<ChemicalElementSymbol>()?;
    m.add_class::<ChemicalElement>()?;
    #[cfg(feature = "atomic_mass")]
    let _ = m.add_function(wrap_pyfunction!(atomic_mass_of, m)?);
    #[cfg(feature = "period_and_group")]
    let _ = m.add_function(wrap_pyfunction!(period_of, m)?);
    #[cfg(feature = "period_and_group")]
    let _ = m.add_function(wrap_pyfunction!(group_of, m)?);
    #[cfg(feature = "van_der_Waals_radius")]
    let _ = m.add_function(wrap_pyfunction!(van_der_waals_radius_of, m)?);
    #[cfg(feature = "covalent_radius")]
    let _ = m.add_function(wrap_pyfunction!(covalent_radius_of, m)?);
    #[cfg(feature = "metallic_radius")]
    let _ = m.add_function(wrap_pyfunction!(metallic_radius_of, m)?);
    Ok(())
}
