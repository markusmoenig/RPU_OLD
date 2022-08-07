use crate::prelude::*;

#[derive()]
pub enum Object {
    Empty,
    AnalyticalObject(Box<dyn Analytical>),
    Element2D(Box<dyn Element2D>),
}

impl Object {

    pub fn render(self, ctx: &mut Context ) {

    }
}