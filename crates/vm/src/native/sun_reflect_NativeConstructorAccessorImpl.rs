#![allow(non_snake_case)]

use crate::native::{common, new_fn, JNIEnv, JNINativeMethod, JNIResult};
use crate::oop::{self, Oop};
use crate::runtime;
use crate::util;

pub fn get_native_methods() -> Vec<JNINativeMethod> {
    vec![new_fn(
        "newInstance0",
        "(Ljava/lang/reflect/Constructor;[Ljava/lang/Object;)Ljava/lang/Object;",
        Box::new(jvm_newInstance0),
    )]
}

fn jvm_newInstance0(_env: JNIEnv, args: Vec<Oop>) -> JNIResult {
    let ctor = args.get(0).unwrap();
    let arguments = args.get(1).unwrap();

    let clazz = common::reflect::get_Constructor_clazz(ctor);
    let target_cls = {
        let clazz = util::oop::extract_ref(&clazz);
        let v = clazz.read().unwrap();
        match &v.v {
            oop::RefKind::Mirror(mirror) => mirror.target.clone().unwrap(),
            _ => unreachable!(),
        }
    };

    let name = {
        let cls = target_cls.read().unwrap();
        cls.name.clone()
    };

    let signature = common::reflect::get_Constructor_signature(ctor);

    let name = unsafe { std::str::from_utf8_unchecked(name.as_slice()) };
    info!("newInstance0 {}:{}", name, signature);

    let mut ctor_args = Vec::new();
    {
        match arguments {
            Oop::Null => (),
            Oop::Ref(rf) => {
                let v = rf.read().unwrap();
                match &v.v {
                    oop::RefKind::Array(ary) => {
                        ctor_args.extend_from_slice(ary.elements.as_slice());
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }

    let oop = Oop::new_inst(target_cls.clone());
    ctor_args.insert(0, oop.clone());
    runtime::invoke::invoke_ctor(target_cls, signature.as_bytes(), ctor_args);

    Ok(Some(oop))
}
