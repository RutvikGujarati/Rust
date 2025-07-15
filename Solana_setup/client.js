const { Connection, PublicKey, Transaction, SystemProgram, Keypair } = require('@solana/web3.js');

// Connect to Solana testnet
const connection = new Connection('https://api.testnet.solana.com', 'confirmed');

async function callHelloWorldProgram(programId, payer) {
    try {
        // Create a transaction
        const transaction = new Transaction();
        
        // Add instruction to call your program
        // For a simple hello world program, we just need to call it
        transaction.add({
            keys: [],
            programId: new PublicKey(programId),
            data: Buffer.from([]) // Empty instruction data
        });
        
        // Send and confirm transaction
        const signature = await connection.sendTransaction(transaction, [payer]);
        console.log('Transaction sent:', signature);
        
        // Wait for confirmation
        const confirmation = await connection.confirmTransaction(signature);
        console.log('Transaction confirmed:', confirmation);
        
        return signature;
    } catch (error) {
        console.error('Error calling program:', error);
        throw error;
    }
}

// Example usage (you'll need to replace with your actual program ID and payer keypair)
// const programId = 'YOUR_PROGRAM_ID_HERE';
// const payer = Keypair.fromSecretKey(/* your secret key */);
// callHelloWorldProgram(programId, payer);

module.exports = { callHelloWorldProgram }; 