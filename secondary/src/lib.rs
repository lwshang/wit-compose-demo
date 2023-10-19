wit_bindgen::generate!({
    world: "secondary",

    exports: {
        "example:secondary/foo": Foo,
    },
});

struct Foo;

impl exports::example::secondary::foo::Guest for Foo {
    fn bar() -> i64 {
        42
    }
}
