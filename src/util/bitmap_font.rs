use std::collections::HashMap;

use ggez::{
    context::Has,
    graphics::{self, Canvas, DrawParam, Drawable},
};

use crate::color::{RGBA8Ext, EMPTY};

use super::{PixelPoint, PixelSize, SpriteSize};

// From bracketlib terminal
pub fn to_char(c: u8) -> char {
    match c {
        1 => '☺',
        2 => '☻',
        3 => '♥',
        4 => '♦',
        5 => '♣',
        6 => '♠',
        7 => '•',
        8 => '◘',
        9 => '○',
        10 => '◙',
        11 => '♂',
        12 => '♀',
        13 => '♪',
        14 => '♫',
        15 => '☼',

        16 => '►',
        17 => '◄',
        18 => '↕',
        19 => '‼',
        20 => '¶',
        21 => '§',
        22 => '▬',
        23 => '↨',
        24 => '↑',
        25 => '↓',
        26 => '→',
        27 => '←',
        28 => '∟',
        29 => '↔',
        30 => '▲',
        31 => '▼',

        32 => ' ',
        33 => '!',
        34 => '"',
        35 => '#',
        36 => '$',
        37 => '%',
        38 => '&',
        39 => '\'',
        40 => '(',
        41 => ')',
        42 => '*',
        43 => '+',
        44 => ',',
        45 => '-',
        46 => '.',
        47 => '/',

        48 => '0',
        49 => '1',
        50 => '2',
        51 => '3',
        52 => '4',
        53 => '5',
        54 => '6',
        55 => '7',
        56 => '8',
        57 => '9',
        58 => ':',
        59 => ';',
        60 => '<',
        61 => '=',
        62 => '>',
        63 => '?',

        64 => '@',
        65 => 'A',
        66 => 'B',
        67 => 'C',
        68 => 'D',
        69 => 'E',
        70 => 'F',
        71 => 'G',
        72 => 'H',
        73 => 'I',
        74 => 'J',
        75 => 'K',
        76 => 'L',
        77 => 'M',
        78 => 'N',
        79 => 'O',

        80 => 'P',
        81 => 'Q',
        82 => 'R',
        83 => 'S',
        84 => 'T',
        85 => 'U',
        86 => 'V',
        87 => 'W',
        88 => 'X',
        89 => 'Y',
        90 => 'Z',
        91 => '[',
        92 => '\\',
        93 => ']',
        94 => '^',
        95 => '_',

        96 => '`',
        97 => 'a',
        98 => 'b',
        99 => 'c',
        100 => 'd',
        101 => 'e',
        102 => 'f',
        103 => 'g',
        104 => 'h',
        105 => 'i',
        106 => 'j',
        107 => 'k',
        108 => 'l',
        109 => 'm',
        110 => 'n',
        111 => 'o',

        112 => 'p',
        113 => 'q',
        114 => 'r',
        115 => 's',
        116 => 't',
        117 => 'u',
        118 => 'v',
        119 => 'w',
        120 => 'x',
        121 => 'y',
        122 => 'z',
        123 => '{',
        124 => '|',
        125 => '}',
        126 => '~',
        127 => '⌂',

        128 => 'Ç',
        129 => 'ü',
        130 => 'é',
        131 => 'â',
        132 => 'ä',
        133 => 'à',
        134 => 'å',
        135 => 'ç',
        136 => 'ê',
        137 => 'ë',
        138 => 'è',
        139 => 'ï',
        140 => 'î',
        141 => 'ì',
        142 => 'Ä',
        143 => 'Å',

        144 => 'É',
        145 => 'æ',
        146 => 'Æ',
        147 => 'ô',
        148 => 'ö',
        149 => 'ò',
        150 => 'û',
        151 => 'ù',
        152 => 'ÿ',
        153 => 'Ö',
        154 => 'Ü',
        155 => '¢',
        156 => '£',
        157 => '¥',
        158 => '₧',
        159 => 'ƒ',

        160 => 'á',
        161 => 'í',
        162 => 'ó',
        163 => 'ú',
        164 => 'ñ',
        165 => 'Ñ',
        166 => 'ª',
        167 => 'º',
        168 => '¿',
        169 => '⌐',
        170 => '¬',
        171 => '½',
        172 => '¼',
        173 => '¡',
        174 => '«',
        175 => '»',

        176 => '░',
        177 => '▒',
        178 => '▓',
        179 => '│',
        180 => '┤',
        181 => '╡',
        182 => '╢',
        183 => '╖',
        184 => '╕',
        185 => '╣',
        186 => '║',
        187 => '╗',
        188 => '╝',
        189 => '╜',
        190 => '╛',
        191 => '┐',

        192 => '└',
        193 => '┴',
        194 => '┬',
        195 => '├',
        196 => '─',
        197 => '┼',
        198 => '╞',
        199 => '╟',
        200 => '╚',
        201 => '╔',
        202 => '╩',
        203 => '╦',
        204 => '╠',
        205 => '═',
        206 => '╬',
        207 => '╧',

        208 => '╨',
        209 => '╤',
        210 => '╥',
        211 => '╙',
        212 => '╘',
        213 => '╒',
        214 => '╓',
        215 => '╫',
        216 => '╪',
        217 => '┘',
        218 => '┌',
        219 => '█',
        220 => '▄',
        221 => '▌',
        222 => '▐',
        223 => '▀',

        224 => 'α',
        225 => 'ß',
        226 => 'Γ',
        227 => 'π',
        228 => 'Σ',
        229 => 'σ',
        230 => 'µ',
        231 => 'τ',
        232 => 'Φ',
        233 => 'Θ',
        234 => 'Ω',
        235 => 'δ',
        236 => '∞',
        237 => 'φ',
        238 => 'ε',
        239 => '∩',

        240 => '≡',
        241 => '±',
        242 => '≥',
        243 => '≤',
        244 => '⌠',
        245 => '⌡',
        246 => '÷',
        247 => '≈',
        248 => '°',
        249 => '∙',
        250 => '·',
        251 => '√',
        252 => 'ⁿ',
        253 => '²',
        254 => '■',

        _ => ' ',
    }
}

