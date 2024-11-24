use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug)]
pub struct ClmmData {
    pub describer: u64,
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit_x64: u128,
    pub is_base_input: bool,
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_deserialize() {
        let s =
            "f8c69e91e17587c8504147080000000000000000000000000000000000000000000000000000000001";

        let mut b = (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect::<Vec<_>>();

        let x = ClmmData::deserialize(&mut b.as_slice()).expect("TODO: panic message");
        dbg!(x);
    }
}
