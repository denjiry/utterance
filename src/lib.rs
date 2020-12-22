use std::marker::PhantomData;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

fn m() {
    let first = PhantomData;
}
