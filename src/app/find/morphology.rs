use leptos::prelude::*;
use magnifungi_shared_types::{entry_types::{
    entry_enum::*, entry_tint::EntryTint, entry_value_unit::EntryValueUnit, Entry, FreeText
}, view_trait::IntoFindView};

use super::{
    CapShapeEnum, CapSurfaceEnum, FindPart, PrintMethodEnum, RingTypeEnum,
    SporeAttachmentEnum, SporeDispersionTypeEnum, SporeSpacingEnum, StemHollowEnum,
    StemSurfaceEnum
};



/// Description of the specimen's morphology
#[derive(Default, Clone)]
pub struct Morphology {
    /// Specimen Cap (Pileus)
    pub cap: Option<FindPart<Cap>>,
    /// Specimen Gills or Pores or Teeth
    pub gills_pores_teeth: Option<FindPart<SporeDispersion>>,
    /// Specimen Stem (Stipe)
    pub stem: Option<FindPart<Stem>>,
    /// Specimen Volva
    pub volva: Option<FindPart<Volva>>,
    /// Specimen Ring
    pub ring: Option<FindPart<Ring>>,
    /// Specimen Spore Print
    pub spore_print: Option<FindPart<SporeTint>>,
}
impl IntoFindView for Morphology {
    fn into_any_view(&self) -> AnyView {
        let mut vec:Vec<AnyView> = Vec::new();
        if let Some(f) = &self.cap {
            vec.push(f.into_any_view())
        }
        if let Some(f) = &self.gills_pores_teeth {
            vec.push(f.into_any_view())
        }
        if let Some(f) = &self.stem {
            vec.push(f.into_any_view())
        }
        if let Some(f) = &self.volva {
            vec.push(f.into_any_view())
        }
        if let Some(f) = &self.ring {
            vec.push(f.into_any_view())
        }
        if let Some(f) = &self.spore_print {
            vec.push(f.into_any_view())
        }
        vec.into_any()
    }
}


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
impl IntoFindView for Cap {
    fn into_any_view(&self) -> AnyView {
        todo!()
    }
}


/// Describes dimensions of a specimen part
#[derive(Clone)]
pub struct Dimensions {
    pub diameter: Option<EntryValueUnit>,
    pub height: Option<EntryValueUnit>,
    pub length: Option<EntryValueUnit>,
    pub width: Option<EntryValueUnit>,
}
impl IntoFindView for Dimensions {
    fn into_any_view(&self) -> AnyView {
        let mut vec: Vec<AnyView> = Vec::new();
        if let Some(e) = &self.diameter {
            vec.push(e.into_any_view())
        }
        if let Some(e) = &self.height {
            vec.push(e.into_any_view())
        }
        if let Some(e) = &self.length {
            vec.push(e.into_any_view())
        }
        if let Some(e) = &self.width {
            vec.push(e.into_any_view())
        }
        vec.into_any()
    }
}


/// Describes the specimen's Spore Dispersion
#[derive(Clone)]
pub struct SporeDispersion {
    /// (gilled, pored, toothed, ridged, custom, none)
    pub disp_type: Entry<EntryEnum<SporeDispersionTypeEnum>>,
    /// (free, attached, decurrent, custom, etc.)
    pub attachment: Entry<EntryEnum<SporeAttachmentEnum>>,
    /// (dense, moderate, widely spaced, custom)
    pub spacing: Entry<EntryEnum<SporeSpacingEnum>>,
    pub tint: Entry<EntryTint>
}
impl IntoFindView for SporeDispersion {
    fn into_any_view(&self) -> AnyView {
        vec![
            self.disp_type.into_any_view(),
            self.attachment.into_any_view(),
            self.spacing.into_any_view(),
            self.tint.into_any_view(),
        ].into_any()
    }
}


/// Describes the specimen's stem (stipe)
#[derive(Clone)]
pub struct Stem {
    pub dimensions: Entry<Dimensions>,
    /// (smooth, fibrous, scaly, reticulated, custom, etc.)
    pub surface: Entry<EntryEnum<StemSurfaceEnum>>,
    /// (hollow, solid, chambered, custom)
    pub hollow_solid: Entry<EntryEnum<StemHollowEnum>>,
    pub tint: Entry<EntryTint>,
}
impl IntoFindView for Stem {
    fn into_any_view(&self) -> AnyView {
        todo!()
    }
}


#[derive(Clone)]
pub struct Volva {
    pub desc: Option<Entry<FreeText>>,
}
impl IntoFindView for Volva {
    fn into_any_view(&self) -> AnyView {
        todo!()
    }
}


#[derive(Clone)]
pub struct Ring {
    /// (skirt-like, fragile, persistent, custom, etc.)
    pub desc: Entry<EntryEnum<RingTypeEnum>>,
}
impl IntoFindView for Ring {
    fn into_any_view(&self) -> AnyView {
        self.desc.into_any_view()
    }
}


#[derive(Clone)]
pub struct SporeTint {
    pub tint: Entry<EntryTint>,
    /// (on paper, glass, custom, etc.)
    pub print_method: Entry<EntryEnum<PrintMethodEnum>>,
}
impl IntoFindView for SporeTint {
    fn into_any_view(&self) -> AnyView {
        vec![
            self.tint.into_any_view(),
            self.print_method.into_any_view()
        ].into_any()
    }
}

