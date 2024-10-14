use bindings::{addTwoValue, addValuesTogheter, SomeThingCool};

mod bindings;

fn main() {
    // Create a mutable struct that is defined in C
    let mut cool_struct: SomeThingCool = SomeThingCool { t: 1, d: 2.0 };


    // Will crash inside of the C code, can't be handled 
    //let value = unsafe { addValuesTogheter(std::ptr::null_mut() as *mut _)};

    // Send in a struct by reference in the C code. 
    let value = unsafe { addValuesTogheter(&mut cool_struct as *mut _)};
    println!("Struct added togheter {}", value);

    let value = unsafe { addTwoValue(1, 5) };
    println!("Values added togheter {}", value);
}
