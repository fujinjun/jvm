#![allow(non_snake_case)]

use crate::native::{new_fn, JNIEnv, JNINativeMethod, JNIResult};
use crate::oop::{self, Oop};
use crate::util;
use std::time::Duration;

pub fn get_native_methods() -> Vec<JNINativeMethod> {
    vec![
        new_fn("registerNatives", "()V", Box::new(jvm_registerNatives)),
        new_fn("hashCode", "()I", Box::new(jvm_hashCode)),
        new_fn("clone", "()Ljava/lang/Object;", Box::new(jvm_clone)),
        new_fn("getClass", "()Ljava/lang/Class;", Box::new(jvm_getClass)),
        new_fn("notifyAll", "()V", Box::new(jvm_notifyAll)),
        new_fn("wait", "(J)V", Box::new(jvm_wait)),
    ]
}

fn jvm_registerNatives(_env: JNIEnv, _args: Vec<Oop>) -> JNIResult {
    Ok(None)
}

pub fn jvm_hashCode(_env: JNIEnv, args: Vec<Oop>) -> JNIResult {
    let v = args.get(0).unwrap();
    let v = match v {
        Oop::Null => Oop::new_int(0),
        Oop::Ref(rf) => {
            let hash = rf.read().unwrap().hash_code.clone();
            match hash {
                Some(hash) => Oop::new_int(hash),
                None => {
                    let hash = util::oop::hash_code(v);
                    let mut v = rf.write().unwrap();
                    v.hash_code = Some(hash);
                    Oop::new_int(hash)
                }
            }
        }
        _ => unreachable!(),
    };

    Ok(Some(v))
}

fn jvm_clone(_env: JNIEnv, args: Vec<Oop>) -> JNIResult {
    //    let java_lang_Cloneable = require_class3(None, b"java/lang/Cloneable").unwrap();
    let this_obj = args.get(0).unwrap();
    Ok(Some(this_obj.clone()))
}

fn jvm_getClass(_env: JNIEnv, args: Vec<Oop>) -> JNIResult {
    let v = args.get(0).unwrap();
    let mirror = {
        let rf = util::oop::extract_ref(v);
        let rf = rf.read().unwrap();
        match &rf.v {
            oop::RefKind::Inst(inst) => {
                let cls = inst.class.read().unwrap();
                cls.get_mirror()
            }
            oop::RefKind::Array(ary) => ary.class.read().unwrap().get_mirror(),
            oop::RefKind::Mirror(_mirror) => {
                v.clone()

                /*
                let cls = mirror.target.clone().unwrap();
                let cls = cls.lock().unwrap();
                let name = String::from_utf8_lossy(cls.name.as_slice());
                error!("target cls = {}", name);
                cls.get_mirror()
                */
            }
            t => unimplemented!("t = {:?}", t),
        }
    };
    Ok(Some(mirror))
}

fn jvm_notifyAll(_env: JNIEnv, args: Vec<Oop>) -> JNIResult {
    let this = args.get(0).unwrap();
    let rf = util::oop::extract_ref(this);
    let rf = rf.read().unwrap();
    rf.notify_all();
    Ok(None)
}

fn jvm_wait(_env: JNIEnv, args: Vec<Oop>) -> JNIResult {
    let this = args.get(0).unwrap();
    let millis = args.get(1).unwrap();
    let millis = util::oop::extract_long(millis);
    let rf = util::oop::extract_ref(this);
    let rf = rf.read().unwrap();
    if millis == 0 {
        rf.wait();
    } else {
        rf.wait_timeout(Duration::from_millis(millis as u64));
    }
    Ok(None)
}
