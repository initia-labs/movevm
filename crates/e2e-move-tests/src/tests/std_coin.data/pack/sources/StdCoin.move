module 0x2::StdCoin {
    use 0x1::coin;
    use std::string;
    use std::signer;
    use std::option;

    struct Std {}

    struct CapStore has key{
        burn: coin::BurnCapability,
        freeze: coin::FreezeCapability,
        mint: coin::MintCapability
    }

    entry fun init(sender: &signer) {
        let (mint, burn, freeze) = coin::initialize(
            sender, 
            option::none(), 
            string::utf8(b"Std Coin"), 
            string::utf8(b"STDC"), 
            8,
            string::utf8(b""),
            string::utf8(b""),
        );

        move_to(sender, CapStore {
            burn, freeze, mint
        });
    }

    entry fun mint(sender: &signer, account_to: address, amount: u64)  acquires CapStore {
        let sender_address = signer::address_of(sender);
        let caps = borrow_global<CapStore>(sender_address);
        
        let minted = coin::mint(&caps.mint, amount);
        coin::deposit(account_to, minted);
    }

    spec init {
    	pragma intrinsic;
	}

    spec mint {
    	pragma intrinsic;
	}
}
