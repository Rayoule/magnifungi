use leptos::prelude::*;

use super::entry_types::{DateTime, FreeText, Location, EntryName};




#[derive(Default, Clone)]
pub struct FindInfos {
    pub id: FindId,
    pub source: FindSource,
    pub name: FindName,
    pub observation_type: FindObservationType,
    pub date: FindDate,
    pub location: FindLocation
}


/// Find Id struct
#[derive(Default, Clone)]
pub struct FindId {
    pub id: u32,
}


/// Source struct
#[derive(Default, Clone)]
pub struct FindSource {
    /// Owner Name
    pub owner_name: SourceName,
    /// Creation date & time
    pub creation_date: DateTime,
    /// Last update date & time
    pub last_updated: DateTime,
}

/// How created this find and who found the specimen
/// Depends on FindObservationType
#[derive(Default, Clone)]
pub struct SourceName {
    /// Name/Alias of the person that created this Find (and who found the specimen by default)
    owner_name: EntryName,
    /// Name/Alias of the person that found the specimen if different from Find owner
    finder_name: Option<EntryName>,
}



/// Find Name struct
#[derive(Default, Clone)]
pub struct FindName {
    /// Given by the user (ex: "Russula #1" or "Weird mushroom by the old tree")
    pub find_name: EntryName,
    /// Scientific name as identified by the user
    pub species_name: EntryName,
    /// Common name as identified by the user
    pub common_name: EntryName,
}


/// Determines if the specimen was found by the user or present to them
#[derive(Clone)]
pub enum FindObservationType {
    FieldObservation,
    ExhibitedSpecimen,
}
impl Default for FindObservationType {
    fn default() -> Self {
        Self::FieldObservation
    }
}



/// Find Name struct
/// Depends on FindObservationType
#[derive(Default, Clone)]
pub struct FindDate {
    /// When was the specimen found
    pub found_date: Option<DateTime>,
    /// When was the specimen observed after being found
    pub observed_date: Option<DateTime>,
}



/// Determines where the find was found
/// Depends on FindObservationType
#[derive(Default, Clone)]
pub struct FindLocation {
    /// Where the specimen was found
    /// (ex: "Vosges forest, Munster")
    pub specimen_location: Option<Location>,
    /// Where the specimen was observed after being found
    /// (ex: "Mycology Event", "Herbarium", "Mushroom Fair")
    pub exhibit_location: Option<Location>,
    /// Description of the habitat in which was found the specimen
    /// (ex: "Mycology Event", "Herbarium", "Mushroom Fair")
    pub habitat_desc: Option<FreeText>,
    /// Was the specimen found on a host (for mycorrhizal associations or parasitic fungi)
    pub host: Option<FreeText>,
}
