# A personalized Rust backend web server for quick releases

### Setup Dev
1. `sh scripts/set_up_infra.sh`
2. Create a .env file
```
DATABASE_URL=postgres://postgres:postgres@localhost:5432/kodingkorp
AUTO_MIGRATE=TRUE
PORT=3000
HOST=0.0.0.0
IAM_JWT_SECRET=TEST_SECRET
```
3. `cargo install cargo-watch`
4. `cargo watch -x run`

### Structure

```shell
.
├── dev_infra # set up dev infra using docker
├── migration # holds database migrations
├── scripts # holds shell scripts
├── src
│   ├── app # All application logic
│   │   ├── capabilities # capabilities built into application
│   │   │   ├── common # common capabilities need across services
│   │   │   │   ├── config
│   │   │   │   ├── global_model
│   │   │   ├── iam # Identity Access Management service
│   │   │   │   ├── controllers # holds all routes maintained by IAM
│   │   │   │   │   ├── authentication # Authentication routes
│   │   │   │   │   ├── users # user level protected routes
│   │   │   │   ├── entities # database entities
│   │   │   │   ├── enums 
│   │   │   │   ├── helpers # general utility methods
│   │   │   │   ├── models # IAM specific models
│   │   │   │   ├── services
│   │   │   │   │   ├── auth 
│   │   │   │   │   ├── iam # main service exposed by IAM
│   │   │   │   │   ├── users
│   │   │   │   ├── constants.rs # Constants of IAM
│   │   ├── routes # General routes
│   │   ├── types # General Types
│   │   ├── bootstrap.rs # Bootstrap the poem App
│   ├── lib.rs 
│   └── main.rs # Entry point
├── Cargo.lock
├── Cargo.toml
├── LICENSE
└── README.md
```
