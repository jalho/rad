# DTEK2058 Advanced Software Project

## Brief description of system functionality

TODO.

## High-level system architecture and used technological stack

### Terminology

- **RDS** = _Rust Dedicated Server_. Name of the Rust video game server program
  distributed via _SteamCMD_.

- **Rust (programming language)** = Language used for implementing some of the
  stuff in this project.

- **Rust (video game)** = A survival game available on Steam.

- **SteamCMD** = A program used to acquire game server programs for games played
  on _Steam_ platform.

- **systemd** = A system for automating managing of services on Linux.

### Components

```
.
├── rcon-cli
│   ├── Tool for managing RDS.
│   └── CLI program implemented in Rust language.
├── rds-sync
│   ├── Server for synchronizing RDS state with web clients.
│   ├── WebSocket server implemented in Rust language.
│   └── TODO: add database?
├── rds-ui-web
│   ├── Web frontend for rds-sync: shows RDS state, such as player positions
│   │   on game world map in real time.
│   └── React web app written in TypeScript.
├── rust-dedicated-server
│   └── The video game server (RDS) instance, wrapped in helper service(s) that
│       manage server updates, do health checks and automatic restarts etc.
│       using systemd.
└── tls-reverse-proxy
    ├── Provides TLS for communication between web clients and rds-sync.
    └── Nginx instance.
```

## Used development methods and software tools

TODO.

## Project schedule (planned or actualized)

TODO.

## Estimated own contribution to the project in hours

100 %

---

# Deploying

There's a bundle of compiled binaries and stuff in this GitHub repo's releases
section. A new release bundle can be made using `make-release.sh`.

Deploy a VM with at least 16 GB of RAM. RustDedicated requires that much. Then
install all the stuff there.

```
wget https://raw.githubusercontent.com/jalho/DTEK2058-Advanced-Software-Project/master/install.sh
bash install.sh
```
