import React, { useEffect, useState } from 'react';
import { PublicKey } from '@solana/web3.js';
import { useAnchorWallet, useConnection } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { AnchorProvider } from '@project-serum/anchor';
import { getOrCreateCounterAccount, getCounter, incrementCounter } from './solana';

function App() {
  const { connection } = useConnection();
  const wallet = useAnchorWallet();
  const [counterValue, setCounterValue] = useState<number | null>(null);
  const [counterPubkey, setCounterPubkey] = useState<PublicKey | null>(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const initialize = async () => {
      if (wallet && connection) {
        setLoading(true);
        try {
          // Crea un proveedor de Anchor que engloba la conexión y el wallet
          const provider = new AnchorProvider(connection, wallet, AnchorProvider.defaultOptions());
          const pubkey = await getOrCreateCounterAccount(provider);
          setCounterPubkey(pubkey);
          const value = await getCounter(provider, pubkey);
          setCounterValue(value);
        } catch (error) {
          console.error("Error initializing account:", error);
        } finally {
          setLoading(false);
        }
      }
    };
    initialize();
  }, [wallet, connection]);

  const handleIncrement = async () => {
    if (wallet && connection && counterPubkey) {
      setLoading(true);
      try {
        const provider = new AnchorProvider(connection, wallet, AnchorProvider.defaultOptions());
        await incrementCounter(provider, counterPubkey);
        const newValue = await getCounter(provider, counterPubkey);
        setCounterValue(newValue);
      } catch (error) {
        console.error("Error incrementing counter:", error);
      } finally {
        setLoading(false);
      }
    }
  };

  return (
    <div style={{ padding: '20px', textAlign: 'center', fontFamily: 'sans-serif' }}>
      <h1>Contador de Solana</h1>
      <WalletMultiButton />
      {wallet ? (
        <>
          {loading ? (
            <p>Cargando...</p>
          ) : (
            <>
              <p>Valor actual del contador: <strong>{counterValue !== null ? counterValue : 'N/A'}</strong></p>
              <button onClick={handleIncrement} style={{ padding: '10px 20px', fontSize: '16px', cursor: 'pointer', borderRadius: '5px', border: '1px solid #ccc' }}>Incrementar Contador</button>
            </>
          )}
        </>
      ) : (
        <p>Por favor, conecta tu wallet para interactuar con el programa.</p>
      )}
    </div>
  );
}

export default App;

