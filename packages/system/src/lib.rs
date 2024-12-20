use sysinfo::{Components, Disks, Networks, System};

#[derive(Debug)]
pub struct SysInfo {
    system: System,
    pub system_name: String,
    pub system_version: String,
    pub kernel: String,
    pub hostname: String,
}

impl SysInfo {
    pub fn new() -> SysInfo {
        let mut info = SysInfo {
            system: System::new_all(),
            system_name: String::new(),
            system_version: String::new(),
            kernel: String::new(),
            hostname: String::new(),
        };
        info.refresh();

        info
    }

    pub fn refresh(&mut self) {
        self.system.refresh_all();

        self.system_name = System::name().unwrap_or("Unknown".to_string());
        self.system_version = System::long_os_version().unwrap_or("Unknown".to_string());
        self.kernel = System::kernel_version().unwrap_or("Unknown".to_string());
        self.hostname = System::host_name().unwrap_or("Unknown".to_string());
    }

    pub fn print_processes(&self) {
        for (pid, process) in self.system.processes() {
            println!("[{pid}] {:?} {:?}", process.name(), process.disk_usage());
        }
    }

    pub fn print_cpus(&self) {
        println!("CPUs: {}", self.system.cpus().len());
    }

    pub fn print_memory(&self) {
        let sys = &self.system;
        println!("=> memory:");
        println!("total memory: {} bytes", sys.total_memory());
        println!("used memory : {} bytes", sys.used_memory());
        println!("total swap  : {} bytes", sys.total_swap());
        println!("used swap   : {} bytes", sys.used_swap());
    }

    pub fn print_components() {
        let components = Components::new_with_refreshed_list();
        println!("=> components:");
        for component in &components {
            println!("{component:?}");
        }
    }

    pub fn print_disks() {
        println!("=> disks:");
        let disks = Disks::new_with_refreshed_list();
        for disk in &disks {
            println!("{disk:?}");
        }
    }

    pub fn print_network() {
        let networks = Networks::new_with_refreshed_list();
        println!("=> networks:");
        for (interface_name, data) in &networks {
            println!(
                "{interface_name}: {} B (down) / {} B (up)",
                data.total_received(),
                data.total_transmitted(),
            );
            // If you want the amount of data received/transmitted since last call
            // to `Networks::refresh`, use `received`/`transmitted`.
        }
    }
}
