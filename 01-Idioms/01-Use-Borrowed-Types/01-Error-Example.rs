fn three_vowels(word: &String) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true;
                }
            }
            _ => vowel_count = 0,
        }
    }
    false
}

fn main() {
    let ferris = "Ferris".to_string();
    let curious = "Curious".to_string();
    println!("{}: {}", ferris, three_vowels(&ferris));
    println!("{}: {}", curious, three_vowels(&curious));

    // This works fine, but the following two lines would fail:
    // println!("Ferris: {}", three_vowels("Ferris"));
    // println!("Curious: {}", three_vowels("Curious"));
}

/*
error[E0308]: mismatched types
  --> src/main.rs:24:41
   |
24 |     println!("Ferris: {}", three_vowels("Ferris"));
   |                            ------------ ^^^^^^^^ expected `&String`, found `&str`
   |                            |
   |                            arguments to this function are incorrect
   |
   = note: expected reference `&String`
              found reference `&'static str`
note: function defined here
  --> src/main.rs:1:4
   |
1  | fn three_vowels(word: &String) -> bool {
   |    ^^^^^^^^^^^^ -------------
 */