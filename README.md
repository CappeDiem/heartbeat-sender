## Heartbeat sender
Created as a way to monitor servers that are not pingable to Uptime Kuma instance using push type monitor. 
(*There would have been a better way to do this but wanted to do something in rust so did this*)

## Environment variables
``HEARTBEAT_URL`` URL that the application uses for a get request

``HEARTBEAT_INTERVAL`` Interval in millis on which the application sends the requests

```
HEARTBEAT_URL="https://example.com/heartbeat"
HEARTBEAT_INTERVAL=10000
```
