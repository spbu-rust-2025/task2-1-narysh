use std::cmp::min;

fn preprocessing_string(s: &str) -> String {
    s.chars().flat_map(|c| ['|', c]).skip(1).collect()
}
fn find_max_palindrome(s: &str) -> String {
    let string = preprocessing_string(s);
    let chars: Vec<char> = string.chars().collect();
    let len_string = string.len();
    let mut radii = vec![0; len_string];
    let mut center = 0;
    let mut radius = 0;
    let mut best_center = 0;
    let mut best_radius = 0;
    for i in 0..len_string {
        let mut j = if i > center + radius {
            0
        } else {
            min(radii[2 * center - i], center + radius - i)
        };
        while i - j > 0 && i + j < len_string - 1 && chars[i - j - 1] == chars[i + j + 1] {
            j += 1
        }
        radii[i] = j;
        if j > best_radius {
            best_center = i;
            best_radius = j;
        }
        if i + j > radius + center {
            center = i;
            radius = j;
        }
    }
    let result_chars: Vec<char> =
        chars[best_center - best_radius..=best_center + best_radius].to_vec();
    result_chars.into_iter().filter(|c| *c != '|').collect()
}
fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    println!("{}", find_max_palindrome(input));
}
