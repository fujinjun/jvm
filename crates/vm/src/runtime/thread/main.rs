use crate::oop::{self, Oop};
use crate::runtime::thread::thread_pool;
use crate::runtime::{self, init_vm, vm, DataArea, JavaCall, JavaThread};
use crate::types::{ClassRef, FrameRef, JavaThreadRef, MethodIdRef};
use crate::util;
use std::borrow::Borrow;

pub struct MainThread {
    pub class: String,
    pub args: Option<Vec<String>>,
    dispatch_uncaught_exception_called: bool,
}

impl MainThread {
    pub fn new(class: String, args: Option<Vec<String>>) -> Self {
        Self {
            class,
            args,
            dispatch_uncaught_exception_called: false,
        }
    }

    pub fn run(&mut self) {
        let vm = vm::VM::new(3);

        //attach 'main' thread
        vm.threads.attach_current_thread();

        info!("init vm start");
        init_vm::initialize_jvm();
        info!("init vm end");

        let main_class = oop::class::load_and_init(self.class.as_bytes());

        let mir = {
            let cls = main_class.read().unwrap();

            /*
            path info should be included in "--cp", and avoid same class load 2
            times, otherwise, "<clinit>" invoked 2 times.

            For example:
              "MyFile.java":
                private static File gf = newFile();

            if allowed, as follows:
              "cargo run -- --cp $JDK:$MY_TEST test/with_package/my.ns.HelloWorld"
            will cause "<clinit>" invoked 2 times, "newFile()" invoked 2 times,
            maybe create 2 files.

            should be like this:
              "cargo run -- --cp $JDK:$MY_TEST:test/with_package my.ns.HelloWorld"
            */
            if self.class.as_bytes() != cls.name.as_slice() {
                panic!("Error: Could not find or load main class {}", self.class);
            }

            cls.get_static_method(b"main", b"([Ljava/lang/String;)V")
        };

        let jt = runtime::thread::current_java_thread();
        match mir {
            Ok(mir) => {
                let arg = self.build_main_arg();
                let area = DataArea::new(0, 1);
                area.write().unwrap().stack.push_ref(arg);
                match JavaCall::new(area.clone(), mir) {
                    Ok(mut jc) => {
                        jt.write().unwrap().is_alive = true;
                        jc.invoke(Some(area), true);
                        jt.write().unwrap().is_alive = false;
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!("NotFound \"main\""),
        }

        if jt.read().unwrap().ex.is_some() {
            self.uncaught_ex(main_class);
        }

        //detach main thread
        vm.threads.detach_current_thread();

        vm.threads.join_all();
    }
}

impl MainThread {
    fn build_main_arg(&self) -> Oop {
        let args = match &self.args {
            Some(args) => args
                .iter()
                .map(|it| util::oop::new_java_lang_string2(it))
                .collect(),
            None => vec![],
        };

        //build ArrayOopDesc
        let ary_str_class = runtime::require_class3(None, b"[Ljava/lang/String;").unwrap();
        Oop::new_ref_ary2(ary_str_class, args)
    }

    fn uncaught_ex(&mut self, main_cls: ClassRef) {
        if self.dispatch_uncaught_exception_called {
            self.uncaught_ex_internal();
        } else {
            self.dispatch_uncaught_exception_called = true;
            self.call_dispatch_uncaught_exception(main_cls);
        }
    }

    fn call_dispatch_uncaught_exception(&mut self, main_cls: ClassRef) {
        let jt = runtime::thread::current_java_thread();
        let v = {
            let jt = jt.read().unwrap();
            jt.java_thread_obj.clone()
        };
        match v {
            Some(v) => {
                let cls = {
                    let v = util::oop::extract_ref(&v);
                    let v = v.read().unwrap();
                    match &v.v {
                        oop::RefKind::Inst(inst) => inst.class.clone(),
                        _ => unreachable!(),
                    }
                };

                let mir = {
                    let cls = cls.read().unwrap();
                    cls.get_this_class_method(
                        b"dispatchUncaughtException",
                        b"(Ljava/lang/Throwable;)V",
                    )
                };

                match mir {
                    Ok(mir) => {
                        let ex = {
                            let mut jt = jt.write().unwrap();
                            jt.take_ex().unwrap()
                        };
                        let args = vec![v.clone(), ex];
                        let mut jc = JavaCall::new_with_args(mir, args);
                        let area = runtime::DataArea::new(0, 0);
                        jc.invoke(Some(area), false);
                    }
                    _ => self.uncaught_ex_internal(),
                }
            }

            None => self.uncaught_ex_internal(),
        }
    }

    fn uncaught_ex_internal(&mut self) {
        let jt = runtime::thread::current_java_thread();
        let ex = {
            let mut jt = jt.write().unwrap();
            jt.take_ex().unwrap()
        };

        let cls = {
            match &ex {
                Oop::Ref(v) => {
                    let v = v.read().unwrap();
                    match &v.v {
                        oop::RefKind::Inst(inst) => inst.class.clone(),
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        };
        let detail_message = {
            let v = {
                let cls = cls.read().unwrap();
                let id = cls.get_field_id(b"detailMessage", b"Ljava/lang/String;", false);
                cls.get_field_value(&ex, id)
            };

            util::oop::extract_str(&v)
        };
        let name = {
            let cls = cls.read().unwrap();
            cls.name.clone()
        };

        let cls_name = String::from_utf8_lossy(name.as_slice());
        error!("Name={}, detailMessage={}", cls_name, detail_message);
    }
}
