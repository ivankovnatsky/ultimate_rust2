// 1. Bring the macros `error, warn, info, debug, trace` into scope from the log package with a
// `use` statement.
//
// You should be able to run `cargo build --lib` successfully after this step (and each step in this
// file)
//
// Hint: You need to update Cargo.toml to add the `log` dependency, first.

use log::*;

#[derive(Debug)]
pub struct Frog {
    energy: u8,
    sleeping: bool,
}

impl Frog {
    pub fn new() -> Self {
        debug!("A new Frog has been created");
        Default::default()
    }
    pub fn hop(&mut self) {
        self.energy -= 1;
        info!("Frog hopped, and {} energy is left", self.energy);
        if self.energy == 0 {
            warn!("Frog will go to sleep since he ran out of energy");
            self.sleep();
        }
    }
    pub fn sleep(&mut self) {
        if self.sleeping {
            error!("Frog is already asleep");
        } else {
            self.sleeping = true;
        }
    }
}

impl Default for Frog {
    fn default() -> Self {
        let frog = Frog {
            energy: 5,
            sleeping: false,
        };
        trace!(
            "Default value was generated, with the debug representation: {:?}",
            frog
        );

        frog
    }
}
