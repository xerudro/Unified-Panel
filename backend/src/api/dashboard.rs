use axum::{
    response::{Html, IntoResponse},
    Extension,
};
use serde_json::json;

/// Get dashboard stats (for HTMX auto-refresh)
pub async fn get_stats() -> impl IntoResponse {
    let html = r#"
        <!-- Stat Card 1 - Servers -->
        <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm hover:shadow-md transition-shadow animate-fade-in">
            <div class="flex items-center justify-between mb-4">
                <div class="w-12 h-12 bg-blue-100 dark:bg-blue-900/30 rounded-lg flex items-center justify-center">
                    <i data-lucide="server" class="w-6 h-6 text-blue-600 dark:text-blue-400"></i>
                </div>
                <span class="text-xs font-medium text-green-600 dark:text-green-400 bg-green-100 dark:bg-green-900/30 px-2 py-1 rounded-full">
                    +12%
                </span>
            </div>
            <h3 class="text-2xl font-bold mb-1">12</h3>
            <p class="text-sm text-gray-600 dark:text-gray-400">Total Servers</p>
        </div>

        <!-- Stat Card 2 - Websites -->
        <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm hover:shadow-md transition-shadow animate-fade-in" style="animation-delay: 0.1s">
            <div class="flex items-center justify-between mb-4">
                <div class="w-12 h-12 bg-green-100 dark:bg-green-900/30 rounded-lg flex items-center justify-center">
                    <i data-lucide="globe" class="w-6 h-6 text-green-600 dark:text-green-400"></i>
                </div>
                <span class="text-xs font-medium text-green-600 dark:text-green-400 bg-green-100 dark:bg-green-900/30 px-2 py-1 rounded-full">
                    +8%
                </span>
            </div>
            <h3 class="text-2xl font-bold mb-1">45</h3>
            <p class="text-sm text-gray-600 dark:text-gray-400">Active Websites</p>
        </div>

        <!-- Stat Card 3 - Users -->
        <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm hover:shadow-md transition-shadow animate-fade-in" style="animation-delay: 0.2s">
            <div class="flex items-center justify-between mb-4">
                <div class="w-12 h-12 bg-purple-100 dark:bg-purple-900/30 rounded-lg flex items-center justify-center">
                    <i data-lucide="users" class="w-6 h-6 text-purple-600 dark:text-purple-400"></i>
                </div>
                <span class="text-xs font-medium text-green-600 dark:text-green-400 bg-green-100 dark:bg-green-900/30 px-2 py-1 rounded-full">
                    +24%
                </span>
            </div>
            <h3 class="text-2xl font-bold mb-1">156</h3>
            <p class="text-sm text-gray-600 dark:text-gray-400">Users</p>
        </div>

        <!-- Stat Card 4 - Uptime -->
        <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm hover:shadow-md transition-shadow animate-fade-in" style="animation-delay: 0.3s">
            <div class="flex items-center justify-between mb-4">
                <div class="w-12 h-12 bg-orange-100 dark:bg-orange-900/30 rounded-lg flex items-center justify-center">
                    <i data-lucide="activity" class="w-6 h-6 text-orange-600 dark:text-orange-400"></i>
                </div>
                <span class="text-xs font-medium text-green-600 dark:text-green-400 bg-green-100 dark:bg-green-900/30 px-2 py-1 rounded-full">
                    98.9%
                </span>
            </div>
            <h3 class="text-2xl font-bold mb-1">Online</h3>
            <p class="text-sm text-gray-600 dark:text-gray-400">Uptime</p>
        </div>
    "#;
    Html(html)
}

/// Get recent activity (for HTMX auto-refresh)
pub async fn get_activity() -> impl IntoResponse {
    let html = r#"
        <div class="flex items-start space-x-3">
            <div class="w-2 h-2 bg-green-500 rounded-full mt-2"></div>
            <div class="flex-1">
                <p class="text-sm font-medium">Server deployed successfully</p>
                <p class="text-xs text-gray-500 dark:text-gray-400">web-prod-01 • 5 minutes ago</p>
            </div>
        </div>
        <div class="flex items-start space-x-3">
            <div class="w-2 h-2 bg-blue-500 rounded-full mt-2"></div>
            <div class="flex-1">
                <p class="text-sm font-medium">New user registered</p>
                <p class="text-xs text-gray-500 dark:text-gray-400">john@example.com • 15 minutes ago</p>
            </div>
        </div>
        <div class="flex items-start space-x-3">
            <div class="w-2 h-2 bg-yellow-500 rounded-full mt-2"></div>
            <div class="flex-1">
                <p class="text-sm font-medium">SSL certificate expiring soon</p>
                <p class="text-xs text-gray-500 dark:text-gray-400">example.com • 1 hour ago</p>
            </div>
        </div>
    "#;
    Html(html)
}

