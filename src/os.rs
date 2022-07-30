use sysinfo::{SystemExt, CpuExt};

#[napi]
pub fn homedir() -> Option<String> {
    match dirs::home_dir() {
        Some(path) => Some(path.display().to_string()),
        None => None,
    }
}

#[napi]
pub fn cachedir() -> Option<String> {
    match dirs::cache_dir() {
        Some(path) => Some(path.display().to_string()),
        None => None,
    }
}

#[napi]
pub fn tempdir() -> String {
    std::env::temp_dir().display().to_string()
}

#[napi]
pub fn hostname() -> Option<String> {
    match hostname::get() {
        Ok(name) => Some(name.to_str().unwrap().to_string()),
        Err(..) => None,
    }
}

#[napi]
pub fn platform() -> String {
    std::env::consts::OS.to_string()
}

#[napi]
pub fn arch() -> String {
    std::env::consts::ARCH.to_string()
}

#[napi]
pub fn release() -> Option<String> {
    match sys_info::os_release() {
        Ok(name) => Some(name),
        Err(..) => None,
    }
}

#[napi]
pub fn uptime() -> Option<i64> {
    match sys_info::boottime() {
        Ok(time) => Some(time.tv_sec),
        Err(..) => None
    }
}

#[napi(object)]
pub struct CpuInfo {
    pub model: String,
    pub speed: i64,
    pub usage: f64,
    pub ventor_id: String,
}

#[napi]
pub fn cpus() -> Vec<CpuInfo> {
    let mut all_cpus = Vec::new();
    let sys = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::new().with_cpu(sysinfo::CpuRefreshKind::everything()),
    );
    
    for cpu in sys.cpus() {
        all_cpus.push(CpuInfo {
            model: cpu.brand().to_string(),
            speed: cpu.frequency() as i64,
            usage: f64::from(cpu.cpu_usage()),
            ventor_id: cpu.vendor_id().to_string()
        })
    }

    all_cpus
}

// Memory functions implementation

#[napi]
pub fn total_memory() -> i64 {
    let sys = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::new().with_memory(),
    );

    sys.total_memory() as i64
}

#[napi]
pub fn used_memory() -> i64 {
    let sys = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::new().with_memory(),
    );

    sys.used_memory() as i64
}

#[napi]
pub fn available_memory() -> i64 {
    let sys = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::new().with_memory(),
    );

    sys.available_memory() as i64
}

#[napi]
pub fn free_memory() -> i64 {
    let sys = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::new().with_memory(),
    );

    sys.free_memory() as i64
}

#[napi]
pub fn total_swap() -> i64 {
    let sys = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::new().with_memory(),
    );

    sys.total_swap() as i64
}

#[napi]
pub fn used_swap() -> i64 {
    let sys = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::new().with_memory(),
    );

    sys.used_swap() as i64
}

// in progress, waiting for https://github.com/EstebanBorai/network-interface/issues/13
/*#[napi(object)]
pub struct NetworkInfo {
    pub name: String,
    pub address: String,
    /*pub interface_name: String,
    pub received: BigInt,
    pub total_received: BigInt,
    pub transmitted: BigInt,
    pub total_transmitted: BigInt,
    pub packets_received: BigInt,
    pub total_packets_received: BigInt,
    pub packets_transmitted: BigInt,
    pub total_packets_transmitted: BigInt,
    pub errors_on_received: BigInt,
    pub total_errors_on_received: BigInt,
    pub errors_on_transmitted: BigInt,
    pub total_errors_on_transmitted: BigInt,*/
}

#[napi]
pub fn network_interfaces() -> Vec<NetworkInfo> {
    let mut all_network_interfaces = Vec::new();


    /*let mut all_network_interfaces = Vec::new();
    let sys = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::new().with_networks_list(),
    );

    for (name, data) in sys.networks() {
        all_network_interfaces.push(NetworkInfo {
            interface_name: name.to_string(),
            received: BigInt::from(data.received()),
            total_received: BigInt::from(data.total_received()),
            transmitted: BigInt::from(data.transmitted()),
            total_transmitted: BigInt::from(data.total_transmitted()),
            packets_received: BigInt::from(data.packets_received()),
            total_packets_received: BigInt::from(data.total_packets_received()),
            packets_transmitted: BigInt::from(data.packets_transmitted()),
            total_packets_transmitted: BigInt::from(data.total_packets_transmitted()),
            errors_on_received: BigInt::from(data.errors_on_received()),
            total_errors_on_received: BigInt::from(data.total_errors_on_received()),
            errors_on_transmitted: BigInt::from(data.errors_on_transmitted()),
            total_errors_on_transmitted: BigInt::from(data.total_errors_on_transmitted()),
        })
    }

    all_network_interfaces*/
}*/