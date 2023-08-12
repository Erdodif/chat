# Changelog

Only breaking/important changes are listed here. For a full list of changes, see the [commit history](https://github.com/hypergonial/chat/commits/main/).

## 2023.08.12-1

- Changed the container names to omit the `chat-` prefix. You may need to update your `.env` file's `POSTGRES_HOST` by changing it to `db`.
- Added attachment support to messages. Attachments are stored in MinIO, an S3-compatible object storage service. The MinIO instances are automatically started when running `docker compose up` and listen on port `:9000`.
- When creating messages, the endpoint `POST /channels/{channel_id}/messages` now expects `multipart/form-data` instead of `application/json`. Details of this change can be found on the route's [documentation page](./rest/channels.md).

## 2023.08.06-1

- Made `User.display_name` nullable. If the user has no display name, clients are expected to use the username instead.