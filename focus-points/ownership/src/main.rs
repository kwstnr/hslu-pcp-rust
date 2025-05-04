fn main() {
    println!("Hello, world!");

    // stack based values
    let int: i32 = 5;

    print_stack_value(int);
    println!("the creator of the int still has a copy of it: {int}");


    let s1 = String::from("This is on the heap");

    print_heap_value_without_returning_ownership(s1);

    // the following line thnrows a compiler error because the borrow-checker realised that we
    // don't have ownership of s1 here anymore
    // println!("We don' have ownership anymore, so can't print the string: {s1}");
    
    let s2 = String::from("This is also on the heap");

    // this works, because the function got a readonly reference to the variable
    print_heap_value_by_borrowing(&s2);
    println!("We still have owhnership, the function only borrowed a reference: {s2}");

    // we can have as many readonly references as we want
    let borrowed1 = &s2;
    let borrowed2 = &s2;
    print_heap_value_by_borrowing(&borrowed1);
    print_heap_value_by_borrowing(&borrowed2);

    let s3 = String::from("I am being moved around");

    let moved_s3 = print_heap_value_with_returning_ownership(s3);

    // The following line does also not compile
    // println!("s3 is no longer valid: {s3}")

    println!("But we have the new variable: {moved_s3}");

    let mut s4 = String::from("I am mutable");
    
    // this does not compile, since moved_s3 is not mutable (by default)
    // mutable_borrow(&mut moved_s3);

    mutable_borrow(&mut s4);
    println!("{s4}");

    // but we can't have multiple mutable and immutable references to the same variable
    let mut s5 = String::from("I am also mutable");

    let borrow1 = &mut s5;

    // this does still compile, because the mutable borrows, are not used yet!
    let borrow2 = &mut s5;
    let borrow3 = &s5;

    // the following lines do not compile because we can't have multiple mutable and immutable
    // borrows.
    // print_heap_value_by_borrowing(&borrow1);
    // print_heap_value_by_borrowing(&borrow2);
    // print_heap_value_by_borrowing(&borrow3);
}

fn print_stack_value(some_integer: i32) {
    println!("the fuction has the integers: {some_integer}");
}

fn print_heap_value_without_returning_ownership(string: String) {
    println!("The function now has ownership over the string, it is moved into here: {string}");
}

fn print_heap_value_by_borrowing(string: &String) {
    println!("The function borrowed a readonly reference of the string: {string}");
}

fn print_heap_value_with_returning_ownership(string: String) -> String {
    println!("This function receives ownership but returns it in the end, the value is moved and moved back: {string}");
    string
}

fn mutable_borrow(string: &mut String) {
    *string = String::from("I have been mutated");
}
