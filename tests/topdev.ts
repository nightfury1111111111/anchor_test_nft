import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Topdev } from '../target/types/topdev';

describe('topdev', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Topdev as Program<Topdev>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
