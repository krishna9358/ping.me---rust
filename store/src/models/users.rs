use crate::store::Store;

impl Store {
    pub fn create_user(&self) {
        println!("Hello world")
    }

    pub fn get_user(&self) -> String {
        String::from("1")
    }
}
