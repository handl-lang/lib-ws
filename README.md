# lib-ws

The `ws` port contract — RFC 6455 client step-surface implemented by `ws-c` (libcurl) and `ws-whatwg-*` (host APIs).

## Layout

```text
src/        the .hnl sources
pkg.hnl     package manifest — modules, deps, description, license
scripts/hnl-gate.sh   the gate
workflows/gate.yaml       bupha entry (handl-gates)
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
