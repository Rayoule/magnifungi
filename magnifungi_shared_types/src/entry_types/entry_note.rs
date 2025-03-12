use super::{IntoFindView, EntryName, FreeText};



#[derive(Clone)]
pub enum EntryNote {
    TextNote(TextNote),
    AudioNote(AudioNote),
    DrawingNote(DrawingNote),
}
impl Default for EntryNote {
    fn default() -> Self {
        Self::TextNote(TextNote::default())
    }
}
impl IntoFindView for EntryNote {}



#[derive(Default, Clone)]
pub struct TextNote {
    pub title: Option<EntryName>,
    pub content: FreeText,
}
impl IntoFindView for TextNote {}


#[derive(Default, Clone)]
pub struct AudioNote {
    //
}
impl IntoFindView for AudioNote {}


#[derive(Default, Clone)]
pub struct DrawingNote {
    //
}
impl IntoFindView for DrawingNote {}

