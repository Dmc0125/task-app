# Task-app

## Development

### Database

To use database in development, install [docker](https://www.docker.com/)

```sh
docker-compose up
```

### Backend

Set up env config
```env
DATABASE_URL=

BASE_URL=
CLIENT_URL=

CLIENT_SIGNIN_SUCCESS_URL=
CLIENT_SIGNIN_FAIL_URL=

DISCORD_CLIENT_SECRET=
DISCORD_CLIENT_ID=

GOOGLE_CLIENT_SECRET=
GOOGLE_CLIENT_ID=

SIGNATURE_KEY=
```

#### Database

Install sea-orm-cli

```sh
cargo install sea-orm-cli
```

Migrate database

- Migrate up
```sh
sea-orm-cli migrate up
```
- Migrate down
```sh
sea-orm-cli migrate down
```

Generate entities
```sh
sea-orm-cli generate entities -o src/backend
```

#### Server

Run

```sh
cd backend
cargo watch -x run
```
