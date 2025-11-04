# Unified Hosting Panel - Architecture

## Technology Stack

### Backend
- **Language**: Rust
- **Framework**: Axum (high-performance async web framework)
- **Database**: PostgreSQL with SQLx (compile-time checked queries)
- **Authentication**: JWT tokens + secure session management
- **Real-time**: Server-Sent Events (SSE) for live updates

### Frontend
- **Interactivity**: HTMX (no JavaScript frameworks)
- **Styling**: Tailwind CSS
- **Templating**: Askama (type-safe Jinja2-like templates)
- **Icons**: Lucide icons (SVG)
- **Theme**: Dark/Light mode with smooth transitions

### Automation
- **Scripts**: Bash, Python
- **Configuration Management**: Ansible playbooks
- **Workflow Automation**: n8n integration

## Project Structure

```
unified-panel/
├── backend/                    # Rust backend
│   ├── src/
│   │   ├── main.rs            # Application entry point
│   │   ├── api/               # API routes
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs        # Authentication endpoints
│   │   │   ├── servers.rs     # Server management
│   │   │   ├── users.rs       # User management
│   │   │   ├── dns.rs         # DNS management
│   │   │   ├── websites.rs    # Website management
│   │   │   └── monitoring.rs  # Metrics & monitoring
│   │   ├── models/            # Database models
│   │   ├── handlers/          # Request handlers
│   │   ├── services/          # Business logic
│   │   ├── middleware/        # Auth, CORS, logging
│   │   ├── database/          # Database connection & migrations
│   │   ├── utils/             # Utility functions
│   │   └── config.rs          # Configuration
│   ├── templates/             # Askama HTML templates
│   │   ├── base.html          # Base layout
│   │   ├── components/        # Reusable components
│   │   ├── pages/             # Full pages
│   │   └── partials/          # HTMX partial responses
│   ├── static/                # Static assets
│   │   ├── css/               # Tailwind CSS output
│   │   ├── js/                # Minimal JavaScript
│   │   └── icons/             # SVG icons
│   ├── migrations/            # SQL migrations
│   └── Cargo.toml             # Rust dependencies
│
├── automation/                 # Automation scripts
│   ├── scripts/
│   │   ├── bash/              # Bash scripts
│   │   └── python/            # Python scripts
│   ├── ansible/               # Ansible playbooks
│   │   ├── playbooks/
│   │   ├── roles/
│   │   └── inventory/
│   └── n8n/                   # n8n workflows
│       └── workflows/
│
├── frontend-dev/              # Frontend development
│   ├── tailwind.config.js     # Tailwind configuration
│   ├── package.json           # Node dependencies
│   └── styles/                # Source CSS
│
└── docs/                      # Documentation
    ├── API.md
    ├── DEPLOYMENT.md
    └── DEVELOPMENT.md
```

## Core Features (MVP)

### 1. Authentication & Authorization
- JWT-based authentication
- Role-based access control (Admin, Reseller, User)
- Session management
- 2FA/TOTP support
- Rate limiting on login attempts

### 2. Server Management
- Physical server CRUD
- VPS management
- Server status monitoring (online, offline, maintenance)
- Server metrics (CPU, memory, disk, network)
- SSH terminal integration

### 3. User Management
- User CRUD operations
- Role assignment
- Profile management
- Access logs

### 4. Dashboard & Monitoring
- Real-time metrics dashboard
- Server statistics
- Alert system
- Activity logs

### 5. UI/UX
- Polished, professional design
- Smooth animations
- Dark/light theme switcher
- Responsive layout
- Loading states & skeleton screens

## Data Flow

### HTMX Pattern
```
User Action → HTMX Request → Rust Handler →
Database Query → Template Render → Partial HTML Response →
HTMX Swap → UI Update
```

### Authentication Flow
```
Login Form → POST /api/auth/login → Validate Credentials →
Generate JWT → Set HTTP-only Cookie → Redirect to Dashboard
```

### Real-time Updates
```
Client connects SSE endpoint → Server streams events →
HTMX listens → Triggers partial update → UI updates automatically
```

## Database Schema (Core Tables)

```sql
-- Users & Authentication
profiles (id, email, role, company, timezone, avatar_url)
user_security_settings (user_id, ip_restrictions, mfa_enabled, session_timeout)
login_attempts (id, email, ip_address, user_agent, success, timestamp)

-- Servers
servers (id, name, ip_address, status, type, location, specs)
server_metrics (id, server_id, cpu, memory, disk, network, timestamp)

-- Websites
websites (id, user_id, domain, status, application_type, php_version)
hosting_packages (id, name, storage, bandwidth, databases, price)

-- DNS
domains (id, user_id, name, status, registrar, expires_at)
dns_records (id, domain_id, type, name, value, ttl, priority)

-- Logs
logs (id, level, category, message, user_id, server_id, timestamp)
```

## Security Considerations

1. **Input Validation**: All inputs validated and sanitized
2. **SQL Injection**: SQLx prevents SQL injection with compile-time checks
3. **XSS Protection**: Templates auto-escape by default
4. **CSRF Protection**: CSRF tokens for state-changing operations
5. **Rate Limiting**: Prevent brute force attacks
6. **Secure Sessions**: HTTP-only, secure, SameSite cookies
7. **Password Hashing**: Argon2id for password storage
8. **Audit Logging**: All critical actions logged

## Performance Optimizations

1. **Connection Pooling**: PostgreSQL connection pool
2. **Async Operations**: Full async/await throughout
3. **Minimal JavaScript**: HTMX reduces bundle size
4. **CDN Ready**: Static assets can be served from CDN
5. **Database Indexing**: Proper indexes on query columns
6. **Caching**: Redis for session and frequently accessed data

## Development Workflow

1. **Hot Reload**: cargo-watch for Rust code
2. **Tailwind Watch**: Automatic CSS compilation
3. **Database Migrations**: SQLx migrations for schema changes
4. **Testing**: Unit tests for business logic, integration tests for API
5. **Linting**: clippy for Rust best practices

## Deployment

1. **Containerization**: Docker for easy deployment
2. **Reverse Proxy**: Nginx for SSL termination and static file serving
3. **Process Management**: systemd for Rust application
4. **Database**: Managed PostgreSQL or self-hosted
5. **Monitoring**: Prometheus + Grafana for metrics

## Future Enhancements

- VPS provisioning (Hetzner, DigitalOcean, AWS)
- Email server management
- DNS server integration (PowerDNS)
- Billing & invoicing system
- Backup management
- File manager with web interface
- API webhooks for external integrations
- Mobile-responsive PWA
