pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


// cargo new --lib "30 - Creating a library with cargo" --name creating_a_library_with_cargo --vcs none

cargo new --lib "31 - Documenting your code" --name documenting_your_code --vcs none