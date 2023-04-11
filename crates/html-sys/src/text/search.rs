/// The HTML `<search>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/search)
#[doc(alias = "search")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Search {
    global_attrs: crate::GlobalAttributes,
}
impl crate::RenderElement for Search {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<search")?;
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</search>")?;
        Ok(())
    }
}
impl std::fmt::Display for Search {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Search {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Search {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}