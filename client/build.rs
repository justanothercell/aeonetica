use rerun_except::rerun_except;

fn main() {
	rerun_except(&["runtime/**", "target/**"]).unwrap();
}