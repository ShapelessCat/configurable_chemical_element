use macro_util::generate_element_types;

generate_element_types!();

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: not done!
    #[test]
    fn test_element_types() {
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
