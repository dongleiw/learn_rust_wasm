const hello = import("../../pkg/hello");


hello.then((m)=>{
	const str = "hello rust wasm";
	console.log(`length of [${str}] is ${m.strlen(str)}`);
	m.init_wasm();
})

