# lib-ws

The `ws` **port contract** — RFC 6455 client step-surface implemented
by `ws-c` (libcurl) and `ws-whatwg-*` (host APIs). Pure `.hnl`.

## Layout

| path | role |
| --- | --- |
| `src/index.hnl` | the contract |
| `pkg.hnl` | package manifest — `modules: ["ws"]` |
| `scripts/hnl-gate.sh`, `ci.sh` | the gate / bupha entry |

## Contract

- Step-based WebSocket client surface (RFC 6455); backends:
  `lib-ws-c` (libcurl transport for the `concurrent-epoll`
  scheduler), `lib-ws-whatwg-c`/`lib-ws-whatwg-wasm` (WHATWG surface
  over host APIs).

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
