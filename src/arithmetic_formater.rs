pub fn arithmetic_formater(problems: Vec<&str>) -> String {
    let len_problems = problems.len();

    if len_problems > 4 {
        String::from("Error: Too many problems.")
    } else {
        let mut first_line: String;
        let mut second_line: String;
        let mut separator: String;

        for problem in problems {
            let problem_array: Vec<&str> = problem.split_whitespace().collect();
            let first_nb: String = String::from(problem_array[0]);
            let second_nb: String = String::from(problem_array[2]);
            let operator: String = String::from(problem_array[1]);

            let spaces: usize = create_spaces(&first_nb, &second_nb);
            first_line = format!(
                "{}    {}",
                &first_line,
                create_line2(first_nb, &operator, &spaces)
            );
        }

        first_line
    }
}

fn create_line2<'a>(number: String, operator: &String, spaces: &usize) -> String {
    let mut line: String = number;
    for space in 0..(spaces - &number.len()) - 1 {
        line.push_str(" ");
        line.push_str(line.chars().as_str());
    }
    println!("{}", line);
    line
}

fn create_spaces(first_nb: &String, second_nb: &String) -> usize {
    let len_first_nb: usize = first_nb.len();
    let len_second_nb: usize = second_nb.len();

    if len_first_nb > len_second_nb {
        len_first_nb + 2
    } else {
        len_second_nb + 2
    }
}
