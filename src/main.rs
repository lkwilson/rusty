mod sub_mod;

fn main() {
  let exit_code = sub_mod::main();

  println!("Module exited with code {}", exit_code);
  std::process::exit(exit_code.into());
}
