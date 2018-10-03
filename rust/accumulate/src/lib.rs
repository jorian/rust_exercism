/// What should the type of _function be?
pub fn map<F, T, U>(mut input: Vec<T>, mut _function: F) -> Vec<U>
    where F: FnMut(T) -> U
{
    let mut output = Vec::new();
    let len = input.len();

    for _i in 0..len {
        output.push(_function(input.pop().unwrap()));
    }

    output
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
}

