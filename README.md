# Live Gateway

A POC (proof of concept) system to modernize a front-end project with live updates from various programs while minimizing changes to those source codes.

There are no intended changes to be made to this POC. It has served its purpose.

## What is it

This program listens to UDP messages on one thread and handles a websocket server on another thread. Any message arriving on the specified UDP port will be broadcast to the WebSocket clients. Any message arriving on the websocket will be received but ignored.

## How to run it

```console
$ ./live_gateway --udp_port 3030 --websocket_port 3031
```

## Examples

### WebSocket Client

HTML
```html
<!DOCTYPE html>
<html lang="en">
    <head>
        <title>Live Gateway</title>
    </head>
    <body>
        <h1>Live Gateway</h1>
        <div id="log">
            <p><em>Connecting...</em></p>
        </div>
        <input type="text" id="text" />
        <button type="button" id="send">send</button>
        <script type="text/javascript">
        const chat = document.getElementById('log');
        const text = document.getElementById('text');
        const uri = 'ws://127.0.0.1:3031/listen';
        const ws = new WebSocket(uri);
        function message(data) {
            const line = document.createElement('p');
            line.innerText = data;
            chat.appendChild(line);
        }
        ws.onopen = function() {
            chat.innerHTML = '<p><em>Connected!</em></p>';
        };
        ws.onmessage = function(msg) {
            message('<Server>: ' + msg.data);
        };
        ws.onclose = function() {
            chat.getElementsByTagName('em')[0].innerText = 'Disconnected!';
        };
        send.onclick = function() {
            const msg = text.value;
            ws.send(msg);
            text.value = '';
            message('<You>: ' + msg);
        };
        </script>
    </body>
</html>
```

### UDP Clients

Bash
```console
$ nc -u 127.0.0.1 3030
Hello World
```

PHP
```php
<?php
    $sock = socket_create(AF_INET, SOCK_DGRAM, SOL_UDP);

    $msg = "Hello World";
    $len = strlen($msg);

    socket_sendto($sock, $msg, $len, 0, '127.0.0.1', 3030);
    socket_close($sock);
?>
```

Ruby
```ruby
require 'socket'

sock = UDPSocket.new
sock.connect("127.0.0.1", 3030)
sock.send "Hello World", 0
```

Python
```python
import socket

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.connect(("127.0.0.1", 3030))
sock.send(b"Hello World")
```
