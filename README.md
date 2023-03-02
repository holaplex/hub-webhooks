# Hub Webhooks

Register Hub webhooks and broadcast messages using [Svix](https://svix.com/).

## Getting Started

```
docker compose up -d
cargo run --bin holaplex-hub-webhooks
```

Visit [http://localhost:3008/playground](http://localhost:3008/playground) to access the GraphQL playground.

### Generating Svix auth token

```
docker exec -it {svix_container_id} /bin/bash
$ 
```

### Seed a Svix application

Ensure you have the [Svix CLI](https://github.com/svix/svix-cli) installed on your machine. Set the uid of the Svix application to the organization id.

```
# create a svix application
SVIX_AUTH_TOKEN={SVIX_AUTH_TOKEN} SVIX_SERVER_URL=http://localhost:8071 svix application create \
 --data-name="Example Org" \
 --data-uid=623db483-6fe9-428e-9682-56111c7a478d
{
  "createdAt": "2023-03-02T12:50:51.587547Z",
  "id": "app_2MSWR0EwWzhtwJGgPekaoVByqgN",
  "name": "Example Org",
  "rateLimit": null,
  "uid": "623db483-6fe9-428e-9682-56111c7a478d",
  "updatedAt": "2023-03-02T12:50:51.591488Z"
}

# setup hub-webhooks db
sea migrate up --database-url postgres://postgres:holaplex@localhost:5440/hub_webhooks

# insert organization_application record
psql postgres://postgres:holaplex@localhost:5440/hub_webhooks
hub_webhooks=# INSERT INTO organization_applications VALUES ('{svix_app_id}', '{organization_id}');
```
