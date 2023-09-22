const fs = require("fs")

function main() {
	const fileString = fs.readFileSync('input.txt', 'utf8');

	let floor = 0;
	let enteredBasementPos = -1;
	for (let x = 0; x < fileString.length; x++) {
		const dir = fileString[x];
		if (dir == "(") {
			floor++;
		}
		else if (dir == ")") {
			floor--;
		}

		if (enteredBasementPos == -1 && floor == -1) {
			enteredBasementPos = x + 1;
		}
	}

	console.log("Final Floor: ", floor);
	console.log("First Entered Basement: ", enteredBasementPos);
}

main()
