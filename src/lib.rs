pub use self::caesar::{ encrypt, decrypt };

mod caesar {
    fn enc(x: char) -> char {
        let ap: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        let index: usize = match ap.binary_search(&x) {
            Ok(ind) => (ind + 13) % 26,
            Err(_) => {
                panic!("Something is wrong");
            }
        };

        ap[index]
    }

    fn dec(x: char) -> char {
        let ap: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        let mut index: usize;
        match ap.binary_search(&x) {
            Ok(mut ind) => {
                ind = (ind - 13) % 26;
                if ind < 0 {
                    ind = !ind;
                }
                index = ind;
            },
            Err(_) => {
                panic!("Something is wrong");
            }
        };

        ap[index]
    }

    pub fn encrypt(text: &String) -> String {
        let mut res: String = String::new();

        for item in text.chars() {
            res.push(enc(item));
        }

        res
    }

    pub fn decrypt(text: &String) -> String {
        let mut res: String = String::new();

        for item in text.chars() {
            res.push(dec(item));
        }

        res
    }
}
