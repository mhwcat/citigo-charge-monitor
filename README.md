# Citigo Charge Monitor
[![Docker build](https://github.com/mhwcat/citigo-charge-monitor/workflows/Docker%20build/badge.svg)](https://github.com/mhwcat/3dhw/actions)

Tool for monitoring [Skoda Citigo e-iV](https://ev-database.org/car/1190/Skoda-CITIGOe-iV) charging. Main feature is ability to set target state of charge which is absent from official app for some reason. Made possible by excellent [Skodaconnect](https://github.com/lendy007/skodaconnect) Python library which does most of heavy lifting here.
## Services
|Name|Purpose|Stack|
|---|---|---|
|citigo-charge-monitor-api-server|API serving data for other services|Rust (actix-web, sqlx)
|citigo-charge-monitor-front-app|Webapp for managing target SOC and browsing recent charge sessions|JS (Vue 3)|
|citigo-charge-monitor-worker|Async worker fetching data from Skodaconnect API|Python|
|citigo-charge-monitor-revproxy|Nginx-based reverse proxy (optional)|Nginx in Docker|
## Building and running
Backend for data storage is MySQL 8 (for persistent data like users, vehicles, charge sessions) and Redis 6.2 for user sessions. Other DB engines that have compatible binary protocol with MySQL (like MariaDB) should also work.
### Docker
All services have Dockerfiles for building and running. Docker-compose  file is provided for setting everything up together. Docker volume is created for database container.

Fill missing information in [docker-compose.yml](docker-compose.yml) (replace all `CHANGEIT` strings):
* set correct timezone
* in `citigo-worker` service set your vehicle's VIN, credentials to Skodaconnect API (e-mail and password used in official Skoda app) and credentials for worker service user (you will create that user later).

Customize any other settings as you like. Next, execute:
```bash
docker-compose build
docker-compose up -d
```
This will build and start all services. If everything goes well you should have four `citigo-*` services running + MySQL engine and Redis. Webapp is available at `http://localhost:8080/` and API at `http://localhost:8080/api`. 

Next, create user for worker service and (optionally) for webapp. `citigo-charge-monitor-api-server` generates registration token at startup which is required when creating new users. Copy token from API service logs, which should look like this:
```
2022-04-17 16:59:52.839 INFO [citigo_charge_monitor_api_server]: Generated registration token: d9faa37a-9a73-4be9-8625-651dcbc5b4ae
```
Then, create new user using `curl` or similar tool:
```bash
curl -v -X POST -H '{"Authorization": "Bearer d9faa37a-9a73-4be9-8625-651dcbc5b4ae"}' -H '{"Content-type": "application/json"}' -d '{"username": "worker", "password": "workerpass"}' http://localhost:8080/api/auth/register
```
Then you can restart worker service
```bash
docker-compose restart citigo-worker
```
and check logs to see vehicle information retrieved from Skoda API.
You can now login to webapp and configure target state of charge for selected vehicle.
### Manual
### API server
Install [Rustlang](https://www.rust-lang.org/learn/get-started) and execute 
```bash
cd citigo-charge-monitor-api-server/
cargo build --release
```
Compiled binary will be saved in `target/release/` directory and can be executed like this (customize environment variables as you need):
```bash
CITIGO_DATABASE_URL="mysql://citigo:citigo@localhost/citigo_charge_monitor" CITIGO_REDIS_URL="redis://localhost" CITIGO_API_BASE_ADDR="0.0.0.0:8000" ./citigo-charge-monitor-api-server
```
By default this app logs to stdout, this can be customized in [log4rs.yml](citigo-charge-monitor-api-server/log4rs.yml) file (refer to [Log4rs docs](https://docs.rs/log4rs/latest/log4rs/)).
### Worker
Install Python (3.10+ is recommended) and pip. Execute
```bash
cd citigo-charge-monitor-worker/
pip install -r requirements.txt
```
Similar to API server, set all settings in environment variables when executing worker, e.g:
```bash
CITIGO_API_BASE_URL="http://localhost:8000/" CITIGO_API_USERNAME="worker" CITIGO_API_PASSWORD="workerpass" CITIGO_SKODACONNECT_USERNAME="CHANGEIT" CITIGO_SKODACONNECT_PASSWORD="CHANGEIT" CITIGO_VEHICLE_VIN="CHANGEIT" python main.py
```
### Front app
Note that webapp assumes that API server will be available at `api/` URL. You can change that in [http-common.ts](citigo-charge-monitor-front-app/src/http-common.ts) file (`baseURL` property). 

Install [Node 17+](https://nodejs.org/en/). Execute:
```bash
cd citigo-charge-monitor-front-app/
npm install
npm run build
```
Output files will be saved to `dist/` directory. You can also run development server by executing:
```bash
npm run serve
```
## TODO
* Use [actix-web-httpauth](https://crates.io/crates/actix-web-httpauth) in API server once compatibility issues with actix-cors are resolved (see [src/services/auth.rs](citigo-charge-monitor-api-server/src/services/auth.rs))
* Document endpoints in API server using [paperclip-rs](https://github.com/paperclip-rs/paperclip)
* Add tests to API server
