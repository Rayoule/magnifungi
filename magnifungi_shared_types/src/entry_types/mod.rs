use entry_trait::IntoFindView;


pub mod entry_enum;
pub mod entry_tint;
pub mod entry_value_unit;
pub mod entry_list;
pub mod entry_image;
pub mod entry_note;
pub mod entry_trait;




#[derive(Default, Clone)]
pub struct EntryName {
    pub name: String,
}
impl IntoFindView for EntryName {}


#[derive(Default, Clone)]
pub struct FreeText {
    pub content: String,
}
impl FreeText {
    fn new(note_content: &str) -> Self {
        FreeText { content: note_content.to_string() }
    }
}
impl IntoFindView for FreeText {}


#[derive(Default, Clone)]
pub struct DateTime {
    pub id: u32
}
impl IntoFindView for DateTime {}


#[derive(Default, Clone)]
pub struct Location {
    pub name: Option<String>,
    pub gps_coords: Option<GpsCoordinates>,
}
impl IntoFindView for Location {}


#[derive(Default, Clone)]
pub struct GpsCoordinates {
    // To do
}
impl IntoFindView for GpsCoordinates {}
