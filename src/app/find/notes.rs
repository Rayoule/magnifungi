use leptos::prelude::*;
use magnifungi_shared_types::{
    entry_types::{
        entry_list::EntryList, entry_note::EntryNote
    }, view_trait::IntoFindView
};





#[derive(Default, Clone)]
pub struct Notes {
    pub notes: EntryList<EntryNote>,
}
impl IntoFindView for Notes {
    fn into_any_view(&self) -> AnyView {
        self.notes.into_any_view()
    }
}

