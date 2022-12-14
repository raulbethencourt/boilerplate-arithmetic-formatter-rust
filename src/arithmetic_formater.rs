pub fn arithmetic_formater(problems: Vec<&str>) -> String {
    let len_problems = problems.len();

    if len_problems > 4 {
        String::from("Error: Too many problems.")
    } else {
        let mut first_line: String = String::from("");
        let mut second_line: String = String::from("");
        let mut separator: String = String::from("");

        for problem in problems.iter() {
            let problem_array: Vec<&str> = problem.split_whitespace().collect();
            let first_nb: String = String::from(problem_array[0]);
            let second_nb: String = String::from(problem_array[2]);
            let operator: String = String::from(problem_array[1]);

            let spaces: usize = create_spaces(&first_nb, &second_nb);
            first_line = format!("{}    {}", &mut first_line, create_line(first_nb, spaces));
        }

        first_line
    }
}

fn create_line(mut number: String, spaces: usize) -> String {
    let mut line: String = number;
    let mut counter: usize = (spaces - line.len()) - 1 ;
    loop {
        let mut counter: usize = if counter > 0 {
            counter = counter - 1;
            counter
        } else {
            break;
        }; 
        line.push(' ');
        let mut extra: &str = &mut line.as_str();
        line.push_str(&mut extra);
        
    }
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
