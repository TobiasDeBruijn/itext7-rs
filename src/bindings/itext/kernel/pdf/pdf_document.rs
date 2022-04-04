use ejni::{Class, Object};
use crate::bindings::itext::kernel::pdf::document_properties::DocumentProperties;
use crate::bindings::itext::kernel::pdf::pdf_reader::PdfReader;
use crate::error::PdfResult;
use crate::bindings::itext::kernel::pdf::pdf_writer::PdfWriter;
use crate::bindings::itext::kernel::pdf::stamping_properties::StampingProperties;
use crate::traits::ClassName;
use crate::types::Env;

pub struct PdfDocument<'a> {
    env: Env<'a>,
    obj: Object<'a>,
}

impl<'a> ClassName for PdfDocument<'a> {
    fn name() -> &'static str {
        "com/itextpdf/kernel/pdf/PdfDocument"
    }
}

impl<'a> PdfDocument<'a> {
    pub fn into_inner(self) -> Object<'a> {
        self.obj
    }

    pub fn new_with_reader(env: Env<'a>, reader: PdfReader<'a>) -> PdfResult<Self> {
        let obj = env.new_object(Self::name(), format!("(L{reader};)V", reader = PdfReader::name()), &[reader.into_inner().into()])?;
        Ok(Self {
            env,
            obj: Object::new(obj, Class::for_name(env, Self::name())?),
        })
    }

    pub fn new_with_reader_and_document_properties(env: Env<'a>, reader: PdfReader<'a>, document_properties: DocumentProperties<'a>) -> PdfResult<Self> {
        let obj = env.new_object(Self::name(), format!("(L{reader};L{properties};)V", reader = PdfReader::name(), properties = DocumentProperties::name()), &[reader.into_inner().into(), document_properties.into_inner().into()])?;
        Ok(Self {
            env,
            obj: Object::new(obj, Class::for_name(env, Self::name())?),
        })
    }

    pub fn new_with_reader_and_writer(env: Env<'a>, reader: PdfReader<'a>, writer: PdfWriter<'a>) -> PdfResult<Self> {
        let obj = env.new_object(Self::name(), format!("(L{reader};L{writer};)V", reader = PdfReader::name(), writer = PdfWriter::name()), &[reader.into_inner().into(), writer.into_inner().into()])?;
        Ok(Self {
            env,
            obj: Object::new(obj, Class::for_name(env, Self::name())?),
        })
    }

    pub fn new_with_reader_and_writer_and_stamping_properties(env: Env<'a>, reader: PdfReader<'a>, writer: PdfWriter<'a>, stamping_properties: StampingProperties<'a>) -> PdfResult<Self> {
        let obj = env.new_object(Self::name(), format!("(L{reader};L{writer};L{stamping};)V", reader = PdfReader::name(), writer = PdfWriter::name(), stamping = StampingProperties::name()), &[reader.into_inner().into(), writer.into_inner().into(), stamping_properties.into_inner().into()])?;
        Ok(Self {
            env,
            obj: Object::new(obj, Class::for_name(env, Self::name())?),
        })
    }

    pub fn new_with_writer(env: Env<'a>, writer: PdfWriter<'a>) -> PdfResult<Self> {
        let obj = env.new_object(Self::name(), format!("(L{writer};)V", writer = PdfWriter::name()), &[writer.into_inner().into()])?;
        Ok(Self {
            env,
            obj: Object::new(obj, Class::for_name(env, Self::name())?),
        })
    }

    pub fn new_with_writer_and_document_properties(env: Env<'a>, writer: PdfWriter<'a>, document_properties: DocumentProperties<'a>) -> PdfResult<Self> {
        let obj = env.new_object(Self::name(), format!("(L{writer};L{props};)V", writer = PdfWriter::name(), props = DocumentProperties::name()), &[writer.into_inner().into(), document_properties.into_inner().into()])?;
        Ok(Self {
            env,
            obj: Object::new(obj, Class::for_name(env, Self::name())?),
        })
    }
}

