fn sepreit_number_str(str: &str) -> (String, String) {
    let mut key = String::from("");
    let mut hash = String::from("");
    for item in str.split("") {
        if item.parse::<i32>().is_ok() {
            key.push_str(item);
        } else {
            hash.push_str(item);
        }
    }

    return (key, hash);
}

fn handle_decode(key: String, char: char) -> String {
    let mut out = String::from("");

    let alphabet = ('a'..='z').into_iter().collect::<Vec<char>>();

    // get index item

    let mut index_of_char = 0;
    for (j, c) in alphabet.iter().enumerate() {
        if *c == char {
            index_of_char = j;
            break;
        }
    }

    // decoded char
    let code = key.parse::<usize>().unwrap();
    for (index, char) in alphabet.iter().enumerate() {
        if ((index) * code) % 26 == index_of_char {
            out.push(*char)
        }
    }

    out
}

fn decode(s: &str) -> String {
    let mut output = String::from("");

    let (key, hash) = sepreit_number_str(s);

    for char in hash.chars() {
        let res = handle_decode(key.clone(), char);
        if res.len() > 1 {
            output.push_str("Impossible to decode");
            break;
        } else {
            output.push_str(&res)
        }
    }

    output
}

fn dotest(s: &str, exp: &str) -> () {
    let ans = decode(s);
    println!("{:?}", ans == exp);
    assert_eq!(ans, exp, "Testing: {}", s);
}

fn basic_tests() {
    let mut s = "10559625hbkeohysnztuuqdznnkkcgjndbujej";
    dotest(s, "dtiygdoenhxqqsfhnniimkpnftqpyp");

    s = "1273409kuqhkoynvvknsdwljantzkpnmfgf";
    dotest(s, "uogbucwnddunktsjfanzlurnyxmx");

    s = "761328qockcouoqmoayqwmkkic";
    dotest(s, "Impossible to decode");
}
fn main() {
    basic_tests()
}
