fn variable_binding() {
    let an_integer: u32 = 1u32;
    let a_boolean: bool = true;
    let unit: () = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer: u32 = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable: u32 = 3u32;

    let _noisy_unused_variable: u32 = 2u32;
    // FIXME ^ Prefix with an underscore to suppress the warning
    // Please note that warnings may not be shown in a browser
}

fn mutability_binding() {
    let _immutable_binding: i32 = 1;
    let mut mutable_binding: i32 = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error! Cannot assign a new value to an immutable variable
    // _immutable_binding += 1;
}

fn scope_binding() {
    // This binding lives in the main function
    let long_lived_binding: i32 = 1;

    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding: i32 = 2;

        println!("inner short: {}", short_lived_binding);
    }
    // End of the block

    // Error! `short_lived_binding` doesn't exist in this scope
    // println!("outer short: {}", short_lived_binding);
    // FIXME ^ Comment out this line

    println!("outer long: {}", long_lived_binding);
}
fn shadow_binding() {
    let shadowed_binding: i32 = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        // This binding *shadows* the outer one
        let shadowed_binding: &str = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    // This binding *shadows* the previous binding
    let shadowed_binding: i32 = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}

fn declare_binding() {
    // Declare a variable binding
    let a_binding: i32;

    {
        let x: i32 = 12i32;

        // Initialize the binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding: i32;

    // Error! Use of uninitialized binding
    // println!("another binding: {}", another_binding);
    // FIXME ^ Comment out this line

    another_binding = 1;

    println!("another binding: {}", another_binding);
}

fn freeze_binding() {
    let mut _mutable_integer: i32 = 7;

    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer: i32 = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        // _mutable_integer = 50;
        // FIXME ^ Comment out this line

        println!("  frozen {}", _mutable_integer);

        // `_mutable_integer` goes out of scope
    }

    // Ok! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
    println!("unfrozen {}", _mutable_integer);
}

fn main() {
    println!("########################################");
    println!("variable_binding");
    println!("########################################");
    variable_binding();
    println!("########################################");
    println!("mutability_binding");
    println!("########################################");
    mutability_binding();
    println!("########################################");
    println!("scope_binding");
    println!("########################################");
    scope_binding();
    println!("########################################");
    println!("shadow_binding");
    println!("########################################");
    shadow_binding();
    println!("########################################");
    println!("declare_binding");
    println!("########################################");
    declare_binding();
    println!("########################################");
    println!("declare_binding");
    println!("########################################");
    declare_binding();
    println!("########################################");
    println!("freeze_binding");
    println!("########################################");
    freeze_binding();
}
