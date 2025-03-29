const TITLE_INDENT: &str = "\t";
const EXERCISE_INDENT: &str = "\t\t";
const EXERCISE_INDENT_W_NEWLINE: &str = "\n\t\t";

pub fn print_title(title: &str) {
  println!("\n{}{title}", TITLE_INDENT);
}

pub fn print_w_exercise_indent(output: String) {
  println!("{}{}", EXERCISE_INDENT, output.replace('\n', EXERCISE_INDENT_W_NEWLINE));
}