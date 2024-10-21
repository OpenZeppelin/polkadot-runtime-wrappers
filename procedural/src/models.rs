use proc_macro2::Ident;
use syn::ItemStruct;

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
    UnknownAbstraction,
    NoAbstractionAttribute,
}

impl TryFrom<ItemStruct> for Abstractions {
    type Error = ConversionError;
    fn try_from(value: ItemStruct) -> Result<Self, Self::Error> {
        let is_pallet = value.attrs.iter().any(|f| {
            let Ok(path) = f.meta.require_path_only() else {
                return false;
            };
            let Ok(ident) = path.require_ident() else {
                return false;
            };
            ident == "abstraction"
        });
        if !is_pallet {
            return Err(ConversionError::NoAbstractionAttribute);
        }

        Abstractions::try_from(value.ident)
    }
}

impl TryFrom<Ident> for Abstractions {
    type Error = ConversionError;
    fn try_from(value: Ident) -> Result<Self, Self::Error> {
        if "Assets".eq_ignore_ascii_case(&value.to_string()) {
            Ok(Abstractions::Assets)
        } else if "XCM".eq_ignore_ascii_case(&value.to_string()) {
            Ok(Abstractions::XCM)
        } else if "EVM".eq_ignore_ascii_case(&value.to_string()) {
            Ok(Abstractions::EVM)
        } else if "System".eq_ignore_ascii_case(&value.to_string()) {
            Ok(Abstractions::System)
        } else if "Governance".eq_ignore_ascii_case(&value.to_string()) {
            Ok(Abstractions::Governance)
        } else if "Consensus".eq_ignore_ascii_case(&value.to_string()) {
            Ok(Abstractions::Consensus)
        } else {
            Err(ConversionError::UnknownAbstraction)
        }
    }
}