/// Get system health (for HTMX auto-refresh)
pub async fn get_health() -> impl IntoResponse {
    let html = r#"
        <div>
            <div class="flex items-center justify-between mb-2">
                <span class="text-sm font-medium">CPU Usage</span>
                <span class="text-sm text-gray-600 dark:text-gray-400">45%</span>
            </div>
            <div class="h-2 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
                <div class="h-full bg-gradient-to-r from-blue-500 to-blue-600 rounded-full transition-all duration-500" style="width: 45%"></div>
            </div>
        </div>
        <div>
            <div class="flex items-center justify-between mb-2">
                <span class="text-sm font-medium">Memory</span>
                <span class="text-sm text-gray-600 dark:text-gray-400">62%</span>
            </div>
            <div class="h-2 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
                <div class="h-full bg-gradient-to-r from-green-500 to-green-600 rounded-full transition-all duration-500" style="width: 62%"></div>
            </div>
        </div>
        <div>
            <div class="flex items-center justify-between mb-2">
                <span class="text-sm font-medium">Disk Space</span>
                <span class="text-sm text-gray-600 dark:text-gray-400">38%</span>
            </div>
            <div class="h-2 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
                <div class="h-full bg-gradient-to-r from-purple-500 to-purple-600 rounded-full transition-all duration-500" style="width: 38%"></div>
            </div>
        </div>
    "#;
    Html(html)
}

/// Get server overview (for HTMX auto-refresh)
pub async fn get_servers() -> impl IntoResponse {
    let html = r#"
        <!-- Server Card 1 -->
        <div class="p-4 bg-gray-50 dark:bg-gray-900 rounded-lg hover:shadow-md transition-shadow">
            <div class="flex items-center justify-between mb-3">
                <div class="flex items-center space-x-2">
                    <div class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                    <span class="font-medium">web-prod-01</span>
                </div>
                <span class="text-xs bg-green-100 dark:bg-green-900/30 text-green-600 dark:text-green-400 px-2 py-1 rounded-full">
                    Active
                </span>
            </div>
            <div class="space-y-2 text-sm">
                <div class="flex justify-between">
                    <span class="text-gray-600 dark:text-gray-400">CPU</span>
                    <span class="font-medium">45%</span>
                </div>
                <div class="flex justify-between">
                    <span class="text-gray-600 dark:text-gray-400">Memory</span>
                    <span class="font-medium">62%</span>
                </div>
                <div class="flex justify-between">
                    <span class="text-gray-600 dark:text-gray-400">Location</span>
                    <span class="font-medium">US-East</span>
                </div>
            </div>
        </div>

        <!-- Server Card 2 -->
        <div class="p-4 bg-gray-50 dark:bg-gray-900 rounded-lg hover:shadow-md transition-shadow">
            <div class="flex items-center justify-between mb-3">
                <div class="flex items-center space-x-2">
                    <div class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                    <span class="font-medium">web-prod-02</span>
                </div>
                <span class="text-xs bg-green-100 dark:bg-green-900/30 text-green-600 dark:text-green-400 px-2 py-1 rounded-full">
                    Active
                </span>
            </div>
            <div class="space-y-2 text-sm">
                <div class="flex justify-between">
                    <span class="text-gray-600 dark:text-gray-400">CPU</span>
                    <span class="font-medium">38%</span>
                </div>
                <div class="flex justify-between">
                    <span class="text-gray-600 dark:text-gray-400">Memory</span>
                    <span class="font-medium">54%</span>
                </div>
                <div class="flex justify-between">
                    <span class="text-gray-600 dark:text-gray-400">Location</span>
                    <span class="font-medium">US-West</span>
                </div>
            </div>
        </div>

        <!-- Server Card 3 -->
        <div class="p-4 bg-gray-50 dark:bg-gray-900 rounded-lg hover:shadow-md transition-shadow">
            <div class="flex items-center justify-between mb-3">
                <div class="flex items-center space-x-2">
                    <div class="w-2 h-2 bg-yellow-500 rounded-full"></div>
                    <span class="font-medium">db-prod-01</span>
                </div>
                <span class="text-xs bg-yellow-100 dark:bg-yellow-900/30 text-yellow-600 dark:text-yellow-400 px-2 py-1 rounded-full">
                    Maintenance
                </span>
            </div>
            <div class="space-y-2 text-sm">
                <div class="flex justify-between">
                    <span class="text-gray-600 dark:text-gray-400">CPU</span>
                    <span class="font-medium">12%</span>
                </div>
                <div class="flex justify-between">
                    <span class="text-gray-600 dark:text-gray-400">Memory</span>
                    <span class="font-medium">28%</span>
                </div>
                <div class="flex justify-between">
                    <span class="text-gray-600 dark:text-gray-400">Location</span>
                    <span class="font-medium">EU-Central</span>
                </div>
            </div>
        </div>
    "#;
    Html(html)
}

