Quero que você implemente um projeto Anchor completo, pequeno e didático, chamado `neobank-anchor`, para Solana.

Contexto do desafio:
- O objetivo é demonstrar domínio de Anchor, PDAs, inicialização de contas, movimentação de SOL/tokens via CPI e validações corretas.
- O repositório final precisará compilar, rodar testes com `anchor test`, ser deployável na devnet e usar Helius RPC no fluxo de devnet.
- O README deve explicar claramente o programa, instruções, testes e deploy.

O que construir:
1. Um programa Anchor de “neobank” on-chain, simples.
2. Fluxo mínimo desejado:
   - `initialize_account`: cria a conta bancária PDA do usuário.
   - `deposit_sol`: transfere SOL do usuário para um vault controlado pelo programa.
   - `withdraw_sol`: transfere SOL do vault para o usuário, apenas se ele for o owner.
   - `initialize_token_vault`: cria o vault SPL para determinado mint.
   - `deposit_spl`: deposita tokens SPL do usuário no vault do programa.
   - `withdraw_spl`: saca tokens SPL do vault para o usuário, apenas se ele for o owner.
3. Usar PDAs corretamente:
   - PDA de state da bank account por owner
   - PDA autoridade do vault
   - conta/vault para SOL custodiado
   - vault ATA/PDA para tokens SPL por mint
4. Validar corretamente:
   - somente o dono opera a própria conta
   - checagem de saldo suficiente
   - mints corretas
   - owners corretos
5. Usar CPI com `SystemProgram` e SPL Token, preferencialmente `transfer_checked` para SPL.
6. Criar erros customizados.
7. Criar pelo menos uma suíte de testes funcional com `anchor test` cobrindo:
   - initialize_account
   - deposit_sol / withdraw_sol
   - initialize_token_vault
   - deposit_spl / withdraw_spl
   - assert dos saldos finais
8. Escrever README completo com:
   - visão geral
   - instruções disponíveis
   - como configurar Helius RPC via `.env`/`solana config`
   - como rodar `anchor build`, `anchor test` e `anchor deploy`
   - local para colar o Program ID da devnet

Decisões de implementação:
- Mantenha o projeto simples e fácil de ler.
- Prefira um único programa Anchor.
- Organize o código para outro dev entender rápido.
- Não usar Surfpool/LiteSVM como principal; manter `anchor test` como suíte principal porque isso é exigência do desafio.
- Se precisar simplificar, a “consulta de saldo” pode ser feita por leitura do state e dos vaults no teste/cliente, sem uma instrução dedicada.

O que eu quero na sua resposta:
1. Criar/editar todos os arquivos necessários do projeto.
2. No final, me mostrar:
   - árvore de arquivos relevante
   - resumo do que foi implementado
   - comandos exatos para build, test e deploy em devnet com Helius
   - qualquer passo manual restante
