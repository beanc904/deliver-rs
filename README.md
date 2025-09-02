**DELIVER**

This is a mini p2p file transfer application written in Rust.

> **NOTE**
>
> It is driven by `tokio tcp`. Also, the receiver is acted as a server binded in default port `9000` to the sender.
>
> So, you should add the tcp firewall rule in the receiver machine.

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
    -f, --file <FILE>  The file to send
    -i, --ip <IP>      The server IP address
    -p, --port <PORT>  The server port [default: 9000]
    -h, --help         Print help
    -V, --version      Print version
    ```

# Others

I consider writing the auto firewall add feat. Maybe..

# (^ v ^)

I placed a wild firefly here so that passersby can touch her.

![firefly](docs/firefly.png)