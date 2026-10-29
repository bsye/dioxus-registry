# dioxus-registry

[shadcn/ui](https://ui.shadcn.com) components that [dioxus-components](https://github.com/DioxusLabs/components)
does not have yet, in its layout, for Dioxus 0.8. Start here: shadcn's chat set.

| Component | What it is |
|---|---|
| `message` | One chat message: avatar, header, content, footer, aligned to either side |
| `bubble` | A chat bubble in seven variants, with reactions |
| `marker` | A status line or separator inside a transcript, with a shimmer |
| `attachment` | A file or image on a message, with upload states, sizes and actions |
| `message_scroller` | A transcript that follows new messages, anchors new turns, keeps its place when history loads and shows scroll buttons |
| `input_group` | An input or textarea with addons and buttons, like a chat composer |
| `spinner` | A loading indicator |
| `direction` | Left to right or right to left, shared with components |

## Install

```sh
dx components add --git https://github.com/bsye/dioxus-registry message bubble message_scroller
```

Components land in `src/components/<name>`; link each `style.css` and the theme from dioxus-components. `attachment`,
`input_group` and `message_scroller` use `button` from dioxus-components. `dx components` 0.8.0-alpha.3 cannot
download over https ("there is no TLS stream available"); clone this repo with `git` and use
`--git file:///path/to/clone` until that is fixed.

## Layout

| Path | Holds |
|---|---|
| `primitives/` | `dioxus-registry-primitives`: headless behaviour (the message scroller and the direction context) with tests |
| `preview/src/components/<name>/` | Each component: `component.json`, `component.rs`, `style.css`, `docs.md` and demos in `variants/` |
| `preview/src/components/button/` | dioxus-components' button, copied unchanged so the preview builds; not offered here |

Pins match dioxus-components' `dioxus-v0.8.0-alpha.3` branch: Dioxus `0.8.0-alpha.3`, `dioxus-primitives` rev
`9db1d9a`, `dioxus-icons` `0.2.0-alpha.3`. Styling is CSS modules on the variables of `dx-components-theme.css`.

## Checks

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo clippy --workspace --target wasm32-unknown-unknown -- -D warnings
cargo test --workspace
```

## Credits

Ports of [shadcn/ui](https://github.com/shadcn-ui/ui) (MIT), including its headless message scroller, built on
[dioxus-components](https://github.com/DioxusLabs/components) (MIT or Apache-2.0). Icons are
[Lucide](https://lucide.dev) through dioxus-icons. Licensed under MIT or Apache-2.0, at your option.
