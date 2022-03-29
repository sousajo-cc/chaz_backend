# Chaz backend

This project serves as the server side implementation for the persistent high
score table of [chaz](https://github.com/lcjgames/chaz).

## Setup

```bash
sudo apt-get install libsqlite3-dev
cargo install diesel_cli --no-default-features --features sqlite
diesel migration run
```

### Development 

You need to create a `.env` file with:
```asm
DATABASE_URL=<your_database_name>
```