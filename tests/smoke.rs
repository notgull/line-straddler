// SPDX-License-Identifier: LGPL-3.0-or-later OR MPL-2.0
// This file is a part of `line-straddler`.
//
// `line-straddler` is free software: you can redistribute it and/or modify it under the
// terms of either:
//
// * GNU Lesser General Public License as published by the Free Software Foundation, either
//   version 3 of the License, or (at your option) any later version.
// * Mozilla Public License as published by the Mozilla Foundation, version 2.
// * The Patron License (https://github.com/notgull/line-straddler/blob/main/LICENSE-PATRON.md)
//   for sponsors and contributors, who can ignore the copyleft provisions of the above licenses
//   for this project.
//
// `line-straddler` is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU Lesser General Public License or the Mozilla Public License for more
// details.
//
// You should have received a copy of the GNU Lesser General Public License and the Mozilla
// Public License along with `line-straddler`. If not, see <https://www.gnu.org/licenses/>.

use approx_eq::assert_approx_eq;
use line_straddler::{Color, Glyph, GlyphStyle, LineGenerator, LineType};

#[test]
fn lines() {
    let style = GlyphStyle {
        bold: false,
        color: Color::rgba(0, 0, 0, 255),
    };

    // Two lines of two glyphs.
    let glyphs = [
        Glyph {
            line_y: 0.0,
            font_size: 4.0,
            width: 2.0,
            x: 0.0,
            style,
        },
        Glyph {
            line_y: 0.0,
            font_size: 4.0,
            width: 2.0,
            x: 3.0,
            style,
        },
        Glyph {
            line_y: 5.0,
            font_size: 4.0,
            width: 2.0,
            x: 0.0,
            style,
        },
        Glyph {
            line_y: 5.0,
            font_size: 4.0,
            width: 2.0,
            x: 3.0,
            style,
        },
    ];

    let styles = [
        (LineType::Overline, 0.0, 5.0),
        (LineType::Underline, 4.0, 9.0),
        (LineType::StrikeThrough, 2.0, 7.0),
    ];

    for (style, first_line_y, second_line_y) in styles {
        // Run the algorithm.
        let mut alg = LineGenerator::new(style);
        let mut lines = vec![];

        for glyph in glyphs {
            lines.extend(alg.add_glyph(glyph));
        }
        lines.extend(alg.pop_line());

        assert_eq!(lines.len(), 2);
        assert_approx_eq!(lines[0].y as f64, first_line_y);
        assert_approx_eq!(lines[0].start_x as f64, 0.0);
        assert_approx_eq!(lines[0].end_x as f64, 5.0);

        assert_approx_eq!(lines[1].y as f64, second_line_y);
        assert_approx_eq!(lines[1].start_x as f64, 0.0);
        assert_approx_eq!(lines[1].end_x as f64, 5.0);
    }
}

#[test]
fn mid_line_switch() {
    // The color of the glyphs switches mid-line.
    let style1 = GlyphStyle {
        bold: false,
        color: Color::rgba(0, 0, 0, 255),
    };
    let style2 = GlyphStyle {
        bold: false,
        color: Color::rgba(255, 255, 255, 255),
    };

    let glyphs = [
        Glyph {
            line_y: 0.0,
            font_size: 4.0,
            width: 2.0,
            x: 0.0,
            style: style1,
        },
        Glyph {
            line_y: 0.0,
            font_size: 4.0,
            width: 2.0,
            x: 3.0,
            style: style1,
        },
        Glyph {
            line_y: 0.0,
            font_size: 4.0,
            width: 2.0,
            x: 6.0,
            style: style2,
        },
        Glyph {
            line_y: 0.0,
            font_size: 4.0,
            width: 2.0,
            x: 9.0,
            style: style2,
        },
    ];

    let mut alg = LineGenerator::new(LineType::Overline);
    let mut lines = vec![];

    for glyph in glyphs {
        lines.extend(alg.add_glyph(glyph));
    }
    lines.extend(alg.pop_line());

    assert_eq!(lines.len(), 2);
    assert_approx_eq!(lines[0].y as f64, 0.0);
    assert_approx_eq!(lines[0].start_x as f64, 0.0);
    assert_approx_eq!(lines[0].end_x as f64, 6.0);
    assert_eq!(lines[0].style, style1);

    assert_approx_eq!(lines[1].y as f64, 0.0);
    assert_approx_eq!(lines[1].start_x as f64, 6.0);
    assert_approx_eq!(lines[1].end_x as f64, 11.0);
    assert_eq!(lines[1].style, style2);
}

#[test]
fn full_line_then_switch() {
    let style = GlyphStyle {
        bold: false,
        color: Color::rgba(0, 0, 0, 255),
    };

    // Copied from the piet-cosmic-text usecase.
    let glyphs = [
        Glyph {
            line_y: 3.2000008,
            font_size: 32.0,
            width: 17.828125,
            x: 0.0,
            style,
        },
        Glyph {
            line_y: 3.2000008,
            font_size: 32.0,
            width: 8.890625,
            x: 17.828125,
            style,
        },
        Glyph {
            line_y: 3.2000008,
            font_size: 32.0,
            width: 20.28125,
            x: 26.71875,
            style,
        },
        Glyph {
            line_y: 3.2000008,
            font_size: 32.0,
            width: 19.6875,
            x: 47.0,
            style,
        },
        Glyph {
            line_y: 3.2000008,
            font_size: 32.0,
            width: 10.171875,
            x: 66.6875,
            style,
        },
        Glyph {
            line_y: 3.2000008,
            font_size: 32.0,
            width: 26.8125,
            x: 76.859375,
            style,
        },
        Glyph {
            line_y: 3.2000008,
            font_size: 32.0,
            width: 20.359375,
            x: 103.671875,
            style,
        },
        Glyph {
            line_y: 35.2,
            font_size: 32.0,
            width: 17.828125,
            x: 0.0,
            style,
        },
        Glyph {
            line_y: 35.2,
            font_size: 32.0,
            width: 8.890625,
            x: 17.828125,
            style,
        },
    ];

    let styles = [
        (LineType::Overline, 0.0, 5.0),
        (LineType::Underline, 4.0, 9.0),
        (LineType::StrikeThrough, 2.0, 7.0),
    ];

    for (style, _, _) in styles {
        // Run the algorithm.
        let mut alg = LineGenerator::new(style);
        let mut lines = vec![];

        for glyph in glyphs {
            lines.extend(alg.add_glyph(glyph));
        }
        lines.extend(alg.pop_line());

        assert!(
            (lines[0].start_x - lines[0].end_x).abs() > 0.0001,
            "style={:?}",
            style
        );
    }
}

#[test]
fn colors() {
    let color = Color::rgba(1, 2, 3, 4);
    assert_eq!(color.red(), 1);
    assert_eq!(color.green(), 2);
    assert_eq!(color.blue(), 3);
    assert_eq!(color.alpha(), 4);
    assert_eq!(color.components(), [1, 2, 3, 4]);
}

#[test]
fn other_coverage() {
    println!("{:?}", Color::default().clone());
    assert_eq!(Color::default(), Color::default());
    println!("{:?}", LineGenerator::new(LineType::Overline));
}
