use leptos::prelude::*;
use magnifungi_shared_types::{entry_types::{
    entry_enum::*, entry_list::EntryList, entry_tint::EntryTint, find_types::FEntry, FreeText}, view_trait::IntoFindView};

use super::{ChemicalReactionEnum, EdibilityEnum, FPart, OdorEnum, TasteEnum};




#[derive(Default, Clone)]
pub struct ChemicalAttributes {
    /// How much is this specimen edible ?
    /// (edible, not edible, poisonous, medicinal, unknown)
    pub edible: FEntry<EntryEnum<EdibilityEnum>>,
    /// What does the specimen smell like ?
    /// (mild, fruity, anise-like, pungent, custom, etc.)
    pub odor: FEntry<EntryEnum<OdorEnum>>,
    /// What does the specimen taste like ?
    /// (mild, bitter, acrid, custom, etc.)
    pub taste: FEntry<EntryEnum<TasteEnum>>,
    /// Is there latex and what does it look like ?
    pub latex: Option<FPart<Latex>>,
    /// Is there a bruising reaction ?
    pub bruising_reaction: Option<FEntry<BruisingReaction>>,
    /// Are there chemical reactions that have been tested on the specimen ?
    pub chemical_reactions: FPart<EntryList<EntryEnum<ChemicalReactionEnum>>>,
}
impl IntoFindView for ChemicalAttributes {
    fn into_any_view(&self) -> leptos::prelude::AnyView {
        let mut vec = vec![
            self.edible.into_any_view(),
            self.odor.into_any_view(),
            self.taste.into_any_view(),
        ];
        if let Some(f) = &self.latex {
            vec.push(f.into_any_view())
        };
        if let Some(f) = &self.bruising_reaction {
            vec.push(f.into_any_view())
        };
        vec.push(self.chemical_reactions.into_any_view());
        vec.into_any()
    }
}



#[derive(Clone)]
/// What does the latex looks like ?
pub struct Latex {
    /// Tint of the lastex
    pub tint: FEntry<EntryTint>,
    /// Note about reactions with the air or else
    pub note: Option<FEntry<FreeText>>,
}
impl IntoFindView for Latex {
    fn into_any_view(&self) -> AnyView {
        let mut vec = vec![self.tint.into_any_view()];
        if let Some(f) = &self.note {
            vec.push(f.into_any_view())
        };
        vec.into_any()
    }
}


#[derive(Clone)]
/// Describes a bruising reaction
pub struct BruisingReaction {
    /// Initial tint
    pub initial_tint: FEntry<EntryTint>,
    /// Tint after reaction
    pub after_tint: Option<FEntry<EntryTint>>,
    /// Note about the reaction
    pub note: Option<FEntry<FreeText>>,
}
impl IntoFindView for BruisingReaction {
    fn into_any_view(&self) -> AnyView {
        let mut vec = vec![self.initial_tint.into_any_view()];
        if let Some(f) = &self.after_tint {
            vec.push(f.into_any_view())
        }
        if let Some(f) = &self.note {
            vec.push(f.into_any_view())
        }
        vec.into_any()
    }
}
