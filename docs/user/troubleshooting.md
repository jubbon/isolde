# Troubleshooting

Common issues and solutions when using Isolde.

## Docker Issues

### Docker daemon not running
**Problem:** Error messages about Docker not being available.

**Solution:** Start Docker Desktop or the Docker daemon.
```bash
# Check if Docker is running
docker ps

# Start Docker Desktop or the Docker daemon
# On Linux with systemd:
sudo systemctl start docker
sudo systemctl enable docker
```

### Permission denied
**Problem:** "permission denied" when running Docker commands or accessing Docker socket inside container.

**Solution:** Add your user to the docker group.
```bash
# Add user to docker group on host
sudo usermod -aG docker $USER

# Log out and back in for group change to take effect
```

### Docker build fails
**Problem:** Build errors during container creation, often related to base images or network issues.

**Solution:**
1. Check Docker has sufficient resources (memory, disk space)
2. Verify network connectivity for pulling base images
3. Try pulling the base image manually: `docker pull mcr.microsoft.com/devcontainers/base:ubuntu`
4. For proxy issues, see [Proxy Configuration](../devcontainer/proxy.md)

## Devcontainer Issues

### Container fails to build
**Problem:** Build errors during container creation.

**Solution:** Check the build logs for specific errors. Common causes:
- **Invalid language version** - Verify the version is supported by the template
- **Missing dependencies** - Check template requirements in `templates/{name}/template-info.yaml`
- **Network issues** - Try [proxy configuration](../devcontainer/proxy.md) if behind a firewall
- **Syntax errors** - Validate `devcontainer.json`: `cat .devcontainer/devcontainer.json | jq`

### Container won't start
**Problem:** Build fails or container exits immediately after starting.

**Solution:**
1. Check Docker is running: `docker ps`
2. Verify `devcontainer.json` syntax: `cat .devcontainer/devcontainer.json | jq`
3. Review build logs in VS Code Output panel
4. Check for port conflicts with existing containers: `docker ps -a`

### VS Code doesn't connect
**Problem:** Container builds but VS Code won't reconnect or prompts repeatedly.

**Solution:**
1. Press `F1` → "Dev Containers: Rebuild Container"
2. Check Docker is running: `docker ps`
3. Verify devcontainer.json syntax
4. Check VS Code Dev Containers extension is installed and updated

### Setup wizard runs repeatedly
**Problem:** After rebuilding the Dev Container, Claude Code's setup wizard runs repeatedly, asking about color scheme, file system trust, and other initial configuration.

**Solution:** Create a persistent machine-id on your host machine:
```bash
# On host machine (not in container)
mkdir -p ~/.config/devcontainer
uuidgen > ~/.config/devcontainer/machine-id
```

The `devcontainer.json` already includes the mount configuration for this file. Rebuild the container after creating it.

### VS Code extensions missing
**Problem:** Expected extensions not installed in the container.

**Solution:**
1. Check `.devcontainer/devcontainer.json` extensions list
2. Manually install in container: Press `Ctrl+Shift+X`
3. Rebuild container: `F1 → Dev Containers: Rebuild Container`

## Claude Code CLI Issues

### Claude not found in container
**Problem:** `claude: command not found` after container starts.

**Solution:** The claude-code feature should install it automatically. Check:
1. `.devcontainer/devcontainer.json` includes the claude-code feature
2. Check the feature installation logs in VS Code Output panel
3. Try rebuilding the container

### Provider not working
**Problem:** Claude CLI can't connect to provider or `ANTHROPIC_AUTH_TOKEN`/`ANTHROPIC_BASE_URL` are empty.

**Solution:** Check [provider configuration](../devcontainer/providers.md). Verify:
- Provider is specified in devcontainer.json
- API keys are configured in the correct location
- Network allows access to the API endpoint

Debug steps:
```bash
# Check provider file exists
cat ~/.config/devcontainer/provider

# Check bashrc was updated
grep "provider=" ~/.bashrc

# Verify environment variables
echo $ANTHROPIC_AUTH_TOKEN
echo $ANTHROPIC_BASE_URL

# Rebuild container to trigger postCreateCommand
# In VS Code: F1 → Dev Containers: Rebuild Container
```

### Proxy issues affecting Claude
**Problem:** Claude Code can't reach API or downloads fail.

**Solution:**
1. Verify proxy is accessible from host: `curl -x http://proxy:port https://claude.ai`
2. Check `NO_PROXY` includes necessary exclusions (localhost, 127.0.0.1)
3. Ensure proxy is configured for both build and runtime in devcontainer.json
4. See [Proxy Configuration](../devcontainer/proxy.md) for detailed debugging

Verify proxy variables in the container:
```bash
echo $HTTP_PROXY
echo $HTTPS_PROXY
echo $NO_PROXY
```

## Template/Preset Issues

### Template not found
**Problem:** "template not found" error when creating a project.

**Solution:** List available templates to find valid names:
```bash
./scripts/isolde.sh --list-templates
```

Check the template directory exists:
```bash
ls templates/
cat templates/{name}/template-info.yaml
```

### Preset not found
**Problem:** "preset not found" error when creating a project.

**Solution:** List available presets:
```bash
./scripts/isolde.sh --list-presets
```

For custom presets, verify:
1. `~/.devcontainer-presets.yaml` exists and is valid YAML
2. Preset name matches exactly (case-sensitive)
3. Template referenced by preset exists

### Language version not supported
**Problem:** Requested version is not available for the template.

**Solution:** Check supported versions in template info:
```bash
cat templates/{name}/template-info.yaml
```

Look for the `supported_versions` field. Use a version from that list or omit `--lang-version` to use the default.

## Git Repository Issues

Each project has **two separate git repositories**:
- `project/` - Your code
- `.devcontainer/` - Devcontainer config

### Git status confusion
**Problem:** Changes not showing in git status or unexpected behavior.

**Solution:** Check status separately for each repository:
```bash
cd ~/workspace/my-project/project && git status
cd ../.devcontainer && git status
```

### Updating the devcontainer
**Problem:** Want to get latest devcontainer changes.

**Solution:** The devcontainer configuration is a separate git repository:
```bash
cd ~/workspace/my-project/.devcontainer
git pull origin main
```

Then rebuild the container from VS Code.

## Still Need Help?

Check the [setup guide](../devcontainer/setup.md) or [usage guide](usage.md) for more details.

For bug reports or feature requests, please file an issue on the project repository.
