mod bookmark;

use bookmark::Bookmark;
use sanitize_filename::sanitize;
use scraper::{ElementRef, Html, Selector};
use std::{env, fs, path::PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut bookmarks = "bookmarks.html";
    if args.len() > 1 {
        bookmarks = &args[1];
    }

    let contents = fs::read_to_string(bookmarks)
        .expect("请指定书签html路径或者同一文件夹里面有bookmarks.html");
    let document = Html::parse_document(&contents);
    let archor_sel = Selector::parse("dt > a").unwrap();
    for el in document.select(&archor_sel) {
        let mut names: Vec<String> = vec![];
        // @TODO 错误判断
        let cur_dt = el
            .parent()
            .and_then(|el| el.parent())
            .and_then(|el| el.parent())
            .unwrap();
        let mut dt = cur_dt.clone();
        while dt.value().as_element().unwrap().name() == "dt" {
            let cur_name: String = dt
                .first_child()
                .and_then(ElementRef::wrap)
                .unwrap()
                .inner_html();

            names.push(cur_name);
            // @TODO 错误判断
            dt = dt.parent().and_then(|el| el.parent()).unwrap();
        }
        let output_path: String = names.iter().rev().fold(String::new(), |a, b| {
            format!("{}/{}", a, sanitize(b).trim())
        });

        let b = Bookmark::from_anchor(&el);
        let res = if output_path.starts_with("/") {
            b.write_shortcut(&PathBuf::from(&output_path[1..]))
        } else {
            b.write_shortcut(&PathBuf::from(&output_path))
        };
        if let Err(err) = res {
            eprintln!("{}", err);
        }
    }
}
