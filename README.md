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