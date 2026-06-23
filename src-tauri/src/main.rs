mod windows_support;

fn main() {
    println!("{}", windows_support::local_url("popover.html"));
}
