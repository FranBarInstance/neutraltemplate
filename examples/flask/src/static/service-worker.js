/* Neutral TS - License in the terms described in the LICENSE file. */

self.addEventListener('install', function(event) {
    var offlineRequest = new Request('/pwa/offline.html');
    event.waitUntil(
        fetch(offlineRequest).then(function(response) {
            return caches.open('offline').then(function(cache) {
                return cache.put(offlineRequest, response);
            });
        })
    );
});

self.addEventListener('fetch', function(event) {
    var request = event.request;
    if (request.method === 'GET') {
        event.respondWith(
            fetch(request).catch(function(error) {
                return caches.open('offline').then(function(cache) {
                    return cache.match('/pwa/offline.html');
                });
            })
        );
    }
});

self.addEventListener('push', function (event) {
    if (!(self.Notification && self.Notification.permission === 'granted')) {
        return;
    }
    if (event.data) {
        var json  = event.data.json();
        var image = '/pwa/neutral.jpg';
        var logo  = '/pwa/logo.png';
        if (typeof json.image !== 'undefined') {
            image = json.image;
        }
        if (typeof json.logo !== 'undefined') {
            logo = json.icon;
        }
        if (json !== null) {
            self.registration.showNotification(json.title, {
                body: json.message,
                icon: logo,
                image: image,
                data: json.data,
                actions: json.actions
            });
        }
    }
});

self.addEventListener('notificationclick', function(event) {
    event.notification.close();
    event.waitUntil(
        clients.matchAll({
            type: "window"
        })
        .then(function(clientList) {
            for (var i = 0; i < clientList.length; i++) {
                var client = clientList[i];
                if (client.url == '/' && 'focus' in client) return client.focus();
            }
            if (clients.openWindow) {
                return clients.openWindow(event.notification.data.url);
            }
        })
    );
});
