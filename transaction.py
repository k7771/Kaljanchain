import hashlib
import time
import rsa

class Transaction:
    def __init__(self, sender_public_key, receiver_public_key, amount, timestamp=None, signature=None):
        self.sender_public_key = sender_public_key
        self.receiver_public_key = receiver_public_key
        self.amount = amount
        self.timestamp = timestamp or int(time.time())
        self.signature = signature
        self.hash = self.calculate_hash()

    def calculate_hash(self):
        tx_data = f"{self.sender_public_key}{self.receiver_public_key}{self.amount}{self.timestamp}"
        return hashlib.sha256(tx_data.encode()).hexdigest()

    def sign_transaction(self, private_key):
        if not self.sender_public_key:
            raise ValueError("Transaction must have a sender public key")
        self.signature = rsa.sign(self.hash.encode(), private_key, 'SHA-256')

    def is_valid(self):
        if not self.signature:
            return False
        try:
            rsa.verify(self.hash.encode(), self.signature, self.sender_public_key)
            return True
        except rsa.VerificationError:
            return False

    def __repr__(self):
        return f"Transaction(sender={self.sender_public_key}, receiver={self.receiver_public_key}, amount={self.amount}, hash={self.hash})"
