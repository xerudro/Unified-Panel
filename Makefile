.PHONY: help install dev build clean frontend-install frontend-dev frontend-build backend-dev backend-build

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-20s %s\n", $$1, $$2}'

install: frontend-install ## Install all dependencies
	@echo "✓ All dependencies installed"

frontend-install: ## Install frontend dependencies
	@echo "Installing frontend dependencies..."
	@cd frontend && npm install

frontend-dev: ## Watch and compile CSS for development
	@echo "Starting frontend development mode..."
	@cd frontend && npm run dev

frontend-build: ## Build production CSS
	@echo "Building frontend assets..."
	@cd frontend && npm run build

backend-dev: ## Run backend in development mode
	@echo "Starting backend development server..."
	@cd backend && cargo watch -x run

backend-build: ## Build backend in release mode
	@echo "Building backend for production..."
	@cd backend && cargo build --release

dev: ## Run full development environment (requires tmux or run in separate terminals)
	@echo "Starting development environment..."
	@echo "Run 'make frontend-dev' in one terminal and 'make backend-dev' in another"

build: frontend-build backend-build ## Build everything for production

clean: ## Clean build artifacts
	@echo "Cleaning build artifacts..."
	@cd backend && cargo clean
	@rm -rf frontend/node_modules
	@rm -f backend/static/css/main.css

test: ## Run tests
	@echo "Running tests..."
	@cd backend && cargo test

lint: ## Run linters
	@echo "Running linters..."
	@cd backend && cargo clippy

format: ## Format code
	@echo "Formatting code..."
	@cd backend && cargo fmt
