// Cymbalum, an extensible molecular simulation engine
// Copyright (C) 2015-2016 G. Fraux — BSD license
use toml::Parser;

use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

use input::{Error, Result};
use input::validate;
use input::error::toml_error_to_string;

use simulation::Simulation;
use system::System;

mod system;
mod outputs;
mod propagator;
mod simulations;
mod md;
mod mc;

use self::system::read_system;
use self::simulations::{read_simulation, read_nsteps};

/// A configuration about how to run a single simulation. This contains the
/// system to simulate, the simulation itself and the number of steps to run
/// the simulation.
pub struct SimulationConfig {
    /// The simulated system
    pub system: System,
    /// The simulation object
    pub simulation: Simulation,
    /// The simulation duration
    pub nsteps: usize,
}

/// Read a whole simulation input file.
pub fn read_config<P: AsRef<Path>>(path: P) -> Result<SimulationConfig> {
    let mut file = try!(File::open(path));
    let mut buffer = String::new();
    let _ = try!(file.read_to_string(&mut buffer));

    let mut parser = Parser::new(&buffer);
    let config = match parser.parse() {
        Some(config) => config,
        None => {
            let errors = toml_error_to_string(&parser);
            return Err(Error::TOML(errors));
        }
    };

    try!(validate(&config));

    let system = try!(read_system(&config));
    let simulation = try!(read_simulation(&config));
    let nsteps = try!(read_nsteps(&config));

    Ok(SimulationConfig {
        system: system,
        simulation: simulation,
        nsteps: nsteps,
    })
}

#[cfg(test)]
mod tests {
    use input::read_config;
    use input::testing::bad_inputs;

    #[test]
    fn bad_input() {
        for path in bad_inputs("simulations", "generic") {
            assert!(read_config(path).is_err());
        }
    }
}