# Unified Hosting Panel - AI Coding Agent Instructions

## Architecture Overview

This is a **Rust + HTMX + Tailwind CSS** hosting control panel with Hetzner Cloud VPS integration. The system follows a server-side rendering approach with minimal client-side JavaScript.

### Tech Stack
- **Backend**: Rust (Axum framework) + PostgreSQL (SQLx)
- **Frontend**: HTMX + Tailwind CSS + Alpine.js + Askama templates
- **External APIs**: Hetzner Cloud API for VPS management
- **Deployment**: Systemd (preferred) or Docker

## Key Architectural Patterns

### 1. HTMX-First Frontend
- **No React/Vue**: Use HTMX attributes for interactivity
- **Pattern**: `hx-get="/api/endpoint"` → `hx-target="#container"` → `hx-swap="innerHTML"`
- **Forms**: Always use `hx-post` with proper error handling
- **Real-time**: Use `hx-trigger="every 30s"` for auto-refresh

### 2. Askama Template Structure
- **Base**: All pages extend `templates/base.html`
- **Components**: Reusable pieces in `templates/components/`
- **Pattern**: Create struct with `#[derive(Template)]` + `#[template(path = "file.html")]`

### 3. Database-First Design
- **SQLx**: Compile-time query validation with `sqlx::query_as!()`
- **Migrations**: Always in `migrations/` with sequential numbering
- **Models**: Match database schema exactly in `models/`

### 4. Service Layer Architecture
```rust
// Pattern: Handler → Service → Database
pub async fn create_vps(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateVps>
) -> Result<Json<Vps>, AppError> {
    let vps = vps_service::create_vps(
        &app_state.db,
        &app_state.hetzner_client,
        user_id,
        payload
    ).await?;
    Ok(Json(vps))
}
```

## Critical Development Workflows

### Environment Setup
```bash
# Always run from project root
make frontend-install  # Install Tailwind CSS deps
make frontend-build    # Build CSS for production
cargo install sqlx-cli # For database migrations
```

### Database Changes
```bash
# 1. Create migration
sqlx migrate add descriptive_name
# 2. Write SQL in new file
# 3. Update models to match schema
# 4. Run migration
sqlx migrate run
```

### CSS Development
- **Never edit** `backend/static/css/main.css` directly
- **Always use** Tailwind classes in templates
- **Custom CSS**: Add to `frontend/src/styles.css`
- **Build**: `make frontend-build` or `npm run build` in `frontend/`

## Project-Specific Conventions

### Error Handling
```rust
// Use AppError enum from utils/errors.rs
return Err(AppError::NotFound("VPS not found".to_string()));
return Err(AppError::BadRequest("Invalid input".to_string()));
```

### Hetzner API Integration
- **All VPS operations** must update both Hetzner API and local database
- **Rollback pattern**: If DB fails after Hetzner API call, delete the Hetzner resource
- **Status sync**: Use `sync_vps_status()` to keep local state current

### Authentication Flow
- **JWT tokens**: Stored in HTTP-only cookies
- **Middleware**: `middleware/auth.rs` handles token validation
- **Templates**: Access user via context, not direct token parsing

### UI Patterns
```html
<!-- Loading states -->
<div hx-get="/api/data" hx-trigger="load">
    <div class="animate-pulse-slow">Loading...</div>
</div>

<!-- Status badges -->
<span class="px-2 py-1 rounded-full text-xs {{ status_color }}">
    {{ status|title }}
</span>

<!-- Modals with Alpine.js -->
<div x-show="showModal" x-cloak class="fixed inset-0 z-50">
```

## External Dependencies

### Hetzner Cloud API
- **Authentication**: Bearer token in `HETZNER_API_TOKEN` env var
- **Base URL**: `https://api.hetzner.cloud/v1`
- **Rate Limits**: Built into `HetznerClient` with proper error handling
- **Server Types**: CX11, CX21, CX31, CX41, CX51 with specific pricing

### Database Schema
- **UUIDs**: Primary keys for all user-facing resources
- **Timestamps**: Always `created_at` and `updated_at` with timezone
- **Foreign Keys**: Proper CASCADE behavior for related data

## Build & Deployment

### Development
```bash
# Terminal 1: CSS watch
make frontend-dev
# Terminal 2: Rust with hot reload
make backend-dev
```

### Production Build
```bash
make build  # Builds both frontend CSS and Rust binary
```

### Systemd Service
- **Location**: `unified-panel.service` in root
- **User**: `www-data` for security
- **Environment**: All config via env vars, never hardcoded

## Common Pitfalls

1. **HTMX Swapping**: Always specify correct `hx-target` and `hx-swap` behavior
2. **Lucide Icons**: Call `lucide.createIcons()` after DOM updates
3. **Dark Mode**: Use `dark:` prefixes in Tailwind, theme switching via Alpine.js
4. **Error Responses**: HTMX expects proper HTTP status codes for error handling
5. **Database Transactions**: Use transactions for multi-step operations (VPS creation)

## File Organization

- **API Routes**: `src/api/` - RESTful endpoints
- **Page Handlers**: `src/handlers/pages.rs` - Server-side rendered pages
- **Services**: `src/services/` - Business logic layer
- **Models**: `src/models/` - Database entities
- **Templates**: `templates/` - Askama HTML templates
- **Static Assets**: `backend/static/` - Served directly by Axum

## Testing Strategy

- **Unit Tests**: Test service layer functions with mock databases
- **Integration Tests**: Test API endpoints with test database
- **Template Tests**: Render templates with mock data to verify HTML output

Remember: This is a **server-side rendered application** with progressive enhancement via HTMX. Keep JavaScript minimal and prefer server-side solutions.