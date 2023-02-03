#[derive(argh::FromArgs)]
/// Send ctrl event to process
struct Args {
  /// target pid
  #[argh(positional)]
  pub pid: u32,
}

fn main() {
  let args: Args = argh::from_env();
  send(args.pid);
}

#[allow(unused_variables)]
fn send(process_id: u32) {
  #[cfg(target_os = "windows")]
  {
    use dll_syringe::{process::OwnedProcess, Syringe};

    let target_process = OwnedProcess::from_pid(process_id).unwrap();

    let syringe = Syringe::for_process(target_process);

    let mut dll_path = std::env::current_exe().unwrap();
    dll_path.set_file_name("send_ctrl_event.dll");

    syringe.inject(dll_path.as_path()).unwrap();
  }
}
