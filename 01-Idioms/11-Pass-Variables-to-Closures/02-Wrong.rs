use std::rc::Rc;

fn main() {
    let num1 = Rc::new(1);
    let num2 = Rc::new(2);
    let num3 = Rc::new(3);

    let num2_cloned = num2.clone();
    let num3_borrowed = num3.as_ref();
    let closure = move || {
        *num1 + *num2_cloned + *num3_borrowed;
    };
}