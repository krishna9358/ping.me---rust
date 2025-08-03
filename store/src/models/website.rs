use crate::store::Store;

impl Store {
    pub fn create_website(&self) {
        println!("Hello world")
    }

    pub fn get_website(&self) -> String {
        String::from("1")
    }
}
