
/*

// --------------------- MORPHOLOGY Enums ------------------------

/// Cape Shape Enum
#[derive(EnumIter, Debug, PartialEq, Default, Clone)]
pub enum CapShapeEnum {
    #[default]
    Convex,
    NotEdible,
    Poisonous,
    Medicinal,
    Unknown,
}
(ex: convex, flat, umbonate, bell-shaped, custom, etc.) 
impl IntoEntryEnumerator for CapShapeEnum {
    fn to_str(&self) -> &str {
        match self {

        }
    }
}

/// Cap Surface Enum
#[derive(EnumIter, Debug, PartialEq, Default, Clone)]
pub enum CapSurfaceEnum {
    #[default]
    Edible,
    NotEdible,
    Poisonous,
    Medicinal,
    Unknown,
}
impl IntoEntryEnumerator for CapSurfaceEnum {
    fn to_str(&self) -> &str {
        match self {

        }
    }
}

/// Dispersion Enum
#[derive(EnumIter, Debug, PartialEq, Default, Clone)]
pub enum DispersionEnum {
    #[default]
    Edible,
    NotEdible,
    Poisonous,
    Medicinal,
    Unknown,
}
impl IntoEntryEnumerator for DispersionEnum {
    fn to_str(&self) -> &str {
        match self {

        }
    }
}


/// Attachment Enum
#[derive(EnumIter, Debug, PartialEq, Default, Clone)]
pub enum AttachmentEnum {
    #[default]
    Edible,
    NotEdible,
    Poisonous,
    Medicinal,
    Unknown,
}
impl IntoEntryEnumerator for AttachmentEnum {
    fn to_str(&self) -> &str {
        match self {

        }
    }
}

/// Spacing Enum
#[derive(EnumIter, Debug, PartialEq, Default, Clone)]
pub enum SpacingEnum {
    #[default]
    Edible,
    NotEdible,
    Poisonous,
    Medicinal,
    Unknown,
}
impl IntoEntryEnumerator for SpacingEnum {
    fn to_str(&self) -> &str {
        match self {

        }
    }
}

/// Stem Surface Enum
#[derive(EnumIter, Debug, PartialEq, Default, Clone)]
pub enum StemSurfaceEnum {
    #[default]
    Edible,
    NotEdible,
    Poisonous,
    Medicinal,
    Unknown,
}
impl IntoEntryEnumerator for StemSurfaceEnum {
    fn to_str(&self) -> &str {
        match self {

        }
    }
}

/// Hollow Solid Enum
#[derive(EnumIter, Debug, PartialEq, Default, Clone)]
pub enum HollowSolidEnum {
    #[default]
    Edible,
    NotEdible,
    Poisonous,
    Medicinal,
    Unknown,
}
impl IntoEntryEnumerator for HollowSolidEnum {
    fn to_str(&self) -> &str {
        match self {

        }
    }
}

/// Ting Enum
#[derive(EnumIter, Debug, PartialEq, Default, Clone)]
pub enum RingEnum {
    #[default]
    Edible,
    NotEdible,
    Poisonous,
    Medicinal,
    Unknown,
}
impl IntoEntryEnumerator for RingEnum {
    fn to_str(&self) -> &str {
        match self {

        }
    }
}

/// Print Method Enum
#[derive(EnumIter, Debug, PartialEq, Default, Clone)]
pub enum PrintMethodEnum {
    #[default]
    Edible,
    NotEdible,
    Poisonous,
    Medicinal,
    Unknown,
}
impl IntoEntryEnumerator for PrintMethodEnum {
    fn to_str(&self) -> &str {
        match self {

        }
    }
}
















// --------------------- CHEMICALS ATTRIBUTES Enums ------------------------

use strum_macros::EnumIter;

use super::IntoEntryEnumerator;


/// Edible Enum
#[derive(EnumIter, Debug, PartialEq, Default, Clone)]
pub enum EdibleEnum {
    #[default]
    Edible,
    NotEdible,
    Poisonous,
    Medicinal,
    Unknown,
}
impl IntoEntryEnumerator for EdibleEnum {
    fn to_str(&self) -> &str {
        match self {
            EdibleEnum::Edible      => "Edible",
            EdibleEnum::NotEdible   => "Not Edible",
            EdibleEnum::Poisonous   => "Poisonous",
            EdibleEnum::Medicinal   => "Medicinal",
            EdibleEnum::Unknown     => "Unknown",
        }
    }
}

/// Odor Enum
#[derive(EnumIter, Debug, PartialEq, Default, Clone)]
pub enum OdorEnum {
    #[default]
    Mild,
    Fruity,
    AniseLike,
    Pungent,
    Custom,
}
impl IntoEntryEnumerator for OdorEnum {
    fn to_str(&self) -> &str {
        match self {
            OdorEnum::Mild      => "Mild",
            OdorEnum::Fruity    => "Fruity",
            OdorEnum::AniseLike => "AniseLike",
            OdorEnum::Pungent   => "Pungent",
            OdorEnum::Custom    => "",
        }
    }
    fn is_custom(&self) -> bool {
        *self == OdorEnum::Custom
    }
}

/// Taste Enum
#[derive(EnumIter, Debug, PartialEq, Default, Clone)]
pub enum TasteEnum {
    #[default]
    Mild,
    Bitter,
    Acrid,
    Custom,
}
impl IntoEntryEnumerator for TasteEnum {
    fn to_str(&self) -> &str {
        match self {
            TasteEnum::Mild     => "Mild",
            TasteEnum::Bitter   => "Bitter",
            TasteEnum::Acrid    => "Acrid",
            TasteEnum::Custom   => "",
        }
    }
    fn is_custom(&self) -> bool {
        *self == TasteEnum::Custom
    }
}

/// Chemical Reactions Enum
#[derive(EnumIter, Debug, PartialEq, Default, Clone)]
pub enum ChemicalReactionEnum {
    #[default]
    KOH,
    Ammonia,
    FeSO4,
    Custom,
}
impl IntoEntryEnumerator for ChemicalReactionEnum {
    fn to_str(&self) -> &str {
        match self {
            ChemicalReactionEnum::KOH       => "KOH",
            ChemicalReactionEnum::Ammonia   => "Ammonia",
            ChemicalReactionEnum::FeSO4     => "FeSO4",
            ChemicalReactionEnum::Custom    => "",
        }
    }
    fn is_custom(&self) -> bool {
        *self == ChemicalReactionEnum::Custom
    }
}*/

