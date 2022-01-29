pub mod terminal;
use ansiart::{
    ansiplay::{rodio::OutputStream, Player, PlayerThread},
    AnsiParser, Sequence,
};
use pixels::{Pixels, SurfaceTexture};
use terminal::Terminal;
pub use winit::{self, event::VirtualKeyCode};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const BORDER_SIZE: u32 = 8;

pub enum TerminalEvent {
    RedrawRequested,
    CloseRequested,
    Keypress {
        key_code: VirtualKeyCode,
        alt: bool,
        logo: bool,
        shift: bool,
    },
}

pub fn terminal<F>(
    mut parser: AnsiParser,
    columns: usize,
    rows: usize,
    scale: usize,
    ice_colors: bool,
    mut term_event_loop: F,
) -> !
where
    F: FnMut(&mut AnsiParser, TerminalEvent, &mut Terminal) + 'static,
{
    let mut term = Terminal::new(columns, rows, ice_colors);
    let (width, height) = term.get_dimensions();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(
            (width * scale as u32) + (BORDER_SIZE * 2),
            (height * scale as u32) + (BORDER_SIZE * 2),
        ))
        .with_title("ANSI Art")
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
    let (_stream, stream_handle) = OutputStream::try_default().expect("Audio");
    let mut player = Some(Player::new());
    let mut player_thread: Option<PlayerThread> = None;
    let mut alt = false;
    let mut logo = false;
    let mut shift = false;
    event_loop.run(move |event, _target, control_flow| {
        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::CloseRequested,
                ..
            } if window_id == window.id() => {
                term_event_loop(&mut parser, TerminalEvent::CloseRequested, &mut term);
                if let Some(thread) = player_thread.take() {
                    thread.abort().expect("abort");
                    thread.join().expect("join");
                }
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::ModifiersChanged(state),
                ..
            } if window_id == window.id() => {
                alt = state.alt();
                logo = state.logo();
                shift = state.shift();
            }
            Event::WindowEvent {
                window_id,
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            winit::event::KeyboardInput {
                                virtual_keycode: Some(key_code),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            } if window_id == window.id() => match key_code {
                #[cfg(target_os = "macos")]
                VirtualKeyCode::W if logo => {
                    term_event_loop(&mut parser, TerminalEvent::CloseRequested, &mut term);
                    if let Some(thread) = player_thread.take() {
                        thread.abort().expect("abort");
                        thread.join().expect("join");
                    }
                    *control_flow = ControlFlow::Exit;
                }
                #[cfg(target_os = "linux")]
                #[cfg(target_os = "windows")]
                VirtualKeyCode::F4 if alt => {
                    term_event_loop(&mut parser, TerminalEvent::CloseRequested, &mut term);
                    if let Some(thread) = player_thread.take() {
                        thread.abort().expect("abort");
                        thread.join().expect("join");
                    }
                    *control_flow = ControlFlow::Exit;
                }
                _ => match player_thread {
                    Some(ref thread) => thread.interrupt().expect("Thread Error"),
                    None => {
                        term_event_loop(
                            &mut parser,
                            TerminalEvent::Keypress {
                                key_code,
                                alt,
                                logo,
                                shift,
                            },
                            &mut term,
                        );
                    }
                },
            },
            Event::RedrawRequested(_) => {
                term_event_loop(&mut parser, TerminalEvent::RedrawRequested, &mut term);
                term.next_frame(pixels.get_frame());
                pixels.render().expect("Unable to render");
            }
            _ => {}
        }
        match player_thread {
            Some(ref thread) => {
                if thread.finished_playing().expect("Thread error") {
                    player = Some(
                        player_thread
                            .take()
                            .expect("Player Thread")
                            .join()
                            .expect("Thread Error"),
                    );
                }
            }
            None => {
                for sequence in parser.by_ref() {
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
                        }
                        Sequence::SetScreenMode(value) => term.set_screen_mode(value),
                        Sequence::ResetScreenMode(value) => term.reset_screen_mode(value),
                        Sequence::EraseDisplay(value) => term.erase_display(value),
                        Sequence::EraseInLine(value) => term.erase_in_line(value),
                        Sequence::SelectGraphicsRendition(values) => {
                            term.select_graphics_rendition(&values)
                        }
                        Sequence::SavePosition => term.save_cursor_position(),
                        Sequence::RestorePosition => term.restore_cursor_position(),
                        Sequence::TrueColourBg { r, g, b } => term.rgb_bg(r, g, b),
                        Sequence::TrueColourFg { r, g, b } => term.rgb_fg(r, g, b),
                        Sequence::Music(music) => {
                            player_thread = Some(
                                PlayerThread::new(
                                    player.take().expect("Player"),
                                    &stream_handle,
                                    music,
                                )
                                .expect("Thread error"),
                            );
                            break;
                        }
                        Sequence::Update => break,
                        _ => {}
                    }
                }
            }
        }
        window.request_redraw();
    });
}
