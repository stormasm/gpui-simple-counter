use gpui::*;

pub struct Counter01 {
    count: i32,
}

pub struct Counter02 {
    count: i32,
}

impl Global for CounterModel {}

#[derive(Clone)]
pub struct CounterModel {
    pub inner01: Model<Counter01>,
    pub inner02: Model<Counter02>,
}

impl CounterModel {
    pub fn init(cx: &mut WindowContext) -> Self {
        let this = Self {
            inner01: cx.new_model(|_| Counter01 { count: 0 }),
            inner02: cx.new_model(|_| Counter02 { count: 0 }),
        };
        cx.set_global(this.clone());
        this
    }
    pub fn update(f: impl FnOnce(&mut Self, &mut WindowContext), cx: &mut WindowContext) {
        cx.update_global::<Self, _>(|mut this, cx| {
            f(&mut this, cx);
        });
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(
            WindowOptions {
                bounds: WindowBounds::Fixed(Bounds {
                    origin: Default::default(),
                    size: size(px(1000.), px(500.)).into(),
                }),
                ..Default::default()
            },
            |cx| {
                let view = RenderCounter::build(cx);
                view
            },
        );
        cx.activate(true);
    })
}

pub struct RenderCounter {
    state: CounterModel,
}

impl RenderCounter {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        let view = cx.new_view(|cx| {
            let state = CounterModel::init(cx);
            RenderCounter { state }
        });
        view
    }
}

impl Render for RenderCounter {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        std::dbg!("Rendering counter view");
        let counter_ref_01 = self.state.inner01.read(cx);
        let counter_ref_02 = self.state.inner02.read(cx);

        let increment_button_01 = div()
            .bg(rgb(0x4caf50))
            .text_color(rgb(0xffffff))
            .child("Increment")
            .on_mouse_down(MouseButton::Left, move |_event, cx| {
                std::dbg!("Incrementing counter");
                CounterModel::update(
                    |model, cx| {
                        model.inner01.update(cx, |model, cx| {
                            model.count += 1;
                            cx.notify();
                        });
                    },
                    cx,
                )
            });

        let decrement_button_01 = div()
            .bg(rgb(0x660033))
            .text_color(rgb(0xffffff))
            .child("Decrement")
            .on_mouse_down(MouseButton::Left, move |_event, cx| {
                std::dbg!("Decrementing counter");
                CounterModel::update(
                    |model, cx| {
                        model.inner01.update(cx, |model, cx| {
                            model.count -= 1;
                            cx.notify();
                        });
                    },
                    cx,
                )
            });

        let increment_button_02 = div()
            .bg(rgb(0x4caf50))
            .text_color(rgb(0xffffff))
            .child("Increment")
            .on_mouse_down(MouseButton::Left, move |_event, cx| {
                std::dbg!("Incrementing counter");
                CounterModel::update(
                    |model, cx| {
                        model.inner02.update(cx, |model, cx| {
                            model.count += 1;
                            cx.notify();
                        });
                    },
                    cx,
                )
            });

        let decrement_button_02 = div()
            .bg(rgb(0x660033))
            .text_color(rgb(0xffffff))
            .child("Decrement")
            .on_mouse_down(MouseButton::Left, move |_event, cx| {
                std::dbg!("Decrementing counter");
                CounterModel::update(
                    |model, cx| {
                        model.inner02.update(cx, |model, cx| {
                            model.count -= 1;
                            cx.notify();
                        });
                    },
                    cx,
                )
            });

        div()
            .flex_row()
            .child(
                div()
                    .child(div())
                    .bg(rgb(0x333300))
                    .size_full()
                    .justify_center()
                    .items_center()
                    .text_xl()
                    .text_color(rgb(0xffffff))
                    .child(div())
                    .flex()
                    .child(div().flex().flex_col().children(vec![
                            increment_button_01,
                            decrement_button_01,
                            div()
                                .bg(rgb(0x000000))
                                .text_color(rgb(0xffffff))
                                .child(
                                    format!("The number is: {}!", counter_ref_01.count.to_string())
                                ),
                        ])),
            )
            .flex_row()
            .child(div().flex().flex_col().children(vec![
                            increment_button_02,
                            decrement_button_02,
                            div()
                                .bg(rgb(0x000000))
                                .text_color(rgb(0xffffff))
                                .child(
                                    format!("The number is: {}!", counter_ref_02.count.to_string())
                                ),
                        ]))
    }
}
