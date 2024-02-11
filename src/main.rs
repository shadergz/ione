use std::env;
mod crypto;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return
    }
    let mut sys_keys = crypto::chan::KeysDb::new();
    sys_keys.populate_chain(&args);

    dbg!(args);
}
