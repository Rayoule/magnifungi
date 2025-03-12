use leptos::prelude::*;
use magnifungi_shared_types::entry_types::{
    entry_enum::*, entry_tint::EntryTint, entry_trait::IntoFindView, entry_value_unit::EntryValueUnit, FreeText
};



/// Description of the specimen's morphology
#[derive(Default, Clone)]
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
impl IntoFindView for Morphology {}


/// Describes the specimen's Cap (Pileus)
#[derive(Clone)]
pub struct Cap {
    /// (convex, flat, umbonate, bell-shaped, custom, etc.)
    pub shape: EntryEnum<CapShapeEnum>,
    pub tint: EntryTint,
    /// (smooth, scaly, hairy, cracked, custom, etc.)
    pub surface: EntryEnum<CapSurfaceEnum>,
    pub dimension: Dimensions,
}


/// Describes dimensions of a specimen part
#[derive(Clone)]
pub struct Dimensions {
    pub diameter: Option<EntryValueUnit>,
    pub height: Option<EntryValueUnit>,
    pub length: Option<EntryValueUnit>,
    pub width: Option<EntryValueUnit>,
}


/// Describes the specimen's Spore Dispersion
#[derive(Clone)]
pub struct SporeDispersion {
    /// (gilled, pored, toothed, ridged, custom, none)
    pub disp_type: EntryEnum<SporeDispersionTypeEnum>,
    /// (free, attached, decurrent, custom, etc.)
    pub attachment: EntryEnum<SporeAttachmentEnum>,
    /// (dense, moderate, widely spaced, custom)
    pub spacing: EntryEnum<SporeSpacingEnum>,
    pub tint: EntryTint
}


/// Describes the specimen's stem (stipe)
#[derive(Clone)]
pub struct Stem {
    pub dimensions: Dimensions,
    /// (smooth, fibrous, scaly, reticulated, custom, etc.)
    pub surface: EntryEnum<StemSurfaceEnum>,
    /// (hollow, solid, chambered, custom)
    pub hollow_solid: EntryEnum<StemHollowEnum>,
    pub tint: EntryTint,
}


#[derive(Clone)]
pub struct Volva {
    pub desc: Option<FreeText>,
}


#[derive(Clone)]
pub struct Ring {
    /// (skirt-like, fragile, persistent, custom, etc.)
    pub desc: EntryEnum<RingTypeEnum>,
}


#[derive(Clone)]
pub struct SporeTint {
    pub tint: EntryTint,
    /// (on paper, glass, custom, etc.)
    pub print_method: EntryEnum<PrintMethodEnum>,
}

