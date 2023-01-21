use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use inquire::{
    validator::{StringValidator, Validation},
    Text,
};

pub fn start_interactive_shell() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    // while running.load(Ordering::SeqCst) {
    let validator = |input: &str| {
        if input.chars().count() > 140 {
            Ok(Validation::Invalid(
                "You're only allowed 140 characters.".into(),
            ))
        } else {
            Ok(Validation::Valid)
        }
    };

    let status = Text::new("What are you thinking about?")
        .with_validator(validator)
        .with_autocomplete(vec!["test"])
        .prompt();

    match status {
        Ok(status) => println!("Your status is being published...{}", status),
        Err(err) => println!("Error while publishing your status: {}", err),
    }
    // }
    println!("Got it! Exiting...");
}
