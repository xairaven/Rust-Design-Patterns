fn main() {
    let data = {
        let mut data = get_vec();
        data.sort();
        data
    };

    // Here `data` is immutable.
}