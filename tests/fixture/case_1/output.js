export { };
function _export(target, all) {
    for(var name in all)Object.defineProperty(target, name, {
        writable: true,
        value: all[name],
        enumerable: true,
    });
}
_export(exports, {
    child: ()=>child,
    callChild: ()=>callChild
});
const child = ()=>{
    console.log("Hello World!");
};
const callChild = ()=>{
    (0, exports.child)();
};
