use std::collections::VecDeque;

use wasm_bindgen::JsCast;
use web_sys::{MouseEvent, HtmlCanvasElement, CanvasRenderingContext2d};
use yew::services::{RenderService, Task};
use yew::{html, Component, ComponentLink, Html, NodeRef, ShouldRender, Properties};

use crate::grid::{Grid, GridCell, SimpleCell};

pub struct CanvasGrid {
    canvas: Option<HtmlCanvasElement>,
    ctx: Option<CanvasRenderingContext2d>,
    link: ComponentLink<Self>,
    node_ref: NodeRef,
    render_loop: Option<Box<dyn Task>>,
    props: CanvasGridProps,
    is_mouse_down: bool,
    grid: Grid<SimpleCell>,
    update_queue: VecDeque<(usize, usize)>,
}

pub enum CanvasGridMsg {
    Render,
    MouseUp,
    MouseLeave,
    MouseDown(i32, i32),
    MouseMove(i32, i32),
}

#[derive(Clone, PartialEq, Properties)]
pub struct CanvasGridProps {
    pub width: usize,
    pub height: usize,
    #[prop_or_else(|| 5)]
    pub cell_width: usize,
    #[prop_or_else(|| 5)]
    pub cell_height: usize,
}

impl Component for CanvasGrid {
    type Message = CanvasGridMsg;
    type Properties = CanvasGridProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CanvasGrid {
            canvas: None,
            ctx: None,
            link,
            node_ref: NodeRef::default(),
            render_loop: None,
            is_mouse_down: false,
            grid: Grid::new(props.width, props.height),
            props,
            update_queue: VecDeque::new(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();

        let ctx: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        self.canvas = Some(canvas);
        self.ctx = Some(ctx);

        self.redraw();

        if first_render {
            let render_frame = self.link.callback(|_| CanvasGridMsg::Render);
            let handle = RenderService::new().request_animation_frame(render_frame);

            self.render_loop = Some(Box::new(handle));
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            CanvasGridMsg::Render => {
                self.render_2d();
            }
            CanvasGridMsg::MouseUp => self.is_mouse_down = false,
            CanvasGridMsg::MouseLeave => self.is_mouse_down = false,
            CanvasGridMsg::MouseDown(x, y) => {
                self.is_mouse_down = true;
                let (x, y) = self.pixels_to_coordinates(x, y);
                self.set_cell(x, y, "white");
            }
            CanvasGridMsg::MouseMove(x, y) => if self.is_mouse_down {
                let (x, y) = self.pixels_to_coordinates(x, y);
                self.set_cell(x, y, "white");
            },
        }
        false
    }

    fn view(&self) -> Html {
        let (width, height) = self.pixel_size();
        html! {
            <canvas 
                width={width} 
                height={height}
                onmousedown=self.link.callback(|event: MouseEvent| {
                    CanvasGridMsg::MouseDown(event.offset_x(), event.offset_y())
                })
                onmouseup=self.link.callback(|_| CanvasGridMsg::MouseUp)
                onmouseleave=self.link.callback(|_| CanvasGridMsg::MouseLeave)
                onmousemove=self.link.callback(|event: MouseEvent| {
                    CanvasGridMsg::MouseMove(event.offset_x(), event.offset_y())
                })
                ref={self.node_ref.clone()}
            />
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}

impl CanvasGrid {
    fn set_cell<T: AsRef<str>>(&mut self, x: usize, y: usize, color: T) {
        self.grid.set_cell(x, y, SimpleCell::new(color.as_ref()));
        self.update_queue.push_back((x, y));
    }

    fn pixel_size(&self) -> (f64, f64) {
        let width = self.props.width * self.props.cell_width;
        let height = self.props.height  * self.props.cell_height;
        (width as f64, height as f64)
    }

    fn pixels_to_coordinates(&self, x: i32, y: i32) -> (usize, usize) {
        let x = x / self.props.cell_width as i32;
        let y = y / self.props.cell_height as i32;

        (x as usize, y as usize)
    }

    fn draw_cell(&self, x: usize, y: usize) {
        if let Some(ctx) = &self.ctx {
            if let Some(cell) = self.grid.cell(x, y) {
                let x = x * self.props.cell_width;
                let y = y * self.props.cell_height;
                ctx.set_fill_style(&cell.color().into());
                ctx.fill_rect(x as f64, y as f64, self.props.cell_width as f64, self.props.cell_height as f64);
            }
        }
    }

    fn redraw(&self) {
        if let Some(ctx) = &self.ctx {
            let (width, height) = self.pixel_size();
            ctx.set_fill_style(&"black".into());
            ctx.fill_rect(0.0, 0.0, width, height);    
    
            for index in 0..self.grid.cell_count() {
                let (x, y) = self.grid.cell_coordinates(index);
                self.draw_cell(x, y);
            }
        }
    }

    fn render_2d(&mut self) {
        while let Some((x, y)) = self.update_queue.pop_front() {
            self.draw_cell(x, y);
        }

        let render_frame = self.link.callback(|_| CanvasGridMsg::Render);
        let handle = RenderService::new().request_animation_frame(render_frame);
        self.render_loop = Some(Box::new(handle));
    }
}
