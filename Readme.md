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

## Deploy via DOCKER

```Dockerfile

dsdasds
dsdasds
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




