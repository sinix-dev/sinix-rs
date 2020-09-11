use crate::models::Reply;

pub fn sinix_install(mut webview: tauri::WebviewMut, msg: Option<String>){
  let _file_name = msg.unwrap();

  let reply = Reply {
    data: "something else".to_string(),
  };

  tauri::event::emit(
    &mut webview,
    String::from("sinix-install-response"),
    Some(serde_json::to_string(&reply).unwrap()),
  )
  .expect("failed to emit");
}
