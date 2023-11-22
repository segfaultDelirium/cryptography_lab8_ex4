use md4::{Md4, Digest};

fn hash_vector_to_string(hash_vector: Vec<u8>) -> String{
    hash_vector.into_iter()
        .map(|value| format!("{:x}", value))
        .reduce(|acc, x| {
            format!("{acc}{x}")
        }).unwrap()
}

fn convert_hex_string_to_values_vec(hex_string: String) -> Vec<u8>{
    let mut res_vec = vec![];

    let limit = hex_string.len() / 2;
    let mut chars = hex_string.chars();
    for i in 0..limit{
        let char1 = chars.next().unwrap();
        let char2 = chars.next().unwrap();
        let value = i64::from_str_radix(&format!("{char1}{char2}"), 16).unwrap();
        res_vec.push(value as u8);
    }

    res_vec
}

fn vec_to_hash_string(vec: Vec<u8>) -> String {
    let mut hasher = Md4::new();
    hasher.update(vec);
    let result = hasher.finalize();
    let result_string = hash_vector_to_string(result.to_vec());
    result_string
}


fn main() {

    let m1 = "839c7a4d7a92cb5678a5d5b9eea5a7573c8a74deb366c3dc20a083b69f5d2a3bb3719dc69891e9f95e809fd7e8b23ba6318edd45e51fe39708bf9427e9c3e8b9";
    let m1_as_vec: Vec<u8> = convert_hex_string_to_values_vec(m1.to_string());
    // println!("m1_as_vec: {:?}", m1_as_vec);

    let m2 = "839c7a4d7a92cbd678a5d529eea5a7573c8a74deb366c3dc20a083b69f5d2a3bb3719dc69891e9f95e809fd7e8b23ba6318edc45e51fe39708bf9427e9c3e8b9";
    let m2_as_vec: Vec<u8> = convert_hex_string_to_values_vec(m2.to_string());
    // println!("m2_as_vec: {:?}", m2_as_vec);

    println!("a)");
    let m1_hash_string = vec_to_hash_string(m1_as_vec);
    println!("m1 hash: {m1_hash_string}");

    let m2_hash_string = vec_to_hash_string(m2_as_vec);
    println!("m2 hash: {m2_hash_string}");
    if m1_hash_string == m2_hash_string {
        println!("kolizja");
    }else{
        println!("brak kolizji")
    }
    println!();

    b();
    c();

    println!("bye");
}

fn b(){
    let m1 = "a6af943ce36f0cf4adcb12bef7f0dc1f526dd914bd3da3cafde14467ab129e640b4c41819915cb43db752155ae4b895fc71b9b0d384d06ef3118bbc643ae6384";
    let m1_as_vec: Vec<u8> = convert_hex_string_to_values_vec(m1.to_string());
    // println!("m1_as_vec: {:?}", m1_as_vec);

    let m2 = "a6af943ce36f0c74adcb122ef7f0dc1f526dd914bd3da3cafde14467ab129e640b4c41819915cb43db752155ae4b895fc71b9a0d384d06ef3118bbc643ae6384";
    let m2_as_vec: Vec<u8> = convert_hex_string_to_values_vec(m2.to_string());
    // println!("m2_as_vec: {:?}", m2_as_vec);

    println!("b)");
    let m1_hash_string = vec_to_hash_string(m1_as_vec);
    println!("m1 hash: {m1_hash_string}");

    let m2_hash_string = vec_to_hash_string(m2_as_vec);
    println!("m2 hash: {m2_hash_string}");
    if m1_hash_string == m2_hash_string {
        println!("kolizja");
    }else{
        println!("brak kolizji")
    }
    println!();
}

fn c(){
    let m1 = "76931fac9dab2b36c248b87d6ae33f9a62d7183a5d5789e4b2d6b441e2411dc709e111c7e1e7acb6f8cac0bb2fc4c8bc2ae3baaab9165cc458e199cb89f51b13";
    let m1_as_vec: Vec<u8> = convert_hex_string_to_values_vec(m1.to_string());
    // println!("m1_as_vec: {:?}", m1_as_vec);

    let m2 = "76931fac9dab2b36d248b87d6af33f9a62d7183a5d5789e4b2d6b441e2411dc709e111c7e1e7acb6f8cac0bb2fc4c8bc2ae3baaab9265cc458e199cb89f51b13";
    let m2_as_vec: Vec<u8> = convert_hex_string_to_values_vec(m2.to_string());
    // println!("m2_as_vec: {:?}", m2_as_vec);

    println!("c)");
    let m1_hash_string = vec_to_hash_string(m1_as_vec);
    println!("m1 hash: {m1_hash_string}");

    let m2_hash_string = vec_to_hash_string(m2_as_vec);
    println!("m2 hash: {m2_hash_string}");
    if m1_hash_string == m2_hash_string {
        println!("kolizja");
    }else{
        println!("brak kolizji")
    }
    println!();
}