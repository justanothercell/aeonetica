use rerun_except::rerun_except;

fn main() {
	rerun_except(&["mods/**", "runtime/**", "__pycache__/**", "target/**"]).unwrap();
}