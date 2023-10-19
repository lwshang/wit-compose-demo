wit_bindgen::generate!({
    world: "primary",

    exports: {
        world: Primary,
    },
});

struct Primary;

impl Guest for Primary {
    fn bar() -> i64 {
        example::secondary::foo::bar() * 2
    }
}
