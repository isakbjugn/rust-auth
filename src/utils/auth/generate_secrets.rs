use pasetors::keys::{AsymmetricKeyPair, Generate, SymmetricKey};
use pasetors::paserk::FormatAsPaserk;
use pasetors::version4::V4;
use crate::utils::AppError;

pub fn _generate_symmetric_key() -> Result<String, AppError> {
    // Generate the key and serialize to and from PASERK.
    let sk = SymmetricKey::<V4>::generate()?;
    let mut paserk = String::new();
    sk.fmt(&mut paserk).unwrap();
    Ok(paserk)
}

pub fn _generate_asymmetric_key_pair() -> Result<(String, String), AppError> {
    // Generate the key and serialize to and from PASERK.
    let sk = AsymmetricKeyPair::<V4>::generate()?;
    let mut secret_paserk = String::new();
    let mut public_paserk = String::new();
    sk.secret.fmt(&mut secret_paserk).unwrap();
    sk.public.fmt(&mut public_paserk).unwrap();
    Ok((secret_paserk, public_paserk))
}
