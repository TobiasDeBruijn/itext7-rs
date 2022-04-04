use ejni::{Class, JavaString, Object};
use crate::{Env, PdfResult};
use crate::traits::ClassName;

pub struct File<'a> {
    env: Env<'a>,
    obj: Object<'a>,
}

impl<'a> ClassName for File<'a> {
    fn name() -> &'static str {
        "java/io/File"
    }
}

impl<'a> File<'a> {
    pub fn into_inner(self) -> Object<'a> {
        self.obj
    }

    pub fn new<S: AsRef<str>>(env: Env<'a>, path: S) -> PdfResult<Self> {
        let string = JavaString::from_rust(env, path)?;
        let obj = env.new_object(Self::name(), "(Ljava/lang/String;)V", &[string.into()])?;
        Ok(Self {
            env,
            obj: Object::new(obj, Class::File(env)?),
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

        let class = Class::for_name(&env, File::name());
        assert!(class.is_ok());
    }

    #[test]
    fn new() {
        let jvm = JVM.lock();
        let env = jvm.attach_current_thread().unwrap();

        let file = File::new(&env, PDF_SAMPLE_PATH);
        assert!(file.is_ok());
    }
}