use std::collections::HashMap;
use std::ffi::OsString;

use ai_sandbox::{
    FileSystemSandboxPolicy, NetworkSandboxPolicy, SandboxCommand, SandboxManager, SandboxPolicy,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = SandboxCommand {
        program: OsString::from("/usr/bin/true"),
        args: Vec::new(),
        cwd: std::env::current_dir()?,
        env: HashMap::new(),
    };
    let policy = SandboxPolicy::ReadOnly {
        file_system: FileSystemSandboxPolicy::ReadOnly,
        network_access: NetworkSandboxPolicy::NoAccess,
    };
    let request = SandboxManager::new().create_exec_request(command, policy)?;

    println!(
        "Prepared a {:?} sandbox request with a read-only filesystem and no network access.",
        request.sandbox
    );
    Ok(())
}
