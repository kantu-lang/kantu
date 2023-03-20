type Empty {}

type Unit {
    c: Unit,
}

let identity = fun _(T: Type, t: T): T { t };
let ascribe = identity;


let broken = fun _(u: Unit): Empty {
    todo
};

let broke2 = fun _(u: Unit): Empty {
    match u {
        c => todo,
    }
};

let broken3 = fun _(u: Unit): Empty {
    match ascribe(Unit, todo) {
        c => todo,
    }
};

let broken4 = ascribe(Empty, todo);
