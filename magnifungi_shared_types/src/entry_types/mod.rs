use find_types::FEntry;
use leptos::prelude::*;
use crate::view_trait::IntoFindView;


pub mod find_types;
pub mod entry_enum;
pub mod entry_tint;
pub mod entry_value_unit;
pub mod entry_list;
pub mod entry_image;
pub mod entry_note;



#[derive(Default, Clone)]
pub struct FindId {
    pub id: u32,
}
impl IntoFindView for FindId {
    fn into_any_view(&self) -> AnyView {
        view! {
            <span>{ self.id }</span>
        }.into_any()
    }
}

#[derive(Default, Clone)]
pub struct EntryName {
    pub name: String,
}
impl IntoFindView for EntryName {
    fn into_any_view(&self) -> leptos::prelude::AnyView {
        view! {
            <p class="entry-name" >{ self.name.clone() }</p>
        }.into_any()
    }
}


#[derive(Default, Clone)]
pub struct FreeText {
    pub content: String,
}
impl IntoFindView for FreeText {
    fn into_any_view(&self) -> AnyView {
        view! {
            <p class="freetext" >{ self.content.clone() }</p>
        }.into_any()
    }
}


#[derive(Default, Clone)]
pub struct DateTime {
    pub id: u32
}
impl IntoFindView for DateTime {
    fn into_any_view(&self) -> AnyView {
        view! {
            <p class="date-time" >{ self.id.clone() }</p>
        }.into_any()
    }
}


#[derive(Default, Clone)]
pub struct Location {
    pub name: Option<FEntry<EntryName>>,
    pub gps_coords: Option<FEntry<GpsCoordinates>>,
}
impl IntoFindView for Location {
    fn into_any_view(&self) -> AnyView {
        let mut vec: Vec<AnyView> = Vec::new();
        if let Some(e) = &self.name {
            vec.push(e.into_any_view())
        }
        if let Some(e) = &self.gps_coords {
            vec.push(e.into_any_view())
        }
        vec.into_any()
    }
}


#[derive(Default, Clone)]
pub struct GpsCoordinates {
    pub coords: String
}
impl IntoFindView for GpsCoordinates {
    fn into_any_view(&self) -> AnyView {
        view! {
            <p>{ self.coords.clone() }</p>
        }.into_any()
    }
}
