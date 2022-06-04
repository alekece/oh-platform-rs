# oh-platform-rs

In this project, different job boards can use our API. On a job board, a company is able to create a profile and post a job opening.
Then applicants are able to apply for the jobs. There can be different job boards having different user interfaces, but they all use this same API.

## Requirement
- Rust version 1.61 or newer.
- Diesel CLI with `postgres` features.
- A running [PostgreSQL](https://www.postgresql.org/) instance.
  
## Setup environment

A `.env.example` at the root directory exposes environment both used by `diesel` and the project itself.  
Rename it to `.env` then set all the environment variables before running the following commands :

```bash
source .env
```

## Build locally

Run the following command to fulfill the requirements :

```bash
# Install Rust and cargo alongside rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install ORM and query builder
sudo apt install libpq-dev
cargo install diesel_cli --no-default-features --features "postgres"

# Run migrations (do not forget to launch postgres server first)
diesel setup 
```

Then build the project :


``` bash
cargo build --release
```

If you want to run it :

``` bash
cargo run --release
```
