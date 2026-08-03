# Traefik — operator notes

Traefik is a cloud-native reverse proxy and ingress controller with automatic
service discovery. It routes inbound HTTP/HTTPS traffic to backend services,
discovering them from a provider (most commonly the Docker socket) so routes
appear and disappear as containers start and stop.

These notes cover running the container standalone, matching the compose
example in the [README](../README.md). orca automates the same image, ports,
and data through its generic `service.*` surface.

## Run it

```yaml
# compose.yml
services:
  traefik:
    image: traefik:v3
    container_name: traefik
    restart: unless-stopped
    ports:
      - "80:80/tcp"
      - "443:443/tcp"
      - "8080:8080/tcp"   # dashboard (enable explicitly)
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
      - ./traefik.yml:/etc/traefik/traefik.yml
      - ./data:/data
```

```sh
docker compose up -d
```

The same image runs under Podman, in an LXC guest with nesting, or in a VM —
see the README for per-runtime invocations.

## Ports

| Port  | Purpose                                    |
| ----- | ------------------------------------------ |
| `80`  | HTTP entrypoint                            |
| `443` | HTTPS entrypoint                           |
| `8080`| Dashboard / API (enable explicitly)        |

## Providers & service discovery

Traefik discovers backends from a provider. The Docker provider reads the
mounted Docker socket and picks up containers automatically via labels; the
file provider reads static routing definitions from a watched file. Enable
whichever providers you need in `traefik.yml`.

### Routing a container via Docker labels

Add labels to the service you want exposed. It must share a Docker network
with Traefik:

```yaml
services:
  myservice:
    image: myimage:latest
    labels:
      - 'traefik.enable=true'
      - 'traefik.http.routers.myservice.rule=PathPrefix(`/myservice`)'
      - 'traefik.http.routers.myservice.entrypoints=web'
      - 'traefik.http.services.myservice.loadbalancer.server.port=8080'
```

## Backup & restore

The config file and the `./data` volume are the whole service state. Stop the
container for a clean copy, back up those paths, and restore by putting them
back and starting the container again. With orca this is `service.backup` /
`service.restore`, location-agnostic across docker / podman / lxc / vm.

## Troubleshooting

```sh
# Follow logs
docker compose logs -f traefik

# Confirm a container is on Traefik's network and labelled
docker inspect <container> | grep -A5 Networks
docker compose logs traefik | grep -i error
```

Dashboard not reachable: confirm port `8080` is published and the dashboard is
enabled in `traefik.yml`. Service not routed: confirm `traefik.enable=true`,
the shared network, and that the entrypoint and port in the labels are correct.
</content>
</invoke>
