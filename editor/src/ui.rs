use crate::prelude::*;
use code_editor::prelude::*;

pub struct UI {
    ctx             : Context,

    code_editor     : CodeEditor,
}

impl UI {

    pub fn new() -> Self {

        let mut code_editor = CodeEditor::new();
        code_editor.set_font("resources/Source_Code_Pro/static/SourceCodePro-Regular.ttf");
        code_editor.set_mode(CodeEditorMode::Rhai);
        code_editor.set_font_size(17.0);
        //code_editor.set_text("testing".to_string());

        Self {
            ctx     : Context::new(),

            code_editor,
        }
    }

    pub fn draw(&mut self, frame: &mut [u8], rect: (usize, usize, usize, usize), stride: usize) {
        self.ctx.set_rect(rect);

        self.ctx.rpu.render(frame, self.ctx.rpu_rect, rect.2);
        self.code_editor.draw(frame, self.ctx.code_rect, stride);
    }

    pub fn key_down(&mut self, char: Option<char>, key: Option<WidgetKey>) -> bool {
        if self.code_editor.key_down(char, key) {
            println!("{:?}", self.code_editor.get_text());
            self.ctx.compile(self.code_editor.get_text());
            return true;
        }
        false
    }

    pub fn mouse_down(&mut self, pos: (usize, usize)) -> bool {

        if let Some(p) = self.ctx.to_local(&pos, &self.ctx.code_rect) {
            return self.code_editor.mouse_down(p);
        }
        false
    }

    pub fn mouse_up(&mut self, pos: (usize, usize)) -> bool {
        if let Some(p) = self.ctx.to_local(&pos, &self.ctx.code_rect) {
            return self.code_editor.mouse_up(p);
        }

        false
    }

    pub fn mouse_dragged(&mut self, pos: (usize, usize)) -> bool {
        if let Some(p) = self.ctx.to_local(&pos, &self.ctx.code_rect) {
            return self.code_editor.mouse_dragged(p);
        }
        false
    }

    pub fn mouse_hover(&mut self, pos: (usize, usize)) -> bool {
        if let Some(p) = self.ctx.to_local(&pos, &self.ctx.code_rect) {
            return self.code_editor.mouse_hover(p);
        }
        false
    }

    pub fn mouse_wheel(&mut self, delta: (isize, isize)) -> bool {
        return self.code_editor.mouse_wheel(delta);
        //false
    }

    pub fn modifier_changed(&mut self, shift: bool, ctrl: bool, alt: bool, logo: bool) -> bool {
        self.ctx.shift = shift;
        self.ctx.ctrl = ctrl;
        self.ctx.alt = alt;
        self.ctx.logo = logo;
        return self.code_editor.modifier_changed(shift, ctrl, alt, logo);
    }
}