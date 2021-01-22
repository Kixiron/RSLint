//! No lints are enabled for these snippets, so no lints should trigger

rule_test! {
    no_lints,
    default_conf: |_, _| Ok(()),
    filter: |_| true,
    // Should pass
    { "var a = 3; function b(x) { a++; return x + a; }; setTimeout(function() { b(a); }, 0);" },
    { "(function() { var doSomething = function doSomething() {}; doSomething() }())" },
    { "var arguments;\nfunction bar() { }" },
    {
        "var a = 3;",
        "var b = (x) => {",
        "   a++;",
        "   return x + a;",
        "};",
        "setTimeout(",
        "   () => { b(a); },",
        "   0,",
        ");",
    },
    { "function foo() { var a; } function a() {}" },
    { "function foo() { let a; } var a;" },
    { "function foo() { var a; } var a;" },
    { "var a = 1, b = 2; a + b;" },
    { "function f() { b; }", globals: ["b"] },
    { "a; function f() { b; a; }", globals: ["b", "a"] },
    { "function a(){}  a();" },
    { "a = 1;" },
    { "var a = b;" },
    { "function f() { b; }" },
    { "window" },
    { "require(\"a\");" },
    { "A: break A;" },
    { "A: { foo(); break A; bar(); }" },
    { "A: if (a) { foo(); if (b) break A; bar(); }" },
    { "A: for (var i = 0; i < 10; ++i) { foo(); if (a) break A; bar(); }" },
    { "A: for (var i = 0; i < 10; ++i) { foo(); if (a) continue A; bar(); }" },
    { "A: for (var i = 0; i < 10; ++i) { B: break A; }" },
    { "A: { var A = 0; console.log(A); }" },
    { "A /* comment */: foo" },
    { "let box;\nfor (let prop in box) {\n  box[prop] = parseInt(box[prop]);\n}" },
    { "var box = { a: 2 };\nfor (var prop in box) {\n  box[prop] = parseInt(box[prop]);\n}" },
    { "a;\nvar a;" },
    { "const x = 1; const [y = x] = []; foo(y);" },
    { "const x = 1; const {y = x} = {}; foo(y);" },
    { "const x = 1; const {z: [y = x]} = {}; foo(y);" },
    { "let _x = 10;" },
    { "f({ set foo(a) { return; } });" },
    { "function a(x, y){ return y; }; a();" },
    { "var a = 10;" },
    { "function g(bar, baz) { return baz; }; g();" },
    { "function g(bar, baz) { return 2; }; g();" },
    { "var a = 10; typeof a" },
    { "let x = 10; typeof ((1, 4, (x)))" },
    { "typeof a" },
    { "typeof (a)" },
    { "var b = typeof a" },
    { "typeof a === 'undefined'" },
    { "var a = 10; alert(a);" },
    { "function b(a) { alert(a); }" },
    { "Object.hasOwnProperty.call(a);" },
    { "function a() { alert(arguments);}" },
    { "(() => { var a = 42; alert(a); })();" },
    { "a++; var a = 19;" },
    { "a++; var a = 19;" },
    { "a(); var a = function() {};" },
    { "alert(a[1]); var a = [1, 3];" },
    { "a(); function a() { alert(b); var b = 10; a(); }" },
}
