use leptos::prelude::*;

use super::entry_types::{entry_list::EntryList, entry_note::EntryNote, EntryName, FreeText};




#[derive(Default, Clone)]
pub struct Notes {
    pub notes: EntryList<EntryNote>,
}