/// Get hosting activity (for HTMX auto-refresh)
pub async fn get_hosting_activity() -> impl IntoResponse {
    let html = r#"
        <div class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg">
            <div class="flex items-center space-x-3">
                <div class="w-8 h-8 bg-green-100 dark:bg-green-900/30 rounded-full flex items-center justify-center">
                    <i data-lucide="check" class="w-4 h-4 text-green-600 dark:text-green-400"></i>
                </div>
                <div>
                    <p class="text-sm font-medium">SSL certificate installed</p>
                    <p class="text-xs text-gray-500 dark:text-gray-400">example.com</p>
                </div>
            </div>
            <span class="text-xs text-gray-500 dark:text-gray-400">2 min ago</span>
        </div>
        <div class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg">
            <div class="flex items-center space-x-3">
                <div class="w-8 h-8 bg-blue-100 dark:bg-blue-900/30 rounded-full flex items-center justify-center">
                    <i data-lucide="upload" class="w-4 h-4 text-blue-600 dark:text-blue-400"></i>
                </div>
                <div>
                    <p class="text-sm font-medium">Backup completed</p>
                    <p class="text-xs text-gray-500 dark:text-gray-400">mywebsite.com</p>
                </div>
            </div>
            <span class="text-xs text-gray-500 dark:text-gray-400">15 min ago</span>
        </div>
        <div class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-lg">
            <div class="flex items-center space-x-3">
                <div class="w-8 h-8 bg-purple-100 dark:bg-purple-900/30 rounded-full flex items-center justify-center">
                    <i data-lucide="database" class="w-4 h-4 text-purple-600 dark:text-purple-400"></i>
                </div>
                <div>
                    <p class="text-sm font-medium">Database created</p>
                    <p class="text-xs text-gray-500 dark:text-gray-400">shop_db_prod</p>
                </div>
            </div>
            <span class="text-xs text-gray-500 dark:text-gray-400">1 hour ago</span>
        </div>
    "#;
    Html(html)
}

/// Get hosting storage details (for HTMX auto-refresh)
pub async fn get_hosting_storage() -> impl IntoResponse {
    let html = r#"
        <div>
            <div class="flex items-center justify-between mb-2">
                <span class="text-sm font-medium">Total Storage</span>
                <span class="text-sm text-gray-600 dark:text-gray-400">245 GB / 500 GB</span>
            </div>
            <div class="h-2 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
                <div class="h-full bg-gradient-to-r from-blue-500 to-blue-600 rounded-full transition-all duration-500" style="width: 49%"></div>
            </div>
        </div>
        <div>
            <div class="flex items-center justify-between mb-2">
                <span class="text-sm font-medium">Databases</span>
                <span class="text-sm text-gray-600 dark:text-gray-400">82 GB</span>
            </div>
            <div class="h-2 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
                <div class="h-full bg-gradient-to-r from-purple-500 to-purple-600 rounded-full transition-all duration-500" style="width: 16%"></div>
            </div>
        </div>
        <div>
            <div class="flex items-center justify-between mb-2">
                <span class="text-sm font-medium">Emails</span>
                <span class="text-sm text-gray-600 dark:text-gray-400">38 GB</span>
            </div>
            <div class="h-2 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
                <div class="h-full bg-gradient-to-r from-green-500 to-green-600 rounded-full transition-all duration-500" style="width: 8%"></div>
            </div>
        </div>
        <div>
            <div class="flex items-center justify-between mb-2">
                <span class="text-sm font-medium">Backups</span>
                <span class="text-sm text-gray-600 dark:text-gray-400">125 GB</span>
            </div>
            <div class="h-2 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
                <div class="h-full bg-gradient-to-r from-orange-500 to-orange-600 rounded-full transition-all duration-500" style="width: 25%"></div>
            </div>
        </div>
    "#;
    Html(html)
}
