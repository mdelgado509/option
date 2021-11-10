// a function for testing the `is_some` method 
fn test_is_some() {
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_some(), true);

    let x: Option<u32> = None;
    assert_eq!(x.is_some(), false);
}

// a function for testing the `is_none` method
fn test_is_none() {
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_none(), false);

    let x: Option<u32> = None;
    assert_eq!(x.is_none(), true);
}

// a function for teseting the `as_ref` method
fn test_as_ref() {
    let text: Option<String> = Some(String::from("Hello, world!"));
    
    // this line below would move text to the method call below
    // text cannot be moved and then borrowed for the assertion below
    // let text_length: Option<usize> = text.map(|s| s.len());

    // instead we use `as_ref` to cast `Option<String>` to `Option<&String>`
    // this ref is then consumed by `map` leaving `text` on the stack
    let text_length: Option<usize> = text.as_ref().map(|s| s.len());
    assert_eq!(text, Some(String::from("Hello, world!")));
    assert_eq!(text_length, Some(13));
}

fn main() {
    test_is_some();
    
    test_is_none();

    test_as_ref();
}
