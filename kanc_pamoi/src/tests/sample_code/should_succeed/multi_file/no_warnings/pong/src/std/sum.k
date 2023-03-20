pub type Sum(L: Type, R: Type) {
    inl(L: Type, l: L, R: Type): Sum(L, R),
    inr(L: Type, R: Type, r: Type): Sum(L, R),
}
