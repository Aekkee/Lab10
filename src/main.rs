use rand::Rng; 
use std::fs::File; 
use clap::{App, Arg};
use std::io::Write;

#[derive(Debug)]
enum Check {
    Inside((Point,f32)),
    Outside((Point,f32)),
}

#[derive(Debug)]
enum Check2 {
    InsideCircle1((Point,f32,f32)),
    InsideCircle2((Point,f32,f32)),
    InsideBoth((Point,f32,f32)),
    OutsideBoth((Point,f32,f32)),
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: f32
}

#[derive(Debug)]
struct Bound {
    c1: Circle,
    c2: Circle
}

#[derive(Debug)]
struct RandConfig {
    x_min: f32, x_max: f32,
    y_min: f32, y_max: f32
}

fn main() {
    // let mut rng = rand::thread_rng();
    // let cfg = RandConfig{
    //     x_min: -1.5, x_max: 1.5,
    //     y_min: -1.5, y_max: 1.5
    // };
    // let c1 = Circle{center: Point{x: -0.1, y: -0.1}, radius: 0.8};
    // let c2 = Circle{center: Point{x: 1., y: -0.1}, radius: 0.8};
    // let bo = Bound{ c1, c2};
    // let pt_list: Vec<_> = gen_point_list(&mut rng, &cfg, 50);
    // let loc_list = locate_n_point2(&bo, &pt_list);
    // for loc in loc_list {
    //     println!("{loc:?}");
    // }
    commandline2()
}

fn gen_point_list<R: Rng>(rng: &mut R, config: &RandConfig, n:i32) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();
    for _ in 0..n{
        result.push(Point{ x: (rng.gen_range(config.x_min ..= config.x_max) * 100.0).round() / 100.0, y: (rng.gen_range(config.y_min ..= config.y_max) * 100.0).round() / 100.0});
    }
    return result
}

fn locate_n_point(c: &Circle, plist: &Vec<Point>) -> Vec<Check> {
    let mut result: Vec<Check> = Vec::new();
    for i in plist {
        let d = (((i.x - c.center.x).powf(2.) + (i.y - c.center.y).powf(2.)).sqrt() * 100.0).round() / 100.;
        if d <= c.radius {
            result.push(Check::Inside((Point{x: i.x, y: i.y}, d)))
        } else {
            result.push(Check::Outside((Point{x: i.x, y: i.y}, d)))
        }
    }
    return result
}

#[test]
fn test_locate() {
    let mut rng = rand::thread_rng();
    let cfg = RandConfig{
        x_min: -1.5, x_max: 1.5,
        y_min: -1.5, y_max: 1.5
    };
    let c = Circle{center: Point{x: -0.1, y: -0.1}, radius: 0.8};
    let pt_list: Vec<_> = gen_point_list(&mut rng, &cfg, 50);
    for i in &pt_list {
        assert!(
            i.x >= cfg.x_min && i.x <= cfg.x_max && i.y >= cfg.y_min && i.y <= cfg.y_max
        )
    }
    assert_eq!(format!("{:?}", locate_n_point(&c, &pt_list)), format!("{:?}", locate_n_point(&c, &pt_list)))
}

extern crate clap;

fn commandline1() {
    let matches = App::new("My Program")
    .version("1.0")
    .author("Your Name")
    .about("Description of your program")
    .arg(
        Arg::with_name("number")
            .short("n")
            .long("number")
            .value_name("NUMBER")
            .help("Number of layer")
            .required(true),
        )
        .get_matches();

let vec: Vec<f32> = matches.value_of("number").unwrap().split(",").into_iter().map(|x| x.parse().unwrap_or(0.)).collect();

let mut file = File::create("output1.html").expect("Failed");
let mut rng = rand::thread_rng();
let cfg = RandConfig{
    x_min: vec[0], x_max: vec[1],
    y_min: vec[2], y_max: vec[3]
};

let (w, h) = (600, 600);    // SVG image size
let scale = (h as f32) / (cfg.y_max - cfg.y_min);
let c = Circle{center: Point{x: vec[5], y: vec[6]}, radius: vec[7]};
let pt_list: Vec<_> = gen_point_list(&mut rng, &cfg, vec[4] as i32);
let loc_list = locate_n_point(&c, &pt_list);
let _= file.write(b"<html>\n");
let _= file.write(b"<body>\n");
let _= file.write(b"<h1>layer list</h1>\n");
let _= file.write(format!("<svg width=\"{w}\" height=\"{h}\" xmlns=\"http://www.w3.org/2000/svg\">\n").as_bytes());
let _= file.write(
    format!(
        "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"none\" stroke=\"{}\" />\n",
        (c.center.x - cfg.x_min) * scale,
        (-c.center.y - cfg.y_min) * scale,
        c.radius * scale,
        "black"
    )
    .as_bytes(),
);
for i in loc_list {
    let _= match i {
        Check::Inside(t) => file.write(
            format!(
                "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" />\n",
                (t.0.x - cfg.x_min) * scale,
                (-t.0.y - cfg.y_min) * scale,
                5,
                "green"
            )
            .as_bytes(),
        ),
        Check::Outside(t) => file.write(
            format!(
                "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" />\n",
                (t.0.x - cfg.x_min) * scale,
                (-t.0.y - cfg.y_min) * scale,
                5,
                "red"
            )
            .as_bytes(),
        )
    };
}
let _= file.write(b"\n</svg>\n</body>\n</html>");
}

