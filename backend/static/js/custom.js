/**
 * Custom JavaScript for Unified Hosting Panel
 * Minimal client-side enhancements
 */

// Theme Management
const ThemeManager = {
  init() {
    // Initialize theme from localStorage or system preference
    const savedTheme = localStorage.getItem('darkMode');
    const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;

    if (savedTheme === 'true' || (!savedTheme && systemPrefersDark)) {
      document.documentElement.classList.add('dark');
    }

    // Listen for system theme changes
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
      if (localStorage.getItem('darkMode') === null) {
        if (e.matches) {
          document.documentElement.classList.add('dark');
        } else {
          document.documentElement.classList.remove('dark');
        }
      }
    });
  }
};

// VPS Status Colors
const VPSStatus = {
  getColor(status) {
    const colors = {
      'running': 'bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400',
      'stopped': 'bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400',
      'starting': 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-400',
      'stopping': 'bg-orange-100 text-orange-800 dark:bg-orange-900/30 dark:text-orange-400',
      'deleting': 'bg-gray-100 text-gray-800 dark:bg-gray-900/30 dark:text-gray-400',
      'error': 'bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400',
    };
    return colors[status] || 'bg-gray-100 text-gray-800 dark:bg-gray-900/30 dark:text-gray-400';
  }
};

// Copy to Clipboard
const ClipboardManager = {
  copy(text, button) {
    navigator.clipboard.writeText(text).then(() => {
      // Show feedback
      const originalText = button.innerHTML;
      button.innerHTML = '<i data-lucide="check" class="w-4 h-4"></i>';
      lucide.createIcons();

      setTimeout(() => {
        button.innerHTML = originalText;
        lucide.createIcons();
      }, 2000);
    });
  }
};

// Toast Notifications
const Toast = {
  show(message, type = 'info') {
    const toast = document.createElement('div');
    const colors = {
      success: 'bg-green-600',
      error: 'bg-red-600',
      warning: 'bg-yellow-600',
      info: 'bg-blue-600',
    };

    toast.className = `fixed bottom-4 right-4 ${colors[type]} text-white px-6 py-3 rounded-lg shadow-lg z-50 animate-slide-in-right`;
    toast.textContent = message;

    document.body.appendChild(toast);

    setTimeout(() => {
      toast.classList.add('opacity-0', 'transition-opacity', 'duration-300');
      setTimeout(() => toast.remove(), 300);
    }, 3000);
  }
};

// Initialize on DOM ready
document.addEventListener('DOMContentLoaded', () => {
  ThemeManager.init();

  // Initialize Lucide icons
  if (typeof lucide !== 'undefined') {
    lucide.createIcons();
  }
});

// Re-initialize icons after HTMX updates
document.body.addEventListener('htmx:afterSwap', () => {
  if (typeof lucide !== 'undefined') {
    lucide.createIcons();
  }
});

// Export for global use
window.ThemeManager = ThemeManager;
window.VPSStatus = VPSStatus;
window.ClipboardManager = ClipboardManager;
window.Toast = Toast;
