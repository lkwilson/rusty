mod sub_mod;

fn run() -> u8 {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    println!("Usage: {} <sub_module>", args[0]);
    return 1;
  }
  // TODO: Fix redundancy
  let available_modules = ["sub_mod"];
  match args[1].as_str() {
    "sub_mod" => sub_mod::main(),
    _ => {
      println!("Unknown module: {}", args[1]);
      println!("Available modules: {:?}", available_modules);
      1
    }
  }
}

fn main() {
  let exit_code = run();
  println!("Module exited with code {}", exit_code);
  std::process::exit(exit_code.into());
}
