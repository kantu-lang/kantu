type U {
    u: U
}

type TypeParam1(a: U) {}
type TypeParam2(a: U,) {}
type TypeParam3(a: U, b: U) {}
type TypeParam4(a: U, b: U,) {}

type VariantParam1 {
    o(a: U): VariantParam1
}
type VariantParam2 {
    o(a: U,): VariantParam2
}
type VariantParam3 {
    o(a: U, b: U): VariantParam3
}
type VariantParam4 {
    o(a: U, b: U,): VariantParam4
}

type Variant1 {
    o: Variant1
}
type Variant2 {
    o: Variant2,
}
type Variant3 {
    o: Variant3,
    p: Variant3
}
type Variant4 {
    o: Variant4,
    p: Variant4,
}

let fun1 = fun x(a: U): U { U.u };
let fun2 = fun x(a: U,): U { U.u };
let fun3 = fun x(a: U, b: U): U { U.u };
let fun4 = fun x(a: U, b: U,): U { U.u };

let call1 = fun1(U.u);
let call2 = fun2(U.u,);
let call3 = fun3(U.u, U.u);
let call4 = fun4(U.u, U.u,);

let forall1 = forall(a: U) { U };
let forall2 = forall(a: U,) { U };
let forall3 = forall(a: U, b: U) { U };
let forall4 = forall(a: U, b: U,) { U };

let match_case_param1 = match VariantParam1.o(U.u) {
    o(a) => U.u
};
let match_case_param2 = match VariantParam2.o(U.u) {
    o(a,) => U.u
};
let match_case_param3 = match VariantParam3.o(U.u, U.u) {
    o(a, b) => U.u
};
let match_case_param4 = match VariantParam4.o(U.u, U.u) {
    o(a, b,) => U.u
};

type Bool {
    true: Bool,
    false: Bool,
}

let match_case1 = match U.u {
    u => U.u
};
let match_case2 = match U.u {
    u => U.u,
};
let match_case3 = match Bool.true {
    true => U.u,
    false => U.u
};
let match_case4 = match Bool.true {
    true => U.u,
    false => U.u,
};
