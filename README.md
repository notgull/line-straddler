# line-straddler

Figure out where lines should go when underlining/striking through text.

This project is hosted on [SourceHut](https://git.sr.ht/~notgull/line-straddler). The GitHub mirror is kept for convenience.

When you're drawing text, you need to determine where the lines go for text decorations. This crate provides a renderer-agnostic `LineGenerator` that generates `Line` structures for a set of `Glyph`s.

## Example

```rust
use line_straddler::{LineGenerator, Line, LineType, Glyph, GlyphStyle, Color};

// Take some glyphs from, e.g, cosmic-text
// For instance, this is two lines of two glyphs.
let style = GlyphStyle {
    bold: false,
    color: Color::rgba(0, 0, 0, 255),
};
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

// Create a line generator.
let mut alg = LineGenerator::new(LineType::Underline);

// Generate lines for the glyphs.
let mut lines = Vec::new();
for glyph in glyphs {
    lines.extend(alg.add_glyph(glyph));
}
lines.extend(alg.pop_line());

// Draw all of the lines.
for line in lines {
    let point_1 = (line.start_x, line.y);
    let point_2 = (line.end_x, line.y);
    draw_line(point_1, point_2, line.style);
}
```

## License

`line-straddler` is free software: you can redistribute it and/or modify it under the terms of
either:

* GNU Lesser General Public License as published by the Free Software Foundation, either
version 3 of the License, or (at your option) any later version.
* Mozilla Public License as published by the Mozilla Foundation, version 2.

`line-straddler` is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
See the GNU Lesser General Public License or the Mozilla Public License for more details.

You should have received a copy of the GNU Lesser General Public License and the Mozilla
Public License along with `line-straddler`. If not, see <https://www.gnu.org/licenses/>.
