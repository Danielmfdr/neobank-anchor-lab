# Checklist — neobank-anchor

1. Preencher `.env`
2. Conferir toolchain:
   - `rustc --version`
   - `solana --version`
   - `anchor --version`
   - `node --version`
3. Rodar:
   - `anchor build`
   - `anchor test`
4. Gerar/chamar Program ID:
   - `anchor keys list`
   - sincronizar `declare_id!` se necessário
5. Configurar RPC Helius para devnet:
   - `solana config set --url "https://devnet.helius-rpc.com/?api-key=SEU_HELIUS_API_KEY"`
6. Airdrop devnet:
   - `solana airdrop 2`
7. Deploy:
   - `anchor deploy --provider.cluster devnet`
8. Colar Program ID real no README
9. Subir para GitHub público
