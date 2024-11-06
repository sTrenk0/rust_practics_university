mod practic4;
mod practic5;
mod practic6;
mod practic7;
mod practic8;
mod practic9;
mod practic10;
mod practic11;
mod practic12;
mod practic13;
mod practic14;
mod practic15;
mod practic16;

fn main() {
    practic4::draw_diamond();
    practic5::draw_envelope();
    practic6::gcd(56, 98);
    practic7::draw_tree(6);
    practic8::swap_case("hello world!");
    practic9::is_prime(1);
    practic10::rotate("abcdefgh", 1);
    practic11::is_palindrome(11);
    practic12::process_vector(1);
    practic13::count_permutation(&vec![1]);
    practic13::gen_shipments(1);
    let occupied_area = practic14::area_occupied();
    assert_eq!(occupied_area, 60);
    println!("Зайнята площа: {}", occupied_area);
    practic16::solve_puzzle();
}
