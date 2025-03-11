use leptos::prelude::*;
use strum_macros::EnumIter;

use super::entry_types::{entry_enum::{EntryEnumCustomNote, EntryEnumNote, IntoEntryEnumerator}, entry_list::EntryList, entry_tint::EntryTint, FreeText};
use strum::IntoEnumIterator;


#[derive(Default, Clone)]
pub struct ChemicalAttributes {
    /// How much is this specimen edible ?
    /// (edible, not edible, poisonous, medicinal, unknown)
    pub edible: EntryEnumNote<EdibleEnum>,
    /// What does the specimen smell like ?
    /// (mild, fruity, anise-like, pungent, custom, etc.)
    pub odor: EntryEnumCustomNote<OdorEnum>,
    /// What does the specimen taste like ?
    /// (mild, bitter, acrid, custom, etc.)
    pub taste: EntryEnumCustomNote<TasteEnum>,
    /// Is there latex and what does it look like ?
    pub latex: Option<Latex>,
    /// Is there a bruising reaction ?
    pub bruising_reaction: Option<BruisingReaction>,
    /// Are there chemical reactions that have been tested on the specimen ?
    pub chemical_reactions: EntryList<EntryEnumCustomNote<ChemicalReactionEnum>>,
}


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
}


#[derive(Clone)]
/// What does the latex looks like ?
pub struct Latex {
    /// Tint of the lastex
    pub tint: EntryTint,
    /// Note about reactions with the air or else
    pub note: Option<FreeText>,
}


#[derive(Clone)]
/// Describes a bruising reaction
pub struct BruisingReaction {
    /// Initial tint
    pub initial_tint: EntryTint,
    /// Tint after reaction
    pub after_tint: Option<EntryTint>,
    /// Note about the reaction
    pub note: Option<FreeText>,
}