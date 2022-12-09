export { };
exports.foo = foo;
exports.bar = bar;
function foo() {
    foo = ()=>1;
    foo.bar = ()=>2;
    return 3;
}
let bar = function() {
    bar = ()=>1;
    exports.bar.bar = ()=>(0, exports.bar)();
    return 3;
};
