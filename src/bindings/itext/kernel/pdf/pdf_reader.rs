use ejni::{Class, Object};
use crate::java::file::File;
use crate::{Env, PdfResult};
use crate::traits::ClassName;

pub struct PdfReader<'a> {
    env: Env<'a>,
    obj: Object<'a>,
}

impl<'a> ClassName for PdfReader<'a> {
    fn name() -> &'static str {
        "com/itextpdf/kernel/pdf/PdfReader"
    }
}

impl<'a> PdfReader<'a> {
    pub fn into_inner(self) -> Object<'a> {
        self.obj
    }

    pub fn new_with_file(env: Env<'a>, file: File<'a>) -> PdfResult<Self> {
        let obj = env.new_object(Self::name(), format!("(L{};)V", File::name()), &[file.into_inner().into()])?;
        Ok(Self {
            env,
            obj: Object::new(obj, Class::for_name(env, Self::name())?),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::test::{JVM, PDF_SAMPLE_PATH};
    use super::*;

    #[test]
    fn name() {
        let jvm = JVM.lock();
        let env = jvm.attach_current_thread().unwrap();

        let class = Class::for_name(&env, PdfReader::name());
        assert!(class.is_ok());
    }

    #[test]
    fn new_with_file() {
        let jvm = JVM.lock();
        let env = jvm.attach_current_thread().unwrap();

        let file = File::new(&env, PDF_SAMPLE_PATH).unwrap();
        let reader = PdfReader::new_with_file(&env, file);

        assert!(reader.is_ok());
    }
}