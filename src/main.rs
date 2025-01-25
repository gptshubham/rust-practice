#![allow(unused_variables)] // compiler directive
/*
fn main() {
    // this is a comment in Rust
    println!("Hello rustc! I'm a Rust Developer.") // this is the key line in our program
                                                   // println!("This line will be ignored...")

    /*
    This is a multi line comment in Rust
    Copyright 2025
    Last updated 23/01
    Author: Shubham
    */
}
*/

/*
fn main() {
    println!("Hello, I'm SKG exploring Rust.");
    println!("I'm a Front-End Developer.");
    println!("I have built multiple vanilla JS projects.")
}
*/

// Variables and Mutability

const TAX_RATE: i32 = 30; // constant

type Meters = i32; // type alias

// #[allow(unused_variables)] // compiler directive
fn main() {
    let apples_in_garden = 50;
    let oranges_in_garden = 14 + 6;

    // handling unused variables warning using underscore (_)
    let _fruits_in_garden = apples_in_garden + oranges_in_garden;

    println!("My garden has {} apples.", apples_in_garden);
    println!("My garden has {} fruits.", apples_in_garden + oranges_in_garden);

    // Interpolation
    println!("My garden has {oranges_in_garden} oranges.");
    // println!("My garden has {apples_in_garden + oranges_in_garden} fruits.");
    // error: invalid format string: expected `}`, found `+`
    println!("My garden has {apples_in_garden} apples and {oranges_in_garden} oranges.");
    
    // Positional Arguments to println!()
    println!("My garden has {0} apples and {1} oranges. I can't believe I have {0} apples.", apples_in_garden, oranges_in_garden);

    // Mutability and Immutability of Variables --> (Immutable by default)

    // apples_in_garden = 100; // * Rust Error Code Index *
    // error[E0384]: cannot assign twice to immutable variable `apples_in_garden`
    // cannot assign twice to immutable variable

    // declaring Mutable variables using mut keyword
    let mut bananas = 200;
    println!("{bananas}");
    
    bananas = 300;
    println!("{bananas}");

    // Variable Shadowing
    let grams_of_protein = "100.345";
    println!("Grams of protein as string: {grams_of_protein} ");
    let grams_of_protein = 100.345;
    println!("Grams of protein as float: {grams_of_protein}");
    let mut grams_of_protein = 100;
    println!("Grams of protein as integer: {grams_of_protein}");
    grams_of_protein = 105;
    println!("Grams of protein as integer (Mutable): {grams_of_protein}");

    // Scope

    let coffee_price = 5.99;
    // scope of coffee_price variable => main function

    {
        let coffee_price = 3.99;
        println!("The price of coffee is {coffee_price}");

        let cookie_price = 1.99;

        println!("The price of the cookie is {cookie_price}");

    }

    println!("The price of coffee is {coffee_price}");

    // println!("The price of the cookie is {cookie_price}")
    // error[E0425]: cannot find value `cookie_price` in this scope

    // constant

    let income = 1000000;

    let tax = income * TAX_RATE / 100;

    println!("The tax rate is {TAX_RATE}");
    println!("The tax on income of {income} is {tax}.");

    // Type Alias

    let mile_race_length: Meters = 1600;

    let two_mile_race_length: Meters = 3200;

    println!("A 1 mile race is {mile_race_length} meters long and a two mile race is {two_mile_race_length} meters long.");

    // Compiler Directive => apply above the line or above the main function or to the file

    // #[allow(unused_variables)]
    let mile_race_length: Meters = 1600;

    // #[allow(unused_variables)]
    let two_mile_race_length: Meters = 3200;

}