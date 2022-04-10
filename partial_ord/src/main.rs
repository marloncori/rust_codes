use partial_ord::num_analysis;

fn main() {
    let v1: i16 = 23;
    let v2: i16 = -14;
    let min = match num_analysis::find_min(v1, v2) {
        Some(v) => v,
         None  => 0,
    };
    println!("\n\t Min value = {}", min);

    let v3: i16 = 47;
    let v4: i16 = -89;
    let max = match num_analysis::find_max(v3, v4) {
         Some(v) => v,
         None   => 0,
    };
    println!("\n\t Min value = {}", max);

}
