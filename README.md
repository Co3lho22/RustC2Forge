# RustC2Forge
![](images/name.png)

RustC2Forge is a Command and Control (C2) server implemented in Rust, designed for educational purposes. This project represents my inaugural venture into Rust development, with the primary objective of familiarizing myself with the language. The essence of RustC2Forge is a basic reverse shell capability, laying a foundation for future expansion and the addition of more sophisticated features.

# Environment Setup
To run this project, you must have Rust installed on your computer. For installation instructions and more information on Rust, please visit the official Rust website.

# How it Works
![](images/main.png)

The diagram above outlines the project's main functionality, dividing the application into two distinct workspaces: /client and /server.
The project is ready to run out of the box, but if you want to change the ips or the ports used this are the files you need to look at:
- server/src/main.rs
- client/src/main.rs
- server/src/handler/heartbeats.rs - function listen_for_heartbeats


## Server
To launch the server component, use the following command:
```Shell
cargo run --bin server
```
## Client
To start the client component, execute:
```Shell
cargo run --bin client
```

# Features
Reverse Shell: Facilitates the execution of commands from the server on the connected client, acting as a basic form of a reverse shell.

Heartbeat: A mechanism to check whether the client (victim) is still connected and active.

# Run RustC2Forge Locally
If you just want to quickly test the project you can use you local machine as the server and the client.
In fact this was what i used for my development enviroment.

## Server


# Future Enhancements
File Transfer: Implement the ability to transfer files to and from the client.

Modular Structure: Refactor the codebase to adopt a more modular structure, simplifying integration with malware tools and enhancing the system's overall robustness.

# Contribution
RustC2Forge is open for contributions. Whether it's adding new features, fixing bugs, or improving documentation, your help is welcome. Please feel free to fork the repository, make your changes, and submit a pull request.

# Disclaimer
This project is for educational purposes only. The development and use of C2 servers can be associated with malicious activities. It is essential to use this project responsibly and within the boundaries of ethical guidelines and legal requirements.


