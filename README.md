# DTEK2058 Advanced Software Project

## Brief description of system functionality

TODO.

## High-level system architecture and used technological stack

### Terminology

- **RDS** = _Rust Dedicated Server_. Name of the Rust video game server program
  distributed via _SteamCMD_.

- **Rust (programming language)** = Language used for implementing some of the
  stuff in this project, as opposed to the video game (also relevant in this
  project).

- **Rust (video game)** = A survival game available on Steam. Uses Unity game
  engine. The game is friendly to modding.

- **SteamCMD** = A program used to acquire game server programs for games played
  on _Steam_ platform.

- **systemd** = A system for automating managing of services on Linux.

- **Carbon** = A modding framework for the video game Rust.

- **Unity** = The game engine used by the Rust game. It makes use of .NET and
  is therefore easily moddable by patching relevant DLLs that in turn are easy
  to make sense of because they can be decompiled into easily readable C Sharp
  code. "Easy" as in as opposed to reverse engineering something that is not
  inherently modding friendly.

- **DLL** = Dynamically linked library, i.e. some executable code that some
  other executable code calls to. Important characteristic for this project is
  that these libraries can be recompiled with modifications on the fly, i.e.
  while the caller is being run. This allows for easy modding of a video game
  for example.

- **.NET** = For the purposes of this project, this is a software framework
  used by our game engine of interest, in a way that makes modding the game
  relatively easy. Makes use of DLLs compiled from C Sharp code.

- **C Sharp** = Language used for implementing some of the stuff in this
  project.

### Components

```
.
├── rcon-cli
│   ├── Tool for managing RDS.
│   └── CLI program implemented in Rust language.
├── rds-plugins
│   ├── Carbon plugins, i.e. source code for Unity game engine .NET DLLs
│   │   compatible for loading into RDS by the modding framework.
│   └── C Sharp code.
├── rds-sync
│   ├── WebSocket server for synchronizing RDS state with web clients.
│   ├── TODO: Add a scheduled recurring job to select a new seed and do a "map
│   │         wipe" weekly, and a "blueprint wipe" monthly.
│   ├── TODO: Add a server state change bound Discord alert: use a webhook to
│   │         send a message when server is starting, updating, dead etc.
│   ├── TODO: Add database and store game statistics there.
│   └── Implemented in Rust language.
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

I've been using Debian 12.
