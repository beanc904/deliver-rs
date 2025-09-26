**DELIVER**

This is a mini p2p file transfer application written in Rust.

> [!Important]
>
> It is driven by `tokio tcp`. Also, the receiver is acted as a server binded in default port `9000` to the sender.
>
> So, you should add the tcp firewall rule in the receiver machine.

# Install

```bash
git clone https://github.com/beanc904/deliver-rs.git --depth=1

cd deliver-rs
# install for all user(require sudo)
make install
# install for current user
make install4user
```

> [!CAUTION]
>
> Remember to add `/home/$USER/.local/bin` to `PATH`.

> or if you just want to "touch" the wild baby - `firefly`, you can follow procedures below...
>
> ```bash
> git clone https://github.com/beanc904/deliver-rs.git
> # and then checkout to the `init commit`
> # you can find her in `docs/`. (^ v ^)
> ```

# Usage

The mini program is very easy to use. Just like...

- server
    ```bash
    ❯ receiver --help
    This is a mini p2p file transfer application written in Rust.

    Usage: receiver [OPTIONS]

    Options:
    -p, --port <PORT>  The port to listen on [default: 9000]
    -h, --help         Print help
    -V, --version      Print version
    ```

- client
    ```bash
    ❯ sender --help
    This is a mini p2p file transfer application written in Rust.

    Usage: sender [OPTIONS] --file <FILE>

    Options:
    -f, --file <FILE>  The file(include file and directory) to send
    -i, --ip <IP>      The server IP address
    -p, --port <PORT>  The server port [default: 9000]
    -h, --help         Print help
    -V, --version      Print version
    ```

## Advanced

Consider making it as a yazi plugin.

```toml
# ~/.config/yazi/yazi.toml
[opener]
sender = [
  { run = 'sender -f "$0"', block = true, for = "unix", desc = "P2p Sender..." },
  { run = 'sender -f "%0"', block = true, for = "windows", desc = "P2p Sender..." },
]
receiver = [
  { run = 'receiver', block = true, for = "unix", desc = "P2p Receiver..." },
  { run = 'receiver', block = true, for = "windows", desc = "P2p Receiver..." },
]
# your self config...
# ...

[open]
rules = [
	# Folder
	{ name = "*/", use = [ "edit", "open", "reveal", "sender", "receiver" ] },
	# Text
	{ mime = "text/*", use = [ "edit", "reveal" ] },
	# Image
	{ mime = "image/*", use = [ "open", "reveal" ] },
	# Media
	{ mime = "{audio,video}/*", use = [ "play", "reveal" ] },
	# Archive
	{ mime = "application/{zip,rar,7z*,tar,gzip,xz,zstd,bzip*,lzma,compress,archive,cpio,arj,xar,ms-cab*}", use = [ "extract", "reveal" ] },
	# JSON
	{ mime = "application/{json,ndjson}", use = [ "edit", "reveal" ] },
	{ mime = "*/javascript", use = [ "edit", "reveal" ] },
	# Empty file
	{ mime = "inode/empty", use = [ "edit", "reveal" ] },
	# Fallback
	{ name = "*", use = [ "open", "reveal", "sender", "receiver" ] },
]
# prepend_rules = [
#   # Folder
#   { name = "*/", use = [ "edit", "open", "reveal", "sender", "receiver" ] },
#   # Fallback
#   { name = "*", use = [ "open", "reveal", "sender", "receiver" ] },
#   # your self config...
# ]
# your self config...
# ...
```

> [!Important]
> In `[open]`, you'd better choose the first writing.
> I'm a bit ambigous to the `yazi-default.toml` stored in the [offical repository](https://github.com/sxyazi/yazi/blob/main/yazi-config/preset/yazi-default.toml).

### Effect

- **Sender(client)**:
  ![sender1](/docs/sender1.png)
  ![sender2](/docs/sender2.png)

- **Receiver(server)**:
  ![receiver1](/docs/receiver1.png)
  ![receiver2](/docs/receiver2.png)

# Uninstall

```bash
make uninstall

make uninstall4user
```

## Purge cache and config

- `cache`: 
  - `unix`: `~/.cache/deliver/`
  - `windows`: `~/AppData/Local/deliver/`

- `config`

# Others

I consider writing the auto firewall add feat. Maybe..

# (* ^ *)

I placed a wild firefly here so that passersby can touch her.

But the size of her is too large. So i remove the little baby.