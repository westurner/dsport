use mathrenderrs::{render, MathBackend, MathDisplay};
fn main() {
    println!("{}", render(MathBackend::MathJax, MathDisplay::Inline, "x^2"));
}
