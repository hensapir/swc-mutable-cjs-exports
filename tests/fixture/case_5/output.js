export { };
exports.a = a;
exports.b = b;
exports.c = c;
let a = function() {};
function b() {}
class c {
}
(0, exports.a)();
b();
new c();
let _ = {
    a: exports.a,
    b,
    c
};
a = function() {};
b = function() {};
(0, exports.a)``;
b``;
