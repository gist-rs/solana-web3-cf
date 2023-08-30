# solana-web3-cf

POC: Use solana web3 wasm with cloudflare pages via service binding.

## Features

- [x] User able to pay via solana pay.
  > `http://127.0.0.1:8787/pay/solana:mvines9iiHiQTysrwkJjGf2gb9Ex9jXJX8ns3qwf2kN?amount=0.01&spl-token=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v`
- [ ] Server able to get callback after payment.
  > `http://127.0.0.1:8787/pay/solana:gistmeAhMG7AcKSPCHis8JikGmKT9tRRyZpyMLNNULq?label=1.1+%F0%9F%8D%8B&memo=62623036-6666-5234-a638-313263346534%3A%3A3630522061&amount=1.65&spl-token=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v&redirect_link=http%3A%2F%2F192.168.2.39%3A8788%2Fpay%2Fsolana%2Fcallback`
- [ ] Server can verify tx after callback.
- [ ] Server can mint and mutate state.

## Dev

```
wrangler dev
```

## Release

```
wrangler deploy
```
