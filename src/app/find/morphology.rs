use leptos::prelude::*;

use super::entry_types::{entry_enum::EntryEnumCustomNote, entry_tint::EntryTint, entry_value_unit::EntryValueUnit, FreeText};


/// Description of the specimen's morphology
pub struct Morphology {
    /// Specimen Cap (Pileus)
    pub cap: Option<Cap>,
    /// Specimen Gills or Pores or Teeth
    pub gills_pores_teeth: Option<SporeDispersion>,
    /// Specimen Stem (Stipe)
    pub stem: Option<Stem>,
    /// Specimen Volva
    pub volva: Option<Volva>,
    /// Specimen Ring
    pub ring: Option<Ring>,
    /// Specimen Spore Print
    pub spore_print: Option<SporeTint>,
}


/// Describes the specimen's Cap (Pileus)
pub struct Cap {
    /// (convex, flat, umbonate, bell-shaped, custom, etc.)
    pub shape: EntryEnumCustomNote,
    pub tint: EntryTint,
    /// (smooth, scaly, hairy, cracked, custom, etc.)
    pub surface: EntryEnumCustomNote,
    pub dimension: Dimensions,
}

pub struct Dimensions {
    pub diameter: Option<EntryValueUnit>,
    pub height: Option<EntryValueUnit>,
    pub length: Option<EntryValueUnit>,
    pub width: Option<EntryValueUnit>,
}


pub struct SporeDispersion {
    /// (gilled, pored, toothed, ridged, custom, none)
    pub disp_type: EntryEnumCustomNote,
    /// (free, attached, decurrent, custom, etc.)
    pub attachment: EntryEnumCustomNote,
    /// (dense, moderate, widely spaced, custom)
    pub spacing: EntryEnumCustomNote,
    pub tint: EntryTint
}

/// Describes the specimen's stem (stipe)
pub struct Stem {
    pub dimensions: Dimensions,
    /// (smooth, fibrous, scaly, reticulated, custom, etc.)
    pub surface: EntryEnumCustomNote,
    /// (hollow, solid, chambered, custom)
    pub hollow_solid: EntryEnumCustomNote,
    pub tint: EntryTint,
}

pub struct Volva {
    pub desc: Option<FreeText>,
}

pub struct Ring {
    /// (skirt-like, fragile, persistent, custom, etc.)
    pub desc: EntryEnumCustomNote,
}

pub struct SporeTint {
    pub tint: EntryTint,
    /// (on paper, glass, custom, etc.)
    pub print_method: EntryEnumCustomNote,
}