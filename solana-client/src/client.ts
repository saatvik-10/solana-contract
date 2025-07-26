import * as web3 from '@solana/web3.js';
import * as borsh from 'borsh';
import { CounterSchema } from './counter';

//deployed program ID
const PROGRAM_ID = new web3.PublicKey(process.env.PROGRAM_ID!);

//connection to local validator RPC
const connection = new web3.Connection('http://127.0.0.1:8899', 'confirmed');

//create a new keypair for the payer
const payer = web3.Keypair.generate();

//airdrop 1 SOL to the payer account for transaction fees
//requests SOL from localnet faucet so payer can cover fees
const airDropSignature = await connection.requestAirdrop(
  payer.publicKey,
  web3.LAMPORTS_PER_SOL
);
await connection.confirmTransaction(airDropSignature, 'confirmed');
console.log(`Airdropped 1 SOL to ${payer.publicKey.toBase58()}`);

//generate a new keypair of counter account (program owner)
const counterAccount = web3.Keypair.generate();

//space needed
const space = 4;

//calculate minimum balance to exempt rent for this data size
const rentLamports = await connection.getMinimumBalanceForRentExemption(space);

//create a transaction to create the new account owned by your program
/*
Prepares a system program instruction to create and allocate the counterAccount.
This account is owned by your program and sized for your counter struct.
 */
const createAccountTx = new web3.Transaction().add(
  web3.SystemProgram.createAccount({
    fromPubkey: payer.publicKey,
    newAccountPubkey: counterAccount.publicKey,
    lamports: rentLamports,
    space,
    programId: PROGRAM_ID,
  })
);

//send the transaction to create the counter account
await web3.sendAndConfirmTransaction(
  connection,
  createAccountTx,
  [payer, counterAccount],
  { commitment: 'confirmed' }
);
console.log(`Created counter account: ${counterAccount.publicKey.toBase58()}`);

//initialize the counter account with a value of 0
/*
Normally the program initializes the account; here we send serialized data as instruction data or initialize from client (depending on your program)
*/
const counter = { value: 0 };
const serializedCounter = Buffer.from(borsh.serialize(CounterSchema, counter));

for (let i = 0; i < 4; i++) {
  //transaction instruction to call your program to increment the counter
  const incrementInstruction = new web3.TransactionInstruction({
    keys: [
      { pubkey: counterAccount.publicKey, isSigner: false, isWritable: true },
      {
        pubkey: payer.publicKey,
        isSigner: true,
        isWritable: false,
      },
      {
        pubkey: web3.SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      }
    ],
    programId: PROGRAM_ID,
    data: serializedCounter, //empty data for increment instruction
  });

  const incrementTx = new web3.Transaction().add(incrementInstruction);

  //send transaction calling your program to increment the counter
  await web3.sendAndConfirmTransaction(connection, incrementTx, [payer], {
    commitment: 'confirmed',
  });
  console.log(
    `Incremented counter for account: ${counterAccount.publicKey.toBase58()}`
  );
}

//fetch the counter account data and deserialize it
const updatedAccountInfo = await connection.getAccountInfo(
  counterAccount.publicKey
);
if (!updatedAccountInfo) {
  throw new Error('Counter account not found');
}

const updatedCounter = borsh.deserialize(
  CounterSchema,
  updatedAccountInfo.data
) as { value: number };

console.log(`Updated counter value: ${updatedCounter?.value}`);
