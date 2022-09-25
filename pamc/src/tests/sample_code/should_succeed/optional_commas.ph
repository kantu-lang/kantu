type U {
    .U: U
}

type TypeParam1(a: U) {}
type TypeParam2(a: U,) {}
type TypeParam3(a: U, b: U) {}
type TypeParam4(a: U, b: U,) {}

type VariantParam1 {
    .O(a: U): VariantParam1
}
type VariantParam2 {
    .O(a: U,): VariantParam2
}
type VariantParam3 {
    .O(a: U, b: U): VariantParam3
}
type VariantParam4 {
    .O(a: U, b: U,): VariantParam4
}

type Variant1 {
    .O: Variant1
}
type Variant2 {
    .O: Variant2,
}
type Variant3 {
    .O: Variant3,
    .P: Variant3
}
type Variant4 {
    .O: Variant4,
    .P: Variant4,
}

let fun1 = fun x(a: U): U { U.U };
let fun2 = fun x(a: U,): U { U.U };
let fun3 = fun x(a: U, b: U): U { U.U };
let fun4 = fun x(a: U, b: U,): U { U.U };

let call1 = fun1(U.U);
let call2 = fun2(U.U,);
let call3 = fun3(U.U, U.U);
let call4 = fun4(U.U, U.U,);

let forall1 = forall(a: U) { U };
let forall2 = forall(a: U,) { U };
let forall3 = forall(a: U, b: U) { U };
let forall4 = forall(a: U, b: U,) { U };

let match_case_param1 = match VariantParam1.O(U.U) {
    .O(a) => U.U
};
let match_case_param2 = match VariantParam2.O(U.U) {
    .O(a,) => U.U
};
let match_case_param3 = match VariantParam3.O(U.U) {
    .O(a, b) => U.U
};
let match_case_param4 = match VariantParam4.O(U.U) {
    .O(a, b,) => U.U
};

type Bool {
    .True: Bool,
    .False: Bool,
}

let match_case1 = match Bool.True {
    .True => U.U
};
let match_case2 = match Bool.True {
    .True => U.U,
};
let match_case3 = match Bool.True {
    .True => U.U,
    .False => U.U
};
let match_case4 = match Bool.True {
    .True => U.U,
    .False => U.U,
};
