# rustdoor
A simple backdoor made in rust

-----------

## Legal Disclaimer
I am **NOT** responsible for any damage or misuse cause by this program, you are responsible for your actions, deploying this tool without permission **IS** illegal. Think before you type.

-----------

## What's this?
This is a fun project I am working on in order to learn rust. To put it simply, it is a backdoor that you connect to, with netcat for example, and it allows you to execute commands on the remote system. It currently supports:
- Windows
- Linux

## How to use
0. Make sure you have rust installed, check the link for instructions on how to install
```bash
https://www.rust-lang.org/tools/install
```

1. Get a copy of this repo locally
```bash
git clone https://github.com/threadexio/rustdoor.git && cd rustdoor
```

Also, install all the target platforms from the `Makefile` with:
```bash
make install
```

2. Compile the source
```bash
make release
```
or
```bash
make debug
```
After that you should be able to find the binaries in the `target` directory

3. How to run
```
Usage: ./rustdoor [IP:PORT] [PASSWORD]

[IP:PORT]	|	Local IP and port to listen on
[PASSWORD]	|	A simple password used for basic authentication

Examples:
	./rustdoor 0.0.0.0:8000 P@55W0rd1
	.\rustdoor.exe 10.1.33.7:53 1337
```
-----------

## Intended use case
CTFs? I guess, I don't really have a use for this

-----------

## TODO:
- [ ] Encryption
- [ ] Port forwarding
- [ ] Proxy
