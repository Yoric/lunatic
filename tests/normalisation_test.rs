//! Test functionality of WASM functions generated by normalisation.

use lunatic_vm::linker::{engine, LunaticLinker};
use lunatic_vm::normalisation::patch;
use lunatic_vm::process::MemoryChoice;
use wasmtime::{ExternRef, Module, Val};

#[test]
fn externref_save_drop() {
    // Setup
    let wasm = wat::parse_str(
        r#"
    (module
        ;; Importing to force the generation of _lunatic_externref_save and _lunatic_externref_drop
        ;; by the normaliser's `extern_func_ref.rs` patch.
        (import "lunatic" "spawn" (func $spawn (param i32 i32 i64) (result i32)))
        (import "lunatic" "drop_externref" (func (;1;) (param i32)))
    )
    "#,
    )
    .unwrap();

    let (_, wasm) = patch(wasm.as_ref()).unwrap();

    let engine = engine();
    let module = Module::new(&engine, wasm).unwrap();

    let mut linker = LunaticLinker::new(engine, module, 0, MemoryChoice::New(32)).unwrap();
    let wasmtime_linker = linker.linker();
    wasmtime_linker
        .func("test", "echo", |value: i32| -> Option<ExternRef> {
            Some(ExternRef::new(value))
        })
        .unwrap();

    let instance = linker.instance().unwrap();

    // Save tests
    let externref_save = instance.get_func("_lunatic_externref_save").unwrap();

    let externref = Some(ExternRef::new(0));
    let index = externref_save.call(&[Val::ExternRef(externref)]).unwrap()[0].i32();
    // Stdin, Stdout & Stderr occupy places 0-2
    assert_eq!(Some(3), index);

    let externref = Some(ExternRef::new(1));
    let index = externref_save.call(&[Val::ExternRef(externref)]).unwrap()[0].i32();
    assert_eq!(Some(4), index);

    let externref = Some(ExternRef::new(2));
    let index = externref_save.call(&[Val::ExternRef(externref)]).unwrap()[0].i32();
    assert_eq!(Some(5), index);

    let externref = Some(ExternRef::new(3));
    let index = externref_save.call(&[Val::ExternRef(externref)]).unwrap()[0].i32();
    assert_eq!(Some(6), index);

    let externref = Some(ExternRef::new(4));
    let index = externref_save.call(&[Val::ExternRef(externref)]).unwrap()[0].i32();
    assert_eq!(Some(7), index);

    let externref = Some(ExternRef::new(5));
    let index = externref_save.call(&[Val::ExternRef(externref)]).unwrap()[0].i32();
    assert_eq!(Some(8), index);

    let externref = Some(ExternRef::new(6));
    let index = externref_save.call(&[Val::ExternRef(externref)]).unwrap()[0].i32();
    assert_eq!(Some(9), index);

    let externref = Some(ExternRef::new(7));
    let index = externref_save.call(&[Val::ExternRef(externref)]).unwrap()[0].i32();
    assert_eq!(Some(10), index);

    let externref = Some(ExternRef::new(8));
    let index = externref_save.call(&[Val::ExternRef(externref)]).unwrap()[0].i32();
    assert_eq!(Some(11), index);

    // Externref value check
    let table = instance
        .get_table("__lunatic_externref_resource_table")
        .unwrap();
    let value_externref = table.get(3).unwrap().unwrap_externref().unwrap();
    let value = value_externref.data().downcast_ref::<i32>().unwrap();
    assert_eq!(*value, 0);

    let value_externref = table.get(4).unwrap().unwrap_externref().unwrap();
    let value = value_externref.data().downcast_ref::<i32>().unwrap();
    assert_eq!(*value, 1);

    let value_externref = table.get(5).unwrap().unwrap_externref().unwrap();
    let value = value_externref.data().downcast_ref::<i32>().unwrap();
    assert_eq!(*value, 2);

    let value_externref = table.get(6).unwrap().unwrap_externref().unwrap();
    let value = value_externref.data().downcast_ref::<i32>().unwrap();
    assert_eq!(*value, 3);

    let value_externref = table.get(7).unwrap().unwrap_externref().unwrap();
    let value = value_externref.data().downcast_ref::<i32>().unwrap();
    assert_eq!(*value, 4);

    let value_externref = table.get(8).unwrap().unwrap_externref().unwrap();
    let value = value_externref.data().downcast_ref::<i32>().unwrap();
    assert_eq!(*value, 5);

    let value_externref = table.get(9).unwrap().unwrap_externref().unwrap();
    let value = value_externref.data().downcast_ref::<i32>().unwrap();
    assert_eq!(*value, 6);

    let value_externref = table.get(10).unwrap().unwrap_externref().unwrap();
    let value = value_externref.data().downcast_ref::<i32>().unwrap();
    assert_eq!(*value, 7);

    let value_externref = table.get(11).unwrap().unwrap_externref().unwrap();
    let value = value_externref.data().downcast_ref::<i32>().unwrap();
    assert_eq!(*value, 8);

    // Drop tests
    let externref_drop = instance.get_func("_lunatic_externref_drop").unwrap();

    externref_drop.call(&[Val::I32(4)]).unwrap();
    let value_externref = table.get(4).unwrap().unwrap_externref();
    assert!(value_externref.is_none());

    externref_drop.call(&[Val::I32(6)]).unwrap();
    let value_externref = table.get(6).unwrap().unwrap_externref();
    assert!(value_externref.is_none());

    // Now 1 & 3 are free
    let externref = Some(ExternRef::new(1337));
    let index = externref_save.call(&[Val::ExternRef(externref)]).unwrap()[0].i32();
    assert_eq!(Some(6), index);

    let externref = Some(ExternRef::new(1338));
    let index = externref_save.call(&[Val::ExternRef(externref)]).unwrap()[0].i32();
    assert_eq!(Some(4), index);

    let value_externref = table.get(4).unwrap().unwrap_externref().unwrap();
    let value = value_externref.data().downcast_ref::<i32>().unwrap();
    assert_eq!(*value, 1338);

    let value_externref = table.get(6).unwrap().unwrap_externref().unwrap();
    let value = value_externref.data().downcast_ref::<i32>().unwrap();
    assert_eq!(*value, 1337);
}

#[test]
fn externref_multivalue_return_as_params() {
    // TODO: Currently the normalisation only works with already defined functions.
    // This makes it really hard to test ad-hoc functions, as they are not part of the linker.
    // The normaliser should take a predefined linker, so that we can inject functions for testing.
}
