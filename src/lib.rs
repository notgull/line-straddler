// SPDX-License-Identifier: LGPL-3.0-or-later OR MPL-2.0
// This file is a part of `piet-cosmic-text`.
//
// `piet-cosmic-text` is free software: you can redistribute it and/or modify it under the
// terms of either:
//
// * GNU Lesser General Public License as published by the Free Software Foundation, either
//   version 3 of the License, or (at your option) any later version.
// * Mozilla Public License as published by the Mozilla Foundation, version 2.
// * The Patron License (https://github.com/notgull/piet-cosmic-text/blob/main/LICENSE-PATRON.md)
//   for sponsors and contributors, who can ignore the copyleft provisions of the above licenses
//   for this project.
//
// `piet-cosmic-text` is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU Lesser General Public License or the Mozilla Public License for more
// details.
//
// You should have received a copy of the GNU Lesser General Public License and the Mozilla
// Public License along with `piet-cosmic-text`. If not, see <https://www.gnu.org/licenses/>.

//! A library for calculating lines and how they go through text.

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![cfg_attr(not(feature = "std"), no_std)]

/// A glyph to be rendered.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Glyph {
    /// The y coordinate of the glyph's line.
    pub line_y: f32,

    /// The font size of the glyph in pixels.
    pub font_size: f32,

    /// The width of the glyph's bounding box.
    pub width: f32,

    /// The X coordinate of the glyph's bounding box.
    pub x: f32,

    /// The style of the glyph.
    pub style: GlyphStyle,
}

/// Glyph styling information.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct GlyphStyle {
    /// Whether or not the glyph is bold.
    pub bold: bool,

    /// The color of the glyph.
    pub color: Color,
}

/// 32-bit RGBA color.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct Color(u32);

impl Color {
    /// Create a new color from the given RGBA values.
    #[inline]
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32))
    }

    /// Get the red component of the color.
    #[inline]
    pub fn red(self) -> u8 {
        ((self.0 >> 24) & 0xFF) as u8
    }

    /// Get the green component of the color.
    #[inline]
    pub fn green(self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }

    /// Get the blue component of the color.
    #[inline]
    pub fn blue(self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    /// Get the alpha component of the color.
    #[inline]
    pub fn alpha(self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    /// Get an array of the components.
    #[inline]
    pub fn components(self) -> [u8; 4] {
        [self.red(), self.green(), self.blue(), self.alpha()]
    }
}

/// The horizontal line that needs to be rendered.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Line {
    /// The Y coordinate of the line.
    pub y: f32,

    /// The X coordinate of the line's start.
    pub start_x: f32,

    /// The X coordinate of the line's end.
    pub end_x: f32,

    /// The style of the line.
    pub style: GlyphStyle,
}

/// What kind of lind are we trying to produce?
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum LineType {
    /// This is an overline.
    Overline,

    /// This is a strike-through.
    StrikeThrough,

    /// This is an underline.
    Underline,
}

impl LineType {
    /// Get the offset of the line given the font size.
    fn offset(self, font_size: f32) -> f32 {
        match self {
            Self::Overline => 0.0,
            Self::StrikeThrough => font_size / 2.0,
            Self::Underline => font_size,
        }
    }
}

/// The generator for lines.
#[derive(Debug)]
pub struct LineGenerator {
    /// The line we are currently creating, if any.
    ongoing_line: Option<OngoingLine>,

    /// The type of line we are currently creating.
    line_type: LineType,
}

impl LineGenerator {
    /// Create a new, empty line generator.
    #[inline]
    pub fn new(ty: LineType) -> Self {
        Self {
            ongoing_line: None,
            line_type: ty,
        }
    }

    /// Pop the current line out of the generator.
    #[inline]
    pub fn pop_line(&mut self) -> Option<Line> {
        self.ongoing_line.take().map(Into::into)
    }

    /// Add a new glyph to the generator.
    ///
    /// Returns a new line if one was created.
    #[inline]
    pub fn add_glyph(&mut self, glyph: impl Into<Glyph>) -> Option<Line> {
        self.add_glyph_impl(glyph.into())
    }

    #[inline]
    fn add_glyph_impl(&mut self, glyph: Glyph) -> Option<Line> {
        // See if we need to start a new line.
        if let Some(line) = self.ongoing_line.as_mut() {
            if approx_eq(line.last_line_y, glyph.line_y)
                && line.end_x <= glyph.x
                && approx_eq(line.font_size, glyph.font_size)
                && line.style == glyph.style
            {
                // Just extend the current line.
                line.end_x = glyph.x + glyph.width;
                return None;
            }
        }

        // Just start a new line.
        let mut old_line = self.ongoing_line.replace(OngoingLine {
            y: glyph.line_y + self.line_type.offset(glyph.font_size),
            last_line_y: glyph.line_y,
            start_x: glyph.x,
            end_x: glyph.x + glyph.width,
            style: glyph.style,
            font_size: glyph.font_size,
        });

        // Make sure the old line ends where the new glyph begins if it's on the same line.
        if let Some(old_line) = old_line.as_mut() {
            if approx_eq(old_line.y, glyph.line_y) {
                old_line.end_x = glyph.x;
            }
        }

        old_line.map(Into::into)
    }
}

#[derive(Debug)]
struct OngoingLine {
    /// The Y coordinate of the line.
    y: f32,

    /// The X coordinate of the line's start.
    start_x: f32,

    /// The current X coordinate of the line's end.
    end_x: f32,

    /// The style of the line so far.
    style: GlyphStyle,

    /// The line y of the last glyph we observed.
    last_line_y: f32,

    /// The font size we last observed.
    font_size: f32,
}

impl From<OngoingLine> for Line {
    fn from(line: OngoingLine) -> Self {
        Self {
            y: line.y,
            start_x: line.start_x,
            end_x: line.end_x,
            style: line.style,
        }
    }
}

/// Tell if two floats are approximately equal.
fn approx_eq(a: f32, b: f32) -> bool {
    abs(a - b) < EPSILON
}

macro_rules! float_switch {
    ($i:ident => [$std:expr] [$libm:expr]) => {{
        #[cfg(feature = "std")]
        {
            $std
        }

        #[cfg(all(not(feature = "std"), feature = "libm"))]
        {
            $libm
        }

        #[cfg(all(not(feature = "std"), not(feature = "libm")))]
        {
            compile_error!("Either the `std` or `libm` feature must be enabled");
        }
    }};
}

/// Absolute value of a float.
fn abs(a: f32) -> f32 {
    float_switch!(
        a => [a.abs()] [libm::fabsf(a)]
    )
}

const EPSILON: f32 = 0.001;
