pub fn raindrops(n: u32) -> String {
    let mut sound_vec: Vec<String> = Vec::new();
    let mut sound: String = n.to_string();
    
    if is_pling(n) { sound_vec.push(String::from("Pling")) }
    if is_plang(n) { sound_vec.push(String::from("Plang")) }
    if is_plong(n) { sound_vec.push(String::from("Plong")) }

    if sound_vec.len() > 0 {
        sound = sound_vec.into_iter().collect();
    }
    sound
}

fn is_pling(n: u32) -> bool {
   n % 3 == 0 
}

fn is_plang(n: u32) -> bool {
    n % 5 == 0
}

fn is_plong(n: u32) -> bool {
    n % 7 == 0
}
