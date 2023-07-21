# Actix Svelte + Surreal DB Template

Here's an actix-web template that uses SvelteKit built and served as static files.
With simple CRUD setup using Surreal DB as a permanent storage for application data (Todos).

* Derived from original repo: [actix_svelte_template](https://github.com/nelsontkq/actix_svelte_template): 
   - bugfixes, 
   - updated deps, 
   - updated CRUD app that uses SurrealDB instead of SQLite.

## Features

- [Actix web](https://actix.rs/) server
- [SurrealDB](https://surrealdb.com/) SurrealDB is the ultimate multi-model database for tomorrow's applications for storing todos data.
- [SvelteKit](https://kit.svelte.dev/) for frontend, served as static files


## Setup

We use Vite's proxy in the dev environment and serve svelte as static files in production.

### Dev Requirements

- [ ] Rust: `curl https://sh.rustup.rs -sSf | sh`
- [ ] node: https://nodejs.org/en/download/current/
- [ ] Docker engine and docker image of SurrealDB.


### Script
Install dependencies:
```shell
cd client && npm install
cd ..
npm install
```

Start docker img of SurrealDB:
```shell
docker run --rm --pull always --name surrealdb -p 8000:8000 -v ~/mydata:/mydata surrealdb/surrealdb:latest start file:/mydata/mydatabase.db --user root --pass root
```

Start actix backend and svelte frontend with one command (from root directory):
```bash
npm run dev
```

All traffic to localhost:3000/api/* will be forwarded to the actix web project, and anything else to the SvelteKit frontend.

### Build

You can build the project with cargo. The `build.rs` will automatically compile the frontend to static files in the ./client/build directory.

```bash
cargo build --release
```

For convenience a Dockerfile was created which handles compiling the frontend to static files and building the Actix Web server into a 20mb Alpine image.

```bash
docker build -t actix-svelte-surreal .
docker run -d -p 8080:8080 actix-svelte-surreal
```
