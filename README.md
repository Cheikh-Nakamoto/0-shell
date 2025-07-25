# 🖥️ 0-shell

### 🥅 Goal

- [x] Create a shell that can execute basic commands
- [x] Commands
    - [x] echo
    - [x] cd
    - [x] ls, including flags -l, -a, and -F
    - [x] pwd
    - [x] cat
    - [x] cp
    - [x] rm, including flags -r
    - [x] mv
    - [x] mkdir
    - [x] exit
- [x] Interruption with Ctrl+D
- [x] Colors for errors

### ⚙️ How to run

1. Clone the repository
2. Open a terminal in the repository folder and run the following commands:

```bash
make clean
make build

# Run the shell
./0-shell
# or
make run
```

### Tree

```text
0-shell
├── src
│   ├── commands
│   │   ├── mod.rs
│   │   ├── cat.rs
│   │   ├── cp.rs
│   │   ├── echo.rs
│   │   ├── exit.rs
│   │   ├── ls.rs
│   │   ├── mkdir.rs
│   │   ├── mv.rs
│   │   ├── pwd.rs
│   │   ├── rm.rs
│   │   └── touch.rs
│   ├── utils
│   │   ├── mod.rs
│   │   ├── color.rs
│   │   ├── date.rs
│   │   ├── error.rs
│   │   ├── messages.rs
│   │   ├── path.rs
│   │   └── utils.rs
│   ├── main.rs
│   └── shell.rs
├── .gitignore
├── Cargo.toml
├── Makefile
└── README.md
```

### Contributors

- Louis Isaac Jean Samba DIOUF
- Cheikh Mounirou Coly DIOUF
- Franchis Janel MOKOMBA

###### &copy; 2025 - Powered by Zone01 Dakar
