# yggdrasil_wrapper

A quick and messy attempt to start Yggdrasil from Godot using a Rust wrapper.

⚠️ This is experimental, so treat it with care.

---

## 🚀 What This Does

This wrapper:

- Automatically generates a `yggdrasil.conf` if missing
- Starts Yggdrasil with admin privileges (on Windows)
- Extracts your IPv6 address from Yggdrasil's logs
- Writes it to `yggdrasil_ipv6.txt` for use in Godot

---

## 🛠️ Prerequisites

1. [Yggdrasil executable](https://yggdrasil-network.github.io/)  
   - Note: Official Windows releases only include `.msi` installers, not standalone `.exe`s.
   - But you can [build it yourself](https://github.com/yggdrasil-network/yggdrasil-go).

2. [Wintun](https://www.wintun.net/) driver (required by Yggdrasil for Windows networking)

---

## 📦 How to Use

1. Clone this repo.
2. Edit `main.rs` and **update this line** with working Yggdrasil peers in your area:
   ```rust
   config_str = config_str.replace("Peers: []", "Peers: [\"tls://your-peer\"]");
3. (Optional) Rename the Yggdrasil binary if needed, or make sure it's named yggdrasil.exe.

4. Build the project:
    "cargo build --release"
5. Place the compiled wrapper (target/release/yggdrasil_wrapper.exe), yggdrasil.exe, and wintun.dll in the same folder.

6. Run it from command line or double-click it.

A file called yggdrasil_ipv6.txt will be created in the same folder. Your Godot project can read this file to get the current IPv6 address.

---

## 👾 Godot Integration (Workaround)
Due to permission elevation and networking constraints, Godot cannot directly launch this wrapper on its own. To work around this, see the extender branch:

- The extender (branch) is a small native helper that Godot can launch (you will need to build that project too).
- It starts this wrapper with admin permissions.

---
## 🧪 Roadmap / Ideas
Open a TCP socket so Godot can communicate directly with the wrapper
Add better error messages and configuration feedback

## Godot Example
extends Node

var pid := 0
var ipv6_addr:String="";
var dirpath


func _notification(what: int) -> void:
	if what == NOTIFICATION_WM_CLOSE_REQUEST:
		if pid:
			OS.kill(pid)

func _ready() -> void:
	#Use absolute path in editor for testing
	dirpath=OS.get_executable_path().get_base_dir()
   var wrapper_path = dirpath + "/mp_extender.exe"
	var pid_output = []
	var code = OS.execute(wrapper_path, [], pid_output)
	if code == 0:
		pid = int(pid_output[0])
	wait_and_read_log()


func _exit_tree():
	if pid:
		OS.kill(pid)


func wait_and_read_log():
	await get_tree().create_timer(10.0).timeout  # wait a few seconds
	var log_file = dirpath + "/yggdrasil_ipv6.txt"
	var f = FileAccess.open(log_file, FileAccess.READ)
	if f:
		ipv6_addr = f.get_as_text()
		print(ipv6_addr)
