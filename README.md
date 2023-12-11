## Build wasm

```bash
wasm-pack build --release --no-pack --no-typescript --out-dir "./extension/lib" --out-name "yare" --target web
```

## Some old boilerplate code for nice UI

```js
// Get the window and document objects.
console.log('Extension loaded');

WebAssembly.instantiateStreaming(fetch(browser.runtime.getURL('simple.wasm')), imports).then(obj => {
    console.log("WASM loaded");
});


window.addEventListener('load', () => {
	const document = window.document;

	// Get the sidebar element.
	let sidebar = document.getElementsByClassName('Layout-sidebar').item(0);
	if (!sidebar) {
		console.log('Sidebar not found');
		return;
	}

	// Create the elements.
	let row = document.createElement('div');
	row.className = 'BorderGrid-row';

	let cell = document.createElement('div');
	cell.className = 'BorderGrid-cell';

	let button = document.createElement('button');
	button.className = 'btn';
	button.textContent = 'Count code';
	button.addEventListener('click', () => {
		console.log('Counting code...');
	});

	// Append the elements to the sidebar.
	cell.appendChild(button);
	row.appendChild(cell);
	sidebar.appendChild(row);
});
``

