use leptos::prelude::*;

use infos::FindInfos;
use magnifungi_shared_types::view_trait::IntoFindView;
use morphology::Morphology;
use chemical_attributes::ChemicalAttributes;
use notes::Notes;
use photos::Photos;



pub mod infos;
pub mod morphology;
pub mod chemical_attributes;
pub mod photos;
pub mod notes;


/// Find structure that contains all find datas
#[derive(Default, Clone)]
pub struct Find {
    pub infos: FCategory<FindInfos>,
    pub morphology: FCategory<Morphology>,
    pub chemical_attributes: FCategory<ChemicalAttributes>,
    pub photos: FCategory<Photos>,
    pub notes: FCategory<Notes>,
}
impl IntoFindView for Find {
    fn into_any_view(&self) -> AnyView {
        vec![
            self.infos.into_any_view(),
            self.morphology.into_any_view(),
            self.chemical_attributes.into_any_view(),
            self.photos.into_any_view(),
            self.notes.into_any_view(),
        ].into_any()
    }
}


// Generate all IntoEntryEnum enums from enums.json
use strum_macros::EnumIter;
use magnifungi_shared_types::entry_types::entry_enum::IntoEntryEnum;
use magnifungi_macros::generate_enums_from_path;
generate_enums_from_path!();



/*
--- Find Structure ---

Find
|-Category
    |-Part
        |-SubPart
        |   |-Entry
        |-Entry


--- Find Content ---

General Information

    Id: Locally unique ID of the find

    Source
        - Finder
            - Find Owner: Name or alias of the person who created this find
            - Specimen Finder: Name or alias of the person who found the specimen (if different from the find owner)
        - Creation Date & Time
        - Last Update Date & Time

    Name
        - Find Name: Given by the finder (e.g., "Russula #1" or "Weird mushroom by the old tree")
        - Species Name: Scientific name as identified by the finder
        - Common Name: Species' common name as identified by the finder
    
    Observation Type
        - Field Observation (specimen found in the wild)
        - Exhibited Specimen (specimen brought to an event or exhibition)
    
    Date
        - Match -> Find Type
            => Field Observation:
                - Date & Time
            => Exhibited Specimen:
                - Specimen Date & Time (when collected)
                - Exhibit Date & Time (when observed by the find owner)
    
    Location
        - Match -> Find Type
            => Field Observation:
                - Location:
                    - Name (given by the finder)
                    - GPS Coordinates
            => Exhibited Specimen:
                - Specimen Location:
                    - Name (e.g., "Vosges forest, Munster")
                    - GPS Coordinates
                - Exhibition Location:
                    - Name (e.g., "Mycology Event", "Herbarium", "Mushroom Fair")
                    - GPS Coordinates
        - Habitat Description: Free text
        - Host: If associated with a tree or plant (e.g., mycorrhizal or parasitic fungi)

Morphology
    
    Cap (Pileus)
        - Shape: CapShapeEnum (Convex, Flat, Umbonate, Bell-shaped, Custom)
        - Tint:
            - Description: Free text
            - Color: Color Picker
        - Surface: CapSurfaceEnum (Smooth, Scaly, Hairy, Cracked, Custom)
        - Dimensions:
            - Diameter: ?
            - Height: ?
            - Length: ?
            - Width: ?
    
    Spore Dispersion
        - Type: SporeDispersionTypeEnum (Gilled, Pored, Toothed, Ridged, None, Custom)
        - Attachment: SporeAttachmentEnum (Free, Attached, Decurrent, Custom)
        - Spacing: SporeSpacingEnum (Dense, Moderate, Widely Spaced, Custom)
        - Tint:
            - Description: Free text
            - Color: Color Picker
    
    Stem (Stipe)
        - Dimensions:
            - Diameter: ?
            - Height: ?
            - Length: ?
            - Width: ?
        - Surface: StemSurfaceEnum (Smooth, Fibrous, Scaly, Reticulated, Custom)
        - Hollow/Solid: StemHollowEnum (Hollow, Solid, Chambered, Custom)
        - Tint:
            - Description: Free text
            - Color: Color Picker
    
    Volva & Ring
        - Description: Free text
        - Ring: RingTypeEnum (Skirt-Like, Fragile, Persistent, Custom)
    
    Spore Print
        - Tint:
            - Description: Free text
            - Color: Color Picker
        - Print Method: PrintMethodEnum (On Paper, Glass, Custom)
    
Chemical Attributes

    Edibility: EdibilityEnum (Edible, Not Edible, Poisonous, Medicinal, Unknown)
    
    Odor: OdorEnum (Mild, Fruity, Anise-Like, Pungent, Custom)
        
    Taste: TasteEnum (Mild, Bitter, Acrid, Custom)
    
    Latex
        - Tint:
            - Description: Free text
            - Color: Color Picker
        - Note: Reaction with air or other substances
    
    Bruising Reaction
        - Description: Free text
        - Tint Before / After:
            - Before:
                - Description: Free text
                - Color: Color Picker
            - After:
                - Description: Free text
                - Color: Color Picker
    
    Chemical Reactions
        - List of Reactions:
            - Reaction Type: ChemicalReactionEnum (KOH, Ammonia, FeSO₄, Custom)

Photos
    - List of Photos

Notes
    - Enum (Text Note, Audio Note, Drawing Note)

*/

