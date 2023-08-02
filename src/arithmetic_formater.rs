use std::str::FromStr;

pub fn arithmetic_formater(problems: Vec<&str>) -> String {
    let mut first_line: String = String::from("");
    let mut second_line: String = String::from("");
    let mut separator_line: String = String::from("");
    let mut result_line: String = String::from("");
    let len_problems = problems.len();

    for problem in problems {
        let problem_array: Vec<&str> = problem.split_whitespace().collect();
        let first_nb: String = problem_array[0].to_string();
        let second_nb: String = problem_array[2].to_string();
        let operator: String = problem_array[1].to_string();
        let spaces: usize = create_spaces(&first_nb, &second_nb);

        // handling errors
        match true {
            true if len_problems > 5 => return String::from("Error: Too many problems."),
            true if first_nb.len() > 4 || second_nb.len() > 4 => {
                return String::from("Error: Numbers cannot be more than four digits.")
            }
            true if &operator != "+" && &operator != "-" => {
                return String::from("Error: Operator must be '+' or '-'.")
            }
            true if !first_nb.chars().all(char::is_numeric)
                || !second_nb.chars().all(char::is_numeric) =>
            {
                return String::from("Error: Numbers must only contain digits.")
            }
            _ => (),
        }

        //pirnt first line
        first_line = format!(
            "{} {}",
            &mut first_line,
            create_line(&first_nb, &spaces, &String::from(" "))
        );

        //pirnt second line
        second_line = format!(
            "{} {}",
            &mut second_line,
            create_line(&second_nb, &spaces, &operator)
        );

        //pirnt separation line
        let mut counter: usize = spaces;
        let mut separator: String = String::from("");
        while counter > 0 {
            separator.push('-');
            counter -= 1;
        }
        separator_line = format!(
            "{} {}",
            &mut separator_line,
            create_line(&separator, &spaces, &String::from("-"))
        );

        //pirnt result line
        let mut sum: i32 = 0;
        let sum_first_number: i32 = i32::from_str(problem_array[0]).unwrap();
        let sum_second_number: i32 = i32::from_str(problem_array[2]).unwrap();
        if &operator == "+" {
            sum += sum_first_number + sum_second_number;
        } else {
            sum = sum_first_number - sum_second_number - sum;
        }
        result_line = format!(
            "{} {}",
            &mut result_line,
            create_line(&sum.to_string(), &spaces, &String::from(" "))
        );
    }

    //put all lines together
    let problems_lines: Vec<String> = vec![first_line, second_line, separator_line, result_line];
    let arranged_problems: String = problems_lines.join("\n");
    arranged_problems
}

fn create_line(number: &String, spaces: &usize, operator: &String) -> String {
    let mut white_spaces: String = String::from("");
    let mut counter: usize = spaces - number.len();

    loop {
        if counter > 0 {
            white_spaces.push(' ');
            counter -= 1;
        } else {
            break;
        };
    }

    let line = format!("{}{}{}", operator, white_spaces, number);
    line.to_string()
}

fn create_spaces(first_nb: &String, second_nb: &String) -> usize {
    let len_first_nb: usize = first_nb.len();
    let len_second_nb: usize = second_nb.len();

    if len_first_nb > len_second_nb {
        len_first_nb + 1
    } else {
        len_second_nb + 1
    }
}
