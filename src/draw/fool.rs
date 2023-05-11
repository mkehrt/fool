use crate::color::colors;
use crate::draw::star;
use crate::log::log;
use crate::{Context, HEIGHT};

static STAR_RADIUS: f64 = 0.35;
static STROKE_WIDTH: f64 = 0.1;

struct Node<'a>(&'a str, f64, f64);
struct Edge<'a>(&'a str, &'a str);

fn nodes() -> Vec<Node<'static>> {
    vec![
        Node("HeadTop", 2.0, 0.0),
        Node("Forehead", 3.5, 0.0),
        Node("Chin", 4.0, 1.67),
        Node("AdamsApple", 3.5, 2.25),
        Node("Nape", 2.6, 2.2),
        Node("HeadBack", 1.5, 1.2),
    ]
}

fn edges() -> Vec<Edge<'static>> {
    vec![
        Edge("HeadTop", "Forehead"),
        Edge("Forehead", "Chin"),
        Edge("Chin", "AdamsApple"),
        Edge("Nape", "HeadBack"),
        Edge("HeadBack", "HeadTop"),
    ]
}

fn node_for_name(name_to_find: &str) -> Node {
    nodes()
        .into_iter()
        .find(|Node(name, _x, _y)| &name_to_find == name)
        .expect("Couldn't find node.")
}

pub fn draw(context: &mut Context) {
    context.save();

    let _ = context.scale(1.0, -1.0);
    let _ = context.translate(0.0, -HEIGHT);
    let _ = context.translate(6.0, 10.0);

    for Node(_name, x, y) in nodes() {
        let star_angle = 0.0;
        star::draw(context, colors::FOOL, x, y, STAR_RADIUS, star_angle);
    }

    context.set_stroke_style(&colors::FOOL.into());
    context.set_line_width(STROKE_WIDTH);

    for Edge(begin, end) in edges() {
        let Node(_name, begin_x, begin_y) = node_for_name(begin);
        let Node(_name, end_x, end_y) = node_for_name(end);

        context.begin_path();

        context.move_to(begin_x, begin_y);
        context.line_to(end_x, end_y);

        context.stroke();

        log(format!("Line {:} {:} {:} {:}", begin_x, begin_y, end_x, end_y).as_str());
    }

    context.restore();
}
