module Hello where

data Nat : Set where
  zero : Nat
  suc  : Nat → Nat

_+_ : Nat → Nat → Nat
zero  + n = n
suc m + n = suc (m + n)

data List (A : Set) : Set where
  []  : List A
  _∷_ : A → List A → List A
