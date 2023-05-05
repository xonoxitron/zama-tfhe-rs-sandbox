use tfhe::integer::gen_keys_radix;
use tfhe::shortint::parameters::PARAM_MESSAGE_2_CARRY_2;

pub fn run() {
    // We create keys to create 16 bits integers
    // using 8 blocks of 2 bits
    let (cks, sks) = gen_keys_radix(&PARAM_MESSAGE_2_CARRY_2, 8);

    let clear_a = 2382u16;
    let clear_b = 29374u16;

    let mut a = cks.encrypt(clear_a as u64);
    let mut b = cks.encrypt(clear_b as u64);

    let encrypted_max = sks.smart_max_parallelized(&mut a, &mut b);
    let decrypted_max: u64 = cks.decrypt(&encrypted_max);
    println!("[shortint_circuit] decrypted_max: {}", decrypted_max);
    assert_eq!(decrypted_max as u16, clear_a.max(clear_b))
}
