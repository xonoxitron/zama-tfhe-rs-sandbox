use tfhe::shortint::prelude::*;

pub fn run() {
    // Generate a set of client/server keys
    // with 2 bits of message and 2 bits of carry
    let (client_key, server_key) = gen_keys(PARAM_MESSAGE_2_CARRY_2);

    let msg1 = 3;
    let msg2 = 2;

    // Encrypt two messages using the (private) client key:
    let ct_1 = client_key.encrypt(msg1);
    let ct_2 = client_key.encrypt(msg2);

    // Homomorphically compute an addition
    let ct_add = server_key.unchecked_add(&ct_1, &ct_2);

    // Define the Hamming weight function
    // f: x -> sum of the bits of x
    let f = |x: u64| x.count_ones() as u64;

    // Generate the accumulator for the function
    let acc = server_key.generate_accumulator(f);

    // Compute the function over the ciphertext using the PBS
    let ct_res = server_key.apply_lookup_table(&ct_add, &acc);

    // Decrypt the ciphertext using the (private) client key
    let output = client_key.decrypt(&ct_res);
    println!("[shortint_circuit] output: {}", output);
    assert_eq!(output, f(msg1 + msg2));
}
