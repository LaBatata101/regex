use regex::regex::Regex;

pub fn main() {
    let re = Regex::new("[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+");
    re.is_match("example.samplemail@gmail.com");
}