fn locate_n_point2(b: &Bound, plist: &Vec<Point>) -> Vec<Check2> {
    let mut result: Vec<Check2> = Vec::new();
    for i in plist {
        let d1 = (((i.x - b.c1.center.x).powf(2.) + (i.y - b.c1.center.y).powf(2.)).sqrt() * 100.0).round() / 100.;
        let d2 = (((i.x - b.c2.center.x).powf(2.) + (i.y - b.c2.center.y).powf(2.)).sqrt() * 100.0).round() / 100.;
        if (d1 <= b.c1.radius) && (d2 <= b.c2.radius){
            result.push(Check2::InsideBoth((Point{x: i.x, y: i.y}, d1, d2)))
        } else if d1 <= b.c1.radius {
            result.push(Check2::InsideCircle1((Point{x: i.x, y: i.y}, d1, d2)))
        } else if d2 <= b.c2.radius {
            result.push(Check2::InsideCircle2((Point{x: i.x, y: i.y}, d1, d2)))
        } else {
            result.push(Check2::OutsideBoth((Point{x: i.x, y: i.y}, d1, d2)))
        }
    }
    return result
}

fn commandline2() {
    let matches = App::new("My Program")
    .version("1.0")
    .author("Your Name")
    .about("Description of your program")
    .arg(
        Arg::with_name("number")
            .short("n")
            .long("number")
            .value_name("NUMBER")
            .help("Number of layer")
            .required(true),
        )
        .get_matches();

let vec: Vec<f32> = matches.value_of("number").unwrap().split(",").into_iter().map(|x| x.parse().unwrap_or(0.)).collect();

let mut file = File::create("output2.html").expect("Failed");
let mut rng = rand::thread_rng();
let cfg = RandConfig{
    x_min: vec[0], x_max: vec[1],
    y_min: vec[2], y_max: vec[3]
};

let (w, h) = (600, 600);    // SVG image size
let scale = (h as f32) / (cfg.y_max - cfg.y_min);
let c1 = Circle{center: Point{x: vec[5], y: vec[6]}, radius: vec[7]};
let c2 = Circle{center: Point{x: vec[8], y: vec[9]}, radius: vec[10]};
let bo = Bound{c1, c2};
let pt_list: Vec<_> = gen_point_list(&mut rng, &cfg, vec[4] as i32);
let loc_list = locate_n_point2(&bo, &pt_list);
let _= file.write(b"<html>\n");
let _= file.write(b"<body>\n");
let _= file.write(b"<h1>layer list</h1>\n");
let _= file.write(format!("<svg width=\"{w}\" height=\"{h}\" xmlns=\"http://www.w3.org/2000/svg\">\n").as_bytes());
let _= file.write(
    format!(
        "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"none\" stroke=\"{}\" />\n",
        (bo.c1.center.x - cfg.x_min) * scale,
        (-bo.c1.center.y - cfg.y_min) * scale,
        bo.c1.radius * scale,
        "black"
    )
    .as_bytes(),
);
let _= file.write(
    format!(
        "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"none\" stroke=\"{}\" />\n",
        (bo.c2.center.x - cfg.x_min) * scale,
        (-bo.c2.center.y - cfg.y_min) * scale,
        bo.c2.radius * scale,
        "black"
    )
    .as_bytes(),
);
for i in loc_list {
    let _= match i {
        Check2::InsideCircle1(t) => file.write(
            format!(
                "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" />\n",
                (t.0.x - cfg.x_min) * scale,
                (-t.0.y - cfg.y_min) * scale,
                5,
                "green"
            )
            .as_bytes(),
        ),
        Check2::InsideCircle2(t) => file.write(
            format!(
                "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" />\n",
                (t.0.x - cfg.x_min) * scale,
                (-t.0.y - cfg.y_min) * scale,
                5,
                "blue"
            )
            .as_bytes(),
        ),
        Check2::InsideBoth(t) => file.write(
            format!(
                "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" />\n",
                (t.0.x - cfg.x_min) * scale,
                (-t.0.y - cfg.y_min) * scale,
                5,
                "pink"
            )
            .as_bytes(),
        ),
        Check2::OutsideBoth(t) => file.write(
            format!(
                "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" />\n",
                (t.0.x - cfg.x_min) * scale,
                (-t.0.y - cfg.y_min) * scale,
                5,
                "red"
            )
            .as_bytes(),
        )
    };
}
let _= file.write(b"\n</svg>\n</body>\n</html>");
}