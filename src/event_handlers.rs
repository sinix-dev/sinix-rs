use crate::models::Reply;
use std::path::Path;
use std::fs;
use std::io;
use zip;
use dirs;

fn extract_and_copy(file_name: String){
  let file = fs::File::open(&file_name).unwrap();
  let mut archive = zip::ZipArchive::new(file).unwrap();

  let home_dir = dirs::home_dir().unwrap();
  let games_dir = Path::new(&home_dir).join(".sinix/games").to_str().unwrap().to_string();

  for i in 0..archive.len() {
    let mut file = archive.by_index(i).unwrap();
    let outpath = Path::new(&games_dir).join(file.sanitized_name());

    {
      let comment = file.comment();
      if !comment.is_empty() {
        println!("File {} comment: {}", i, comment);
      }
    }

    if (&*file.name()).ends_with("/") {
      println!(
        "File {} extracted to \"{}\"",
        i,
        outpath.as_path().display()
      );

      fs::create_dir_all(&outpath).unwrap();
    } else {
      println!(
        "File {} extracted to \"{}\" ({} bytes)",
        i,
        outpath.as_path().display(),
        file.size()
      );

      if let Some(p) = outpath.parent() {
        if !p.exists() {
          fs::create_dir_all(&p).unwrap();
        }
      }

      let mut outfile = fs::File::create(&outpath).unwrap();
      io::copy(&mut file, &mut outfile).unwrap();
    }

    // Get and Set permissions
    #[cfg(unix)]
    {
      use std::os::unix::fs::PermissionsExt;

      if let Some(mode) = file.unix_mode() {
        fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
      }
    }
  }
}

pub fn sinix_install(mut webview: tauri::WebviewMut, msg: Option<String>){
  // Install a new app
  let file_name = msg.unwrap();
  extract_and_copy(file_name);

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
