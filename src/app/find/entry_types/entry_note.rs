use leptos::prelude::*;

use super::{Entry, EntryName, FreeText};



pub enum EntryNote {
    TextNote(TextNote),
    AudioNote(AudioNote),
    DrawingNote(DrawingNote),
}
impl Entry for EntryNote {}


pub struct TextNote {
    pub title: Option<EntryName>,
    pub content: FreeText,
}
impl Entry for TextNote {}


pub struct AudioNote {
    //
}
impl Entry for AudioNote {}


pub struct DrawingNote {
    //
}
impl Entry for DrawingNote {}

