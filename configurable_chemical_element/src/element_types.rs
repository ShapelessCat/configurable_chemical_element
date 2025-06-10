use macro_util::generate_element_types;

generate_element_types!();

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: not done!
    #[test]
    fn test_element_types() {
        let json = r#"
        [
  {
    "symbol": "H",
    "name": "Hydrogen",
    "atomic_number": 1,
    "atomic_mass": 1.008,
    "period": 1,
    "group": 1,
    "van_der_Waals_radius": 1.2,
    "covalent_radius": 0.31,
    "metallic_radius": 0.2
  },
  {
    "symbol": "He",
    "name": "Helium",
    "atomic_number": 2,
    "atomic_mass": 4.002602,
    "period": 1,
    "group": 18,
    "van_der_Waals_radius": 1.4,
    "covalent_radius": 0.28,
    "metallic_radius": 0.2
  },
  {
    "symbol": "Li",
    "name": "Lithium",
    "atomic_number": 3,
    "atomic_mass": 6.94,
    "period": 2,
    "group": 1,
    "van_der_Waals_radius": 1.82,
    "covalent_radius": 1.28,
    "metallic_radius": 1.52
  }
]
        "#;

        let r: Vec<serde_json::Value> = serde_json::from_str(json).unwrap();
        for v in r {
            v.as_object()
                .unwrap()
                .iter()
                .filter(|(k, _)| *k == "name")
                .for_each(|(k, v)| {
                    println!("{}: {}", k, v);
                });
            let x = serde_json::from_value::<ChemicalElement>(v).unwrap();
            println!("{:?}", x);
        }

        for atomic_number in 0..=3 {
            let e: Result<ChemicalElement, String> = atomic_number.try_into();
            println!("{:?}", e);
        }

        for i in 1u8..=3 {
            let e: Result<ChemicalElementName, String> =
                std::num::NonZeroU8::new(i).unwrap().get().try_into();
            println!("{:?}", e);
            let e: ChemicalElement = e.unwrap().into();
            // let index: u8 = unsafe { mem::transmute(e) };
            // let atomic_number = index + 1u8;
            // let e: ChemicalElement = atomic_number.into();
            println!("{:?}", e);
        }
    }
}
