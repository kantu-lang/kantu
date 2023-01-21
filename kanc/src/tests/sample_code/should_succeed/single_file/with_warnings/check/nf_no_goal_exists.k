type Nat {
    o: Nat,
    s(n: Nat): Nat,
}

let no_goal_exists = check (goal = Nat) {
    Nat.o
};

let no_goal_exists_question = check (goal = ?) {
    Nat.o
};
