# URL shortener sample for [ohkami](https://github.com/kana-rus/ohkami) on Cloudflare Workers

This is clone of [Hono implementation](https://github.com/yusukebe/url-shortener) in ohkami.

## Prerequisites

- Rust toolchain of latest version with `wasm32-unknown-unknown` target
- npm

In addition, `wasm-opt` is recommended to be installed.

## Setup

```sh
npm create cloudflare ./path/to/project-dir -- --template https://github.com/kana-rus/ohkami-templates/worker
```
```sh
cd ./path/to/project-dir
```
```sh
npx wrangler login
```
```sh
npx wrangler kv:namespace create KV

# and add the outputs in wrangler.toml:
#
# [[kv_namespaces]]
# binding = "KV"
# id      = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
```

and if you push the project to your GitHub repo, **You should add `wrangler.toml` into .gitignore**！

## Local dev

```sh
npm run dev
```

## Publish
```sh
npm run deploy
```
If you register your workers.dev subdomain at this time, it takes some minutes for DNS records to update and it's good time to take a coffee break.

_**note**_ : Up to version 0.17, ohkami only supports HTTP/1.1 and you can only access the published worker by `http://<project-name>.<subdomain>.workers.dev`.
