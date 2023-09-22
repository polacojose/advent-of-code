import fs from "fs"

function main() {

	let fileString = fs.readFileSync("input.txt", "utf8");

	part1(fileString);
	part2(fileString);
}

function part1(fileString: string) {
	const visited: Array<[number, number]> = [];
	let pos: [number, number] = [0, 0];
	visited.push([...pos]);
	for (const dir of fileString) {

		pos = adjPos(dir, pos);

		let contains = false;

		for (const v of visited) {
			if (v[0] == pos[0] && v[1] == pos[1]) {
				contains = true;
				break;
			}
		}

		if (contains) {
			continue;
		}

		visited.push([...pos]);
	}

	console.log("Visited: ", visited.length);
}

function part2(fileString: string) {
	const visited: Array<[number, number]> = [];
	let posA: [number, number] = [0, 0];
	let posB: [number, number] = [0, 0];
	visited.push([...posA]);
	for (let i = 0; i < fileString.length; i++) {
		const dir = fileString[i];

		let contains = false;
		var pos: [number, number];
		if (i % 2 == 0) {
			posA = adjPos(dir, posA);
			pos = posA;
		} else {
			posB = adjPos(dir, posB);
			pos = posB;
		}

		for (const v of visited) {
			if (v[0] == pos[0] && v[1] == pos[1]) {
				contains = true;
				break;
			}
		}

		if (contains) {
			continue;
		}

		visited.push([...pos]);
	}

	console.log("Visited: ", visited.length);
}

function adjPos(dir: string, pos: [number, number]): [number, number] {
	switch (dir) {
		case '^':
			pos[1]--;
			break;
		case '<':
			pos[0]--;
			break;
		case '>':
			pos[0]++;
			break;
		case 'v':
			pos[1]++;
			break;
	}
	return pos
}

main()
