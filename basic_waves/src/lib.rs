/*! # Basic Waves
A selection of infinite sources for [Rodio](rodio) that generate sound waves.
 - [Noise Wave](https://en.wikipedia.org/wiki/White_noise)
 - [Saw Wave](https://en.wikipedia.org/wiki/Sawtooth_wave)
 - [Sine Wave](https://en.wikipedia.org/wiki/Sine_wave)
 - [Square Wave](https://en.wikipedia.org/wiki/Square_wave)
 - [Triangle Wave](https://en.wikipedia.org/wiki/Triangle_wave)
*/
#[cfg(test)]
mod test;

mod noise;
mod saw;
mod sine;
mod square;
mod triangle;
pub use noise::NoiseWave;
/// Re-export Rodio
pub use rodio;
pub use saw::SawWave;
pub use sine::SineWave;
pub use square::SquareWave;
pub use triangle::TriangleWave;
