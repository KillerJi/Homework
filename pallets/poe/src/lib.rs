#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_module, decl_storage, decl_event, decl_error, ensure, StorageMap
};
use frame_system::ensure_signed;
use sp_std::vec::Vec;

pub trait Trait: frame_system::Trait {

    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as PoeModule {
        
        Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
    }
}


decl_event! {
    pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
        
        ClaimCreated(AccountId, Vec<u8>),
        
        ClaimRevoked(AccountId, Vec<u8>),

        ClaimTransfer(AccountId, AccountId, Vec<u8>),
    }
}

decl_error! {
    pub enum Error for Module<T: Trait> {
       
        ProofAlreadyClaimed,
        
        NoSuchProof,
        
        NotProofOwner,

        SameOwner,
    }
}


decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        type Error = Error<T>;

        fn deposit_event() = default;

        #[weight = 10_000]
        fn create_claim(origin, proof: Vec<u8>) {

            let sender = ensure_signed(origin)?;

            ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

            let current_block = <frame_system::Module<T>>::block_number();

            Proofs::<T>::insert(&proof, (&sender, current_block));

            Self::deposit_event(RawEvent::ClaimCreated(sender, proof));
        }

        #[weight = 10_000]
        fn revoke_claim(origin, proof: Vec<u8>) {
    
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

            let (owner, _) = Proofs::<T>::get(&proof);

            ensure!(sender == owner, Error::<T>::NotProofOwner);

            Proofs::<T>::remove(&proof);

            Self::deposit_event(RawEvent::ClaimRevoked(sender, proof));
        }

        #[weight = 10_000]
        fn transfer_claim(origin, proof: Vec<u8>,sender: T::AccountId)
        {
            let owner = ensure_signed(origin)?;
            let (get,_) = <Proofs<T>>::get(&proof);
            ensure!(get == owner, Error::<T>::NotProofOwner);
            ensure!(<Proofs<T>>::contains_key(&proof), Error::<T>::NoSuchProof);
            ensure!(sender != owner, Error::<T>::SameOwner);

            <Proofs<T>>::remove(&proof);
            <Proofs<T>>::insert(&proof, (&sender, <frame_system::Module<T>>::block_number()) );

            Self::deposit_event(RawEvent::ClaimTransfer(owner,sender, proof));
        }
   }
}