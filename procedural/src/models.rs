use proc_macro2::Ident;

pub enum Abstractions {
    Assets,
    XCM,
    EVM,
    System,
    Governance,
    Consensus,
}

#[derive(Debug)]
pub enum ConversionError {
    UnknownAbstraction(Ident),
}

impl TryFrom<Ident> for Abstractions {
    type Error = ConversionError;
    fn try_from(value: Ident) -> Result<Self, Self::Error> {
        if value == "Assets" {
            Ok(Abstractions::Assets)
        } else if value == "XCM" {
            Ok(Abstractions::XCM)
        } else if value == "EVM" {
            Ok(Abstractions::EVM)
        } else if value == "System" {
            Ok(Abstractions::System)
        } else if value == "Governance" {
            Ok(Abstractions::Governance)
        } else if value == "Consensus" {
            Ok(Abstractions::Consensus)
        } else {
            Err(ConversionError::UnknownAbstraction(value))
        }
    }
}
