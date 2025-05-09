from block import Block
from transaction import Transaction
import time

class Blockchain:
    def __init__(self, difficulty=4):
        self.chain = [self.create_genesis_block()]
        self.pending_transactions = []
        self.difficulty = difficulty
        self.mining_reward = 50

    def create_genesis_block(self):
        return Block(0, "0", [], int(time.time()))

    def get_latest_block(self):
        return self.chain[-1]

    def add_block(self, block):
        block.previous_hash = self.get_latest_block().hash
        block.mine_block(self.difficulty)
        self.chain.append(block)
        print(f"Block {block.index} added to the chain.")

    def add_transaction(self, transaction):
        if not transaction.is_valid():
            raise ValueError("Invalid transaction")
        self.pending_transactions.append(transaction)

    def mine_pending_transactions(self, miner_address):
        block = Block(len(self.chain), self.get_latest_block().hash, self.pending_transactions)
        block.mine_block(self.difficulty)
        self.chain.append(block)
        print(f"Block {block.index} mined and added to the chain.")

        # Reward miner
        reward_tx = Transaction("SYSTEM", miner_address, self.mining_reward)
        self.pending_transactions = [reward_tx]

    def is_chain_valid(self):
        for i in range(1, len(self.chain)):
            current_block = self.chain[i]
            previous_block = self.chain[i - 1]
            if current_block.hash != current_block.calculate_hash():
                print(f"Block {current_block.index} has been tampered.")
                return False
            if current_block.previous_hash != previous_block.hash:
                print(f"Block {current_block.index} has incorrect previous hash.")
                return False
        return True

    def __repr__(self):
        return f"Blockchain(chain_length={len(self.chain)}, difficulty={self.difficulty}, pending_tx={len(self.pending_transactions)})"
