fn main() {

    let (x, y);

    let birth_years: [u16; 5] = [1987, 1990, 2000, 1975, 1996];

    [x,.., y] = birth_years;

    assert_eq!(x, 1987);

    assert_eq!(y, 1996);

    assert_eq!(x, y, " Test: {} and {}", x, y);

}

/*
thread 'main' panicked at src/main.rs:13:5:
assertion `left == right` failed:  Test: 1987 and 1996
  left: 1987
 right: 1996

*/
