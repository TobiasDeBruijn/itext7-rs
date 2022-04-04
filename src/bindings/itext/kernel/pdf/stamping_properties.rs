use ejni::{Class, Object};
use crate::{Env, PdfResult};
use crate::traits::ClassName;

pub struct StampingProperties<'a> {
    env: Env<'a>,
    obj: Object<'a>,
}

impl<'a> ClassName for StampingProperties<'a> {
    fn name() -> &'static str {
        "com/itextpdf/kernel/pdf/StampingProperties"
    }
}

impl<'a> StampingProperties<'a> {
    pub fn into_inner(self) -> Object<'a> {
        self.obj
    }

    pub fn new(env: Env<'a>) -> PdfResult<Self> {
        let obj = env.new_object(Self::name(), "()V", &[])?;
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
    fn name() {
        let jvm = JVM.lock();
        let env = jvm.attach_current_thread().unwrap();

        let class = Class::for_name(&env, StampingProperties::name());
        assert!(class.is_ok());
    }

    #[test]
    fn new() {
        let jvm = JVM.lock();
        let env = jvm.attach_current_thread().unwrap();

        let props = StampingProperties::new(&env);
        assert!(props.is_ok());
    }
}