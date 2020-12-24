const data = require("./data");

class Tile {
  constructor(x = 0, y = 0, z = 0, flipped = false) {
    this.x = x;
    this.y = y;
    this.z = z;
    this.flipped = flipped;
  }

  coords() {
    return `${this.x},${this.y},${this.z}`;
  }

  flip() {
    this.flipped = !this.flipped;
    return this;
  }

  move(dir) {
    switch (dir) {
      case "e":
        this.x += 1;
        this.y -= 1;
        break;
      case "se":
        this.z += 1;
        this.y -= 1;
        break;
      case "ne":
        this.x += 1;
        this.z -= 1;
        break;
      case "nw":
        this.y += 1;
        this.z -= 1;
        break;
      case "w":
        this.y += 1;
        this.x -= 1;
        break;
      case "sw":
        this.z += 1;
        this.x -= 1;
        break;
    }
    return this;
  }
}

function partOne() {
  const tileMap = {};
  for (let instr of data) {
    const DIR_RE = /e|se|sw|w|nw|ne/g;
    let tile = new Tile();
    let match = null;

    while ((match = DIR_RE.exec(instr)) !== null) {
      tile.move(match[0]);
    }

    const coords = tile.coords();
    if (tileMap[coords]) {
      tile = tileMap[coords];
    }
    tile.flip();
    tileMap[coords] = tile;
  }

  return Object.values(tileMap).reduce((acc, curr) => acc + (curr.flipped ? 1 : 0), 0);
}

console.log("Part 1: ", partOne());
