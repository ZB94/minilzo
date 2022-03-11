fn main() {
    cc::Build::new()
        .include("minilzo-2.10")
        .file("minilzo-2.10/minilzo.c")
        .compile("minilzo")
}
