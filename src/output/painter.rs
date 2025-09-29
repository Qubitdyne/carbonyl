use std::{
    env,
    io::{self, Stdout, Write},
};

use crate::gfx::{Color, Point, Size};
use crate::utils::log;

use super::{
    binarize_quandrant,
    sixel::{Error as SixelError, Frame},
    Cell,
};

pub struct Painter {
    output: Stdout,
    buffer: Vec<u8>,
    cursor: Option<Point<u32>>,
    true_color: bool,
    background: Option<Color>,
    foreground: Option<Color>,
    background_code: Option<u8>,
    foreground_code: Option<u8>,
    sixel: Option<SixelState>,
}

struct SixelState {
    configured: bool,
    geometry: Size<u32>,
    pending: Option<Frame>,
    scrolling: bool,
}

impl Painter {
    pub fn new() -> Painter {
        Painter {
            buffer: Vec::new(),
            cursor: None,
            output: io::stdout(),
            background: None,
            foreground: None,
            background_code: None,
            foreground_code: None,
            sixel: None,
            true_color: match std::env::var("COLORTERM").unwrap_or_default().as_str() {
                "truecolor" | "24bit" => true,
                _ => false,
            },
        }
    }

    pub fn true_color(&self) -> bool {
        self.true_color
    }

    pub fn set_true_color(&mut self, true_color: bool) {
        self.true_color = true_color
    }

    pub fn enable_sixel(&mut self, geometry: Size<u32>) {
        self.sixel.get_or_insert_with(|| {
            let scrolling = env::var("CARBONYL_SIXEL_SCROLL")
                .ok()
                .and_then(|value| {
                    let normalized = value.trim().to_ascii_lowercase();

                    match normalized.as_str() {
                        "1" | "true" | "on" | "yes" => Some(true),
                        "0" | "false" | "off" | "no" => Some(false),
                        _ => None,
                    }
                })
                .unwrap_or(true);

            SixelState {
                configured: false,
                geometry,
                pending: None,
                scrolling,
            }
        });
    }

    pub fn queue_sixel_background(&mut self, pixels: &[u8], size: Size<u32>) -> bool {
        let Some(state) = self.sixel.as_mut() else {
            return false;
        };

        let exceeds_width = state.geometry.width != 0 && size.width > state.geometry.width;
        let exceeds_height = state.geometry.height != 0 && size.height > state.geometry.height;

        if exceeds_width || exceeds_height {
            log::error!(
                "failed to encode sixel frame: viewport {size:?} exceeds terminal graphics geometry {:?}",
                state.geometry
            );
            state.pending = None;

            return false;
        }

        let expected = size.width as usize * size.height as usize * 4;

        if pixels.len() < expected {
            log::error!(
                "failed to encode sixel frame: unexpected buffer size (expected {expected}, actual {})",
                pixels.len()
            );
            state.pending = None;

            return false;
        }

        match Frame::from_viewport(pixels, size) {
            Ok(frame) => {
                state.pending = Some(frame);

                true
            }
            Err(SixelError::InvalidSize(invalid)) => {
                log::error!("failed to encode sixel frame: viewport {invalid:?} is invalid");
                state.pending = None;

                false
            }
            Err(SixelError::Encode(error)) => {
                log::error!("failed to encode sixel frame: {error}");
                state.pending = None;

                false
            }
        }
    }

    fn sixel_enabled(&self) -> bool {
        self.sixel.is_some()
    }

    pub fn begin(&mut self) -> io::Result<()> {
        write!(self.buffer, "\x1b[?25l\x1b[?12l")?;

        if let Some(state) = self.sixel.as_mut() {
            if !state.configured {
                if state.scrolling {
                    write!(self.buffer, "\x1b[?80h")?;
                } else {
                    write!(self.buffer, "\x1b[?80l")?;
                }
                state.configured = true;
            }

            if let Some(frame) = state.pending.take() {
                write!(self.buffer, "\x1b[H")?;
                self.buffer.extend_from_slice(&frame.bytes);
                write!(self.buffer, "\x1b[H")?;
            }
        }

        Ok(())
    }

    pub fn end(&mut self, cursor: Option<Point>) -> io::Result<()> {
        if let Some(cursor) = cursor {
            write!(
                self.buffer,
                "\x1b[{};{}H\x1b[?25h\x1b[?12h",
                cursor.y + 1,
                cursor.x + 1
            )?;
        }

        self.output.write(self.buffer.as_slice())?;
        self.output.flush()?;
        self.buffer.clear();
        self.cursor = None;

        Ok(())
    }

    pub fn paint(&mut self, cell: &Cell) -> io::Result<()> {
        let &Cell {
            cursor,
            quadrant,
            ref grapheme,
            image,
        } = cell;

        if self.sixel_enabled() && grapheme.is_none() && image {
            return Ok(());
        }

        let (char, background, foreground, width) = if let Some(grapheme) = grapheme {
            if grapheme.index > 0 {
                return Ok(());
            }

            (
                grapheme.char.as_str(),
                quadrant
                    .0
                    .avg_with(quadrant.1)
                    .avg_with(quadrant.2)
                    .avg_with(quadrant.3),
                grapheme.color,
                grapheme.width as u32,
            )
        } else {
            let (char, background, foreground) = binarize_quandrant(quadrant);

            (char, background, foreground, 1)
        };

        if self.cursor != Some(cursor) {
            write!(self.buffer, "\x1b[{};{}H", cursor.y + 1, cursor.x + 1)?;
        };

        self.cursor = Some(cursor + Point::new(width, 0));

        if self.background != Some(background) {
            self.background = Some(background);

            if self.true_color {
                write!(
                    self.buffer,
                    "\x1b[48;2;{};{};{}m",
                    background.r, background.g, background.b,
                )?
            } else {
                let code = background.to_xterm();

                if self.background_code != Some(code) {
                    self.background_code = Some(code);

                    write!(self.buffer, "\x1b[48;5;{code}m")?
                }
            }
        }

        if self.foreground != Some(foreground) {
            self.foreground = Some(foreground);

            if self.true_color {
                write!(
                    self.buffer,
                    "\x1b[38;2;{};{};{}m",
                    foreground.r, foreground.g, foreground.b,
                )?
            } else {
                let code = foreground.to_xterm();

                if self.foreground_code != Some(code) {
                    self.foreground_code = Some(code);

                    write!(self.buffer, "\x1b[38;5;{code}m")?
                }
            }
        }

        self.buffer.write_all(char.as_bytes())?;

        Ok(())
    }
}
