use super::IntoFindView;


#[derive(Default, Clone)]
pub struct EntryList<T: IntoFindView> {
    pub entries: Vec<T>,
}
impl<T: IntoFindView> IntoFindView for EntryList<T> {}