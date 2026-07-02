<p align="center">
  <img src="assets/icon-256.png" width="120" alt="traefik" />
</p>

# traefik

Traefik is a cloud-native reverse proxy and ingress controller with automatic service discovery.

A first-party [orca](https://github.com/argyle-labs/orca) plugin (service-backend).

This repo is **self-contained** — the steps below run traefik **by hand, without orca**. orca automates exactly this (same image, ports, and data) through one generic surface.

---

## Run it without orca

### Docker Compose

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

### Other runtimes

**Podman** — the compose above works with `podman compose up -d`, or run it directly:

```sh
podman run -d --name traefik --restart unless-stopped \
    -p 80:80/tcp \
    -p 443:443/tcp \
    -p 8080:8080/tcp \
    -v /var/run/docker.sock:/var/run/docker.sock:ro \
    -v ./traefik.yml:/etc/traefik/traefik.yml \
    -v ./data:/data \
    traefik:v3
```

**LXC** — on a container-capable LXC (e.g. a Proxmox LXC with nesting enabled) run the same image via Docker/Podman as above, or install traefik from upstream directly on the guest: <https://traefik.io/>.

**VM** — install traefik from upstream (<https://traefik.io/>) or run the same container image inside the VM; expose port `80`.

**Unraid** — add via *Community Applications*, or *Docker → Add Container* with image `traefik:v3`, port `80`, and the volume paths above.

### Dependencies

Requires access to the Docker socket (or another provider) for service discovery.

### Ports & data

| | |
|---|---|
| Default port | `8080` |
| Upstream | <https://traefik.io/> |


### Backup & restore

Back up the config/data volume(s) above — that's the whole service state (stop the container first for a clean copy). Restore by putting them back and starting it.

> With orca this is **`service.backup` / `service.restore`** — location-agnostic (docker / podman / lxc / vm), one command regardless of where traefik runs. No per-service backup script.

## With orca

orca drives this plugin through the single generic `service.*` surface — no per-plugin tools:

```sh
orca service.deploy traefik      # render + launch on any supported runtime
orca service.status traefik      # health + rich diagnostics (typed payload)
orca service.backup traefik      # location-agnostic backup (tar; PBS on Proxmox)
orca service.configure traefik   # apply config via the upstream API
```

## Layout

- `src/` — the plugin (pure Rust): the `ServiceBackend` descriptor + `configure` / `status`.
- [CAPABILITIES.md](CAPABILITIES.md) — the service-backend contract checklist.
- `assets/` — plugin icon.
