fn main() {
    // Bad
    let x = vec![42, 43];
    let y = x.iter();
    let z = y.map(|i| *i);

    // Why is this bad?
    // Readability, this can be written more concisely

    // Nice
    let x = vec![42, 43];
    let y = x.iter();
    let z = y.cloned();
}