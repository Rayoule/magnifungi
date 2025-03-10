use leptos::prelude::*;

use super::Entry;


pub struct EntryList<T: Entry> {
    pub entries: Vec<T>,
}
impl<T: Entry> Entry for EntryList<T> {}