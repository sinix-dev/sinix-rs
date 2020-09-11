mod db;
mod event_handlers;
mod models;

use std::path::{Path, PathBuf};
use tauri::Webview;
use std::fs;
use dirs;

pub fn init() {
  let home = dirs::home_dir().unwrap();
  let games: String = Path::new(&home).join(".sinix/games").to_str().unwrap().to_string();
  let data: String = Path::new(&home).join(".sinix/data").to_str().unwrap().to_string();

  println!("{:?}\n{:?}", games, data);

  fs::create_dir_all(&games).unwrap();
  fs::create_dir_all(&data).unwrap();
}

pub fn tauri_handler(webview: &mut Webview, _source: String) {
  // handler function for tauri app setup callback

  let webview = webview.as_mut();

  tauri::event::listen(String::from("sinix-install"), move |msg| {
    event_handlers::sinix_install(webview.clone(), msg)
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