#[cfg(test)]
pub(crate) mod test {
    use ejni::Class;
    use tempfile::NamedTempFile;
    use crate::java::file::File;
    use crate::test::{JVM, PDF_SAMPLE_PATH};
    use super::*;

    pub fn get_document_w(env: Env) -> PdfDocument {
        let file = File::new(&env, NamedTempFile::new().unwrap().path().to_str().unwrap()).unwrap();
        let writer = PdfWriter::new_with_file(env, file).unwrap();

        let doc = PdfDocument::new_with_writer(env, writer).unwrap();
        doc
    }

    #[test]
    fn name() {
        let jvm = JVM.lock();
        let env = jvm.attach_current_thread().unwrap();

        assert!(Class::for_name(&env, PdfDocument::name()).is_ok());
    }

    #[test]
    fn new_with_reader() {
        let jvm = JVM.lock();
        let env = jvm.attach_current_thread().unwrap();

        let file = File::new(&env, PDF_SAMPLE_PATH).unwrap();
        let reader = PdfReader::new_with_file(&env, file).unwrap();

        let doc = PdfDocument::new_with_reader(&env, reader);
        assert!(doc.is_ok());
    }

    #[test]
    fn new_with_reader_and_document_properties() {
        let jvm = JVM.lock();
        let env = jvm.attach_current_thread().unwrap();

        let file = File::new(&env, PDF_SAMPLE_PATH).unwrap();
        let reader = PdfReader::new_with_file(&env, file).unwrap();
        let props = DocumentProperties::new(&env).unwrap();

        let doc = PdfDocument::new_with_reader_and_document_properties(&env, reader, props);
        assert!(doc.is_ok());
    }

    #[test]
    fn new_with_reader_and_writer() {
        let jvm = JVM.lock();
        let env = jvm.attach_current_thread().unwrap();

        let file_r = File::new(&env, PDF_SAMPLE_PATH).unwrap();
        let reader = PdfReader::new_with_file(&env, file_r).unwrap();

        let file_w = File::new(&env, NamedTempFile::new().unwrap().path().to_str().unwrap()).unwrap();
        let writer = PdfWriter::new_with_file(&env, file_w).unwrap();

        let doc = PdfDocument::new_with_reader_and_writer(&env, reader, writer);
        assert!(doc.is_ok());
    }

    #[test]
    fn new_with_reader_and_writer_and_stamping_properties() {
        let jvm = JVM.lock();
        let env = jvm.attach_current_thread().unwrap();

        let file_r = File::new(&env, PDF_SAMPLE_PATH).unwrap();
        let reader = PdfReader::new_with_file(&env, file_r).unwrap();

        let file_w = File::new(&env, NamedTempFile::new().unwrap().path().to_str().unwrap()).unwrap();
        let writer = PdfWriter::new_with_file(&env, file_w).unwrap();

        let props = StampingProperties::new(&env).unwrap();

        let doc = PdfDocument::new_with_reader_and_writer_and_stamping_properties(&env, reader, writer, props);
        assert!(doc.is_ok());
    }

    #[test]
    fn new_with_writer() {
        let jvm = JVM.lock();
        let env = jvm.attach_current_thread().unwrap();

        let file = File::new(&env, NamedTempFile::new().unwrap().path().to_str().unwrap()).unwrap();
        let writer = PdfWriter::new_with_file(&env, file).unwrap();

        let doc = PdfDocument::new_with_writer(&env, writer);
        assert!(doc.is_ok());
    }

    #[test]
    fn new_with_writer_and_document_properties() {
        let jvm = JVM.lock();
        let env = jvm.attach_current_thread().unwrap();

        let file = File::new(&env, NamedTempFile::new().unwrap().path().to_str().unwrap()).unwrap();
        let writer = PdfWriter::new_with_file(&env, file).unwrap();

        let props = DocumentProperties::new(&env).unwrap();

        let doc = PdfDocument::new_with_writer_and_document_properties(&env, writer, props);
        assert!(doc.is_ok());
    }
}