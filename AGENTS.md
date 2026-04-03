# AGENTS.md — Neobank Anchor demo

Você está trabalhando em um projeto demonstrativo de Solana usando Anchor.

## Objetivo
Criar um programa `neobank_anchor_demo` em Anchor, simples mas correto, que demonstre:
- PDAs com `seeds` e `bump`
- conta on-chain controlada pelo usuário
- depósito e saque de SOL
- depósito e saque de SPL tokens via CPI
- controle de acesso básico: somente o dono opera sua conta
- testes automatizados com `anchor test`
- deploy em devnet e README claro

## Escopo desejado
Implementar uma conta bancária on-chain por usuário:
- `initialize_account`
- `deposit_sol`
- `withdraw_sol`
- `initialize_token_vault` para um mint SPL
- `deposit_spl`
- `withdraw_spl`
- consulta de saldo por leitura da conta/state e dos vaults

## Preferências técnicas
- Uma PDA de estado por usuário.
- Uma PDA/vault para SOL custodiado pelo programa.
- Vault ATA/PDA por mint SPL quando necessário.
- Usar `has_one = owner`, `seeds`, `bump`, `constraint`.
- Usar CPI com `SystemProgram` para SOL e SPL Token para tokens.
- Manter contabilidade mínima e coerente no state.
- Erros customizados.
- Testes TypeScript cobrindo pelo menos um caminho feliz para SOL e um para SPL.

## Requisitos do desafio
- Anchor como framework principal.
- `anchor test` como suíte principal.
- README claro com Helius RPC para devnet.
- Deploy em devnet.

## O que evitar
- juros, lending, governança, múltiplos donos, freeze, features desnecessárias
- excesso de abstração
