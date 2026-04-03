# neobank-anchor

Neobank on-chain simples em Solana usando Anchor. Cada usuário tem uma conta bancária PDA própria e pode depositar ou sacar SOL e tokens SPL com controle de acesso básico: só o dono da conta pode operar seus fundos.

Programa on-chain: `neobank_anchor_demo`

Program ID deployado na devnet:

```text
DvTZxzMYaBKTTHCuaR9QQoZXxeqWuxqwgDoZmTt9kFd7
```

## O que o programa faz

Este projeto existe para demonstrar, de forma pequena e didática:

- criação de PDAs com `seeds` e `bump`
- inicialização de contas Anchor
- custódia de SOL em PDA controlada pelo programa
- custódia de SPL em vault por mint
- movimentação de SOL via CPI com `SystemProgram`
- movimentação de SPL via CPI com `transfer_checked`
- validações de ownership e saldo suficiente
- testes automatizados com `anchor test`

## Instruções disponíveis

- `initialize_account`
  Cria a conta bancária PDA do usuário, o vault de SOL e registra os bumps.

- `deposit_sol`
  Transfere lamports do owner para o vault de SOL do programa.

- `withdraw_sol`
  Transfere lamports do vault de SOL de volta para o owner.

- `initialize_token_vault`
  Cria o vault SPL do programa para um mint específico.

- `deposit_spl`
  Transfere tokens SPL do ATA do usuário para o vault do programa.

- `withdraw_spl`
  Transfere tokens SPL do vault do programa para o ATA do usuário.

## Controle de acesso

- Cada `bank_account` pertence a um único `owner`.
- As instruções operacionais usam `has_one = owner`.
- As PDAs são validadas com `seeds` e `bump`.
- Saques exigem saldo suficiente antes da CPI.
- Vaults SPL usam o `vault_authority` PDA como autoridade.

## Como o saldo é consultado

O desafio não exige uma instrução de consulta dedicada. Neste projeto, a leitura é feita assim:

- saldo SOL: campo `bank_account.sol_balance`
- saldo SPL: campo `amount` do `token_vault` do mint desejado

Em outras palavras, a fonte de verdade de SOL é o state da conta bancária, e a de SPL é o vault ATA do programa para aquele mint.

## PDAs usadas

- `bank-account + owner`
  State principal da conta bancária do usuário.

- `sol-vault + owner`
  Vault de SOL custodiado pelo programa.

- `vault-authority + owner`
  PDA que assina CPIs e é dona dos vaults SPL.

## Estrutura relevante

```text
.
├── Anchor.toml
├── Cargo.toml
├── package.json
├── programs
│   └── neobank_anchor_demo
│       ├── Cargo.toml
│       └── src/lib.rs
├── tests
│   └── neobank_anchor_demo.ts
└── migrations
    └── deploy.ts
```

## Entregáveis do desafio

- Repositório Anchor com programa, testes e documentação
- Programa deployado na devnet com Program ID no README
- README com visão geral, instruções e comandos de uso
- Suíte básica de testes rodando com `anchor test`

## Critérios atendidos

- o programa compila
- o programa foi deployado na devnet
- depósito e saque de SOL funcionam
- depósito e saque de SPL funcionam
- PDAs são usadas com `seeds` e `bump`
- há teste automatizado passando com `anchor test`
- o fluxo de devnet usa Helius RPC

## Pré-requisitos

- Rust toolchain
- Solana CLI
- Node.js 18+ e npm
- Anchor CLI `0.32.1`

Este repositório inclui:

- `.anchorversion` com `0.32.1`
- `rust-toolchain.toml` com `1.89.0`

Se você usa `avm`, basta alinhar a versão:

```bash
avm install 0.32.1 --from-source
avm use 0.32.1
anchor --version
```

O `--from-source` é útil em Linux quando binários precompilados do Anchor falham por incompatibilidade de GLIBC.

## Instalação

```bash
npm install
```

## Como rodar build

```bash
NO_DNA=1 anchor build
```

## Como rodar os testes

Os testes em [tests/neobank_anchor_demo.ts](/home/daniel/DEV/Learning/Solana-Earn/solana-anchor-codex-starter/neobank-anchor/tests/neobank_anchor_demo.ts) cobrem:

- `initialize_account`
- `deposit_sol`
- `withdraw_sol`
- `initialize_token_vault`
- `deposit_spl`
- `withdraw_spl`
- asserts dos saldos finais de SOL e SPL

Execute:

```bash
NO_DNA=1 anchor test
```

Resultado validado localmente:

```text
1 passing
```

## Configurar Helius para devnet

1. Copie o exemplo:

```bash
cp .env.example .env
```

2. Preencha `.env` com sua API key Helius.

Exemplo esperado:

```bash
HELIUS_API_KEY=sua_api_key_real
HELIUS_DEVNET_RPC=https://devnet.helius-rpc.com/?api-key=${HELIUS_API_KEY}
SOLANA_WALLET=~/.config/solana/id.json
```

3. Exporte as variáveis:

```bash
set -a
source .env
set +a
```

4. Configure o Solana CLI para usar a wallet e o RPC da Helius:

```bash
solana config set --keypair "${SOLANA_WALLET}" --url "${HELIUS_DEVNET_RPC}"
solana config get
```

Se quiser garantir que o Anchor use a mesma wallet:

```bash
export ANCHOR_WALLET="${SOLANA_WALLET}"
```

## Deploy em devnet

Garanta que a wallet tenha SOL em devnet:

```bash
solana balance
solana airdrop 2 --url "${HELIUS_DEVNET_RPC}"
```

Build:

```bash
NO_DNA=1 anchor build
```

Deploy usando Helius:

```bash
NO_DNA=1 anchor deploy \
  --provider.cluster "${HELIUS_DEVNET_RPC}" \
  --provider.wallet "${SOLANA_WALLET}"
```

Verificação do programa na devnet usando Helius:

```bash
solana program show DvTZxzMYaBKTTHCuaR9QQoZXxeqWuxqwgDoZmTt9kFd7 \
  --url "${HELIUS_DEVNET_RPC}"
```

## Arquivos principais

- Programa: [lib.rs](programs/neobank_anchor_demo/src/lib.rs)
- Testes: [neobank_anchor_demo.ts](tests/neobank_anchor_demo.ts)
- Configuração Anchor: [Anchor.toml](Anchor.toml)

## Observações de implementação

- O state da conta bancária guarda contabilidade mínima: owner, endereços das PDAs, bumps, saldo lógico de SOL e quantidade de vaults SPL criados.
- O saldo SPL não é espelhado no state; a fonte de verdade é o `token_vault`.
- O vault de SOL mantém rent exemption da própria conta PDA. O campo `sol_balance` rastreia apenas depósitos e saques feitos pelas instruções do programa.
- O programa é simples de propósito, porque o foco do desafio é mostrar domínio de Anchor, PDAs, CPI e validações corretas, não adicionar features extras.
