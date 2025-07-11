use tokio::process::Command;
use tokio::io::{AsyncBufReadExt, BufReader};
use std::process::Stdio;
use std::env;
use std::fs;
use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exe_path = env::current_exe()?;
    let exe_dir = exe_path.parent().expect("Failed to get executable directory");
    
    let yggdrasil_binary = if cfg!(windows) {
        "yggdrasil.exe"
    } else {
        "yggdrasil"
    };
    
    let yggdrasil_path = exe_dir.join(yggdrasil_binary);
    let config_path = exe_dir.join("yggdrasil.conf");
    
    if !yggdrasil_path.exists() {
        eprintln!("ERROR: Yggdrasil binary not found at: {:?}", yggdrasil_path);
        return Ok(());
    }
    
    let mut cmd = Command::new(&yggdrasil_path);
    if !config_path.exists() {
        let output = Command::new(&yggdrasil_path)
            .arg("-genconf")
            .output()
            .await?;
        
        if !output.status.success() {
            eprintln!("ERROR: Config file could not be created");
            return Ok(());
        }
        
        let mut config_str = String::from_utf8(output.stdout)?;
        config_str = config_str.replace("Listen: []", "Listen: [\"tls://[::]:0\"]");
        //https://publicpeers.neilalexander.dev/ 
        //Add public peers here
        config_str = config_str.replace("Peers: []", "Peers: [\"tls://[2001:470:1f13:e56::64]:39575\"]");
        
        fs::write(&config_path, config_str)?;
        println!("Generated config saved to: {:?}", config_path);
       
    }
    cmd.arg("-useconffile").arg(&config_path);
    
    cmd.stdout(Stdio::piped());
    
    let mut child = cmd.spawn()
        .map_err(|e| format!("Failed to start yggdrasil: {} (Path: {:?})", e, yggdrasil_path))?;
    
    let stdout = child.stdout.take().expect("Failed to open stdout");
    let mut reader = BufReader::new(stdout).lines();
    
    // IPv6 regex to match valid IPv6 addresses
    let ipv6_regex = Regex::new(r"([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:)*::([0-9a-fA-F]{1,4}:)*[0-9a-fA-F]{1,4}|::1|::")?;
    let mut ipv6_found = false;
    
    while let Ok(Some(line)) = reader.next_line().await {
        if !ipv6_found {
            if let Some(caps) = ipv6_regex.captures(&line) {
                let ipv6 = caps.get(0).map(|m| m.as_str()).unwrap_or("");
                
                // Write IPv6 to file
                let out_path = exe_dir.join("yggdrasil_ipv6.txt");
                fs::write(&out_path, ipv6)?;
                
                println!("IPv6 address saved: {}", ipv6);
                ipv6_found = true;
                // Don't break - keep the process running
            }
        }
    }
    
    // Wait for the process to exit naturally
    child.wait().await.ok();
    
    Ok(())
}
