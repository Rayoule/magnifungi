use leptos::prelude::*;

use super::{Entry, EntryName, FreeText};



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
impl Entry for EntryNote {}



#[derive(Default, Clone)]
pub struct TextNote {
    pub title: Option<EntryName>,
    pub content: FreeText,
}
impl Entry for TextNote {}


#[derive(Default, Clone)]
pub struct AudioNote {
    //
}
impl Entry for AudioNote {}


#[derive(Default, Clone)]
pub struct DrawingNote {
    //
}
impl Entry for DrawingNote {}

