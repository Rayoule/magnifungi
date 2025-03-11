use magnifungi_shared_types::entry_types::{
    entry_enum::{
        EntryEnumCustomNote, EntryEnumNote
    },
    entry_list::EntryList, entry_tint::EntryTint, FreeText};




#[derive(Default, Clone)]
pub struct ChemicalAttributes {
    /*
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
    */
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
