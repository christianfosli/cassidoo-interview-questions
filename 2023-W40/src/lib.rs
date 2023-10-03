pub fn add_operators(source: isize, target: isize) -> Vec<String> {
    let mut results = Vec::new();
    let mut expression = String::new();
    add_operators_helper(source, target, &mut expression, 0, 0, 0, &mut results);
    results
}

fn add_operators_helper(
    source: isize,
    target: isize,
    expression: &mut String,
    result: isize,
    last_operand: isize,
    current_index: usize,
    results: &mut Vec<String>,
) {
    let source_str = source.to_string();

    if current_index == source_str.len() {
        if result == target {
            results.push(expression.clone());
        }
        return;
    }

    for i in current_index..source_str.len() {
        let num: isize = source_str[current_index..=i].parse().unwrap();
        if current_index == 0 {
            add_operators_helper(
                source,
                target,
                &mut format!("{num}"),
                num,
                num,
                i + 1,
                results,
            );
        } else {
            // Add operator '+'
            add_operators_helper(
                source,
                target,
                &mut format!("{expression}+{num}"),
                result + num,
                num,
                i + 1,
                results,
            );
            // Add operator '-'
            add_operators_helper(
                source,
                target,
                &mut format!("{expression}-{num}"),
                result - num,
                -num,
                i + 1,
                results,
            );
            // Add operator '*'
            add_operators_helper(
                source,
                target,
                &mut format!("{expression}*{num}"),
                result - last_operand + last_operand * num,
                last_operand * num,
                i + 1,
                results,
            );
            // Add operator '/'
            if num != 0 {
                add_operators_helper(
                    source,
                    target,
                    &mut format!("{expression}/{num}"),
                    result - last_operand + last_operand / num,
                    last_operand / num,
                    i + 1,
                    results,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![String::from("1+2+3"), String::from("1*2*3")],
            add_operators(123, 6)
        );

        assert_eq!(Vec::<&'static _>::new(), add_operators(3456237490, 9191)); // Nope
    }
}
