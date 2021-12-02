mod sub_mod;

fn run() -> u8 {
  let args: Vec<String> = std::env::args().collect();
  let selected;
  if args.len() != 2 {
    selected = "";
  } else {
    selected = args[1].as_str();
  }
  // TODO: Fix redundancy
  let available_modules = [
    "sub_mod",
  ];
  match selected {
    "sub_mod" => sub_mod::main(),
    "" | _=> {
      println!("Usage: {} <sub_module>", args[0]);
      if selected.len() != 0 {
        println!("Unknown module: {}", selected);
      } else {
        println!("Available modules: {:?}", available_modules);
      }
      1
    }
  }
}

fn main() {
  let exit_code = run();
  println!("Module exited with code {}", exit_code);
  std::process::exit(exit_code.into());
}
