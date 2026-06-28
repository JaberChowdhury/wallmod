use iced::widget::canvas::{self, Canvas, Frame, Geometry, Program, Stroke};
use iced::{Color, Element, Length, Point, Rectangle};
use crate::modules::histogram::HistogramData;
use crate::app::Message;

pub struct HistogramChart<'a> {
    data: &'a HistogramData,
}

impl<'a> HistogramChart<'a> {
    pub fn new(data: &'a HistogramData) -> Self {
        Self { data }
    }

    pub fn view(self) -> Element<'a, Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fixed(150.0))
            .into()
    }
}

impl<'a> Program<Message> for HistogramChart<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        
        // Find max with log scaling
        // To make smaller details visible, we apply a logarithmic scale:
        // y = log(count + 1)
        let max = self.data.max_count as f32;
        if max == 0.0 {
            return vec![frame.into_geometry()];
        }
        
        let max_log = (max + 1.0).ln();
        let w = bounds.width;
        let h = bounds.height;
        let step = w / 255.0;

        let draw_channel = |f: &mut Frame, bins: &[u32; 256], color: Color| {
            let mut path_builder = canvas::path::Builder::new();
            path_builder.move_to(Point::new(0.0, h));

            for (i, &count) in bins.iter().enumerate() {
                let x = i as f32 * step;
                let val_log = (count as f32 + 1.0).ln();
                let normalized = val_log / max_log;
                let y = h - (normalized * h);
                path_builder.line_to(Point::new(x, y));
            }
            path_builder.line_to(Point::new(w, h));
            path_builder.close();
            let path = path_builder.build();

            f.fill(&path, color);
            
            // outline
            let mut stroke_color = color;
            stroke_color.a = 0.9; // More opaque border
            f.stroke(&path, Stroke::default().with_color(stroke_color).with_width(1.0));
        };

        // Draw RGB and Luma channels
        draw_channel(&mut frame, &self.data.r, Color::from_rgba(1.0, 0.2, 0.2, 0.35));
        draw_channel(&mut frame, &self.data.g, Color::from_rgba(0.2, 1.0, 0.2, 0.35));
        draw_channel(&mut frame, &self.data.b, Color::from_rgba(0.3, 0.5, 1.0, 0.35));
        draw_channel(&mut frame, &self.data.luma, Color::from_rgba(0.9, 0.9, 0.9, 0.25));

        vec![frame.into_geometry()]
    }
}
