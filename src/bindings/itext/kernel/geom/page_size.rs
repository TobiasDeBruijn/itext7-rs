use ejni::{Class, Object};
use jni::objects::JValue;
use crate::{Env, PdfResult};
use crate::traits::ClassName;

pub struct PageSize<'a> {
    env: Env<'a>,
    obj: Object<'a>,
}

impl<'a> ClassName for PageSize<'a> {
    fn name() -> &'static str {
        "com/itextpdf/kernel/geom/PageSize"
    }
}

macro_rules! page_size_static {
    ($name:ident,$name_lit:expr) => {
        pub fn $name(env: Env<'a>) -> PdfResult<Self> {
            let obj = env.get_static_field(Self::name(), $name_lit, format!("L{};", Self::name()))?.l()?;
            Ok(Self {
                env,
                obj: Object::new(obj, Class::for_name(env, Self::name())?),
            })
        }
    }
}

impl<'a> PageSize<'a> {
    pub fn into_inner(self) -> Object<'a> {
        self.obj
    }

    pub fn new(env: Env<'a>, width: f32, height: f32) -> PdfResult<Self> {
        let obj = env.new_object(Self::name(), "(FF)V", &[JValue::Float(width), JValue::Float(height)])?;
        Ok(Self {
            env,
            obj: Object::new(obj, Class::for_name(env, Self::name())?)
        })
    }

    page_size_static!(a0, "A0");
    page_size_static!(a1, "A1");
    page_size_static!(a2, "A2");
    page_size_static!(a3, "A3");
    page_size_static!(a4, "A4");
    page_size_static!(a5, "A5");
    page_size_static!(a6, "A6");
    page_size_static!(a7, "A7");
    page_size_static!(a8, "A8");
    page_size_static!(a9, "A9");
    page_size_static!(a10, "A10");

    page_size_static!(b0, "B0");
    page_size_static!(b1, "B1");
    page_size_static!(b2, "B2");
    page_size_static!(b3, "B3");
    page_size_static!(b4, "B4");
    page_size_static!(b5, "B5");
    page_size_static!(b6, "B6");
    page_size_static!(b7, "B7");
    page_size_static!(b8, "B8");
    page_size_static!(b9, "B9");
    page_size_static!(b10, "B10");

    page_size_static!(default, "DEFAULT");
    page_size_static!(executive, "EXECUTIVE");
    page_size_static!(ledger, "LEDGER");
    page_size_static!(legal, "LEGAL");
    page_size_static!(letter, "LETTER");
    page_size_static!(tabloid, "TABLOID");
}

#[cfg(test)]
mod test {
    use crate::test::JVM;
    use super::*;

    #[test]
    fn new() {
        let jvm = JVM.lock();
        let env = jvm.attach_current_thread().unwrap();

        let size = PageSize::new(&env, 10.0, 10.0);
        assert!(size.is_ok());
    }

    macro_rules! size_test {
        ($name:ident) => {
            #[test]
            fn $name() {
                let jvm = JVM.lock();
                let env = jvm.attach_current_thread().unwrap();

                let size = PageSize::$name(&env);
                assert!(size.is_ok());
            }
        }
    }

    size_test!(a0);
    size_test!(a1);
    size_test!(a2);
    size_test!(a3);
    size_test!(a4);
    size_test!(a5);
    size_test!(a6);
    size_test!(a7);
    size_test!(a8);
    size_test!(a9);
    size_test!(a10);

    size_test!(b0);
    size_test!(b1);
    size_test!(b2);
    size_test!(b3);
    size_test!(b4);
    size_test!(b5);
    size_test!(b6);
    size_test!(b7);
    size_test!(b8);
    size_test!(b9);
    size_test!(b10);

    size_test!(default);
    size_test!(executive);
    size_test!(ledger);
    size_test!(legal);
    size_test!(letter);
    size_test!(tabloid);
}