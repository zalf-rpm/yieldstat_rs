extern crate rusqlite;
extern crate time;
extern crate yieldstat_rs;

fn main() {
    let f = yieldstat_rs::yieldstat::pre_crop_factor(1017, 1300);
    println!("factor is {}", f);
}
