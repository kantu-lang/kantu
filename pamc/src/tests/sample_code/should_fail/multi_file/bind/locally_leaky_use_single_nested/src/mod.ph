mod foo;
use foo.Foo;

let foo_identity = fun _(x: Foo): Foo {
    x
};
