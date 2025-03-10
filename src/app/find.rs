use leptos::prelude::*;

use infos::FindInfos;
use morphology::Morphology;
use chemical_attributes::ChemicalAttributes;
use notes::Notes;
use photos::Photos;



pub mod infos;
pub mod morphology;
pub mod chemical_attributes;
pub mod photos;
pub mod notes;
pub mod entry_types;




pub struct Find {
    pub infos: FindInfos,
    pub morphology: Morphology,
    pub chemical_attributes: ChemicalAttributes,
    pub photos: Photos,
    pub notes: Notes,
}




/* Note: the person that created the "Find" will be referred as the finder
and corresponds to the field "Find owner"


--- Find Structure ---

General Informations

    Id: Locally unique id of the find

    Source
        - Finder
            - Find onwer: ? Name/Alias of the person that created this Find
            - Specimen finder: ? Name/Alias of the person that found the
                specimen if different from Find owner
        - Creation date & time
        - Last update date & time

    Name
        - Find name: Given by the finder
            (ex: "Russula #1" or "Weird mushroom by the old tree")
        - Species name: ? Scientific name as identified by the finder
        - Common name: ? Species common name as identified by the finder
    
    Observation Type
        - Field observation
            (if the specimen was just found on the field)
        - Exhibited specimen
            (if the specimen was brought somewhere by someone, like at an exhibition)

    Date
        - Date & Time: Enum: Match -> Find Type
            => Field observation:
                - Date & time
            => Exhibited specimen:
                - Specimen date & time: when the specimen was collected
                    - Date & time
                - Exhibit date & time: when the specimen was observed by the Find owner
                    - Date & time
    
    Location
        - Location: Enum: Match -> Find Type
            => Field observation:
                - Location:
                    - Name: Given by the finder
                    - GPS coordinates
            => Exhibited specimen:
                - Specimen Location:
                    - Name: Given by the finder
                        (ex: "Vosges forest, Munster")
                    - GPS coordinates
                - Exhibition location:
                    - Name: Given by the finder
                        (ex: "Mycology Event", "Herbarium", "Mushroom Fair")
                    - GPS coordinates
        - Habitat description: Free text
            (ex: "Mycology Event", "Herbarium", "Mushroom Fair")
        - Host: ? some tree or plant if mycorrhizal associations or parasitic fungi
    

Morphology
    
    Cap (Pileus)
        - Shape: (ex: convex, flat, umbonate, bell-shaped, custom, etc.)
            + note ?
        - Tint:
            - Tint description: free text
            - Color: Color picker
        - Surface: Enum (smooth, scaly, hairy, cracked, custom, etc.)
            - note ?
        - Dimensions:
            - Diameter: ?
            - Height: ?
            - Length: ?
            - Width: ?
    
    SporeDispersasion
        - Type:
            - Enum (gilled, pored, toothed, ridged, custom, none)
            - note ?
        - Attachment: (free, attached, decurrent, custom, etc.)
            - note ?
        - Spacing: (dense, moderate, widely spaced, custom)
            - note ?
        - Tint:
            - Tint description: free text
            - Color: Color picker
    
    Stem (Stipe)
        - Dimension:
            - Diameter: ?
            - Height: ?
            - Length: ?
            - Width: ?
        - Surface: Enum (smooth, fibrous, scaly, reticulated, custom, etc.)
            - note ?
        - Hollow/Solid: Enum (hollow, solid, chambered, custom)
            - note?
        - Tint:
            - Tint description: free text
            - Color: Color picker
    
    Volva & Ring ?
        - Description: free text
        - Ring (Annulus): ? Enum (skirt-like, fragile, persistent, custom, etc.)
            - note ?
    
    Spore Print ?
        - Tint:
            - Tint description: free text
            - Color: Color picker
        - Print Method: ? Enum (on paper, glass, custom, etc.)
            - note ?


Chemical Attributes

    Edible: ? Enum (edible, not edible, poisonous, medicinal, unknown)

    Odor: ? Enum (mild, fruity, anise-like, pungent, custom, etc.)
        - note ?
    
    Taste: ? Enum (mild, bitter, acrid, custom, etc.)
        - note ?

    Latex: ?
        - Tint:
            - Tint description: free text
            - Color: Color picker
        - note ? (reaction with air or else)
    
    Bruising Reaction ?
        - Tint description: free text
        - Tint before / after:
            - Tint before:
                - Tint description: free text
                - Color: Color picker
            - Tint after:
                - Tint description: free text
                - Color: Color picker
    
    Chemical Reactions ?
        - List of reactions
            - Reaction: Enum (KOH, ammonia, FeSO₄ reactions, custom, etc.)
                - note ?


Photos
    - List of photos ?

Notes
    - Enum (Text note, Audio note, Drawing note)
        
*/
