use entry_trait::Entry;


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
impl Entry for EntryName {}


#[derive(Default, Clone)]
pub struct FreeText {
    pub content: String,
}
impl FreeText {
    fn new(note_content: &str) -> Self {
        FreeText { content: note_content.to_string() }
    }
}
impl Entry for FreeText {}


#[derive(Default, Clone)]
pub struct DateTime {
    pub id: u32
}
impl Entry for DateTime {}


#[derive(Default, Clone)]
pub struct Location {
    pub name: Option<String>,
    pub gps_coords: Option<GpsCoordinates>,
}
impl Entry for Location {}


#[derive(Default, Clone)]
pub struct GpsCoordinates {
    // To do
}
impl Entry for GpsCoordinates {}
