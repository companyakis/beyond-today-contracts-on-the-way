fn main() {

    let mut year: u16 = 2003;

    while year > 1990 {

        year -= 1;

        if year == 1997 {

            continue;
        }

        if year == 1995 {

            break;
        }

        println!("Year is {year}");
    }

    println!("Year final value: {year}")
}

/*
Year is 2002
Year is 2001
Year is 2000
Year is 1999
Year is 1998
Year is 1996
Year final value: 1995
*/
