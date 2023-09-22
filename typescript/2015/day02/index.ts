import fs from "fs"


function main() {

	const file = fs.readFileSync("index.txt", "utf8");

	let totalSA = 0;
	let totalRibbon = 0;
	for (const line of file.split('\n')) {
		let dia = line.split('x');
		if (dia.length < 3) {
			continue;
		}

		const [l, w, h] = parseDimensions(dia);
		const [a, b, c] = sides(l, w, h);

		const sA = 2 * a + 2 * b + 2 * c;
		const slack = Math.min(Math.min(a, b), c);

		let sidesLs = [l, w, h];
		sidesLs.sort((a, b) => a - b);
		let ribbonMain = 2 * sidesLs[0] + 2 * sidesLs[1];

		totalSA += sA + slack;
		totalRibbon += ribbonMain + l * w * h;
	}

	console.log("Total Paper: ", totalSA);
	console.log("Total Ribbon: ", totalRibbon);
}

main()
function sides(l: number, w: number, h: number): [number, number, number] {
	const a = l * w;
	const b = l * h;
	const c = w * h;
	return [a, b, c];
}

function parseDimensions(dia: string[]): [number, number, number] {
	const l = parseInt(dia[0]);
	const w = parseInt(dia[1]);
	const h = parseInt(dia[2]);
	return [l, w, h];
}

