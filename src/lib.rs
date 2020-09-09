pub mod db;

use tauri::Webview;
use serde::Serialize;

#[derive(Serialize)]
struct Reply {
  data: String,
}

pub fn blah() {
  println!("Blah bleh bluh");
}

pub fn tauri_handler(webview: &mut Webview, source: String) {
  let mut webview = webview.as_mut();

  tauri::event::listen(String::from("js-event"), move |msg| {
    println!("got js-event with message '{:?}'", msg);
    let reply = Reply {
      data: "something else".to_string(),
    };

    tauri::event::emit(
      &mut webview,
      String::from("rust-event"),
      Some(serde_json::to_string(&reply).unwrap()),
    )
    .expect("failed to emit");
  });
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
      let mut db_instance = super::db::init(String::from("sanket143.db"));
      db_instance.set("name", &"sanket").unwrap();
    }
}
