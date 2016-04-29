pub use self::caesar::{ encrypt, decrypt };

mod caesar {
    fn enc(x: char) -> char {
        let ap: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        let res: char;
        match ap.binary_search(&x) {
            Ok(ind) => {
                res = ap[(ind + 13) % 26];
            },
            Err(_) => {
                res = x;
            }
        };

        res
    }

    fn dec(x: char) -> char {
        let ap: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        let res: char;
        match ap.binary_search(&x) {
            Ok(mut index) => {
                if index < 13 {
                    index = index + 26;
                }
                res = ap[index - 13];
            },
            Err(_) => {
                res = x;
            }
        };

        res
    }

    pub fn encrypt(text: &String) -> String {
        let mut res: String = String::new();

        for item in text.chars() {
            res.push(enc(item));
        }

        String::from(res.trim())
    }

    pub fn decrypt(text: &String) -> String {
        let mut res: String = String::new();

        for item in text.chars() {
            res.push(dec(item));
        }

        String::from(res.trim())
    }
}
