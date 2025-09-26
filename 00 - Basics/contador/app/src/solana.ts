import { Connection, PublicKey, SystemProgram } from '@solana/web3.js';
import { Program, AnchorProvider } from '@project-serum/anchor';
import { Idl } from '@project-serum/anchor/dist/cjs/idl';
import idl from './idl.json';

// Asegúrate de que este ID sea el que obtuviste en el despliegue
export const programId = new PublicKey('Hasw11c3aWnyQNxVwYeLszA1ipGV8ZzrbuewmWcSZUPL');

// Función para inicializar o obtener la cuenta del contador
export async function getOrCreateCounterAccount(provider: AnchorProvider): Promise<PublicKey> {
  const [counterPubkey] = await PublicKey.findProgramAddress(
      [Buffer.from("greeting-account"), provider.wallet.publicKey.toBuffer()],
      programId
  );

  const counterAccount = await provider.connection.getAccountInfo(counterPubkey);
  
  if (counterAccount === null) {
    console.log('PDA does not exist. Initializing...');
    const program = new Program(idl as any as Idl, programId, provider);
    
    await program.methods.initialize()
      .accounts({
          greetingAccount: counterPubkey,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
      })
      .rpc();
    console.log('PDA initialized!');
  } else {
    console.log('PDA already exists.');
  }
  return counterPubkey;
}

// Función para leer el contador
export async function getCounter(provider: AnchorProvider, counterPubkey: PublicKey): Promise<number> {
    const program = new Program(idl as any as Idl, programId, provider);
    const counterAccount = await program.account.greetingAccount.fetch(counterPubkey);
    
    return counterAccount.counter as number;
}

// Función para incrementar el contador
export async function incrementCounter(provider: AnchorProvider, counterPubkey: PublicKey): Promise<void> {
    const program = new Program(idl as any as Idl, programId, provider);
    await program.methods.increment()
        .accounts({
            greetingAccount: counterPubkey,
        })
        .rpc();
}