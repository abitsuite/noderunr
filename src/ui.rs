// src/ui.rs

/* Import modules. */
pub mod notif;
pub mod welcome;

#[cfg(test)]
#[path = "ui/welcome.test.rs"]
mod welcome_test;

#[cfg(test)]
#[path = "ui/notif.test.rs"]
mod notif_test;
