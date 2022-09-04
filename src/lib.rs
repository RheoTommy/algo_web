// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use itertools::izip;
use rand::prelude::SliceRandom;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use seed::{prelude::*, *};
use std::fmt::{Display, Formatter};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    model_default()
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    numbers: Vec<i64>,
    points: Vec<(u64, u64)>,
    is_used: Vec<bool>,
    selected: Option<usize>,
    page: Page,
    is_finished: bool,
    name: String,
}

fn model_default() -> Model {
    let mut rng = StdRng::from_seed([13; 32]);

    let x = [6, 18, 30];
    let y = [16, 28, 40, 52, 64, 76];
    let mut points = x
        .iter()
        .flat_map(|&i| y.iter().map(move |&j| (i as i64, j as i64)))
        .map(|(i, j)| (i + rng.gen_range(-2, 3), j + rng.gen_range(-2, 3)))
        .map(|(i, j)| (i as u64, j as u64))
        .collect::<Vec<_>>();
    points.shuffle(&mut rng);

    let n = rng.gen_range(12, 13);
    let numbers = (0..n).map(|_| rng.gen_range(-100, 101)).collect::<Vec<_>>();

    points.resize(n, (50, 50));

    Model {
        numbers,
        points,
        is_used: vec![false; n],
        selected: None,
        page: Page::Play,
        is_finished: true,
        name: String::default(),
    }
}

#[derive(Eq, PartialEq)]
enum Page {
    Play,
    Ranking,
}

impl Display for Page {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Page::Play => "Play",
                Page::Ranking => "Ranking",
            }
        )
    }
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Clicked(usize),
    ClickedPlay,
    ClickedRanking,
    ClickedRollBack,
    ClickedSendButton,
    ChangedTextArea(String),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Clicked(id) => match model.selected {
            Some(a_id) => {
                if a_id != id {
                    model.numbers[a_id] -= model.numbers[id];
                    model.is_used[id] = true;
                }
                model.selected = None;

                if model.is_used.iter().filter(|&&b| b).count() + 1 == model.is_used.len() {
                    model.is_finished = true;
                }
            }
            None => {
                model.selected = Some(id);
            }
        },
        Msg::ClickedPlay => model.page = Page::Play,
        Msg::ClickedRanking => model.page = Page::Ranking,
        Msg::ClickedRollBack => {}
        Msg::ClickedSendButton => {}
        Msg::ChangedTextArea(str) => model.name = str,
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.

const HEADER_HEIGHT: u64 = 10;
const BOARD_HEIGHT: u64 = 40;
const BOARD_WIDTH: u64 = 80;

fn view(model: &Model) -> Node<Msg> {
    div![
        style! {
            St::VerticalAlign => "top",
            St::Height => vh(100),
            St::MinHeight => vh(100),
            St::Width => vw(100),
            St::MinWidth => vw(100),
        },
        view_header(),
        match model.page {
            Page::Play => view_play(model),
            Page::Ranking => a!("ranking"),
        }
    ]
}

fn view_header() -> Node<Msg> {
    header![
        style! {
            St::Background => "#4385f4",
            St::Color => "white",
            St::Height => vh(HEADER_HEIGHT),
            St::Width => vw(100),
            St::Padding => "20px 50px",
            St::BoxSizing => "border-box",
            St::Position => "fixed",
            St::Top => px(0),
            St::Left => px(0),
            St::Display => "flex",
            St::AlignItems => "center",
            St::FontSize => px(30),
            St::UserSelect => "none",
            St::ZIndex => "1",
        },
        h1!("Algo"),
        ul![
            style! {St::Display => "flex"},
            header_li(Page::Play, Msg::ClickedPlay),
            header_li(Page::Ranking, Msg::ClickedRanking),
        ]
    ]
}

fn header_li(page: Page, msg: Msg) -> Node<Msg> {
    li![
        style! {
            St::ListStyle => "none",
            St::MarginLeft => vw(5),
        },
        page.to_string(),
        ev(Ev::Click, move |_| msg),
    ]
}

fn view_play(model: &Model) -> Node<Msg> {
    div![
        style! {
            St::Position => "absolute",
            St::Width => vw(100),
            St::Height => vh(100 - HEADER_HEIGHT),
            St::Top => vh(HEADER_HEIGHT),
        },
        div![
            style! {
                St::BackgroundImage => r##"url("../img/black_board.png")"##,
                St::BackgroundRepeat => "no-repeat",
                St::BackgroundPosition => "center top",
                St::BackgroundSize => "contain",
                St::Width => vw(BOARD_WIDTH),
                St::MinWidth => vw(BOARD_WIDTH),
                St::Height => vw(BOARD_HEIGHT),
                St::MinHeight => vw(BOARD_HEIGHT),
                St::Margin => "0 auto",
            },
            izip!(&model.numbers, &model.points, &model.is_used)
                .enumerate()
                .filter(|(_, (_, _, &b))| !b)
                .map(|(id, (&n, &(x, y), _))| view_num(
                    id,
                    n,
                    x,
                    y,
                    model.selected.map(|idx| idx == id).unwrap_or(false)
                ))
                .collect::<Vec<_>>(),
        ],
        div![
            button! {
                "一手戻す",
                style!{
                    St::FontSize => px(30),
                },
                ev(Ev::Click, |_| Msg::ClickedRollBack),
            },
            style! {
                St::Position => "relative",
                St::Padding => "0 0 5vw 5vw",
            },
            IF!(model.is_finished => view_result(model)),
        ],
    ]
}

fn view_num(id: usize, n: i64, x: u64, y: u64, is_selected: bool) -> Node<Msg> {
    div![
        n.to_string(),
        style! {
            St::Position => "absolute",
            St::Top => vw(x),
            St::Left => vw(y),
            St::FontSize => px(40),
            St::Color => if is_selected {"yellow"} else {"lightgray"}
            St::UserSelect => "none",
        },
        ev(Ev::Click, move |_| Msg::Clicked(id)),
    ]
}

fn view_result(model: &Model) -> Node<Msg> {
    let score = *model
        .numbers
        .iter()
        .zip(model.is_used.iter())
        .find(|(_, &b)| !b)
        .unwrap()
        .0;
    div![
        style! {
            St::Top => vw(BOARD_HEIGHT),
            St::FontSize => px(40),
        },
        h2!(format!("最終スコア：{}", score)),
        p!("今回の結果を順位表に登録する↓"),
        div![
            input! {
                style!{
                    St::FontSize => px(30),
                    St::Margin => "0 0 3vw 3vw",
                },
                attrs!{
                    At::Type => "text",
                    At::Placeholder => "ここに名前を入力してね",
                },
                input_ev(Ev::Input, Msg::ChangedTextArea)
            },
            button! {
                style!{
                    St::FontSize => px(30),
                    St::Margin => "0 0 3vw 3vw",
                },
                "順位表に送信",
                ev(Ev::Click, |_| Msg::ClickedSendButton),
            },
        ],
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
