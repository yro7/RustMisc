use crate::Integer;

pub(crate) fn recursive(n : Integer) -> i32 {

    // types non-implementing the copy trait can only be consumed once,
    // so this function will not compile if using twice the n : Integer object
    if n.value == 0 { return 0; }
    if n.value == 1 { return 1; }


    recursive(Integer { value:n.value-1}) + recursive(Integer { value:n.value-1})
}

pub(crate) fn smart(n : i64) -> i64 {

    let mut a: i64 = 0;
    let mut b: i64 = 1;
    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }

    a
}


