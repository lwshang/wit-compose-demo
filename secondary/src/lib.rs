wit_bindgen::generate!({
    world: "secondary",

    exports: {
        "example:secondary/foo" : Foo,
    },
});

use exports::example::secondary::foo::Guest;

struct Foo;

impl Guest for Foo {
    fn bar() -> i64 {
        42
    }
}
