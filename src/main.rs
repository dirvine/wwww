#![doc(html_logo_url = "http://maidsafe.net/img/Resources/branding/maidsafe_logo.fab2.png",
       html_favicon_url = "http://maidsafe.net/img/favicon.ico",
              html_root_url = "http://dirvine.github.io/wwww")]
//! wwww
//! Answer engine

#![deny(missing_docs, unused_variables, unused_features, unused_attributes)]
#![warn(dead_code)]
#[cfg_attr(test, allow(dead_code))]
extern crate conrod;
extern crate find_folder;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate glutin_window;
extern crate graphics;
extern crate piston;
extern crate piston_window;

use conrod::{CanvasId, Floating, Theme, Widget, WidgetId, UiId};
use gfx_device_gl::{CommandBuffer, Factory, Resources, Output};
use gfx_graphics::{GfxGraphics, GlyphCache};
use glutin_window::{GlutinWindow, OpenGL};
use graphics::Context;
use piston::window::{WindowSettings, Size};
use piston_window::PistonWindow;
use std::cell::RefCell;
use std::rc::Rc;


type Ui = conrod::Ui<GlyphCache<Resources, Factory>>;
type Graphics<'a> = GfxGraphics<'a, Resources, CommandBuffer, Output>;


fn main() {

    // Construct the window.
    let window = {
        let window = GlutinWindow::new(
            OpenGL::_3_2,
            WindowSettings::new("WWWW".to_string(), Size { width: 800, height: 600 })
                .exit_on_esc(true)
        );
        PistonWindow::new(Rc::new(RefCell::new(window)), piston_window::empty_app())
    };

    // construct our `Ui`.
    let mut ui = {
        let assets = find_folder::Search::Both(3, 3).for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        let theme = Theme::default();
        let glyph_cache = GlyphCache::new(&font_path, window.factory.borrow().clone());
        Ui::new(glyph_cache.unwrap(), theme)
    };

    // Poll events from the window.
    for event in window {
        ui.handle_event(&event);
        event.draw_2d(|c, g| draw_ui(&mut ui, c, g));
    }

}


// Draw the Ui.
fn draw_ui<'a>(ui: &mut Ui, c: Context, g: &mut Graphics<'a>) {
    use conrod::color::{blue, light_orange, orange, dark_orange, red, white};
    use conrod::{Button, Colorable, Label, Labelable, Positionable, Sizeable, Split, WidgetMatrix, TextBox};

    // Construct our Canvas tree.
    Split::new(MASTER).flow_down(&[
        Split::new(HEADER).color(blue()).pad_bottom(20.0),
        Split::new(BODY).length(300.0).flow_right(&[
            Split::new(LEFT_COLUMN).color(light_orange()).pad(20.0),
            Split::new(MIDDLE_COLUMN).color(orange()),
            Split::new(RIGHT_COLUMN).color(dark_orange()).pad(20.0),
        ]),
        Split::new(FOOTER).color(blue())
    ]).set(ui);

    Floating::new()
        .label("Blue")
        .middle_of(LEFT_COLUMN)
        .color(blue())
        .label_color(white())
        .set(FLOATING_A, ui);

    Floating::new()
        .label("Orange")
        .middle_of(RIGHT_COLUMN)
        .color(light_orange())
        .label_color(white())
        .set(FLOATING_B, ui);

    Label::new("WWWW").color(orange()).font_size(48).mid_top_of(HEADER).set(TITLE, ui);
    Label::new("What, Why, Where, When").color(blue().complement()).middle_of(HEADER).set(SUBTITLE, ui);

    Label::new("Top Left")
        .color(light_orange().complement())
        .top_left_of(LEFT_COLUMN)
        .set(TOP_LEFT, ui);

    Label::new("Keyword : ")
        .color(orange().complement())
        .top_left_of(MIDDLE_COLUMN)
        .set(KEYWORD, ui);
   
   let mut keyword_string = &mut "Enter Keyword".to_string(); 
   
   TextBox::new(&mut keyword_string)
        .react(|string: &mut String|{ println!("{}", string)})
        .font_size(10)
        .right_from(UiId::Widget(KEYWORD), 2.0)
        .set(KEYWORDBOX, ui);
    
    Label::new("Password : ")
        .color(orange().complement())
        .bottom_left_of(MIDDLE_COLUMN)
        .set(PASSWORD, ui);
   
   let mut password_string = &mut "Enter Pssword".to_string(); 
   
   TextBox::new(&mut password_string)
        .react(|string: &mut String|{ println!("{}", string)})
        .font_size(10)
        .right_from(UiId::Widget(PASSWORD), 2.0)
        .set(PASSWORDBOX, ui);

    Label::new("Bottom Right")
        .color(dark_orange().complement())
        .bottom_right_of(RIGHT_COLUMN)
        .set(BOTTOM_RIGHT, ui);

    WidgetMatrix::new(5, 2)
        .dim([100.0, 100.0])
        .middle_of(MIDDLE_COLUMN)
        .each_widget(ui, |ui, n, _col, _row, xy, dim| {
            Button::new()
                .color(orange().complement())
                .dim(dim)
                .label(&n.to_string())
                .point(xy)
                .react(|| println!("Hey! {:?}", n))
                .set(BUTTON + n, ui);
        });
    
    Button::new().color(red()).dimensions(30.0, 30.0).middle_of(FLOATING_A)
        .react(|| println!("Bing!"))
        .set(BING, ui);
    Button::new().color(red()).dimensions(30.0, 30.0).middle_of(FLOATING_B)
        .react(|| println!("Bong!"))
        .set(BONG, ui);

    ui.draw(c, g);
}


// Canvas IDs.
const MASTER: CanvasId = 0;
const HEADER: CanvasId = MASTER + 1;
const BODY: CanvasId = HEADER + 1;
const LEFT_COLUMN: CanvasId = BODY + 1;
const MIDDLE_COLUMN: CanvasId = LEFT_COLUMN + 1;
const RIGHT_COLUMN: CanvasId = MIDDLE_COLUMN + 1;
const FOOTER: CanvasId = RIGHT_COLUMN + 1;
const FLOATING_A: CanvasId = FOOTER + 1;
const FLOATING_B: CanvasId = FLOATING_A + 1;

// Button matrix dimensions.
const ROWS: usize = 5;
const COLS: usize = 24;

// Widget IDs.
const TITLE: WidgetId = 0;
const SUBTITLE: WidgetId = TITLE + 1;
const TOP_LEFT: WidgetId = SUBTITLE + 1;
const KEYWORD: WidgetId = TOP_LEFT + 1;
const KEYWORDBOX: WidgetId = KEYWORD + 1;
const PASSWORD: WidgetId = KEYWORDBOX + 1;
const PASSWORDBOX: WidgetId = PASSWORD + 1;
const BOTTOM_RIGHT: WidgetId = PASSWORDBOX + 1;
const BUTTON: WidgetId = BOTTOM_RIGHT + 1;
const BING: WidgetId = BUTTON + COLS * ROWS;
const BONG: WidgetId = BING + 1;



#[cfg(test)]
mod test {
#[test]     
fn it_works() {}    
    
}
