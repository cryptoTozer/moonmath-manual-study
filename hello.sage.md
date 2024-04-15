from sage.all import GF, EllipticCurve, factor, Set

def embedding_degree(E, max_k = 50):
  # order of finite-field ord(F)
  p = E.base_field().order()
  # order of elliptic curve = ord(E(F))
  n = E.order()
  # largest prime factor of n
  r = max(factor(n))[0] 

  # find embedding
  k = 1
  while k < r or k <= max_k:
    # Fermat's Little Theorem
    if (p ^ k - 1) % r == 0:
      break
    k += 1
  return k, r

