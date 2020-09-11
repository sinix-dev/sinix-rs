mod db;
mod event_handlers;
mod models;

use tauri::Webview;

pub fn init() {
  // Initialize the Sinix app
  // Directory to store games
  // Cacheing directory
}

pub fn tauri_handler(webview: &mut Webview, _source: String) {
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
