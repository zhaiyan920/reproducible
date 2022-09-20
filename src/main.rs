mod bpf;

fn main() {
    let builder = bpf::test::TestSkelBuilder::default();
    builder.open().unwrap().load().unwrap();

    println!("Hello, world!");
}
