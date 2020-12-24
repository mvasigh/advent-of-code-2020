const data = require("./data");

const coords = (x, y, z) => `${x},${y},${z}`;

const cloneMap = (tileMap) =>
  Object.values(tileMap).reduce((acc, tile) => {
    acc[tile.coords()] = new Tile(tile.x, tile.y, tile.z, tile.flipped);
    return acc;
  }, {});

class Tile {
  constructor(x = 0, y = 0, z = 0, flipped = false) {
    this.x = x;
    this.y = y;
    this.z = z;
    this.flipped = flipped;
  }

  clone() {
    return new Tile(this.x, this.y, this.z, this.flipped);
  }

  coords() {
    return coords(this.x, this.y, this.z);
  }

  neighbors() {
    return [
      [this.x + 1, this.y - 1, this.z],
      [this.x, this.y - 1, this.z + 1],
      [this.x - 1, this.y, this.z + 1],
      [this.x - 1, this.y + 1, this.z],
      [this.x, this.y + 1, this.z - 1],
      [this.x + 1, this.y, this.z - 1],
    ];
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

function createTileMap() {
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

  return tileMap;
}

function shouldTileBeFlipped(tile, map) {
  const { numFlipped } = getNeighborTiles(tile, map);

  // Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
  if (tile.flipped && (numFlipped === 0 || numFlipped > 2)) {
    return false;
    // Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
  } else if (!tile.flipped && numFlipped === 2) {
    return true;
  } else {
    return tile.flipped;
  }
}

function getNeighborTiles(tile, tileMap) {
  const neighbors = tile
    .neighbors()
    .map(([x, y, z]) => tileMap[coords(x, y, z)] || new Tile(x, y, z));

  const numFlipped = neighbors.reduce((acc, t) => acc + Number(t.flipped), 0);

  return { neighbors, numFlipped };
}

function runDay(tileMap) {
  const newMap = {};

  for (let tile of Object.values(tileMap)) {
    const { neighbors } = getNeighborTiles(tile, tileMap);
    for (let _tile of [tile, ...neighbors]) {
      if (newMap[_tile.coords()]) {
        continue;
      }

      const newTile = _tile.clone();
      newTile.flipped = shouldTileBeFlipped(_tile, tileMap);
      newMap[_tile.coords()] = newTile;
    }
  }

  return newMap;
}

function partOne() {
  return Object.values(createTileMap()).reduce(
    (acc, curr) => acc + (curr.flipped ? 1 : 0),
    0
  );
}

function partTwo() {
  let tileMap = createTileMap();
  for (let i = 0; i < 100; i++) {
    tileMap = runDay(tileMap);
  }
  return Object.values(tileMap).reduce(
    (acc, curr) => acc + Number(curr.flipped),
    0
  );
}

console.log("Part 1: ", partOne());
console.log("Part 2: ", partTwo());
