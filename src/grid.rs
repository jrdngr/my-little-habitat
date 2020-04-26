use std::collections::VecDeque;

use wasm_bindgen::JsCast;
use web_sys::{MouseEvent, HtmlCanvasElement, CanvasRenderingContext2d};
use yew::services::{RenderService, Task, ConsoleService};
use yew::{html, Component, ComponentLink, Html, NodeRef, ShouldRender, Properties};

pub struct Grid {
    canvas: Option<HtmlCanvasElement>,
    ctx: Option<CanvasRenderingContext2d>,
    link: ComponentLink<Self>,
    node_ref: NodeRef,
    render_loop: Option<Box<dyn Task>>,
    size: (u32, u32),
    cell_size: (f64, f64),
    is_mouse_down: bool,
    cells: Vec<Cell>,
    update_queue: VecDeque<(u32, u32)>,
}

pub enum GridMsg {
    Render,
    MouseDown(i32, i32),
    MouseUp,
    MouseLeave,
    MouseMove(i32, i32),
}

#[derive(Clone, PartialEq, Properties)]
pub struct GridProps {
    pub size: (u32, u32),
    #[prop_or_else(|| (10, 10))]
    pub cell_size: (u32, u32),
}

impl Component for Grid {
    type Message = GridMsg;
    type Properties = GridProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let cell_count = props.size.0 * props.size.1;
        let cells = vec![Cell{ color: String::from("black") }; cell_count as usize];

        Grid {
            canvas: None,
            ctx: None,
            link,
            node_ref: NodeRef::default(),
            render_loop: None,
            size: props.size,
            cell_size: (props.cell_size.0 as f64, props.cell_size.1 as f64),
            is_mouse_down: false,
            cells,
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
            let render_frame = self.link.callback(|_| GridMsg::Render);
            let handle = RenderService::new().request_animation_frame(render_frame);

            self.render_loop = Some(Box::new(handle));
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            GridMsg::Render => {
                self.render_2d();
            }
            GridMsg::MouseDown(x, y) => {
                self.is_mouse_down = true;
                self.set_cell(self.pixels_to_coordinates(x, y), "white");
            }
            GridMsg::MouseUp => self.is_mouse_down = false,
            GridMsg::MouseLeave => self.is_mouse_down = false,
            GridMsg::MouseMove(x, y) => if self.is_mouse_down {
                self.set_cell(self.pixels_to_coordinates(x, y), "white")
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
                    GridMsg::MouseDown(event.offset_x(), event.offset_y())
                })
                onmouseup=self.link.callback(|_| GridMsg::MouseUp)
                onmouseleave=self.link.callback(|_| GridMsg::MouseLeave)
                onmousemove=self.link.callback(|event: MouseEvent| {
                    GridMsg::MouseMove(event.offset_x(), event.offset_y())
                })
                ref={self.node_ref.clone()}
            />
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}

impl Grid {
    fn cell(&self, x: u32, y: u32) -> Option<&Cell> {
        let index = self.cell_index(x, y);
        self.cells.get(index)
    }

    fn cell_mut(&mut self, x: u32, y: u32) -> Option<&mut Cell> {
        let index = self.cell_index(x, y);
        self.cells.get_mut(index)
    }

    fn set_cell(&mut self, (x, y): (u32, u32), color: &str) {
        if let Some(cell) = self.cell_mut(x, y) {
            cell.set_color(color);
            self.update_queue.push_back((x, y));
        }
    }

    fn cell_index(&self, x: u32, y: u32) -> usize {
        (y * self.size.0 + x) as usize
    }

    fn cell_coordinates(&self, index: usize) -> (u32, u32) {
        let x = index as u32 % self.size.0;
        let y = index as u32 / self.size.1;

        (x, y)
    }

    fn pixel_size(&self) -> (f64, f64) {
        let width = self.size.0 as f64 * self.cell_size.0;
        let height = self.size.1 as f64 * self.cell_size.1;
        (width, height)
    }

    fn pixels_to_coordinates(&self, x: i32, y: i32) -> (u32, u32) {
        let x = x / self.cell_size.0 as i32;
        let y = y / self.cell_size.1 as i32;

        (x as u32, y as u32)
    }

    fn draw_cell(&self, x: u32, y: u32) {
        if let Some(ctx) = &self.ctx {
            if let Some(cell) = self.cell(x, y) {
                let x = x as f64 * self.cell_size.0;
                let y = y as f64 * self.cell_size.1;
                ctx.set_fill_style(&cell.color().into());
                ctx.fill_rect(x, y, self.cell_size.0, self.cell_size.1);
            }
        }
    }

    fn redraw(&self) {
        if let Some(ctx) = &self.ctx {
            let (width, height) = self.pixel_size();
            ctx.set_fill_style(&"black".into());
            ctx.fill_rect(0.0, 0.0, width, height);    
    
            for index in 0..self.cells.len() {
                let (x, y) = self.cell_coordinates(index);
                self.draw_cell(x, y);
            }
        }
    }

    fn render_2d(&mut self) {
        while let Some((x, y)) = self.update_queue.pop_front() {
            self.draw_cell(x, y);
        }

        let render_frame = self.link.callback(|_| GridMsg::Render);
        let handle = RenderService::new().request_animation_frame(render_frame);
        self.render_loop = Some(Box::new(handle));
    }
}

#[derive(Debug, Clone)]
struct Cell {
    color: String,
}

impl Cell {
    fn color(&self) -> &str {
        &self.color
    }

    fn set_color(&mut self, color: &str) {
        self.color = String::from(color);
    }
}
