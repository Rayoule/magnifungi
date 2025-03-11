use leptos::prelude::*;
use magnifungi_shared_types::entry_types::{entry_value_unit::EntryValueUnit, FreeText};



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


/// Describes the specimen's Cap (Pileus)
#[derive(Clone)]
pub struct Cap {
    /*
    /// (convex, flat, umbonate, bell-shaped, custom, etc.)
    pub shape: EntryEnumCustomNote<CapShapeEnum>,
    pub tint: EntryTint,
    /// (smooth, scaly, hairy, cracked, custom, etc.)
    pub surface: EntryEnumCustomNote<CapSurfaceEnum>,
    pub dimension: Dimensions,
    */
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
    /*
    /// (gilled, pored, toothed, ridged, custom, none)
    pub disp_type: EntryEnumCustomNote<DispersionEnum>,
    /// (free, attached, decurrent, custom, etc.)
    pub attachment: EntryEnumCustomNote<AttachmentEnum>,
    /// (dense, moderate, widely spaced, custom)
    pub spacing: EntryEnumCustomNote<SpacingEnum>,
    pub tint: EntryTint
    */
}


/// Describes the specimen's stem (stipe)
#[derive(Clone)]
pub struct Stem {
    /*
    pub dimensions: Dimensions,
    /// (smooth, fibrous, scaly, reticulated, custom, etc.)
    pub surface: EntryEnumCustomNote<StemSurfaceEnum>,
    /// (hollow, solid, chambered, custom)
    pub hollow_solid: EntryEnumCustomNote<HollowSolidEnum>,
    pub tint: EntryTint,
    */
}


#[derive(Clone)]
pub struct Volva {
    pub desc: Option<FreeText>,
}


#[derive(Clone)]
pub struct Ring {
    /*
    /// (skirt-like, fragile, persistent, custom, etc.)
    pub desc: EntryEnumCustomNote<RingEnum>,
    */
}


#[derive(Clone)]
pub struct SporeTint {
    /*
    pub tint: EntryTint,
    /// (on paper, glass, custom, etc.)
    pub print_method: EntryEnumCustomNote<PrintMethodEnum>,
    */
}

