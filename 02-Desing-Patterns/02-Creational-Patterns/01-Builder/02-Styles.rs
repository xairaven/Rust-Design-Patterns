fn main() {
    let mut fb = FooBuilder::new();
    fb.a();
    fb.b();
    let f = fb.build();

    FooBuilder::new()
        .a()
        .b()
        .build()
}