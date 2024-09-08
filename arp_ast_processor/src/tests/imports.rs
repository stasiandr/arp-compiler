use arp_types::sources::Source;

use crate::build_multiple_sources;


#[test]
fn import_class() {
    let sources = [
        Source::new_inline("Main.arp", "from another import MyClass fn func() { let x = MyClass{}; }"),
        Source::new_inline("another.arp", "class MyClass { }"),
    ];

    build_multiple_sources(&sources).unwrap();
}



#[test]
fn import_class_field() {
    let sources = [
        Source::new_inline("Main.arp", "from another import MyClass fn func() { let x = MyClass{}; let y = x.field; }"),
        Source::new_inline("another.arp", "class MyClass { field: int32 }"),
    ];

    build_multiple_sources(&sources).unwrap();
}



#[test]
fn import_child_path() {
    let sources = [
        Source::new_inline("Main.arp", "from path.to.another import MyClass fn func() { let x = MyClass{}; let y = x.field; }"),
        Source::new_inline("path.to.another.arp", "class MyClass { field: int32 }"),
    ];

    build_multiple_sources(&sources).unwrap();
}



#[test]
fn import_class_method() {
    let sources = [
        Source::new_inline("Main.arp", "from another import MyClass fn func() { let x = MyClass{}; let y = x.field; }"),
        Source::new_inline("another.arp", "class MyClass { field: int32 }  impl MyClass { fn instance_method(this) -> int32 { 5 } }"),
    ];

    build_multiple_sources(&sources).unwrap();
}

#[test]
fn import_class_static_method() {
    let sources = [
        Source::new_inline("Main.arp", "from another import MyClass fn func() { let x = MyClass.static_method(); let y = x.field; }"),
        Source::new_inline("another.arp", "class MyClass { field: int32 }  impl MyClass { fn static_method() -> MyClass { MyClass { 5 } } }"),
    ];

    build_multiple_sources(&sources).unwrap();
}

#[test]
fn import_graph() {
    let sources = [
        Source::new_inline("Main.arp", "from Another import AnotherClass    from YetAnother import YetAnotherClass "),
        Source::new_inline("Another.arp", "from SomeOther import SomeOtherClass  class AnotherClass { }"),
        Source::new_inline("YetAnother.arp", "class YetAnotherClass { } "),
        Source::new_inline("SomeOther.arp", "class SomeOtherClass { } "),
    ];

    build_multiple_sources(&sources).unwrap();
}


#[test]
fn multiple_imports() {
    let sources = [
        Source::new_inline("Main.arp", "from YetAnother import YetAnotherClass, SomeOtherClass "),
        Source::new_inline("YetAnother.arp", "class YetAnotherClass { } class SomeOtherClass { } "),
    ];

    build_multiple_sources(&sources).unwrap();
}


#[test]
fn namespaced_imports() {
    let sources = [
        Source::new_inline("Main.arp", "from YetAnother import My.Names.Space.Some.Other.Class "),
        Source::new_inline("YetAnother.arp", "class My.Names.Space.Some.Other.Class { } "),
    ];

    build_multiple_sources(&sources).unwrap();
}