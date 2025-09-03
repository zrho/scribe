use anyhow::Result;
use serde::Serialize;
use tera::Tera;

use crate::header::Header;

pub struct Templates {
    tera: Tera,
}

impl Templates {
    pub fn new() -> Result<Self> {
        let tera = Tera::new("templates/**/*")?;
        Ok(Self { tera })
    }

    pub fn render_index(&self, notes: &[NoteData]) -> Result<String> {
        let mut ctx = tera::Context::new();
        ctx.insert("notes", &notes);
        let html = self.tera.render("index.html", &ctx)?;
        Ok(html)
    }

    pub fn render_note(&self, header: &Header, body: &str) -> Result<String> {
        let mut ctx = tera::Context::new();
        ctx.insert("meta", &header);
        ctx.insert("title", &header.title);
        ctx.insert("date", &header.date);
        ctx.insert("body", &body);
        let html = self.tera.render("note.html", &ctx)?;
        Ok(html)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct NoteData {
    #[serde(flatten)]
    pub header: Header,
    pub link: String,
}
