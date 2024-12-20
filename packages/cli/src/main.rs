use system;

fn main() {
    let mut info = system::SysInfo::new();
    info.refresh();
    dbg!(info);
}
