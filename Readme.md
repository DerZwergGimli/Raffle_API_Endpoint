# Raffle API ENDPOINT

This is an implementation for a REST-API for hosting raffles.

![icon_app](icon.drawio.png)

## Stack

```

+----------------+
|   Raffle BOT   | here: https://github.com/DerZwergGimli/DRRB
+----------------+
        |
+----------------+
|   Raffle API   | in here 
+----------------+
        |
+----------------+
|    MongoDB     | via docker
+----------------+

```

The UI is currently only a discord bot can be found here:
https://github.com/DerZwergGimli/DRRB

## Deploy via Docker-Compose.yaml

```Dockerfile

version: '3.1'

services:
  mongo:
    image: mongo
    restart: always
    ports:
      - "27017:27017"
    environment:
      MONGO_INITDB_ROOT_USERNAME: <db_user>
      MONGO_INITDB_ROOT_PASSWORD: <db_password>
    volumes:
      - /etc/localtime:/etc/localtime:ro
      - db-storage:/data/db


  api:
    image: derzwerggimli/raffle_api
    restart: unless-stopped
    links:
      - mongo
    depends_on:
      - mongo
    environment:
      SERVER_IP: 0.0.0.0
      SERVER_PORT: 8080
      MONGODB_URI: mongodb://<db_user>:<db_password>@mongo:27017
      API_BEARER_TOKEN: <someAPIKEY>
      SOL_WALLET: <wallet_address>
      CHECK_RAFFLE_EXISTS: 'true'
      CHECK_RAFFLE_RUNNING: 'true'
      CHECK_RAFFLE_TIME: 'false'
      CHECK_RAFFLE_DESTINATION: 'true'
      CHECK_RAFFLE_USED_SIGNATURE: 'true'
      CHECK_TOKEN_SYMBOL: 'true'
      CHECK_TX_STATUS: 'true'
    volumes:
      - /etc/localtime:/etc/localtime:ro
    ports:
      - "8080:8080"
  
  bot:
    image: derzwerggimli/raffle_bot
    restart: unless-stopped
    depends_on:
      - api
    links:
      - api
    environment:
       RAFFLE_API_URL: https://api:8080/api/v1
       RAFFLE_API_KEY: <someAPIKEY>
       DISCORD_TOKEN: <DiscordToken>
       RUST_LOG: info
       DELETE_MESSAGE: 'true'
       MESSAGE_TIMER: 10
       GUILD_ID: 886298505126232115
       ROLE_ID: 886419710911053856
       CHANNEL_ID: 960295874154610689
       LOOP_SLEEP: 10
    volumes:
      - /etc/localtime:/etc/localtime:ro
    
volumes:
  db-storage: { }

```

## Endpoints

- GET
- PUT
- POST
- DELTE

## Configuration

Environment variables:

```env
SERVER_IP=0.0.0.0
SERVER_PORT=8080
MONGODB_URI=mongodb://<USERNAME>:<PASSWORD>@localhost:27017
API_BEARER_TOKEN=<SOME_TOKEN>
SOL_WALLET=<SOLANA_WALLET_TO_CHECK>
# The following are used to validate tickets
CHECK_RAFFLE_EXISTS=true
CHECK_RAFFLE_RUNNING=true
CHECK_RAFFLE_TIME=true
CHECK_RAFFLE_DESTINATION=true
CHECK_RAFFLE_USED_SIGNATURE=true
CHECK_TOKEN_SYMBOL=true
CHECK_TX_STATUS=true
```

### Notes

- [cargo_chef_sample](https://www.lpalmieri.com/posts/fast-rust-docker-builds/)
- docker build -t




