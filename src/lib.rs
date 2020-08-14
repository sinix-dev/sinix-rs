pub mod db;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
      let mut db_instance = super::db::init(String::from("sanket143.db"));
      db_instance.set("name", &"sanket").unwrap();
    }
}
