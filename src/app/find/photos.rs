use leptos::prelude::*;
use magnifungi_shared_types::{entry_types::entry_image::EntryImage, view_trait::IntoFindView};




/// Photos of the specimen
#[derive(Default, Clone)]
pub struct Photos {
    pub photos: Vec<EntryImage>,
}
impl IntoFindView for Photos {
    fn into_any_view(&self) -> AnyView {
        self
            .photos
            .iter()
            .map(|e| e.into_any_view())
            .collect_view()
            .into_any()
    }
}