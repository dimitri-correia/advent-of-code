// use core::str;

// pub fn part_2_example() -> String {
//     let input = include_str!("input2_ex.txt");
//     part_2(input)
// }

// pub fn part_2_actual_challenge() -> String {
//     let input = include_str!("input1.txt");
//     part_2(input)
// }

// fn part_2(input: &str) -> String {
//     let mut chars = input.chars();
//     let mut parser_global: Parser;
//     let mut actual_parser: &mut Parser = &mut parser_global;
//     let mut lvl_inside = 0;
//     loop {
//         let c = chars.next();
//         if c.is_none() {
//             break;
//         }
//         let c = c.unwrap();
//         match c {
//             '-' | '0'..='9' => {
//                 let (num, c) = parse_number(&mut chars, c);
//                 let parser = Parser {
//                     type_of_sum: TypeOfSum::Number,
//                     val: Some(num),
//                     inside: None,
//                 };
//                 match c {
//                     ']' => {
//                         assert!(actual_parser.type_of_sum == TypeOfSum::Array);
//                         actual_parser.inside = Some(vec![Box::new(parser)]);
//                     }
//                     '}' => {
//                         sum += num;
//                     }
//                     ',' => {
//                         curr = num;
//                     }
//                     '_' => {
//                         unreachable!("unexpected character");
//                     }
//                 }
//             }
//             '[' => {
//                 lvl_inside += 1;
//                 actual_parser = &mut Parser {
//                     type_of_sum: TypeOfSum::Array,
//                     val: None,
//                     inside: None,
//                 };
//             }
//             ']' => {
//                 sum += curr;
//             }
//             _ => {
//                 sum += curr;
//                 curr = 0;
//             }
//         }
//     }
//     sum.to_string()
// }

// fn parse_number(chars: &mut str::Chars, c: char) -> (i32, char) {
//     let mut num = if c == '-' {
//         -1
//     } else {
//         c.to_digit(10).unwrap() as i32
//     };
//     loop {
//         let c = chars.next();
//         if c.is_none() {
//             break;
//         }
//         let c = c.unwrap();
//         match c {
//             '0'..='9' => {
//                 num = num * 10 + c.to_digit(10).unwrap() as i32;
//             }
//             _ => {
//                 break;
//             }
//         }
//     }
//     (num, c)
// }

// #[derive(PartialEq)]
// enum TypeOfSum {
//     Number,
//     Array,
//     Object,
// }

// struct Parser {
//     type_of_sum: TypeOfSum,
//     val: Option<i32>,
//     inside: Option<Vec<Box<Parser>>>,
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn actual_challenge() {
//         let output = part_2_actual_challenge();
//         assert_eq!("", output);
//     }
//     #[test]
//     fn example_test() {
//         let r = part_2_example();
//         assert_eq!("4", r);
//     }
// }
