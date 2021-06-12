const hello = import("../../pkg/hello");

// call struct methods in rust
function call_struct_methods_in_rust(module){
	let person = null
	document.getElementById("get_person_info").onclick = ()=>{
		if(person==null){
			person = module.Person.new();
			console.log("create person", person)
		}
		console.log( person.ToString() );
	}
	document.getElementById("set_person_info").onclick = ()=>{
		if(person){
			const name = document.getElementById("input_person_name").value;
			const age = document.getElementById("input_person_age").value;
			person.SetName(name);
			person.SetAge(age);
		}
	}
}

function create_delete_element_in_rust(module){
	document.getElementById("create_element_in_rust").onclick = ()=>{
		module.create_element_and_append("2");
	}
	document.getElementById("delete_element_in_rust").onclick = ()=>{
		module.delete_element_in_container("2");
	}
}

hello.then((m)=>{
	call_struct_methods_in_rust(m);
	create_delete_element_in_rust(m);
})

