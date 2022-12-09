export { };
exports.child = child;
exports.callChild = callChild;
const child = ()=>{
    console.log("Hello World!");
};
const callChild = ()=>{
    (0, exports.child)();
};
