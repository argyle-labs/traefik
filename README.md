<p align="center">
  <img src="assets/icon-256.png" width="120" alt="traefik" />
</p>

# traefik

Traefik is a cloud-native reverse proxy and ingress controller with automatic service discovery.

A first-party [orca](https://github.com/argyle-labs/orca) plugin (service-backend).

This repo is **self-contained** — the steps below run traefik **by hand, without orca**. orca automates exactly this (same image, ports, and data) through one generic surface.

---

## Run it without orca

### Docker / Podman

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

Podman: the same file with `podman-compose up -d`.

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
