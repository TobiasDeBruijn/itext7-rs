use ejni::{Class, Object};
use jni::objects::JValue;
use crate::{Env, PdfResult};
use crate::itext::kernel::geom::page_size::PageSize;
use crate::itext::kernel::pdf::pdf_document::PdfDocument;
use crate::traits::ClassName;

pub struct Document<'a> {
    env: Env<'a>,
    obj: Object<'a>,
}

impl<'a> ClassName for Document<'a> {
    fn name() -> &'static str {
        "com/itextpdf/layout/Document"
    }
}

impl<'a> Document<'a> {
    pub fn into_inner(self) -> Object<'a> {
        self.obj
    }

    pub fn new(env: Env<'a>, pdf_document: PdfDocument<'a>) -> PdfResult<Self> {
        let obj = env.new_object(Self::name(), format!("(L{};)V", PdfDocument::name()), &[pdf_document.into_inner().into()])?;
        Ok(Self {
            env,
            obj: Object::new(obj, Class::for_name(env, Self::name())?)
        })
    }

    pub fn new_with_size(env: Env<'a>, pdf_document: PdfDocument<'a>, page_size: PageSize<'a>) -> PdfResult<Self> {
        let obj = env.new_object(Self::name(), format!("(L{};L{};)V", PdfDocument::name(), PageSize::name()), &[pdf_document.into_inner().into(), page_size.into_inner().into()])?;
        Ok(Self {
            env,
            obj: Object::new(obj, Class::for_name(env, Self::name())?)
        })
    }

    pub fn new_with_size_and_flush(env: Env<'a>, pdf_document: PdfDocument<'a>, page_size: PageSize<'a>, immediate_flush: bool) -> PdfResult<Self> {
        let immediate_flush: u8 = if immediate_flush { 1 } else { 0 };
        let  obj = env.new_object(Self::name(), format!("(L{};L{};Z)V", PdfDocument::name(), PageSize::name()), &[pdf_document.into_inner().into(), page_size.into_inner().into(), JValue::Bool(immediate_flush)])?;
        Ok(Self {
            env,
            obj: Object::new(obj, Class::for_name(env, Self::name())?)
        })
    }
}

#[cfg(test)]
mod test {
    use crate::test::JVM;
    use super::*;

    #[test]
    fn new() {
        let jvm = JVM.lock();
        let env = jvm.attach_current_thread().unwrap();

        let pdf = crate::itext::kernel::pdf::pdf_document::test::get_document_w(&env);
        let doc = Document::new(&env, pdf);

        assert!(doc.is_ok());
    }
}