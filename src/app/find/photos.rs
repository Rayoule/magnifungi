use leptos::prelude::*;

use super::entry_types::entry_image::EntryImage;



/// Photos of the specimen
#[derive(Default, Clone)]
pub struct Photos {
    pub photos: Vec<EntryImage>,
}