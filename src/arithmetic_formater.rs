pub fn arithmetic_formater(problems: Vec<&str>) -> &str {
    let len_problems = problems.len();
    println!("{:?}", problems);

    if len_problems > 4 {
        "Error: Too many problems."
    } else {
        for problem in problems {
            let problem_array: Vec<&str> = problem.split_whitespace().collect();
            let first_nb: &str = problem_array[0];
            let second_nb: &str = problem_array[2];
            let operator: &str = problem_array[1];
            let len_first = first_nb.len();
            let len_second = second_nb.len();

            if operator != "+" || operator != "-" {
                return "Error: Operator must be '+' or '-'.";
            } else if !first_nb.chars().next().unwrap().is_numeric()
                || !second_nb.chars().next().unwrap().is_numeric()
            {
                return "Error: Numbers must only contain digits.";
            } else if len_first > 4 || len_second > 4 {
                return "Error: Numbers cannot be more than four digits.";
            } else {
                return "cool";
            }
        }
        return "cool"
    }
}
