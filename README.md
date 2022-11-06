#SiRuS
ver. 1.0
A simple, yet sample, **TLS webshell in Rust**. The aim of this project is to learn Rust doing something fun, so it has to be intended as an experiment, of course feel free to fork it to improve it, also suggestions to improve the code are really appreciated.
<hr>
##How does it work
Just lunch the compiled executable **sirus** passing the certificate (PKCS#12 format) password as parameter: **sirus Zinz#72**. The certificate is embedded as binary resource, of course you can change it to include your own certificate, that must be placed in the **src** folder, at the same level of the source file main.rs, then change the string literal ("zinz.pfx") around line 80.
The server listens to the port **8443** on all IPV4 available interfaces, you can change this value around line 17 of main.rs. 
Once the server is running you can issue command passing the command name in a canonical URL format:
**https://\<IP>:8443/\<cmd>** e.g. https://127.0.0.1:8443/systeminfo. The parameter is decoded and executed against the server OS and the output, eventually the error, is returned to the web page into the TLS stream.
<hr>
##Compiling the software
The software has been compiled on
-  Microsoft Windows 11 Education
10.0.22621 N/D build 22621 (X64)
```bash
cargo b
   Compiling lazy_static v1.4.0
   Compiling urlencoding v2.1.2
   Compiling windows_x86_64_msvc v0.36.1
   Compiling windows-sys v0.36.1
   Compiling schannel v0.1.20
   Compiling native-tls v0.2.10
   Compiling sirus v0.1.0 (C:\Users\*******\source\rust\sirus)
    Finished dev [unoptimized + debuginfo] target(s) in 12.11s
```
- Ubuntu 22.04.1 LTS (Jammy Jellyfish) X64
```bash
cargo b
   Compiling pkg-config v0.3.25
   Compiling autocfg v1.1.0
   Compiling cc v1.0.73
   Compiling proc-macro2 v1.0.47
   Compiling unicode-ident v1.0.5
   Compiling quote v1.0.21
   Compiling libc v0.2.136
   Compiling syn v1.0.103
   Compiling openssl v0.10.42
   Compiling foreign-types-shared v0.1.1
   Compiling cfg-if v1.0.0
   Compiling log v0.4.17
   Compiling native-tls v0.2.10
   Compiling bitflags v1.3.2
   Compiling once_cell v1.15.0
   Compiling openssl-probe v0.1.5
   Compiling urlencoding v2.1.2
   Compiling openssl-sys v0.9.77
   Compiling foreign-types v0.3.2
   Compiling openssl-macros v0.1.0
   Compiling sirus v0.1.0 
   (/home/***/rust_projects/sirus)
    Finished dev [unoptimized + debuginfo] target(s) in 48.47s
```

The TLS connection is manged with the [crate native-tls](https://crates.io/crates/native-tls), that uses **SChannel on Windows** (via the schannel crate) and **OpenSSL** (via the openssl crate) *nix.
If you want to run the program with cargo use the following syntax to pass the password certificate parameter:
```bash
cargo r -- Zinz#72
```
###Dependencies
- urlencoding
- native-tls

###Notes
I used **Openssl 3.0.2** to create the pfx file, since I had issues having previously created the certificates with the 1.1 version, once I run the software on Ubuntu, due to the obsolete crypto security stack implemented.  
On Ubuntu you must have installed the following package:
- pkg-config
- openssl
- libssl-dev



