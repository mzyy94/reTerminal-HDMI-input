use iced_native::layout::{self, Layout};
use iced_native::renderer;
use iced_native::{Color, Element, Length, Point, Rectangle, Size, Widget};

pub struct LevelMeter {
  level: f32,
  height: Length,
}

impl LevelMeter {
  pub fn new(level: f32) -> Self {
    Self {
      level,
      height: Length::Shrink,
    }
  }

  pub fn height(mut self, height: Length) -> Self {
    self.height = height;
    self
  }
}

impl<Message, Renderer> Widget<Message, Renderer> for LevelMeter
where
  Renderer: renderer::Renderer,
{
  fn width(&self) -> Length {
    Length::Shrink
  }

  fn height(&self) -> Length {
    self.height
  }

  fn layout(&self, _renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
    let limits = limits.height(self.height);
    layout::Node::new(Size::new(10.0, limits.max().height))
  }

  fn draw(
    &self,
    renderer: &mut Renderer,
    _style: &renderer::Style,
    layout: Layout<'_>,
    _cursor_position: Point,
    _viewport: &Rectangle,
  ) {
    let position = layout.position();
    let make_quad = |offset: f32, height: f32, area_height: f32| -> renderer::Quad {
      renderer::Quad {
        bounds: Rectangle {
          x: position.x,
          y: position.y + offset * area_height,
          width: 10.0,
          height: height * area_height,
        },
        border_radius: 0.0,
        border_width: 0.0,
        border_color: Color::TRANSPARENT,
      }
    };

    let height = layout.bounds().height;
    renderer.fill_quad(make_quad(0.0, 0.3, height), Color::from_rgb8(192, 0, 0));
    if self.level > 0.85 {
      let h = self.level - 0.85;
      let offset = 0.15 - h;
      renderer.fill_quad(make_quad(offset, h, height), Color::from_rgb8(255, 0, 0));
      renderer.fill_quad(make_quad(0.15, 0.20, height), Color::from_rgb8(255, 255, 0));
      renderer.fill_quad(make_quad(0.35, 0.65, height), Color::from_rgb8(0, 255, 0));
    } else {
      renderer.fill_quad(make_quad(0.15, 0.20, height), Color::from_rgb8(192, 192, 0));
      if self.level > 0.65 {
        let h = self.level - 0.65;
        let offset = 0.35 - h;
        renderer.fill_quad(make_quad(offset, h, height), Color::from_rgb8(255, 255, 0));
        renderer.fill_quad(make_quad(0.35, 0.65, height), Color::from_rgb8(0, 255, 0));
      } else {
        renderer.fill_quad(make_quad(0.35, 0.65, height), Color::from_rgb8(0, 192, 0));
        let h = self.level;
        let offset = 1. - h;
        renderer.fill_quad(make_quad(offset, h, height), Color::from_rgb8(0, 255, 0));
      }
    }
  }
}

impl<'a, Message, Renderer> Into<Element<'a, Message, Renderer>> for LevelMeter
where
  Renderer: renderer::Renderer,
{
  fn into(self) -> Element<'a, Message, Renderer> {
    Element::new(self)
  }
}
