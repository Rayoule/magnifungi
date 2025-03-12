use leptos::prelude::*;
use magnifungi_shared_types::entry_types::{entry_image::EntryImage, entry_trait::IntoFindView};




/// Photos of the specimen
#[derive(Default, Clone)]
pub struct Photos {
    pub photos: Vec<EntryImage>,
}
impl IntoFindView for Photos {}