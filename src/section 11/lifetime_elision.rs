fn main() {
    let str_1 = "some str";
    let str_2 = "other str";
    // lifetime elision - allow compiler automaticly infer the lifetimes in functions and methods signiture following these 3 rules
    // 1. each parameter that is a reference, gets its own lifetime parameter
    // 2. if there's exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
    // 3. if there are multiple input lifetime parameters, but one of them is &self of &mut self, the lifetime of self is assigned to all output lifetime parameteres
    let recieved_str = return_str_2(&str_1, &str_2);
}

// how the compiler see the return_str
// fn return_str<'a>(s_1: &'a str) -> &'a str {
//     s_1
// }

fn return_str(s_1: &str) -> &str {
    s_1
}

// the compiler cannot define what lifetime it should give to the output, cause s_1 and s_2 has its own lifetimes
// fn return_str_2<'a, 'b>(s_1: &'a str, s_2: &'b str) -> &str {
//     s_1
// }

// since we return only s_1, we can set its lifetime to the output
fn return_str_2<'a, 'b>(s_1: &'a str, s_2: &'b str) -> &'a str {
    s_1
}
