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
    width: i32,
    height: i32,
    cell_width: f64,
    cell_height: f64,
    points: Vec<(f64, f64)>,
}

pub enum GridMsg {
    Render,
    Clicked(u32, u32),
}

#[derive(Clone, PartialEq, Properties)]
pub struct GridProps {
    pub width: i32,
    pub height: i32,
    #[prop_or_else(|| 10.0)]
    pub cell_width: f64,
    #[prop_or_else(|| 10.0)]
    pub cell_height: f64,
}

impl Component for Grid {
    type Message = GridMsg;
    type Properties = GridProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Grid {
            canvas: None,
            ctx: None,
            link,
            node_ref: NodeRef::default(),
            render_loop: None,
            width: props.width,
            height: props.height,
            cell_width: props.cell_width,
            cell_height: props.cell_height,
            points: Vec::new(),
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

        ctx.set_fill_style(&"black".into());
        ctx.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);

        self.canvas = Some(canvas);
        self.ctx = Some(ctx);

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
            GridMsg::Clicked(x, y) => {
                self.points.push((x as f64, y as f64));
            }
            _ => (),
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <canvas 
                width={self.width} 
                height={self.height}
                onclick=self.link.callback(|event: MouseEvent| {
                    let x = event.client_x();
                    let y = event.client_y();

                    let eventstr = format!("{} {}", x, y);
                    ConsoleService::new().log(&eventstr);

                    GridMsg::Clicked(x as u32, y as u32)
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
    fn render_2d(&mut self) {
        let ctx = self.ctx.as_ref().expect("Canvas Context not initialized!");

        ctx.set_fill_style(&"white".into());

        for (x, y) in &self.points {
            ctx.fill_rect(*x, *y, self.cell_width, self.cell_width);
        }

        let render_frame = self.link.callback(|_| GridMsg::Render);
        let handle = RenderService::new().request_animation_frame(render_frame);
        self.render_loop = Some(Box::new(handle));
    }
}
