# Raffle API ENDPOINT

This is an implementation for a REST-API for hosting raffles.

![icon_app](icon.drawio.png)

## Stack

```

+----------------+
|   Raffle BOT   |     
+----------------+
        |
+----------------+
|   Raffle API   |     
+----------------+
        |
+----------------+
|    MongoDB     |
+----------------+

```

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
st=>start: Start|past:>http://www.google.com[blank]
e=>end: End|future:>http://www.google.com
op1=>operation: My Operation|past
op2=>operation: Stuff|current
sub1=>subroutine: My Subroutine|invalid
cond=>condition: Yes
or No?|approved:>http://www.google.com
c2=>condition: Good idea|rejected
io=>inputoutput: catch something...|future

st->op1(right)->cond
cond(yes, right)->c2
cond(no)->sub1(left)->op1
c2(yes)->io->e
c2(no)->op2->e

```

### Notes

- [cargo_chef_sample](https://www.lpalmieri.com/posts/fast-rust-docker-builds/)

mkcert -key-file key.pem -cert-file cert.pem 127.0.0.1 localhost




