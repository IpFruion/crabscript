use std::{ops::Deref, sync::Arc};

use miette::NamedSource;

#[derive(Clone, Debug)]
pub enum SourceCode {
    Named(NamedSource<Arc<str>>),
    Unnamed(Arc<str>),
}

impl SourceCode {
    pub fn as_named(&self, name: &str, language: Option<String>) -> Self {
        let mut source = match self {
            SourceCode::Named(s) => NamedSource::new(name, s.inner().clone()),
            SourceCode::Unnamed(s) => NamedSource::new(name, s.clone()),
        };
        if let Some(language) = language {
            source = source.with_language(language);
        }
        SourceCode::Named(source)
    }
}

impl From<String> for SourceCode {
    fn from(value: String) -> Self {
        SourceCode::Unnamed(value.into())
    }
}

impl From<NamedSource<Arc<str>>> for SourceCode {
    fn from(value: NamedSource<Arc<str>>) -> Self {
        SourceCode::Named(value)
    }
}

impl Deref for SourceCode {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            SourceCode::Unnamed(s) => s,
            SourceCode::Named(s) => s.inner(),
        }
    }
}

impl miette::SourceCode for SourceCode {
    fn read_span<'a>(
        &'a self,
        span: &miette::SourceSpan,
        context_lines_before: usize,
        context_lines_after: usize,
    ) -> Result<Box<dyn miette::SpanContents<'a> + 'a>, miette::MietteError> {
        match self {
            SourceCode::Named(s) => s.read_span(span, context_lines_before, context_lines_after),
            SourceCode::Unnamed(s) => s.read_span(span, context_lines_before, context_lines_after),
        }
    }
}
