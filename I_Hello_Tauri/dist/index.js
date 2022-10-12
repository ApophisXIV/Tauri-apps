document.querySelector("#btn-test").addEventListener("click", () => {
	alert("Hello World!");
});

const log_area = document.querySelector("#usb-log");
const port_list = document.querySelector("#ports-list");

const invoke = window.__TAURI__.invoke;

document.querySelector("#btn-test-2").addEventListener("click", () => {
	invoke("get_ports")
		.then((ports) => (port_list.innerHTML = ports.map((port) => `<option value="${port}">${port}</option>`).join("")))
		.catch((err) => alert(err));
});

document.querySelector("#open-port").addEventListener("submit", (e) => {
	e.preventDefault();
	// Al momento de pasar los argumentos estos deben estar en camelCase (Asi lo indica la documentacion)
	invoke("open_port", Object.fromEntries(new FormData(e.target)))
		.then((data) => {
			console.log(data);
			log_area.innerHTML = data;
		})
		.catch((err) => alert(err));
});
