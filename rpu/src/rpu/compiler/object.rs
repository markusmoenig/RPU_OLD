use crate::prelude::*;

#[derive()]
pub enum Object {
    Empty,
    AnalyticalObject(Box<dyn Analytical>),
}

impl Object  {

    pub fn render(self, ctx: &mut Context ) {

    }
}