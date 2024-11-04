# Variable Synth Plugin
[![Automated Builds](https://github.com/OseMine/variable-synth/actions/workflows/build.yml/badge.svg)](https://github.com/OseMine/variable-synth/actions/workflows/build.yml)

A versatile audio synthesizer plugin that implements various waveform types and allows for dynamic parameter control.

## Project Structure

- `src/`
  - `lib.rs`: Main plugin file, contains the plugin implementation and audio processing logic.
  - `params.rs`: Defines the plugin parameters.
  - `synth/`
    - `mod.rs`: Manages the different synthesizer types and their selection.
    - `saw.rs`: Implementation of the saw wave synthesizer.
    - `sine.rs`: Implementation of the sine wave synthesizer.

## Adding a New Synthesizer

1. Create a new file in `src/synth/`, e.g., `square.rs`.
2. Implement the synthesizer in this file:

```rust
pub struct SquareOscillator {
    // Oscillator-specific fields
}

impl Oscillator for SquareOscillator {
    fn new(sample_rate: f32) -> Self {
        // Initialization logic
    }

    fn set_frequency(&mut self, freq: f32) {
        // Frequency setting logic
    }

    fn generate(&mut self) -> f32 {
        // Waveform generation logic
    }

    fn reset(&mut self) {
        // Reset logic
    }
}
```

3. Add the new synthesizer in `src/synth/mod.rs`:

```rust
pub mod square;
use square::SquareOscillator;

// Add the new synthesizer to the SynthType enumeration
pub enum SynthType {
    // ...
    Square,
}

// Update the Synthesizer structure
impl Synthesizer {
    // ...
    pub fn set_synth_type(&mut self, synth_type: SynthType) {
        self.oscillator = match synth_type {
            // ...
            SynthType::Square => Box::new(SquareOscillator::new(self.sample_rate)),
        };
    }
}
```

4. Update `src/params.rs` to add the new synthesizer type as an option:

```rust
#[derive(Enum, PartialEq, Clone)]
pub enum SynthType {
    // ...
    Square,
}
```

5. In `src/lib.rs`, update the `process` function to consider the new synthesizer type:

```rust
self.synth.set_synth_type(match synth_type {
    // ...
    ParamSynthType::Square => SynthType::Square,
});
```

## Contributing

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/AmazingFeature`).
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`).
4. Push to the branch (`git push origin feature/AmazingFeature`).
5. Open a Pull Request.

## Building the Project

1. Ensure Rust and Cargo are installed.
2. Clone the repository: `git clone https://github.com/OseMine/variable-synth.git`
3. Navigate to the project directory: `cd variable-synth`
4. Install all the necessary dependencies: `cargo fetch`
5. Build the project: `cargo xtask bundle variable-synth --release`
6. Find the plugin files in the `target/release` directory.

## Using the Plugin

- Copy the created plugin files to your VST3/CLAP plugin directory.
- Load the plugin in your preferred DAW.

## License

[MIT](https://choosealicense.com/licenses/mit/)