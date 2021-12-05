// mod heaps;
// mod sub_mod;
// mod sub_mod_file;
mod play;

fn main() {
  // let exit_code = heaps::heap::main();
  let exit_code = play::const_arg::main();
  println!("Module exited with code {}", exit_code);
  std::process::exit(exit_code.into());
}
