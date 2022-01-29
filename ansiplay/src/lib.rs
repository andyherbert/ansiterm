mod music;
mod music_sequence_iterator;
mod player;
pub use basic_waves::rodio;
pub use music::Music;
pub use music_sequence_iterator::{IntoMusicSequenceIter, MusicSequenceIterator};
pub use player::{Player, PlayerThread};

#[cfg(test)]
mod test {
    use crate::{Music, Player};
    use basic_waves::rodio::{OutputStream, Sink};

    fn play_str(string: &str) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let mut player = Player::default();
        let music = Music::from(string);
        let sink = Sink::try_new(&stream_handle).expect("sink");
        player.play(music, &sink);
    }

    #[test]
    fn test() {
        play_str(";;;900000");
    }

    #[test]
    fn tutor_tune() {
        play_str("T200 L8 O4 C < B > C F4 C < G#4 A > C4 < F MS GGG MN G4 A# 892.32;1;8;;-19.04 O3 L2 F P2");
    }

    #[test]
    fn tutor_bird_call() {
        play_str("1397;4;2;250 2600;1.2;4;;150");
    }

    #[test]
    fn tutor_steam_ship() {
        play_str("57;15 37;25");
    }

    #[test]
    fn zapped_by_martians() {
        play_str("7000;.12;200;25;-100");
    }

    #[test]
    fn tutor_variation() {
        play_str("100;2;10;5;*");
    }
}
