fn main() {
    {
        let x = Foo::new();
        call(x.clone());
        call(x.clone()); // this can just pass `x`
    }

    ["lorem", "ipsum"].join(" ").to_string();

    Path::new("/a/b").join("c").to_path_buf();
}