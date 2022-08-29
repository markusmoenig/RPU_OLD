
use crate::prelude::*;

use fontdue::{ Font };
use rpu::prelude::*;

type P = (usize, usize);
type R = (usize, usize, usize, usize);

pub struct Context {

    pub font                : Option<Font>,
    pub draw2d              : Draw2D,

    pub rpu                 : RPU,

    pub rect                : (usize, usize, usize, usize),
    pub rpu_rect            : (usize, usize, usize, usize),
    pub code_rect           : (usize, usize, usize, usize),

    pub shift               : bool,
    pub ctrl                : bool,
    pub alt                 : bool,
    pub logo                : bool,
}

impl Context {

    pub fn new() -> Self where Self: Sized {

        let mut font = None;
        if let Some(font_bytes) = std::fs::read("resources/Open_Sans/static/OpenSans/OpenSans-Regular.ttf").ok() {
            if let Some(f) = Font::from_bytes(font_bytes, fontdue::FontSettings::default()).ok() {
                font = Some(f);
            }
        }

        Self {
            font,
            draw2d                      : Draw2D {},
            rpu                         : RPU::new(100, 100),

            rect                        : (0, 0, 0, 0),
            rpu_rect                   : (0, 0, 0, 0),
            code_rect                   : (0, 0, 0, 0),
            shift                       : false,
            ctrl                        : false,
            alt                         : false,
            logo                        : false,
        }
    }

    pub fn set_rect(&mut self, r: R) {
        self.rect = r;

        let div = r.3 / 2;
        self.rpu_rect = (r.0, r.1, r.2, div);
        self.code_rect = (r.0, div, r.2, div);
    }

    pub fn to_local(&self, p: &P, r: &R) -> Option<P> {
        if p.0 >= r.0 && p.1 >= r.1 && p.0 < r.0 + r.2 && p.1 < r.1 + r.3 {
            return Some((p.0 - r.0, p.1 - r.1))
        }
        None
    }

    pub fn compile(&mut self, main_code: String) {
        let rc = self.rpu.compile(main_code);
        println!("rc: {:?}", rc);
    }

}