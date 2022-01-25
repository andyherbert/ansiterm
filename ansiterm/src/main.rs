mod terminal;
use ansiparser::{Parser, Sequence};
use ansiplay::{Player, PlayerThread};
use pixels::{Pixels, SurfaceTexture};
use rodio::OutputStream;
use stdin_receiver::StdInReceiver;
use terminal::Terminal;
pub use winit::event::VirtualKeyCode;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

struct Modifiers {
    alt: bool,
    logo: bool,
    shift: bool,
}

fn main() {
    let (_out, stream) = OutputStream::try_default().unwrap();
    let mut term = Terminal::new(80, 25, false);
    let (width, height) = term.get_dimensions();
    let mut stdin = Some(StdInReceiver::default());
    let title = "ANSi Terminal";
    let event_loop = EventLoop::new();
    const SCALE: u32 = 2;
    const BORDER_SIZE: u32 = 8;
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(
            (width * SCALE) + (BORDER_SIZE * 2),
            (height * SCALE) + (BORDER_SIZE * 2),
        ))
        .with_title(title)
        .with_resizable(false)
        .build(&event_loop)
        .expect("window");
    let mut pixels = {
        let surface_texture = {
            let window_size = window.inner_size();
            SurfaceTexture::new(window_size.width, window_size.height, &window)
        };
        Pixels::new(width, height, surface_texture).expect("cannot create pixels")
    };
    let mut modifiers = Modifiers {
        alt: false,
        logo: false,
        shift: false,
    };
    let mut parser = Parser::default();
    let mut player = Some(Player::new());
    let mut spawned_player: Option<PlayerThread> = None;
    event_loop.run(move |event, _window, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
                if let Some(stdin) = stdin.take() {
                    if let Err(err) = stdin.join() {
                        eprintln!("{err}");
                    }
                }
            }
            Event::WindowEvent {
                event: WindowEvent::ModifiersChanged(state),
                ..
            } => {
                modifiers.alt = state.alt();
                modifiers.logo = state.logo();
                modifiers.shift = state.shift();
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            winit::event::KeyboardInput {
                                virtual_keycode: Some(virtual_keycode),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            } => match virtual_keycode {
                // VirtualKeyCode::Up => term.cursor_up(1),
                // VirtualKeyCode::Down => term.cursor_down(1),
                // VirtualKeyCode::Left => term.cursor_back(1),
                // VirtualKeyCode::Right => term.cursor_forward(1),
                // VirtualKeyCode::Tab => term.tab(),
                #[cfg(target_os = "macos")]
                VirtualKeyCode::W if modifiers.logo => *control_flow = ControlFlow::Exit,
                #[cfg(target_os = "linux")]
                #[cfg(target_os = "windows")]
                VirtualKeyCode::F4 if modifiers.alt => *control_flow = ControlFlow::Exit,
                VirtualKeyCode::Escape => {
                    if let Some(ref player) = spawned_player {
                        player.abort().expect("thread error");
                    }
                }
                _ => {
                    if let Some(ref player) = spawned_player {
                        player.interrupt().expect("thread error");
                    }
                }
            },
            Event::RedrawRequested(_) => {
                term.next_frame(pixels.get_frame());
                pixels.render().expect("Unable to render");
            }
            _ => {}
        }
        if let Some(ref stdin) = stdin {
            match stdin.recv() {
                Ok(Some(bytes)) => parser.input(bytes),
                Ok(None) => {}
                Err(err) => eprintln!("{err}"),
            }
        }
        if let Some(ref thread) = spawned_player {
            match thread.finished_playing() {
                Ok(true) => match spawned_player.take() {
                    Some(thread) => {
                        player = match thread.join() {
                            Ok(player) => Some(player),
                            Err(err) => {
                                eprintln!("{err}");
                                None
                            }
                        }
                    }
                    None => unreachable!("Some(thread) already checked"),
                },
                Ok(false) => window.request_redraw(),
                Err(err) => eprintln!("{err}"),
            }
        } else if let Some(sequence) = parser.next() {
            match sequence {
                Sequence::Literal(byte) => term.literal(byte),
                Sequence::CarriageReturn => term.carriage_return(),
                Sequence::LineFeed => term.line_feed(),
                Sequence::Tab => term.tab(),
                Sequence::CursorUp(amount) => term.cursor_up(amount),
                Sequence::CursorDown(amount) => term.cursor_down(amount),
                Sequence::CursorForward(amount) => term.cursor_forward(amount),
                Sequence::CursorBack(amount) => term.cursor_back(amount),
                Sequence::CursorPosition { row, column } => {
                    term.move_cursor_to(column, row);
                    if column == 1 && row == 1 {
                        window.request_redraw();
                    }
                }
                Sequence::SetScreenMode(value) => term.set_screen_mode(value),
                Sequence::ResetScreenMode(value) => term.reset_screen_mode(value),
                Sequence::EraseDisplay(value) => term.erase_display(value),
                Sequence::EraseInLine(value) => term.erase_in_line(value),
                Sequence::SelectGraphicsRendition(values) => term.select_graphics_rendition(values),
                Sequence::SavePosition => term.save_cursor_position(),
                Sequence::RestorePosition => term.restore_cursor_position(),
                Sequence::SauceRecord(sauce) => {
                    println!("{sauce}");
                }
                Sequence::PabloTrueColourBackground { red, green, blue } => {
                    term.true_colour_bg(red, green, blue)
                }
                Sequence::PabloTrueColourForeground { red, green, blue } => {
                    term.true_colour_fg(red, green, blue)
                }
                Sequence::Music(music) => {
                    if let Some(player) = player.take() {
                        spawned_player = match PlayerThread::new(player, &stream, music) {
                            Ok(thread) => Some(thread),
                            Err(err) => {
                                eprintln!("{err}");
                                None
                            }
                        }
                    }
                }
                Sequence::Update => window.request_redraw(),
                Sequence::Unknown { bytes, terminator } => println!("{:?}, {}", bytes, terminator),
            }
        } else {
            window.request_redraw();
        }
    });
}
