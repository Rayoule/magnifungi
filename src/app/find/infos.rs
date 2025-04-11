use leptos::prelude::*;
use magnifungi_shared_types::{entry_types::{
    find_types::{FEntry, FPart}, DateTime, EntryName, FindId, FreeText, Location
}, view_trait::IntoFindView};


#[derive(Default, Clone)]
pub struct FindInfos {
    pub id: FEntry<FindId>,
    pub source: FPart<FindSource>,
    pub name: FPart<FindName>,
    pub observation_type: FindObservationType,
    pub date: FPart<FindDate>,
    pub location: FPart<FindLocation>
}
impl IntoFindView for FindInfos {
    fn into_any_view(&self) -> AnyView {
        vec![
            self.id.into_any_view(),
            self.source.into_any_view(),
            self.name.into_any_view(),
            self.observation_type.into_any_view(),
            self.date.into_any_view(),
            self.location.into_any_view(),
        ].into_any()
    }
}


/// Source struct
#[derive(Default, Clone)]
pub struct FindSource {
    /// Owner Name
    pub owner_name: FEntry<SourceName>,
    /// Creation date & time
    pub creation_date: FEntry<DateTime>,
    /// Last update date & time
    pub last_updated: FEntry<DateTime>,
}
impl IntoFindView for FindSource {
    fn into_any_view(&self) -> AnyView {
        vec![
            self.owner_name.into_any_view(),
            self.creation_date.into_any_view(),
            self.last_updated.into_any_view(),
        ].into_any()
    }
}

/// How created this find and who found the specimen
/// Depends on FindObservationType
#[derive(Default, Clone)]
pub struct SourceName {
    /// Name/Alias of the person that created this Find (and who found the specimen by default)
    owner_name: FEntry<EntryName>,
    /// Name/Alias of the person that found the specimen if different from Find owner
    finder_name: FEntry<EntryName>,
}
impl IntoFindView for SourceName {
    fn into_any_view(&self) -> AnyView {
        vec![
            self.owner_name.into_any_view(),
            self.finder_name.into_any_view(),
        ].into_any()
    }
}



/// Find Name struct
#[derive(Default, Clone)]
pub struct FindName {
    /// Given by the user (ex: "Russula #1" or "Weird mushroom by the old tree")
    pub find_name: FEntry<EntryName>,
    /// Scientific name as identified by the user
    pub species_name: FEntry<EntryName>,
    /// Common name as identified by the user
    pub common_name: FEntry<EntryName>,
}
impl IntoFindView for FindName {
    fn into_any_view(&self) -> AnyView {
        vec![
            self.find_name.into_any_view(),
            self.species_name.into_any_view(),
            self.common_name.into_any_view(),
        ].into_any()
    }
}


/// Determines if the specimen was found by the user or present to them
#[derive(Default, Clone)]
pub enum FindObservationType {
    #[default]
    FieldObservation,
    ExhibitedSpecimen,
}
impl IntoFindView for FindObservationType {
    fn into_any_view(&self) -> AnyView {
        let e =match self {
            FindObservationType::FieldObservation => "Field Observation",
            FindObservationType::ExhibitedSpecimen => "Exhibited Observation",
        };

        view! {
            <p>{e}</p>
        }.into_any()
    }
}



/// Find Name struct
/// Depends on FindObservationType
#[derive(Default, Clone)]
pub struct FindDate {
    /// When was the specimen found
    pub found_date: Option<FEntry<DateTime>>,
    /// When was the specimen observed after being found
    pub observed_date: Option<FEntry<DateTime>>,
}
impl IntoFindView for FindDate {
    fn into_any_view(&self) -> AnyView {
        let mut vec: Vec<AnyView> = Vec::new();
        if let Some(e) = &self.found_date {
            vec.push(e.into_any_view())
        }
        if let Some(e) = &self.observed_date {
            vec.push(e.into_any_view())
        }
        vec.into_any()
    }
}


/// Determines where the find was found
/// Depends on FindObservationType
#[derive(Default, Clone)]
pub struct FindLocation {
    /// Where the specimen was found
    /// (ex: "Vosges forest, Munster")
    pub specimen_location: Option<FEntry<Location>>,
    /// Where the specimen was observed after being found
    /// (ex: "Mycology Event", "Herbarium", "Mushroom Fair")
    pub exhibit_location: Option<FEntry<Location>>,
    /// Description of the habitat in which was found the specimen
    /// (ex: "Mycology Event", "Herbarium", "Mushroom Fair")
    pub habitat_desc: Option<FEntry<FreeText>>,
    /// Was the specimen found on a host (for mycorrhizal associations or parasitic fungi)
    pub host: Option<FEntry<FreeText>>,
}
impl IntoFindView for FindLocation {
    fn into_any_view(&self) -> AnyView {
        let mut vec: Vec<AnyView> = Vec::new();
        if let Some(e) = &self.specimen_location {
            vec.push(e.into_any_view())
        }
        if let Some(e) = &self.exhibit_location {
            vec.push(e.into_any_view())
        }
        if let Some(e) = &self.habitat_desc {
            vec.push(e.into_any_view())
        }
        if let Some(e) = &self.host {
            vec.push(e.into_any_view())
        }
        vec.into_any()
    }
}
