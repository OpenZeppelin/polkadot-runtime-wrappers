use proc_macro2::Ident;
use syn::ItemStruct;

pub enum ConstructAbstractions {
    Assets,
    Xcm,
    Evm,
    System,
    Governance,
    Consensus,
}

#[derive(Debug)]
pub enum ConversionError {
    UnknownAbstraction,
    NoAbstractionAttribute,
}

impl TryFrom<ItemStruct> for ConstructAbstractions {
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

        ConstructAbstractions::try_from(value.ident)
    }
}

impl TryFrom<Ident> for ConstructAbstractions {
    type Error = ConversionError;
    fn try_from(value: Ident) -> Result<Self, Self::Error> {
        if "Assets".eq_ignore_ascii_case(&value.to_string()) {
            Ok(ConstructAbstractions::Assets)
        } else if "XCM".eq_ignore_ascii_case(&value.to_string()) {
            Ok(ConstructAbstractions::Xcm)
        } else if "EVM".eq_ignore_ascii_case(&value.to_string()) {
            Ok(ConstructAbstractions::Evm)
        } else if "System".eq_ignore_ascii_case(&value.to_string()) {
            Ok(ConstructAbstractions::System)
        } else if "Governance".eq_ignore_ascii_case(&value.to_string()) {
            Ok(ConstructAbstractions::Governance)
        } else if "Consensus".eq_ignore_ascii_case(&value.to_string()) {
            Ok(ConstructAbstractions::Consensus)
        } else {
            Err(ConversionError::UnknownAbstraction)
        }
    }
}

pub enum APIAbstractions {
    Benchmarks,
    System,
    Evm,
    Consensus,
    Assets,
}

impl TryFrom<Ident> for APIAbstractions {
    type Error = ConversionError;
    fn try_from(value: Ident) -> Result<Self, Self::Error> {
        if "Benchmarks".eq_ignore_ascii_case(&value.to_string()) {
            Ok(APIAbstractions::Benchmarks)
        } else if "Assets".eq_ignore_ascii_case(&value.to_string()) {
            Ok(APIAbstractions::Assets)
        } else if "EVM".eq_ignore_ascii_case(&value.to_string()) {
            Ok(APIAbstractions::Evm)
        } else if "System".eq_ignore_ascii_case(&value.to_string()) {
            Ok(APIAbstractions::System)
        } else if "Consensus".eq_ignore_ascii_case(&value.to_string()) {
            Ok(APIAbstractions::Consensus)
        } else {
            Err(ConversionError::UnknownAbstraction)
        }
    }
}
