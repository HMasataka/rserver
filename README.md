# Run Server

```bash
cargo run
```

## Request

```bash
curl -X GET localhost:3000/users
```

```bash
curl -X POST -H "Content-Type: application/json" -d '{"name":"sasaki"}' localhost:3000/users
```

```bash
curl -X PATCH -H "Content-Type: application/json" -d '{"name":"kojiro"}' localhost:3000/users/1
```

```bash
curl -X DELETE localhost:3000/users/1
```
