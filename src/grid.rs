use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use web_sys::CanvasRenderingContext2d;
use yew::services::{RenderService, Task};
use yew::{html, Component, ComponentLink, Html, NodeRef, ShouldRender, Properties};

pub struct Grid {
    canvas: Option<HtmlCanvasElement>,
    ctx: Option<CanvasRenderingContext2d>,
    link: ComponentLink<Self>,
    node_ref: NodeRef,
    render_loop: Option<Box<dyn Task>>,
    width: i32,
    height: i32,
    points: Vec<(u32, u32)>,
}

pub enum GridMsg {
    Render(f64),
}


#[derive(Clone, PartialEq, Properties)]
pub struct GridProps {
    #[prop_or_default]
    pub width: i32,
    #[prop_or_default]
    pub height: i32,
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
            let render_frame = self.link.callback(GridMsg::Render);
            let handle = RenderService::new().request_animation_frame(render_frame);

            self.render_loop = Some(Box::new(handle));
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            GridMsg::Render(timestamp) => {
                self.render_2d(timestamp);
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <canvas width={self.width} height={self.height} ref={self.node_ref.clone()}/>
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}

impl Grid {
    fn render_2d(&mut self, _timestamp: f64) {
        let ctx = self.ctx.as_ref().expect("Canvas Context not initialized!");

        ctx.set_fill_style(&"white".into());

        self.points.push((10, 10));
        self.points.push((11, 10));
        self.points.push((12, 10));

        for (x, y) in &self.points {
            ctx.fill_rect(*x as f64, *y as f64, 1.0, 1.0);
        }

        let render_frame = self.link.callback(GridMsg::Render);
        let handle = RenderService::new().request_animation_frame(render_frame);
        self.render_loop = Some(Box::new(handle));
    }
}
