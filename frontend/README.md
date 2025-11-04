# Frontend Development

This directory contains the frontend assets for the Unified Hosting Panel.

## Tech Stack

- **Tailwind CSS** - Utility-first CSS framework
- **HTMX** - HTML-over-the-wire for interactivity
- **Alpine.js** - Minimal JavaScript framework for UI interactions
- **Lucide Icons** - Beautiful SVG icons

## Directory Structure

```
frontend/
├── src/
│   ├── styles.css      # Main Tailwind CSS file with custom styles
│   └── custom.js       # Minimal custom JavaScript
├── package.json        # Node dependencies
├── tailwind.config.js  # Tailwind configuration
├── postcss.config.js   # PostCSS configuration
└── README.md          # This file
```

## Development

### Prerequisites

- Node.js 18+ and npm

### Installation

```bash
# Install dependencies
npm install
```

### Build CSS

```bash
# Build CSS once
npm run build

# Watch for changes (development)
npm run dev
```

The compiled CSS will be output to `../backend/static/css/main.css`.

## Customization

### Colors

Edit `tailwind.config.js` to customize the color palette:

```js
theme: {
  extend: {
    colors: {
      primary: { ... },
      secondary: { ... },
    }
  }
}
```

### Custom Components

Add custom component styles in `src/styles.css` under the `@layer components` section:

```css
@layer components {
  .my-component {
    @apply bg-white dark:bg-gray-800 rounded-lg;
  }
}
```

### Custom Utilities

Add custom utility classes under the `@layer utilities` section:

```css
@layer utilities {
  .my-utility {
    /* custom styles */
  }
}
```

## Tailwind Plugins

The following Tailwind plugins are included:

- **@tailwindcss/forms** - Better form styles
- **@tailwindcss/typography** - Beautiful typographic defaults

## JavaScript

Minimal JavaScript is used. Most interactivity is handled by:

- **HTMX** for server interactions
- **Alpine.js** for client-side UI state

Custom JavaScript is in `src/custom.js` for:
- Theme management
- Toast notifications
- Clipboard utilities
- Icon initialization

## Production Build

For production, the CSS is automatically minified:

```bash
npm run build
```

## Integration with Backend

The compiled CSS is automatically placed in `backend/static/css/main.css` and referenced in the templates:

```html
<link rel="stylesheet" href="/static/css/main.css">
```

## Dark Mode

Dark mode is implemented using Tailwind's `dark:` variant with class-based strategy. The theme is managed by Alpine.js and persisted in localStorage.

## Custom Styles Included

- Glass morphism effects
- Gradient backgrounds
- Button variants (primary, secondary, danger, success, outline)
- Badge styles with status colors
- Input styles with error states
- Loading spinners
- Modal backdrops
- Navigation active states
- Custom scrollbars
- Progress bars
- Tooltips
- Status indicators
- HTMX loading states

## Hot Reload

During development, run `npm run dev` to watch for changes and automatically rebuild CSS.

In a separate terminal, run the Rust backend with `cargo watch -x run` for full hot reload experience.
