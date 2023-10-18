wit_bindgen::generate!({
    world: "secondary",
    path: "../wit/secondary.wit",

    exports: {
        world: Secondary,
    },
});

struct Secondary;

impl Guest for Secondary {
    fn bar() -> i64 {
        42
    }
}
