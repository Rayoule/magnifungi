use std::default;

use::leptos::prelude::*;
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
impl IntoFindView for EntryNote {
    fn into_any_view(&self) -> AnyView {
        match self {
            EntryNote::TextNote(n) => n.into_any_view(),
            EntryNote::AudioNote(n) => n.into_any_view(),
            EntryNote::DrawingNote(n) => n.into_any_view(),
        }
    }
}



#[derive(Default, Clone)]
pub struct TextNote {
    pub title: Option<EntryName>,
    pub content: FreeText,
}
impl IntoFindView for TextNote {
    fn into_any_view(&self) -> AnyView {
        let title = self
            .title
            .clone()
            .map_or(
                ().into_any(),
                |n| n.into_any_view()
            );
            let content = self
                .content
                .clone()
                .into_any_view();
        view! {
            <div>
                {title}
                {content}
            </div>
        }.into_any()
    }
}


#[derive(Default, Clone)]
pub struct AudioNote {
    //
}
impl IntoFindView for AudioNote {
    fn into_any_view(&self) -> AnyView {
        ().into_any()
    }
}


#[derive(Default, Clone)]
pub struct DrawingNote {
    //
}
impl IntoFindView for DrawingNote {
    fn into_any_view(&self) -> AnyView {
        ().into_any()
    }
}

