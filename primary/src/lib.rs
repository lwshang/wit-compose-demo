wit_bindgen::generate!({
    world: "primary",

    exports: {
        world: Primary,
    },
});

struct Primary;

impl Guest for Primary {
    fn double_bar() -> i64 {
        example::secondary::foo::bar() * 2
    }
}
