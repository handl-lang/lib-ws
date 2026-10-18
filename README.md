# lib-ws

The `ws` port contract — RFC 6455 client step-surface implemented by `ws-c` (libcurl) and `ws-whatwg-*` (host APIs).

Part of the `handl-lang` org split (ADR-0128). The contract is extracted
from the backend packages' documented surfaces — no prior history.

## Layout

```text
src/        the .hnl sources
pkg.hnl     dependencies, when any: `{ deps: { <name>: "branch:<ref>" } }`
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```
