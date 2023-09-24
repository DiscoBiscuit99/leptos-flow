use leptos::{ev::MouseEvent, *};
use leptos_meta::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let nodes = vec![FlowNode {
        id: 0,
        drag_state: DragState {
            dragging: create_rw_signal(cx, false),
            initial_x: create_rw_signal(cx, 0),
            initial_y: create_rw_signal(cx, 0),
            start_x: create_rw_signal(cx, 0),
            start_y: create_rw_signal(cx, 0),
            next_x: create_rw_signal(cx, 0),
            next_y: create_rw_signal(cx, 0),
        },
    }];

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_flow.css"/>

        // sets the document title
        <Title text="Leptos Flow Development App"/>

        <LeptosFlow nodes></LeptosFlow>
    }
}

#[component]
pub fn LeptosFlow(cx: Scope, nodes: Vec<FlowNode>) -> impl IntoView {
    let (nodes, set_nodes) = create_signal(cx, nodes);
    let (dragged_node, set_dragged_node): (ReadSignal<Option<usize>>, WriteSignal<Option<usize>>) =
        create_signal(cx, None);

    let mouse_move = move |event: MouseEvent| {
        log!("mouse move on flow chart...");

        if let Some(id) = dragged_node() {
            let dx = event.client_x()
                - nodes()
                    .get(id)
                    .expect("to find dragged node")
                    .drag_state
                    .start_x
                    .get();

            let next_x = nodes()
                .get(id)
                .expect("to find dragged node")
                .drag_state
                .initial_x
                .get()
                + dx;

            nodes()
                .get(id)
                .expect("to find dragged node")
                .drag_state
                .next_x
                .set(next_x);

            let dy = event.client_y()
                - nodes()
                    .get(id)
                    .expect("to find dragged node")
                    .drag_state
                    .start_y
                    .get();

            let next_y = nodes()
                .get(id)
                .expect("to find dragged node")
                .drag_state
                .initial_y
                .get()
                + dy;

            nodes()
                .get(id)
                .expect("to find dragged node")
                .drag_state
                .next_y
                .set(next_y);
        }
    };

    let mouse_up = move |_event: MouseEvent| {
        log!("mouse up on flow chart...");
        if let Some(id) = dragged_node() {
            let next_x = nodes()
                .get(id)
                .expect("to find dragged node")
                .drag_state
                .next_x
                .get();

            nodes()
                .get(id)
                .expect("to find dragged node")
                .drag_state
                .initial_x
                .set(next_x);

            let next_y = nodes()
                .get(id)
                .expect("to find dragged node")
                .drag_state
                .next_y
                .get();

            nodes()
                .get(id)
                .expect("to find dragged node")
                .drag_state
                .initial_y
                .set(next_y);

            set_dragged_node(None);
        }
    };

    view! { cx,
        <div
            class="w-screen h-screen bg-stone-900"
            on:mouseup=mouse_up
            on:mousemove=mouse_move
        >
             <For
                each=move || nodes()
                key=|node| node.id
                view=move |cx, node: FlowNode| view! { cx,
                    <DefaultNode node set_dragged_node/>
                }
            />
        </div>
    }
}

#[component]
pub fn DefaultNode(
    cx: Scope,
    node: FlowNode,
    set_dragged_node: WriteSignal<Option<usize>>,
) -> impl IntoView {
    let (cursor, set_cursor) = create_signal(cx, "default");

    let x = move || format!("{}px", node.drag_state.next_x.get());
    let y = move || format!("{}px", node.drag_state.next_y.get());

    let mouse_down = move |event: MouseEvent| {
        set_cursor("grabbing");

        node.drag_state.start_x.set(event.client_x());
        node.drag_state.start_y.set(event.client_y());
        set_dragged_node(Some(node.id));
    };

    let mouse_up = move |_event: MouseEvent| set_cursor("grab");
    let mouse_over = move |_event: MouseEvent| set_cursor("grab");

    view! { cx,
        <div
            class="font-semibold text-slate-50 bg-indigo-500 shadow-lg shadow-indigo-500/50 px-6 py-2 w-fit absolute select-none rounded-md "
            style:left=x
            style:top=y
            style:cursor=move || cursor()
            on:mousedown=mouse_down
            on:mouseup=mouse_up
            on:mouseover=mouse_over
        >
            "TEST NODE"
        </div>
    }
}

#[derive(Debug)]
pub enum Position {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug)]
pub struct Data(String);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DragState {
    dragging: RwSignal<bool>,
    initial_x: RwSignal<i32>,
    initial_y: RwSignal<i32>,
    start_x: RwSignal<i32>,
    start_y: RwSignal<i32>,
    next_x: RwSignal<i32>,
    next_y: RwSignal<i32>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FlowNode {
    id: usize,
    drag_state: DragState,
}
