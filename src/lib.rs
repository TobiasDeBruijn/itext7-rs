mod bindings;
mod error;
mod types;
mod traits;

pub use bindings::*;
pub use error::*;
pub use types::*;

#[cfg(test)]
mod test {
    use jni::{JavaVM, JNIVersion, InitArgsBuilder};
    use lazy_static::lazy_static;
    use parking_lot::Mutex;

    pub const PDF_SAMPLE_PATH: &str = const_format::formatcp!("{}/target/sample.pdf", env!("CARGO_MANIFEST_DIR"));

    lazy_static! {
        pub static ref JVM: Mutex<JavaVM> = {
            let args = InitArgsBuilder::new()
                .version(JNIVersion::V8)
                .option("-Xcheck:jni")

                // Add itext7 to the class path
                .option(&format!("-Djava.class.path={}/target/itext7-gradle/build/libs/itext7-gradle-all.jar", env!("CARGO_MANIFEST_DIR")))
                .build()
                .unwrap();

            Mutex::new(JavaVM::new(args).unwrap())
        };
    }
}
