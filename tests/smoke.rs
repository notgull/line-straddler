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

    // Run the algorithm.
    let mut alg = LineGenerator::new(LineType::Overline);
    let mut lines = vec![];

    for glyph in glyphs {
        lines.extend(alg.add_glyph(glyph));
    }
    lines.extend(alg.pop_line());

    assert_eq!(lines.len(), 2);
    assert_approx_eq!(lines[0].y as f64, 0.0);
    assert_approx_eq!(lines[0].start_x as f64, 0.0);
    assert_approx_eq!(lines[0].end_x as f64, 5.0);

    assert_approx_eq!(lines[1].y as f64, 5.0);
    assert_approx_eq!(lines[1].start_x as f64, 0.0);
    assert_approx_eq!(lines[1].end_x as f64, 5.0);
}
