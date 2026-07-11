# Traefik

Reverse proxy for routing traffic to Docker services across one or more hosts.

> This document describes a Traefik deployment pattern. In many homelabs Traefik is superseded by Caddy — keep this only if Traefik is your live reverse proxy.

## How It Works

Traefik runs on a single host alongside a Tailscale container. Services on the same host are discovered automatically via Docker labels. Services on other hosts are routed using the file provider (`dynamic-external-services.yml`).

```
                     Tailscale (HTTPS :443)
                            |
                            v
                      Traefik (:80)
                            |
            +---------------+---------------+
            v               v               v
      Docker labels    Docker labels    File provider
      (local host)     (local host)    (remote hosts)
      +----------+    +----------+    +--------------+
      | Jellyfin |    |  Sonarr  |    | Host B :8096 |
      +----------+    +----------+    +--------------+
```

## Deploy

```bash
cd compose/traefik
docker compose up -d
```

### Required Environment Variables

| Variable             | Source          | Description                          |
| -------------------- | --------------- | ------------------------------------ |
| `TAILSCALE_AUTHKEY`  | secret manager  | Tailscale auth key (`tskey-...`)     |
| `TRAEFIK_IMAGE_TAG`  | `.envrc`        | Traefik image tag (default: `latest`) |
| `TRAEFIK_LOG_LEVEL`  | `.envrc`        | Log level: DEBUG, INFO, WARN, ERROR  |
| `TAILSCALE_HOSTNAME` | `.envrc`        | Tailscale machine name (default: `traefik`) |

## Access

- **Dashboard**: `https://<tailscale-hostname>.your-tailnet.ts.net/dashboard/`
- **Services** (standalone, path-based): `https://<tailscale-hostname>.your-tailnet.ts.net/service-name/`
- **Services** (swarm, host-based): `https://service.<domain>`

## Adding a Local Service

Services running on the same Docker host as Traefik are discovered automatically via labels. Add these to your service's compose file:

```yaml
services:
  myservice:
    image: myimage:latest
    networks:
      - traefik-network
    labels:
      - 'traefik.enable=true'
      - 'traefik.http.routers.myservice.rule=PathPrefix(`/myservice`)'
      - 'traefik.http.routers.myservice.entrypoints=web'
      - 'traefik.http.services.myservice.loadbalancer.server.port=8080'
      # Strip the path prefix before forwarding
      - 'traefik.http.middlewares.myservice-strip.stripprefix.prefixes=/myservice'
      - 'traefik.http.routers.myservice.middlewares=myservice-strip'

networks:
  traefik-network:
    external: true
```

The service must be on the `traefik-network` bridge network.

## Adding a Remote Service (Another Host)

For services running on a different host, add an entry to `dynamic-external-services.yml`. Traefik watches this file and picks up changes automatically.

```yaml
http:
  routers:
    myservice:
      rule: "Host(`myservice.<domain>`)"
      entryPoints:
        - websecure
      service: myservice
      tls:
        certResolver: cloudflare

  services:
    myservice:
      loadBalancer:
        servers:
          - url: "http://<ip>:<port>"
        passHostHeader: true
```

This routes `myservice.<domain>` to the service running on another host at `<ip>:<port>`. No Traefik installation needed on the remote host.

### DNS for Remote Services

For host-based routing (`Host(...)`) to work, DNS must resolve the domain to the Traefik host:

**Option A: Wildcard DNS** (simplest)
- Point `*.<domain>` to the Traefik host IP in your DNS provider

**Option B: Split DNS** (for local + Tailscale access)
- Configure your local DNS (AdGuard Home, Pi-hole, etc.):
  - Local clients: `*.<domain>` -> `<ip>` (Traefik host local IP)
  - Tailscale clients: `*.<domain>` -> `100.x.x.x` (Traefik host Tailscale IP)

## Swarm Mode (Alternative)

For Docker Swarm deployments with Let's Encrypt certificates via Cloudflare DNS challenge.

### Additional Environment Variables

| Variable           | Source          | Description                  |
| ------------------ | --------------- | ---------------------------- |
| `ACME_EMAIL`       | secret manager  | Email for Let's Encrypt      |
| `CF_API_EMAIL`     | secret manager  | Cloudflare account email     |
| `CF_DNS_API_TOKEN` | secret manager  | Cloudflare DNS API token     |
| `DOMAIN`           | `.envrc`        | Base domain (e.g., `example.com`) |

### Prerequisites

```bash
# Create overlay network
docker network create --driver overlay --attachable traefik-network

# Install Tailscale on the Docker host (for remote access)
curl -fsSL https://tailscale.com/install.sh | sh
sudo tailscale up
```

### Deploy

```bash
docker stack deploy -c docker-compose.swarm.yml traefik
```

### Service Labels (Swarm)

```yaml
services:
  myservice:
    image: myimage:latest
    networks:
      - traefik-network
    deploy:
      labels:
        - 'traefik.enable=true'
        - 'traefik.http.routers.myservice.rule=Host(`myservice.<domain>`)'
        - 'traefik.http.routers.myservice.entrypoints=websecure'
        - 'traefik.http.routers.myservice.tls.certresolver=cloudflare'
        - 'traefik.http.services.myservice.loadbalancer.server.port=8080'
```

## Files

| File                              | Purpose                                   |
| --------------------------------- | ----------------------------------------- |
| `docker-compose.yml`              | Standalone mode (Tailscale Serve)         |
| `tailscale-serve.json`            | Tailscale Serve configuration             |
| `dynamic-standalone.yml`          | Path-based routing for standalone mode    |
| `dynamic-dashboard.yml`           | Dashboard routing for swarm mode          |
| `dynamic-external-services.yml`   | Routes to services on other hosts         |

## Troubleshooting

### Can't Access via Tailscale

```bash
docker compose exec tailscale tailscale status
docker compose exec tailscale tailscale ip -4
docker compose logs traefik
```

### Service Not Appearing

```bash
# Verify the service is on traefik-network
docker inspect <container> | grep -A 5 Networks

# Check Traefik logs for errors
docker compose logs traefik | grep -i error
```

### Certificate Issues (Swarm Mode)

```bash
# Test Cloudflare token
curl -X GET "https://api.cloudflare.com/client/v4/user/tokens/verify" \
  -H "Authorization: Bearer $CF_DNS_API_TOKEN"
```
