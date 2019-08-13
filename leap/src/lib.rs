pub fn is_leap_year(year: u64) -> bool {
    if dividable_by(year, 4) {
        if dividable_by(year, 100) {
            if dividable_by(year, 400){
                true
            } else {
                false
            }
        } else {
        true
        }
    } else {
        false
    }
}

fn dividable_by(number: u64, by: u64) -> bool {
    number % by == 0
}
