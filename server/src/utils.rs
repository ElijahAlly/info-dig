// pub fn camel_to_snake_case_capitalized(s: &str) -> String {
//     let mut snake_case = String::new();
//     let mut chars = s.chars().peekable();
//     let mut is_first = true;

//     while let Some(c) = chars.next() {
//         if c.is_uppercase() && !is_first {
//             snake_case.push('_');
//         }
//         snake_case.push(c);
//         is_first = false;
//     }

//     snake_case
// }