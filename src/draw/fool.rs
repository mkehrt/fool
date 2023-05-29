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
        Node("NeckBack", 3.0, 3.1),
        Node("NeckFront", 3.8, 2.7),
        Node("FrontShoulder", 5.0, 2.0),
        Node("FrontInnerElbow", 7.0, 0.0),
        Node("FrontHand", 9.0, -3.8),
        Node("FrontElbow", 7.5, 0.4),
        Node("FrontArmpit", 5.5, 3.0),
        Node("FrontWaist", 5.0, 8.2),
        Node("BackShoulder", 2.0, 3.6),
        Node("BackElbow", 0.3, 6.8),
        Node("BackHand", -0.5, 10.65),
        Node("BackInnerElbow", 1.0, 7.0),
        Node("BackArmpit", 2.5, 4.9),
        Node("BackWaist", 3.0, 8.0),
        Node("BackWaist", 3.0, 8.0),
        Node("FrontWaist", 5.0, 8.2),
        Node("Groin", 6.5, 9.5),
        Node("FrontKnee", 10.0, 12.0),
        Node("FrontAnkle", 8.0, 16.4),
        Node("FrontToes", 9.0, 17.0),
        Node("FrontHeel", 7.0, 17.0),
        Node("FrontInnerKnee", 8.0, 13.0),
        Node("Root", 4.5, 11.5),
        Node("BackKnee", 3.2, 15.8),
        Node("BackAnkle", 1.0, 19.0),
        Node("BackToes", 1.4, 20.4),
        Node("BackHeel", 0.2, 19.0),
        Node("BackInnerKnee", 2.0, 15.8),
        Node("Butt", 2.0, 10.2),
    ]
}

fn edges() -> Vec<Edge<'static>> {
    vec![
        Edge("HeadTop", "Forehead"),
        Edge("Forehead", "Chin"),
        Edge("Chin", "AdamsApple"),
        Edge("NeckFront", "AdamsApple"),
        Edge("Nape", "HeadBack"),
        Edge("HeadBack", "HeadTop"),
        Edge("HeadBack", "Nape"),
        Edge("Nape", "NeckBack"),
        Edge("NeckBack", "BackShoulder"),
        Edge("BackShoulder", "BackElbow"),
        Edge("BackElbow", "BackHand"),
        Edge("BackHand", "BackInnerElbow"),
        Edge("BackInnerElbow", "BackArmpit"),
        Edge("BackArmpit", "BackWaist"),
        Edge("NeckFront", "FrontShoulder"),
        Edge("FrontShoulder", "FrontInnerElbow"),
        Edge("FrontInnerElbow", "FrontHand"),
        Edge("FrontHand", "FrontElbow"),
        Edge("FrontElbow", "FrontArmpit"),
        Edge("FrontArmpit", "FrontWaist"),
        Edge("FrontWaist", "Groin"),
        Edge("Groin", "FrontKnee"),
        Edge("FrontKnee", "FrontAnkle"),
        Edge("FrontAnkle", "FrontToes"),
        Edge("FrontToes", "FrontHeel"),
        Edge("FrontHeel", "FrontInnerKnee"),
        Edge("FrontInnerKnee", "Root"),
        Edge("Root", "BackKnee"),
        Edge("BackKnee", "BackAnkle"),
        Edge("BackAnkle", "BackToes"),
        Edge("BackToes", "BackHeel"),
        Edge("BackHeel", "BackInnerKnee"),
        Edge("BackInnerKnee", "Butt"),
        Edge("Butt", "BackWaist"),
    ]
}

fn node_for_name(name_to_find: &str) -> Node {
    nodes()
        .into_iter()
        .find(|Node(name, _x, _y)| &name_to_find == name)
        .expect(format!("Couldn't find node {:}.", name_to_find).as_str())
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