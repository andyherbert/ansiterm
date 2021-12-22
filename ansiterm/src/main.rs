mod stdin;
mod terminal;
use ansiparser::{Parser, Sequence};
use ansiplay::player::Player;
use pixels::{Pixels, SurfaceTexture};
use rodio::OutputStream;
use stdin::spawn_stdin_channel;
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
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut term = Terminal::new(80, 25, false);
    let (width, height) = term.get_dimensions();
    let stdin = spawn_stdin_channel();
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
    let mut player = Player::new();
    event_loop.run(move |event, _window, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
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
                _ => {}
            },
            Event::RedrawRequested(_) => {
                term.next_frame(pixels.get_frame());
                pixels.render().expect("Unable to render");
            }
            _ => {}
        }
        if let Ok(bytes) = stdin.try_recv() {
            parser.input(bytes);
        }
        if player.is_playing() {
            window.request_redraw();
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
                Sequence::CursorPosition(row, column) => {
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
                Sequence::SauceRecord(_bytes) => {}
                Sequence::SauceComment(_bytes) => {}
                Sequence::PabloTrueColourBackground(r, g, b) => term.true_colour_bg(r, g, b),
                Sequence::PabloTrueColourForeground(r, g, b) => term.true_colour_fg(r, g, b),
                Sequence::Music(entities) => player.spawn_and_play(entities, &stream_handle),
                Sequence::Update => window.request_redraw(),
                Sequence::Unknown(vec, terminator) => {
                    println!("{:?}, {}", vec, terminator);
                }
            }
        } else {
            window.request_redraw();
        }
    });
}
