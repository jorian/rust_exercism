/// What should the type of _function be?
pub fn map<F, T>(mut input: Vec<T>, mut _function: F) -> Vec<T>
    where F: FnMut(T) -> T {

    let mut output = Vec::new();

    for i in 0..input.len() {
        output.push(
            _function(input.remove(i))
        );
    }

    output
}

