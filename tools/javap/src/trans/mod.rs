use classfile::{ClassFile, SignatureType};

mod access_flag;
mod class_file;
mod code;
mod constant_pool_trans;
mod field;
mod instruction;
mod method;
mod signature_type;

pub use self::access_flag::AccessFlagHelper;
pub use self::access_flag::Translator as AccessFlagsTranslator;
pub use self::class_file::Translator as ClassFileTranslator;
pub use self::code::Translator as CodeTranslator;
pub use self::constant_pool_trans::Translator as ConstantPoolTranslator;
pub use self::field::FieldTranslation;
pub use self::field::Translator as FieldTranslator;
pub use self::method::MethodTranslation;
pub use self::method::Translator as MethodTranslator;
pub use self::signature_type::Translator as SignatureTypeTranslator;

pub fn class_source_file(cf: &ClassFile) -> String {
    let x = ClassFileTranslator::new(cf);
    x.source_file()
}

pub fn class_this_class(cf: &ClassFile) -> String {
    let x = ClassFileTranslator::new(cf);
    x.this_class()
}

pub fn class_super_class(cf: &ClassFile) -> String {
    let x = ClassFileTranslator::new(cf);
    x.super_class()
}

pub fn class_access_flags(cf: &ClassFile) -> String {
    let x = ClassFileTranslator::new(cf);
    x.access_flags()
}

pub fn class_access_flags_name(cf: &ClassFile) -> String {
    let x = ClassFileTranslator::new(cf);
    x.access_flags_name()
}

pub fn class_signature_raw(cf: &ClassFile) -> Option<String> {
    let x = ClassFileTranslator::new(cf);
    x.signature_raw()
}

pub fn class_signature(cf: &ClassFile) -> Option<Vec<SignatureType>> {
    let x = ClassFileTranslator::new(cf);
    x.signature()
}

pub fn class_fields(cf: &ClassFile, flags: u16) -> Vec<FieldTranslation> {
    let x = ClassFileTranslator::new(cf);
    x.fields(flags)
}

pub fn class_methods(
    cf: &ClassFile,
    with_line_num: bool,
    with_code: bool,
    flags: u16,
) -> Vec<MethodTranslation> {
    let x = ClassFileTranslator::new(cf);
    x.methods(with_line_num, with_code, flags)
}

pub fn class_parent_interfaces(cf: &ClassFile) -> Vec<String> {
    let x = ClassFileTranslator::new(cf);
    x.parent_interfaces()
}

pub fn class_constant_pool(cf: &ClassFile) -> Vec<String> {
    let x = ConstantPoolTranslator { cf };
    x.get()
}

pub fn class_inner_classes(cf: &ClassFile) -> Vec<String> {
    let x = ClassFileTranslator::new(cf);
    x.inner_classes()
}