fn create_cp437_string() -> String {
    (0..255u8).map(to_char).collect()
}

/// Describes the layout of characters in your
/// bitmap font.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TextMap {
    pub map: HashMap<char, graphics::Rect>,
    sheet_size: SpriteSize,
}

impl TextMap {
    /// Creates a new `TextMap` from a uniform grid of
    /// sprites.  Takes the number of sprites wide and
    /// tall that the bitmap should be, and a string
    /// describing the characters in the map... in order,
    /// left to right, top to bottom.
    ///
    /// The characters do not necessarily need to fill
    /// the entire image.  ie, if your image is 16x16 glyphs
    /// for 256 total, and you only use the first 150 of them,
    /// that's fine.
    ///
    /// The floating point math involved should always be
    /// exact for `Image`'s and sprites with a resolution
    /// that is a power of two, I think.
    fn from_grid(mapping: &str, width: usize, height: usize) -> Self {
        // Assert the given width and height can fit the listed characters.
        let num_chars = mapping.chars().count();
        assert!(
            num_chars <= width * height,
            "expected {:?} characters for this spritesheet (got {:?})",
            width * height,
            num_chars
        );
        let rect_width = 1.0 / (width as f32);
        let rect_height = 1.0 / (height as f32);
        let mut map = HashMap::with_capacity(num_chars);
        let mut current_x = 0;
        let mut current_y = 0;
        for c in mapping.chars() {
            let x_offset = current_x as f32 * rect_width;
            let y_offset = current_y as f32 * rect_height;
            let char_rect = graphics::Rect {
                x: x_offset,
                y: y_offset,
                w: rect_width,
                h: rect_height,
            };
            map.insert(c, char_rect);
            current_x = (current_x + 1) % width;
            if current_x == 0 {
                current_y += 1;
            }
        }

        Self {
            map,
            // TODO: this is currently a float relative to 100% of the spritesheet size. it needs to be mapped back to absolute pixels
            sheet_size: SpriteSize::new(width as i32, height as i32),
        }
    }

    pub fn sheet_size(&self) -> SpriteSize {
        self.sheet_size
    }
}

#[derive(Debug)]
pub struct BitmapFont {
    batch: graphics::InstanceArray,
    text_map: TextMap,
    pub char_size: PixelSize,
    // Rect mesh used to clear backgrounds when needed
    clear_rect: graphics::Mesh,
}

