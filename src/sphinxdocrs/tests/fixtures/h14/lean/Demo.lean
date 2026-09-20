/-! Demo module documentation.

This module exercises the H14 Lean source corpus.
-/
namespace Demo

section Arithmetic

/-- Add two natural numbers. -/
def add (left right : Nat) : Nat := left + right

/-- A proposition about addition. -/
theorem add_zero (left : Nat) : add left 0 = left := by simp [add]

/-- A lemma with a stable source span. -/
lemma add_comm (left right : Nat) : add left right = add right left := by simp [add, Nat.add_comm]

example : add 1 2 = 3 := by decide

inductive Color where
  | red
  | blue

structure Point where
  x : Nat
  y : Nat

class SemigroupLike (value : Type) where
  combine : value -> value -> value

instance : SemigroupLike Nat where
  combine := Nat.add

abbrev Count := Nat
axiom admitted : Prop
opaque hiddenProof : Prop
notation "⟪" value "⟫" => value

end Arithmetic
end Demo
