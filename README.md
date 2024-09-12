# 🚲 Bike Stations API Backend with 🦀 Rust and 🪳 CockroachDB

A small project to demonstrate how a back-end service for managing bike stations could be implemented using Rust and
CockroachDB.

## 📃 Project characteristics

The most important crates this projects uses are:

- **Axum** - Creates the web server and routes the endpoint handlers.
- **SQLx** - Connects with the CockroachDB, providing compile-time query validation.
- **Tokio** - Provides the `async` runtime.
- **Reqwest** - HTTP client used to retrieve data to be ingested from the GBFS JSON files.

*SeaORM* was considered to manage the database, but I dropped it because found datatype issues with CockroachDB.
*Diesel* was also considered but had a steeper learning curve.

### Structure

- `src/` - Contains all Rust code.
  - `main.rs` - Runs the async Tokio main and registers Axum endpoints.
  - `handler/`
      - `stations.rs` - Defines the functions attached to the endpoints, plus some helper functions. These write to the
        DB directly.
  - `model/`
      - `gbfs.rs` - Defines the struct representing the GBFS JSON file used as request body in `/ingest`
      - `stations.rs` - Defines structs mapping the database tables and JSON objects coming from GBFS URLs
- `sql/` - Contains the database configuration
  - `setup_db.sql` - Creates the database and tables. Useful to restore the DB.

## 🏃 Running the project

Clone this repo, and run
```bash
git clone https://github.com/strosek/bike-stations.git
cd bike-stations
docker compose up --build
```

Then you can call the service at port `3000`:

```bash
curl -v -X POST localhost:3000/ingest -H "Content-Type: application/json" --data {<GBFS requset body here>}
curl -v -X GET localhost:3000/stations
```

To reset the database stop the containers and run them again:

```bash
docker compose down
docker compose up
```

## 🚀 Future improvements

This project was created from scratch with a 4-hours coding time limit without much familiarity with the crates used.
This left a lot to be done to bring the app to production quality. Among the most important improvements to be done are:

- [ ] Improve error management, replace unwraps with the right pattern matching and error propagation.
- [ ] HTTP error codes should be accurate and error messages expressive.
- [ ] Better organization of database access objects, including pools and configuration.
- [ ] Database limits should be in place with paging for GET, and setting a limit for POST on how many rows can be
  inserted at once.
- [ ] Proper logging and monitoring should be added.
- [ ] Tests, and doctests.
- [ ] Add documentation.
- [ ] Code should be organized in multiple workspaces to improve compilation time.
- [ ] Implement Deserialization to handle multiple field data types more cleanly (differences in DB vs JSONs)
- [ ] Add logic to prevent issues with duplicates during ingestion