impl BitmapFont {
    pub fn from_grid(
        gfx: &impl Has<graphics::GraphicsContext>,
        image: graphics::Image,
        sprite_sheet_size: &SpriteSize,
    ) -> Self {
        let mapping = create_cp437_string();
        let text_map = TextMap::from_grid(
            &mapping,
            sprite_sheet_size.width as usize,
            sprite_sheet_size.height as usize,
        );
        let batch = graphics::InstanceArray::new(gfx, image, 100, true);

        let sheet_size = text_map.sheet_size();

        let rect_width = 1.0 / (sheet_size.width as f32);
        let rect_height = 1.0 / (sheet_size.height as f32);

        let char_size = PixelSize::new(
            (batch.image().width() as f32 * rect_width) as i32,
            (batch.image().width() as f32 * rect_height) as i32,
        );

        let clear_rect = graphics::Mesh::new_rectangle(
            gfx,
            graphics::DrawMode::fill(),
            graphics::Rect::new_i32(0, 0, char_size.width, char_size.height),
            EMPTY.to_ggez_color(),
        )
        .expect("clear rect");
        Self::new(batch, text_map, char_size, clear_rect)
    }

    pub fn new(
        batch: graphics::InstanceArray,
        text_map: TextMap,
        char_size: PixelSize,
        clear_rect: graphics::Mesh,
    ) -> Self {
        Self {
            batch,
            text_map,
            char_size,
            clear_rect,
        }
    }
    pub fn draw_char(
        &self,
        canvas: &mut Canvas,
        c: char,
        point: &PixelPoint,
        draw_param: Option<DrawParam>,
    ) {
        let base_param = draw_param.unwrap_or_else(DrawParam::new);
        let rect = self.get_for_char(c);
        let dest_rect = graphics::Rect::new_i32(
            point.x,
            point.y,
            self.char_size.width,
            self.char_size.height,
        );
        let draw_param = base_param.src(*rect).dest_rect(dest_rect);

        canvas.draw(&self.clear_rect, draw_param);
        canvas.draw(&self.batch.image(), draw_param);
    }

    pub fn draw_char_overwrite(
        &self,
        canvas: &mut Canvas,
        c: char,
        point: &PixelPoint,
        draw_param: Option<DrawParam>,
    ) {
        self.draw_char(canvas, c, point, draw_param);
    }

    pub fn draw_each_char(
        &self,
        canvas: &mut Canvas,
        text: &str,
        point: &PixelPoint,
        draw_param: Option<DrawParam>,
    ) {
        let draw_params = self.string_to_draw_params(text, point, draw_param);
        for draw_param in draw_params {
            canvas.draw(&self.batch.image(), draw_param);
        }
    }

    pub fn push_text(&mut self, text: &str, point: &PixelPoint, draw_param: Option<DrawParam>) {
        let draw_params = self.string_to_draw_params(text, point, draw_param);
        assert!(self.batch.capacity() > draw_params.len());
        assert!(text.len() == draw_params.len());

        for draw_param in draw_params {
            self.batch.push(draw_param);
        }
    }

    fn string_to_draw_params(
        &self,
        text: &str,
        point: &PixelPoint,
        draw_param: Option<DrawParam>,
    ) -> Vec<DrawParam> {
        let base_param = draw_param.unwrap_or_else(DrawParam::new);
        text.chars()
            // TODO: how to handle whitespace??
            .map(|c| self.get_for_char(c))
            .enumerate()
            .map(|(i, rect)| {
                let dest_rect = graphics::Rect::new_i32(
                    point.x + (i as i32 * self.char_size.width),
                    point.y,
                    self.char_size.width,
                    self.char_size.height,
                );
                base_param.src(*rect).dest_rect(dest_rect)
            })
            .collect()
    }

    pub fn clear(&mut self) {
        self.batch.clear()
    }

    fn get_for_char(&self, c: char) -> &graphics::Rect {
        self.text_map.map.get(&c).unwrap()
    }
}

impl Drawable for BitmapFont {
    fn draw(&self, canvas: &mut Canvas, param: impl Into<DrawParam>) {
        canvas.draw(&self.batch, param)
    }

    fn dimensions(&self, gfx: &impl Has<graphics::GraphicsContext>) -> Option<graphics::Rect> {
        self.batch.dimensions(gfx)
    }
}
