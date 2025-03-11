use super::Entry;


#[derive(Default, Clone)]
pub struct EntryList<T: Entry> {
    pub entries: Vec<T>,
}
impl<T: Entry> Entry for EntryList<T> {}