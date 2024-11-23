use anchor_client::solana_sdk::pubkey::Pubkey;
use solana_sdk::pubkey;

lazy_static! {
    pub static ref TOKEN_PROGRAM_ID: Pubkey =
        pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    pub static ref ASSOCIATED_TOKEN_PROGRAM_ID: Pubkey =
        pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
    pub static ref ORCA_PROGRAM_ID: Pubkey =
        pubkey!("9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP");
    pub static ref MERCURIAL_PROGRAM_ID: Pubkey =
        pubkey!("MERLuDFBMmsHnsBPZw2sDQZHvXFMwp8EdjudcU2HKky");
    pub static ref ARB_PROGRAM_ID: Pubkey = pubkey!("CRQXfRGq3wTkjt7JkqhojPLiKLYLjHPGLebnfiiQB46T");
    pub static ref SABER_PROGRAM_ID: Pubkey =
        pubkey!("SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ");
    pub static ref ALDRIN_V1_PROGRAM_ID: Pubkey =
        pubkey!("AMM55ShdkoGRB5jVYPjWziwk8m5MpwyDgsMWHaMSQWH6");
    pub static ref ALDRIN_V2_PROGRAM_ID: Pubkey =
        pubkey!("CURVGoZn8zycx6FXwwevgBTB2gVvdbGTEpvMJDbgs2t4");
    pub static ref SERUM_PROGRAM_ID: Pubkey =
        pubkey!("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin");
}
