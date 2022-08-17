use crate::prelude::*;

#[derive()]
pub enum Object {
    Empty,
    AnalyticalObject(Box<dyn Analytical>),
    SDF3D(Box<dyn SDF3D>),
    Layout3D(Box<dyn Layout3D>),
    Element2D(Box<dyn Element2D>),
}

// impl Object {

//     pub fn render(self, ctx: &mut Context ) {

//     }
// }