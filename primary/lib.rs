wit_bindgen::generate!({
    world: "primary",
    path: "../wit/primary.wit",

    exports: {
        world: Primary,
    },
});

struct Primary;

impl Guest for Primary {
    fn run() {
        todo!()
    }
}
