type Nat {
    .O: Nat,
    .S(n: Nat): Nat,
}

let no_goal_exists = check (goal = Nat) {
    Nat.O
};

let no_goal_exists_question = check (goal = ?) {
    Nat.O
};
