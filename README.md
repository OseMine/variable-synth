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

This guide explains how to add a new oscillator (synth engine) to the plugin.

### Step 1: Create the Oscillator File

1. In `src/synth/`, create a new file for the oscillator, such as `square.rs`.
2. Implement the oscillator with the required trait methods as follows:

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

### Step 2: Register the Oscillator in `mod.rs`

1. Open `src/synth/mod.rs`.
2. Add the new oscillator as a module:

   ```rust
   pub mod square;
   use square::SquareOscillator;
   ```

3. Extend the `SynthType` enum to include the new oscillator type:

   ```rust
   pub enum SynthType {
       // Other synth types...
       Square,
   }
   ```

4. Update the `Synthesizer` structure to support the new oscillator type:

   ```rust
   impl Synthesizer {
       // Method to set the oscillator type
       pub fn set_synth_type(&mut self, synth_type: SynthType) {
           self.oscillator = match synth_type {
               // Other types...
               SynthType::Square => Box::new(SquareOscillator::new(self.sample_rate)),
           };
       }
   }
   ```

### Step 3: Update Plugin Parameters in `params.rs`

1. In `src/params.rs`, add the new oscillator type to the `SynthType` enum:

   ```rust
   #[derive(Enum, PartialEq, Clone)]
   pub enum SynthType {
       // Other synth types...
       Square,
   }
   ```

### Step 4: Modify the Main Processing Logic

1. In `src/lib.rs`, update the `process` function to account for the new oscillator type. For example:

   ```rust
   self.synth.set_synth_type(match synth_type {
       // Other synth types...
       ParamSynthType::Square => SynthType::Square,
   });
   ```

### Step 5: Rebuild the Project

After implementing the steps above, rebuild the project to test your new oscillator.

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
4. Install all necessary dependencies: `cargo fetch`
5. Build the project: `cargo xtask bundle variable-synth --release`
6. Find the plugin files in the `target/release` directory.

## Using the Plugin

- Copy the created plugin files to your VST3/CLAP plugin directory.
- Load the plugin in your preferred DAW.

## License

[MIT](LICENSE)