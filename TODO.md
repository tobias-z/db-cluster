# To-do of DB Cluster

## Daemon

- [] Implement good logging
- [] Handle admin vs worker
- [] Handle initializing the server
  - [] What kind of config is required?
  - [] Set the running server to be an Admin server
- [] How do we handle networking? DNS of some kind because of multiple servers
  - [] Do we have to run a wrapper container like a pod?
  - [] Do we have to build an image which looks like the one people asked for, but containing things like DNS config, and interceptors?
- [] Authentication of some kind.
