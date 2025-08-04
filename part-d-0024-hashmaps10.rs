use std::collections::HashMap;

fn main() {

    let emp_ayhan = set_salaries(104, 3200);

    let _emp_bengu = set_salaries(17, 2300);

    println!("{:?}", emp_ayhan);

    println!("{:?}", emp_ayhan.get(&104).unwrap())


}

fn set_salaries(emp_id: u8, emp_salary_usd: u16) -> HashMap<u8, u16> {

    let mut emp_sal = HashMap::new();
    emp_sal.insert(emp_id, emp_salary_usd);
    emp_sal
}

// {104: 3200}
// 3200



