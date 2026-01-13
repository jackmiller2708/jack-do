// Multi-line unused variable


// Partially used destructuring (Object)

console.log(usedObjKey);

// Partially used destructuring (Array)

console.log(usedArrEl);

// Multi-line function with unused parameters
function multiLineFunc(
    param1: string,
    
) {
    console.log(param1);
}
multiLineFunc("hello", 42);

// Nested destructuring

console.log(usedNested);

// Exported multi-line (should be kept)
export const exportedMultiLine = {
    x: 1,
    y: 2
};
