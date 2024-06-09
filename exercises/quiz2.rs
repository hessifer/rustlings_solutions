// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            match command {
                Command::Uppercase => output.push(string.to_uppercase()),
                Command::Trim => output.push(string.trim().into()),
                Command::Append(times) => {
                    let mut temp = string.clone();
                    for _ in 0..*times {
                        temp.push_str("bar");
                    }
                    output.push(temp);
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use crate::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }

    #[test]
    fn test_upper() {
        let output = transformer(vec![("hello".into(), Command::Uppercase)]);
        assert_eq!(output[0], "HELLO");
    }

    #[test]
    fn test_trim() {
        let output = transformer(vec![(" all roads lead to rome! ".into(), Command::Trim)]);
        assert_eq!(output[0], "all roads lead to rome!");
    }

    #[test]
    fn test_append_single() {
        let output = transformer(vec![("foo".into(), Command::Append(1))]);
        assert_eq!(output[0], "foobar");
    }

    #[test]
    fn test_append_multiple() {
        let output = transformer(vec![("bar".into(), Command::Append(5))]);
        assert_eq!(output[0], "barbarbarbarbarbar");
    }
}
