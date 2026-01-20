# L-Slash Project Context for AI Agents

## Core Purpose
L-Slash is a URL shortener and alias manager built in Rust. Create short URL aliases (e.g., `l` -> `https://example.com`) with support for:
- Relative path appending (`/l/docs` → `https://example.com/docs`)
- Query string passthrough
- Change history tracking
- User authentication with session management

## Tech Stack
- **Rust 2021** with Axum web framework (tokio async runtime)
- **Sled** embedded key-value database
- **Protocol Buffers** for data serialization
- **Tera** template engine for HTML rendering
- **Tower-HTTP** for middleware
- **SHA-256** for password hashing

## Architecture (3 Layers)
1. **Web**: Axum routes, handlers, auth middleware (`src/web/`)
2. **Services**: Business logic for URLs, auth, users (`src/services/`)
3. **Storage**: Sled wrapper with generic `SledTree<T>` (`src/storage/`)

## Key Data Models (Protobuf)
- **User**: username, email, password_sha256, password_salt, is_admin
- **Record**: name (alias), url, owner, tags
- **RecordChange**: name, url_before, url_after, changed_by, timestamp
- **Session**: key, secret, previous_secret, user_name, expire_time, rotation_time, is_admin

## Main Routes
- `GET /:alias` - Redirect to URL
- `GET /:alias/:relative` - Redirect with relative path
- `GET /` - Main form (auth required)
- `POST /_/` - Create/update alias (auth required)
- `GET /_/:alias` - View/edit alias with change history
- `GET /_list` - List all aliases
- `GET /_login` - Login page (public)
- `POST /_login` - Submit login (public)
- `GET /_debug` - Debug session info (admin only)
- `GET /_health` - Health check

## Security Features
- Session rotation every 5 minutes with 1-minute grace period
- Cookie format: `auth={base64(key)}/{base64(secret)}`
- Auth middleware validates sessions on every request
- Admin-only routes (`/_debug`)
- HttpOnly, Secure (production), SameSite cookies

## Code Organization
```
src/
├── main.rs              # Server entry point
├── lib.rs               # Library root, protobuf includes
├── app.rs               # App builder
├── state.rs             # AppState (store + config)
├── cli/                 # CLI binary for DB operations
│   ├── main.rs
│   ├── flags.rs         # Clap argument definitions
│   └── commands.rs      # Command implementations
├── web/                 # HTTP server
│   ├── routes.rs        # Route definitions
│   ├── handlers/        # Request handlers
│   │   ├── mod.rs       # Main handlers
│   │   ├── http.rs      # HTTP response helpers
│   │   ├── misc.rs      # Health check
│   │   ├── tera.rs      # Template initialization
│   │   └── template/    # HTML/Tera templates
│   └── middleware/
│       └── auth_cookie.rs  # Auth middleware
├── services/            # Business logic
│   ├── auth.rs          # Session & login logic
│   ├── user.rs          # User creation & hashing
│   ├── url.rs           # Alias resolution & insertion
│   └── result.rs        # Error types
├── storage/             # Data persistence
│   └── sled_store.rs    # Sled wrapper with SledTree<T>
└── protos/              # Protobuf definitions
    ├── user.proto
    ├── record.proto
    └── session.proto
```

## Code Conventions
- Protobuf definitions in `src/protos/*.proto`, generated via `build.rs`
- Custom error types with `thiserror`
- Generic database operations via `SledTree<T>`
- HTML templates in `src/web/handlers/template/` (Tera format)
- CLI tool available as alternate binary (`cargo run --bin cli`)
- Database stored in `sled_data/` directory (gitignored)

## Change Tracking
All record updates create a `RecordChange` entry with composite key: `{alias}/{hex_timestamp}`, storing url_before, url_after, and changed_by user. Changes are displayed in the edit form with full history.

## Development Commands
- **Run server**: `cargo run`
- **Run CLI**: `cargo run --bin cli -- [command]`
- **Build**: `cargo build --release`
- **CLI commands**: `list`, `lookup`, `add`, `list_users`, `add_user`, `move_records`

## Key Implementation Details

### Session Management Flow
1. Login validates credentials (SHA-256 hash check)
2. New session created with random key + secret
3. Session expires in 365 days, secret rotates every 5 minutes
4. Cookie sent as `auth={base64(key)}/{base64(secret)}`
5. Middleware validates on every request:
   - Decode cookie
   - Look up session in DB
   - Validate secret (current or previous if within grace period)
   - Check expiration
   - Rotate secret if needed

### URL Resolution Flow
1. Look up alias in `records` tree
2. Parse base URL from record
3. Apply relative path if exists (via `url::Url::join()`)
4. Append query string if exists
5. Redirect (303 See Other) to final URL
6. Fallback to form page if alias not found

### Database Schema (Sled Trees)
- **users**: Key = username, Value = User protobuf
- **sessions**: Key = session_key, Value = Session protobuf
- **records**: Key = alias_name, Value = Record protobuf
- **record_changes**: Key = `{alias}/{hex_timestamp}`, Value = RecordChange protobuf

## Coding Guidelines
When working on this project:
- Maintain clean separation between web/services/storage layers
- Use proper error handling with `Result<T>` and custom error types
- Follow existing patterns for new features
- Keep session security mechanisms intact
- Add change tracking for record modifications
- Use protobuf for new data models if needed
- Follow Rust idioms and best practices
- Test both server and CLI when making storage changes