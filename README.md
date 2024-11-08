# Variable Synth Plugin
[![Automated Builds](https://github.com/OseMine/variable-synth/actions/workflows/build.yml/badge.svg)](https://github.com/OseMine/variable-synth/actions/workflows/build.yml)

A versatile audio synthesizer plugin that implements various waveform types and allows for dynamic parameter control.

## Project Structure

- `src/`
  - `lib.rs`: Main plugin file, contains the plugin implementation and audio processing logic.
  - `params.rs`: Defines the plugin parameters.
  - `synth/`: Contains the waveform generation logic.

## Adding a New Synth Type

This guide explains in detail how to add a new synth type/engine/algorithm to the plugin.

### Step 1: Create the Synth Algorithm File

1. In the `src/synth/` directory, create a new file for your synth algorithm, e.g., `custom_synth.rs`.
2. Implement the `WaveformGenerator` trait for your new synth type:

```rust
use super::WaveformGenerator;

pub struct CustomSynth {
    // Add any necessary fields for your synth
}

impl WaveformGenerator for CustomSynth {
    fn generate(&self, phase: f32) -> f32 {
        // Implement your custom waveform generation logic here
        // The 'phase' parameter represents the current phase of the waveform (0 to 2π)
        // Return a value between -1.0 and 1.0
        
        // Example: Simple sine wave
        phase.sin()
    }
}
```

### Step 2: Update the Waveform Enum

1. Open `src/params.rs`.
2. Add your new synth type to the `Waveform` enum:

```rust
#[derive(Enum, PartialEq, Clone, Copy)]
pub enum Waveform {
    // Existing waveforms...
    #[name = "Custom Synth"]
    CustomSynth,
}
```

### Step 3: Update the Waveform Creation Function

1. Open `src/synth/mod.rs`.
2. Import your new synth type:

```rust
mod custom_synth;
use custom_synth::CustomSynth;
```

3. Update the `create_waveform` function to include your new synth type:

```rust
pub fn create_waveform(waveform: Waveform) -> Arc<dyn WaveformGenerator + Send + Sync> {
    match waveform {
        // Existing waveforms...
        Waveform::CustomSynth => Arc::new(CustomSynth::new()), // Assuming you have a new() method
    }
}
```

### Step 4: Implement Custom Parameters (Optional)

If your synth algorithm requires custom parameters:

1. Add new parameters to `VariableSynthParams` in `src/params.rs`:

```rust
#[derive(Params)]
pub struct VariableSynthParams {
    // Existing parameters...
    #[id = "custom_param"]
    pub custom_param: FloatParam,
}
```

2. Initialize the new parameter in the `Default` implementation:

```rust
impl Default for VariableSynthParams {
    fn default() -> Self {
        Self {
            // Existing parameters...
            custom_param: FloatParam::new(
                "Custom Param",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(50.0)),
        }
    }
}
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